use anyhow::{bail, Result};
use std::{fs, path::Path};

pub fn install_main_config() -> Result<()> {
    let config = crate::template::get_template("nginx.conf")?;
    fs::write("/etc/nginx/nginx.conf", config)?;
    println!("✓ Nginx main config installed");
    Ok(())
}

pub fn install_app_config(project_name: &str, server_name: &str, project_root: &str) -> Result<()> {
    validate_project_name(project_name)?;
    validate_server_name(server_name)?;
    validate_project_root(project_root)?;

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

pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("项目名称不能为空");
    }
    if name.len() > 64 {
        bail!("项目名称过长（最多 64 个字符）");
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        bail!("项目名称仅允许字母、数字、-、_");
    }
    Ok(())
}

fn validate_server_name(value: &str) -> Result<()> {
    if value.is_empty() {
        bail!("站点域名或 IP 不能为空");
    }
    if value.len() > 253 {
        bail!("站点域名或 IP 过长");
    }
    if !value.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ':' || c == '*'
    }) {
        bail!("站点域名或 IP 仅允许字母、数字、.、-、_、:、*");
    }
    Ok(())
}

fn validate_project_root(path: &str) -> Result<()> {
    if path.is_empty() {
        bail!("项目根目录不能为空");
    }
    if path.len() > 512 {
        bail!("项目根目录过长");
    }
    if !path
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '/' || c == '.' || c == '-' || c == '_')
    {
        bail!("项目根目录仅允许字母、数字、/、.、-、_");
    }
    let parsed = Path::new(path);
    if !parsed.is_absolute() {
        bail!("项目根目录必须是绝对路径");
    }
    for component in parsed.components() {
        match component {
            std::path::Component::RootDir | std::path::Component::Normal(_) => {}
            _ => bail!("项目根目录不允许包含 . 或 .."),
        }
    }
    Ok(())
}
