use crate::system::SystemCommand;
use anyhow::Result;

pub fn install() -> Result<()> {
    println!("📦 Updating package list...");
    SystemCommand::run_checked("apt", &["update", "-qq"])?;

    println!("📦 Installing nginx...");
    SystemCommand::run_checked("apt", &["install", "-y", "-qq", "nginx"])?;

    println!("✓ Nginx installed");
    Ok(())
}

pub fn enable_and_start() -> Result<()> {
    SystemCommand::run_checked("systemctl", &["enable", "nginx"])?;
    SystemCommand::run_checked("systemctl", &["start", "nginx"])?;
    println!("✓ Nginx enabled and started");
    Ok(())
}

pub fn remove_default_site() -> Result<()> {
    let _ = SystemCommand::run("rm", &["-f", "/etc/nginx/sites-enabled/default"]);
    Ok(())
}

pub fn test_config() -> Result<()> {
    SystemCommand::run_checked("nginx", &["-t"])?;
    println!("✓ Nginx config test passed");
    Ok(())
}
