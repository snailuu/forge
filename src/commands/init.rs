use crate::{nginx, system::SystemCommand, user::DeployUser};
use anyhow::Result;
use colored::Colorize;
use dialoguer::{Confirm, Input};

pub fn run(
    user: Option<String>,
    project: Option<String>,
    domain: String,
    ssh_key: Option<String>,
) -> Result<()> {
    println!("{}", "==========================================".cyan());
    println!("{}", "  Server Initialization".cyan().bold());
    println!("{}", "==========================================".cyan());
    println!();

    SystemCommand::check_linux()?;
    SystemCommand::check_root()?;

    // 用户创建逻辑
    let username = if let Some(name) = user {
        // 命令行参数提供了用户名，直接使用
        Some(name)
    } else {
        // 交互式询问
        let create_user = Confirm::new()
            .with_prompt("Create deploy user?")
            .default(false)
            .interact()?;

        if create_user {
            Some(
                Input::<String>::new()
                    .with_prompt("User name")
                    .default("deploy".to_string())
                    .interact_text()?,
            )
        } else {
            None
        }
    };

    // 项目创建逻辑
    let project_name = if let Some(name) = project {
        // 命令行参数提供了项目名，验证后使用
        nginx::config::validate_project_name(&name)?;
        Some(name)
    } else {
        // 交互式询问
        let create_project = Confirm::new()
            .with_prompt("Create project directory in /var/www?")
            .default(false)
            .interact()?;

        if create_project {
            let name = Input::<String>::new()
                .with_prompt("Project name")
                .interact_text()?;
            nginx::config::validate_project_name(&name)?;
            Some(name)
        } else {
            None
        }
    };

    // 域名配置
    let server_name = if project_name.is_some() {
        // 如果创建了项目，使用提供的 domain 参数
        domain
    } else {
        // 如果没有创建项目，域名无意义，使用默认值
        if domain != "localhost" {
            println!(
                "{}",
                "⚠ --domain provided but no project created, domain was ignored".yellow()
            );
        }
        "localhost".to_string()
    };

    if let Some(ref user) = username {
        DeployUser::validate_username(user)?;
    }

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
                Some(
                    Input::<String>::new()
                        .with_prompt("SSH public key (ssh-rsa/ssh-ed25519/ecdsa-sha2-nistp256)")
                        .interact_text()?,
                )
            } else {
                None
            }
        };

        DeployUser::setup_ssh(user, ssh_key_to_use)?;

        // 确保 /var/www 目录存在
        std::fs::create_dir_all("/var/www")?;
        println!("✓ Web directory configured");
    } else if ssh_key.is_some() {
        println!(
            "{}",
            "⚠ --ssh-key provided but no user created, SSH key was ignored".yellow()
        );
    }

    if let Some(name) = &project_name {
        let project_path = format!("/var/www/{}", name);
        std::fs::create_dir_all(&project_path)?;

        // 创建默认 index.html
        let template = crate::template::get_template("index.html.template")?;
        let index_html = template.replace("{{PROJECT_NAME}}", name);
        std::fs::write(format!("{}/index.html", project_path), index_html)?;

        if let Some(ref user) = username {
            SystemCommand::run_checked(
                "chown",
                &["-R", &format!("{}:{}", user, user), &project_path],
            )?;
        } else {
            println!(
                "{}",
                "⚠ No deploy user specified, project directory is owned by root".yellow()
            );
        }
        println!("✓ Project directory created: {}", project_path);
    }

    nginx::install::remove_default_site()?;
    nginx::config::install_main_config()?;
    if let Some(name) = &project_name {
        let project_path = format!("/var/www/{}", name);
        nginx::config::install_app_config(name, &server_name, &project_path)?;
    }
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
        println!("Server name: {}", server_name);
    }
    println!();

    Ok(())
}
