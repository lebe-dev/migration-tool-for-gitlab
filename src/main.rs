use std::path::Path;
use std::process::exit;

use clap::{App, SubCommand};
use reqwest::blocking::ClientBuilder;

use crate::config::file::load_config_from_file;
use crate::diag::empty::get_empty_projects;
use crate::logging::get_logging_config;
use crate::migration::migrate_gitlab_instance;

pub mod config;
pub mod logging;
pub mod git;
pub mod migration;
pub mod diag;

#[cfg(test)]
pub mod tests;

const MIGRATE_COMMAND: &str = "migrate";
const SHOW_EMPTY_COMMAND: &str = "show-empty";

const EXIT_CODE_ERROR: i32 = -1;

fn main() {
    let matches = App::new("Migration Tool for GitLab")
        .version("0.2.0")
        .author("Eugene Lebedev <eugene.0x90@gmail.com>")
        .about("Migrate groups and projects from one GitLab instance to another.")
        .subcommand(SubCommand::with_name(MIGRATE_COMMAND)
            .about("Migrate groups and projects from source GitLab instance to target instance")
        )
        .subcommand(SubCommand::with_name(SHOW_EMPTY_COMMAND)
            .about("Show projects without branches from source GitLab instance")
        )
        .get_matches();

    let config_file_path = Path::new("gmt.yml");

    match load_config_from_file(&config_file_path) {
        Ok(app_config) => {

            let logging_config = get_logging_config(&app_config.log_level);
            match log4rs::init_config(logging_config) {
                Ok(_) => {

                    match matches.subcommand_matches(MIGRATE_COMMAND) {
                        Some(_) => {
                            println!("migrating..");

                            match migrate_gitlab_instance(&app_config.source, &app_config.target,
                                                          &app_config.error_handlers) {
                                Ok(_) => {
                                    println!("-----");
                                    println!("migration completed");
                                }
                                Err(e) => {
                                    eprintln!("migration error: {}", e);
                                    eprintln!("details: {}", e.root_cause());
                                    exit(EXIT_CODE_ERROR);
                                }
                            }

                        }
                        None => {}
                    }

                    match matches.subcommand_matches(SHOW_EMPTY_COMMAND) {
                        Some(_) => {
                            println!("gathering data..");

                            let client = ClientBuilder::new().build().unwrap();

                            match get_empty_projects(&client, &app_config.source) {
                                Ok(empty_projects) => {
                                    println!("empty projects at '{}':", app_config.source.public_url);

                                    if !empty_projects.is_empty() {
                                        for empty_project in empty_projects {
                                            println!(" - '{}'", empty_project.path);
                                        }

                                    } else {
                                        println!("no empty projects found");
                                    }

                                }
                                Err(e) => {
                                    eprintln!("migration error: {}", e);
                                    eprintln!("details: {}", e.root_cause());
                                    exit(EXIT_CODE_ERROR);
                                }
                            }

                        }
                        None => {}
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
