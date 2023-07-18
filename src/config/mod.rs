use std::fmt::{Display, Formatter};

use serde::Deserialize;

pub mod file;

#[derive(Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub log_level: String,
    pub git_bin_path: String,
    pub source: InstanceConfig,
    pub target: InstanceConfig,
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "log-level: {}, git-bin-path: '{}', source: {}, target: {}",
               self.log_level, self.git_bin_path, self.source, self.target)
    }
}

#[derive(Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct InstanceConfig {
    pub public_url: String,
    pub git_url: String,
    pub token: String
}

impl Display for InstanceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "public-url: {}, git-url: {}, token: ***********", self.public_url, self.git_url)
    }
}