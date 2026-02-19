use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::process::{Command, Output, Stdio};

pub struct SystemCommand;

impl SystemCommand {
    pub fn run(cmd: &str, args: &[&str]) -> Result<Output> {
        let resolved_cmd = Self::resolve_command(cmd);

        Command::new(&resolved_cmd)
            .args(args)
            .output()
            .with_context(|| {
                format!(
                    "Failed to execute: {} {}{}",
                    cmd,
                    args.join(" "),
                    Self::resolved_hint(cmd, &resolved_cmd)
                )
            })
    }

    pub fn run_checked(cmd: &str, args: &[&str]) -> Result<()> {
        let resolved_cmd = Self::resolve_command(cmd);
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.set_message(format!("Running: {} {}", cmd, args.join(" ")));
        spinner.enable_steady_tick(std::time::Duration::from_millis(100));

        let status = Command::new(&resolved_cmd)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .with_context(|| {
                format!(
                    "Failed to execute: {} {}{}",
                    cmd,
                    args.join(" "),
                    Self::resolved_hint(cmd, &resolved_cmd)
                )
            })?;

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

    fn resolve_command(cmd: &str) -> String {
        if cmd.contains('/') {
            return cmd.to_string();
        }

        if Self::is_command_in_path(cmd) {
            return cmd.to_string();
        }

        for dir in [
            "/usr/local/sbin",
            "/usr/sbin",
            "/sbin",
            "/usr/local/bin",
            "/usr/bin",
            "/bin",
        ] {
            let candidate = Path::new(dir).join(cmd);
            if candidate.is_file() {
                return candidate.to_string_lossy().into_owned();
            }
        }

        cmd.to_string()
    }

    fn is_command_in_path(cmd: &str) -> bool {
        std::env::var_os("PATH")
            .map(|paths| std::env::split_paths(&paths).any(|dir| dir.join(cmd).is_file()))
            .unwrap_or(false)
    }

    fn resolved_hint(original: &str, resolved: &str) -> String {
        if original == resolved {
            String::new()
        } else {
            format!(" (resolved to: {})", resolved)
        }
    }
}
