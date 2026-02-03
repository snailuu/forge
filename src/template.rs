use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn get_template(name: &str) -> Result<String> {
    // 优先从环境变量指定的目录读取
    if let Ok(template_dir) = std::env::var("FORGE_TEMPLATE_DIR") {
        let template_path = PathBuf::from(template_dir).join(name);
        if template_path.exists() {
            return Ok(fs::read_to_string(template_path)?);
        }
    }

    // 回退到嵌入的默认模板
    match name {
        "nginx.conf" => Ok(include_str!("../templates/nginx.conf").to_string()),
        "app.conf.template" => Ok(include_str!("../templates/app.conf.template").to_string()),
        "index.html.template" => Ok(include_str!("../templates/index.html.template").to_string()),
        _ => anyhow::bail!("Unknown template: {}", name),
    }
}
