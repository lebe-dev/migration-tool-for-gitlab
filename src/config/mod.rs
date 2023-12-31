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

    pub migration: MigrationConfig,

    pub error_handlers: ErrorHandlersConfig
}

impl Display for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "log-level: {}, git-bin-path: '{}', source: {}, target: {}, {}, error-handlers: {}",
               self.log_level, self.git_bin_path, self.source, self.target, self.migration, self.error_handlers)
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

#[derive(Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MigrationConfig {
    pub ignore_empty_repos: bool
}

impl Display for MigrationConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "migration.ignore-empty-repos: {}", self.ignore_empty_repos)
    }
}

#[derive(Deserialize,Debug,Clone,PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ErrorHandlersConfig {
    /// Remove repository on target GitLab instance
    /// if `clone & push` step has error(s) (permissions, connection timeouts, etc.).
    pub remove_target_repo_after_clone_error: bool
}

impl Display for ErrorHandlersConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "remove-target-repo-after-clone-error: {}", self.remove_target_repo_after_clone_error)
    }
}