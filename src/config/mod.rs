use std::fmt::{Display, Formatter};

use serde::Deserialize;

pub mod file;

#[derive(Deserialize,Debug,Clone)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub log_level: String,
    pub source: InstanceConfig,
    pub target: InstanceConfig,
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "log-level: {}, source: {}, target: {}",
               self.log_level, self.source, self.target)
    }
}

#[derive(Deserialize,Debug,Clone)]
pub struct InstanceConfig {
    pub host: String,
    pub token: String
}

impl Display for InstanceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "host: {}, token: ***********", self.host)
    }
}