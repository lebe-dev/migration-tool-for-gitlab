use std::path::Path;
use std::process::exit;

use clap::{App, SubCommand};
use reqwest::blocking::ClientBuilder;

use crate::config::file::load_config_from_file;
use crate::logging::get_logging_config;
use crate::migration::group::get_group_list;

pub mod config;
pub mod logging;
pub mod migration;

#[cfg(test)]
pub mod tests;

const MIGRATE_COMMAND: &str = "migrate";

const EXIT_CODE_ERROR: i32 = -1;

fn main() {
    let matches = App::new("Migration Tool for GitLab")
        .version("0.1.0")
        .author("Eugene Lebedev <eugene.0x90@gmail.com>")
        .about("Migrate groups and projects from one GitLab instance to another.")
        .subcommand(SubCommand::with_name(MIGRATE_COMMAND)
            .about("Migrate groups and projects from source GitLab instance to target instance")
        )
        .get_matches();

    let config_file_path = Path::new("gmt.yml");

    match load_config_from_file(&config_file_path) {
        Ok(app_config) => {

            let logging_config = get_logging_config(&app_config.log_level);
            match log4rs::init_config(logging_config) {
                Ok(_) => {

                    let client = ClientBuilder::new().build().unwrap();

                    match get_group_list(&client, &app_config.source) {
                        Ok(groups) => {
                            for group in groups {
                                println!("- group (id {}): '{}'", group.id, group.name)
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            exit(EXIT_CODE_ERROR);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    exit(EXIT_CODE_ERROR);
                }
            }

        }
        Err(e) => {
            eprintln!("unable to load app config: {}", e);
            exit(EXIT_CODE_ERROR)
        }
    }
}
