use std::process::{Command, Output, Stdio};
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};

pub struct SystemCommand;

impl SystemCommand {
    pub fn run(cmd: &str, args: &[&str]) -> Result<Output> {
        Command::new(cmd)
            .args(args)
            .output()
            .with_context(|| format!("Failed to execute: {} {}", cmd, args.join(" ")))
    }

    pub fn run_checked(cmd: &str, args: &[&str]) -> Result<()> {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap()
        );
        spinner.set_message(format!("Running: {} {}", cmd, args.join(" ")));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        let status = Command::new(cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| format!("Failed to execute: {} {}", cmd, args.join(" ")))?;

        spinner.finish_and_clear();

        if !status.success() {
            anyhow::bail!("Command failed: {} {}", cmd, args.join(" "));
        }

        Ok(())
    }

    pub fn check_root() -> Result<()> {
        let output = Self::run("id", &["-u"])?;
        let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if uid != "0" {
            anyhow::bail!("This command requires root privileges. Run with sudo.");
        }
        Ok(())
    }

    pub fn check_linux() -> Result<()> {
        if !cfg!(target_os = "linux") {
            anyhow::bail!(
                "This tool only works on Linux systems.\n\
                Current OS: {}\n\
                Please run this on an Ubuntu/Debian server.",
                std::env::consts::OS
            );
        }
        Ok(())
    }
}
