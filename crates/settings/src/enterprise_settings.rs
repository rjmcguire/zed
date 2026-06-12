use crate::{self as settings, Settings, SettingsContent};
use settings::RegisterSetting;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, RegisterSetting)]
pub struct EnterpriseSettings {
    pub enabled: bool,
    pub allowed_extensions: Option<Vec<String>>,
    pub allowed_ai_providers: Option<Vec<String>>,
    pub organization_id: Option<String>,
    pub audit_url: Option<String>,
}

impl Settings for EnterpriseSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let content = &content.enterprise;
        Self {
            enabled: content.as_ref().and_then(|c| c.enabled).unwrap_or(false),
            allowed_extensions: content.as_ref().and_then(|c| c.allowed_extensions.clone()),
            allowed_ai_providers: content.as_ref().and_then(|c| c.allowed_ai_providers.clone()),
            organization_id: content.as_ref().and_then(|c| c.organization_id.clone()),
            audit_url: content.as_ref().and_then(|c| c.audit_url.clone()),
        }
    }
}
