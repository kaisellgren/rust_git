use super::config_level::ConfigLevel;

pub struct ConfigEntry {
    pub category: String,
    pub key:      String,
    pub value:    String,
    pub level:    ConfigLevel,
}