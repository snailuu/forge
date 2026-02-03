use anyhow::Result;
use std::fs;

pub fn install_main_config() -> Result<()> {
    let config = crate::template::get_template("nginx.conf")?;
    fs::write("/etc/nginx/nginx.conf", config)?;
    println!("✓ Nginx main config installed");
    Ok(())
}
