use crate::system::SystemCommand;
use anyhow::{bail, Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct DeployUser;

impl DeployUser {
    pub fn create(username: &str) -> Result<()> {
        Self::validate_username(username)?;

        if Self::exists(username)? {
            println!("✓ User '{}' already exists", username);
            return Ok(());
        }

        SystemCommand::run_checked("useradd", &["-m", "-s", "/bin/bash", username])?;
        // 默认锁定密码登录，避免弱口令风险
        SystemCommand::run_checked("passwd", &["-l", username])?;

        println!("✓ Created user '{}'", username);
        Ok(())
    }

    pub fn setup_ssh(username: &str, public_key: Option<String>) -> Result<()> {
        Self::validate_username(username)?;

        let user_home = Self::find_home_dir(username)?;
        let ssh_dir = user_home.join(".ssh");
        let auth_keys = ssh_dir.join("authorized_keys");
        let ssh_dir_str = ssh_dir.to_string_lossy().into_owned();
        let auth_keys_str = auth_keys.to_string_lossy().into_owned();

        fs::create_dir_all(&ssh_dir)?;
        SystemCommand::run_checked("chmod", &["700", &ssh_dir_str])?;

        if !auth_keys.exists() {
            fs::File::create(&auth_keys)?;
        }
        SystemCommand::run_checked("chmod", &["600", &auth_keys_str])?;

        let chown_arg = format!("{}:{}", username, username);
        SystemCommand::run_checked("chown", &["-R", &chown_arg, &ssh_dir_str])?;

        if let Some(key) = public_key {
            Self::add_ssh_key(&key, &auth_keys)?;
        } else {
            println!("⚠ No SSH key provided - you can add it to authorized_keys later");
        }

        Ok(())
    }

    fn add_ssh_key(key: &str, auth_keys: &PathBuf) -> Result<()> {
        let key = key.trim();
        if key.contains('\n') || key.contains('\r') {
            bail!("Invalid SSH key format. SSH key must be a single line");
        }
        if !ssh_key_regex().is_match(key) {
            anyhow::bail!(
                "Invalid SSH key format. Must be a valid ssh-rsa, ssh-ed25519, or ecdsa key"
            );
        }

        let mut content = fs::read_to_string(auth_keys).unwrap_or_default();
        if content.lines().any(|line| line.trim() == key) {
            println!("✓ SSH key already exists");
            return Ok(());
        }
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(key);
        content.push('\n');
        fs::write(auth_keys, content)?;

        println!("✓ SSH key added");
        Ok(())
    }

    fn exists(username: &str) -> Result<bool> {
        let output = SystemCommand::run("id", &[username])?;
        Ok(output.status.success())
    }

    pub fn validate_username(username: &str) -> Result<()> {
        if !username_regex().is_match(username) {
            bail!(
                "Invalid user name '{}'. Use lowercase letters, numbers, '_' or '-', start with letter/_ and max 32 chars",
                username
            );
        }
        Ok(())
    }

    fn find_home_dir(username: &str) -> Result<PathBuf> {
        let output = SystemCommand::run("getent", &["passwd", username])?;
        if !output.status.success() {
            bail!("Failed to resolve home directory for user '{}'", username);
        }

        let entry = String::from_utf8(output.stdout)
            .context("Failed to parse getent passwd output as UTF-8")?;
        let home_dir = entry
            .trim()
            .split(':')
            .nth(5)
            .filter(|value| !value.is_empty())
            .context("Missing home directory in passwd entry")?;

        Ok(PathBuf::from(home_dir))
    }
}

fn username_regex() -> &'static Regex {
    static USERNAME_RE: OnceLock<Regex> = OnceLock::new();
    USERNAME_RE.get_or_init(|| {
        Regex::new(r"^[a-z_][a-z0-9_-]{0,31}$").expect("username regex must be valid")
    })
}

fn ssh_key_regex() -> &'static Regex {
    static SSH_KEY_RE: OnceLock<Regex> = OnceLock::new();
    SSH_KEY_RE.get_or_init(|| {
        Regex::new(
            r"^(ssh-rsa|ssh-ed25519|ssh-ecdsa|ecdsa-sha2-nistp(?:256|384|521)) [A-Za-z0-9+/]+={0,3}( [^\r\n]+)?$",
        )
        .expect("ssh key regex must be valid")
    })
}

#[cfg(test)]
mod tests {
    use super::DeployUser;

    #[test]
    fn validate_username_accepts_common_linux_names() {
        assert!(DeployUser::validate_username("deploy").is_ok());
        assert!(DeployUser::validate_username("web_user-1").is_ok());
    }

    #[test]
    fn validate_username_rejects_invalid_values() {
        assert!(DeployUser::validate_username("-bad").is_err());
        assert!(DeployUser::validate_username("BadUpper").is_err());
        assert!(DeployUser::validate_username("name with space").is_err());
    }
}
