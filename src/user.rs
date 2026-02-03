use crate::system::SystemCommand;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub struct DeployUser;

impl DeployUser {
    pub fn create(username: &str) -> Result<()> {
        if Self::exists(username)? {
            println!("✓ User '{}' already exists", username);
            return Ok(());
        }

        SystemCommand::run_checked("useradd", &["-m", "-s", "/bin/bash", username])?;

        let mut child = std::process::Command::new("chpasswd")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        use std::io::Write;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(format!("{}:{}123\n", username, username).as_bytes())?;
        }
        child.wait()?;

        println!("✓ Created user '{}'", username);
        Ok(())
    }

    pub fn setup_ssh(username: &str, public_key: Option<String>) -> Result<()> {
        let ssh_dir = PathBuf::from(format!("/home/{}/.ssh", username));
        let auth_keys = ssh_dir.join("authorized_keys");

        fs::create_dir_all(&ssh_dir)?;
        SystemCommand::run_checked("chmod", &["700", ssh_dir.to_str().unwrap()])?;

        fs::File::create(&auth_keys)?;
        SystemCommand::run_checked("chmod", &["600", auth_keys.to_str().unwrap()])?;

        let chown_arg = format!("{}:{}", username, username);
        SystemCommand::run_checked("chown", &["-R", &chown_arg, ssh_dir.to_str().unwrap()])?;

        if let Some(key) = public_key {
            Self::add_ssh_key(&key, &auth_keys)?;
        } else {
            println!("⚠ No SSH key provided - you can add it later");
        }

        Ok(())
    }

    fn add_ssh_key(key: &str, auth_keys: &PathBuf) -> Result<()> {
        let re = Regex::new(r"^ssh-(rsa|ed25519|ecdsa)")?;
        if !re.is_match(key) {
            anyhow::bail!("Invalid SSH key format. Must start with ssh-rsa, ssh-ed25519, or ssh-ecdsa");
        }

        let mut content = fs::read_to_string(auth_keys).unwrap_or_default();
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
}
