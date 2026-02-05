use anyhow::Result;
use std::fs;

pub fn install_main_config() -> Result<()> {
    let config = crate::template::get_template("nginx.conf")?;
    fs::write("/etc/nginx/nginx.conf", config)?;
    println!("✓ Nginx main config installed");
    Ok(())
}

pub fn install_app_config(project_name: &str, server_name: &str, project_root: &str) -> Result<()> {
    let config = crate::template::get_template("app.conf.template")?;
    let config = config
        .replace("{{PROJECT_NAME}}", project_name)
        .replace("{{PROJECT_ROOT}}", project_root)
        .replace("{{SERVER_NAME}}", server_name);

    fs::create_dir_all("/etc/nginx/conf.d")?;
    let conf_path = format!("/etc/nginx/conf.d/{}.conf", project_name);
    fs::write(&conf_path, config)?;
    println!("✓ Nginx app config installed: {}", conf_path);
    Ok(())
}
