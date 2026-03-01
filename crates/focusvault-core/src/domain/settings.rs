use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GlobalSettings {
    pub default_tab_limit: i32,
    pub daily_vault_capture_cap: Option<i32>,
    pub cleanup_prompt_hours: i32,
    pub enable_daily_reminders: bool,
    pub timezone: String,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            default_tab_limit: 10,
            daily_vault_capture_cap: None,
            cleanup_prompt_hours: 48,
            enable_daily_reminders: false,
            timezone: "UTC".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeviceSettings {
    pub desktop_mode: bool,
    pub local_db_path: Option<String>,
}

impl Default for DeviceSettings {
    fn default() -> Self {
        Self {
            desktop_mode: false,
            local_db_path: None,
        }
    }
}
