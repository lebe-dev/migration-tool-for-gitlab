use std::path::Path;

use anyhow::Context;
use config::Config;
use log::info;

use crate::config::AppConfig;

pub fn load_config_from_file(config_path: &Path) -> anyhow::Result<AppConfig> {
    info!("load config from file: '{}'", config_path.display());

    let config_path = format!("{}", config_path.display());

    let settings = Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build().context("unable to load app config from file")?;

    let config = settings.try_deserialize::<AppConfig>()?;

    info!("config:");
    info!("{}", config);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::config::{AppConfig, ErrorHandlersConfig, InstanceConfig};
    use crate::config::file::load_config_from_file;
    use crate::tests::init_logging;

    #[test]
    fn config_should_be_loaded() {
        init_logging();

        let config_path = Path::new("test-data").join("gmt.yml");
        let config = load_config_from_file(&config_path).unwrap();

        let expected_config = AppConfig {
            log_level: "debug".to_string(),

            git_bin_path: "/usr/bin/git".to_string(),

            source: InstanceConfig {
                public_url: "https://old-gitlab.company.com".to_string(),
                git_url: "ssh://old-gitlab.company.com:2222".to_string(),
                token: "38jg983j4g0922323f".to_string(),
            },

            target: InstanceConfig {
                public_url: "https://gitlab.company.com".to_string(),
                git_url: "ssh://gitlab.company.com".to_string(),
                token: "Fv034g3049gj290j23A".to_string(),
            },

            error_handlers: ErrorHandlersConfig {
                remove_target_repo_after_clone_error: true,
            },
        };

        assert_eq!(expected_config, config)
    }
}