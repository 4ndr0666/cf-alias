use std::fs;

use anyhow::anyhow;
use anyhow::Result;
use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    pub cloudflare_account_id: String,
    pub cloudflare_forward_email: String,
    pub cloudflare_root_domain: String,
    pub cloudflare_token: String,
    pub cloudflare_zone: String,
}

pub fn load_config() -> Result<Config> {
    let dir_path = dirs::home_dir().unwrap().join(".cf-alias");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let file_path = dir_path.join("config.json");
    if !file_path.exists() {
        return Err(anyhow!(
            "Configuration file not found. Please create ~/.cf-alias/config.json and add your Cloudflare details. Refer to the documentation for more information."
        ));
    }

    let config_str = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&config_str)?;

    if config.cloudflare_account_id.is_empty()
        || config.cloudflare_forward_email.is_empty()
        || config.cloudflare_root_domain.is_empty()
        || config.cloudflare_token.is_empty()
        || config.cloudflare_zone.is_empty()
    {
        return Err(anyhow!(
            "Configuration file is empty. Please edit ~/.cf-alias/config.json and add your Cloudflare details."
        ));
    }

    return Ok(config);
}
