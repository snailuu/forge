use crate::{nginx, system::SystemCommand, user::DeployUser};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Input, Confirm};

pub fn run(ssh_key: Option<String>) -> Result<()> {
    println!("{}", "==========================================".cyan());
    println!("{}", "  Server Initialization".cyan().bold());
    println!("{}", "==========================================".cyan());
    println!();

    SystemCommand::check_linux()?;
    SystemCommand::check_root()?;

    // 交互式配置
    let create_user = Confirm::new()
        .with_prompt("Create deploy user?")
        .default(false)
        .interact()?;

    let username = if create_user {
        Some(Input::<String>::new()
            .with_prompt("User name")
            .default("deploy".to_string())
            .interact_text()?)
    } else {
        None
    };

    let create_project = Confirm::new()
        .with_prompt("Create project directory in /var/www?")
        .default(false)
        .interact()?;

    let project_name = if create_project {
        Some(Input::<String>::new()
            .with_prompt("Project name")
            .interact_text()?)
    } else {
        None
    };

    let server_name = if create_project {
        Some(Input::<String>::new()
            .with_prompt("站点域名或 IP")
            .default("localhost".to_string())
            .interact_text()?)
    } else {
        None
    };

    println!();

    nginx::install::install()?;

    if let Some(ref user) = username {
        DeployUser::create(user)?;

        // SSH 密钥配置
        let ssh_key_to_use = if let Some(key) = ssh_key {
            Some(key)
        } else {
            let add_ssh_key = Confirm::new()
                .with_prompt("Add SSH public key for passwordless login?")
                .default(false)
                .interact()?;

            if add_ssh_key {
                Some(Input::<String>::new()
                    .with_prompt("SSH public key (ssh-rsa/ssh-ed25519/ssh-ecdsa)")
                    .interact_text()?)
            } else {
                None
            }
        };

        DeployUser::setup_ssh(user, ssh_key_to_use)?;

        SystemCommand::run_checked("chown", &["-R", &format!("{}:{}", user, user), "/var/www"])?;
        println!("✓ Web directory configured");
    }

    if let Some(name) = &project_name {
        let project_path = format!("/var/www/{}", name);
        std::fs::create_dir_all(&project_path)?;

        // 创建默认 index.html
        let template = crate::template::get_template("index.html.template")?;
        let index_html = template.replace("{{PROJECT_NAME}}", name);
        std::fs::write(format!("{}/index.html", project_path), index_html)?;

        if let Some(ref user) = username {
            SystemCommand::run_checked("chown", &["-R", &format!("{}:{}", user, user), &project_path])?;
        }
        println!("✓ Project directory created: {}", project_path);

        let server_name = server_name.as_deref().unwrap_or("localhost");
        nginx::config::install_app_config(name, server_name, &project_path)?;
    }

    nginx::install::remove_default_site()?;
    nginx::config::install_main_config()?;
    nginx::install::enable_and_start()?;

    println!();
    println!("{}", "==========================================".green());
    println!("{}", "  ✅ Initialization Complete!".green().bold());
    println!("{}", "==========================================".green());
    println!();
    if let Some(user) = username {
        println!("Deploy user: {}", user);
    }
    println!("Web root: /var/www");
    if let Some(name) = project_name {
        println!("Project directory: /var/www/{}", name);
    }
    println!();

    Ok(())
}
