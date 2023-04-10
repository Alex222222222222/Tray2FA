use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use yewdux::store::Store;

#[allow(dead_code)] // TODO remove allow dead_code
pub static I18N: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(Languages::EnGb.get_language(), Languages::EnGb.to_label());
    map.insert(Languages::En.get_language(), Languages::En.to_label());
    map.insert(Languages::ZhCn.get_language(), Languages::ZhCn.to_label());
    map
});

#[derive(Clone, Debug, Store, PartialEq, Deserialize, Serialize, Default)]
#[store(storage = "local")]
pub struct LanguagesConfigState {
    pub config: Languages,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Default)]
pub enum Languages {
    #[default]
    EnGb,
    En,
    ZhCn,
}

impl Languages {
    pub fn get_language(&self) -> String {
        match self {
            Languages::EnGb => "en-GB".to_string(),
            Languages::En => "en".to_string(),
            Languages::ZhCn => "zh-CN".to_string(),
        }
    }

    pub fn to_label(&self) -> String {
        match self {
            Languages::EnGb => "English (UK)".to_string(),
            Languages::En => "English".to_string(),
            Languages::ZhCn => "中文".to_string(),
        }
    }
}

impl Display for Languages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_label())
    }
}

impl From<String> for Languages {
    fn from(s: String) -> Self {
        match s.as_str() {
            "en-GB" => Languages::EnGb,
            "en" => Languages::En,
            "zh-CN" => Languages::ZhCn,
            _ => Languages::default(),
        }
    }
}
