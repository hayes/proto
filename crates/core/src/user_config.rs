use crate::{errors::ProtoError, helpers::get_root, plugin::PluginLocator};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use starbase_utils::toml;

pub const USER_CONFIG_NAME: &str = "config.toml";

#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UserConfig {
    pub auto_clean: bool,
    pub auto_install: bool,
    pub plugins: FxHashMap<String, PluginLocator>,
}

impl UserConfig {
    #[tracing::instrument(skip_all)]
    pub fn load() -> Result<Self, ProtoError> {
        let path = get_root()?.join(USER_CONFIG_NAME);

        if !path.exists() {
            return Ok(UserConfig::default());
        }

        let config: UserConfig = toml::read_file(&path)?;

        Ok(config)
    }
}
