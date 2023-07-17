use anyhow::Context;
use log::{error, info};
use reqwest::blocking::ClientBuilder;

use crate::config::InstanceConfig;
use crate::git::clone::copy_git_repo_from_one_remote_to_another;
use crate::migration::group::{create_gitlab_private_group, get_group_list};
use crate::migration::project::{create_gitlab_private_project, get_project_list};

pub mod domain;
pub mod group;
pub mod project;

pub const PRIVATE_TOKEN_HEADER: &str = "PRIVATE-TOKEN";

pub fn migrate_gitlab_instance(source: &InstanceConfig, target: &InstanceConfig) -> anyhow::Result<()> {
    info!("migrating groups and projects from '{}' to '{}'..", source.public_url, target.public_url);

    let client = ClientBuilder::new().build().unwrap();

    let source_instance_groups = get_group_list(&client, &source)
        .context("unable to get gitlab groups from source instance")?;

    let source_projects = get_project_list(&client, &source)
        .context("cannot get gitlab project list from source instance")?;

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    info!("received groups from source instance:");
    info!("{:?}", source_instance_groups);

    for group in &source_instance_groups {
        info!("- group (id {}): '{}'", group.id, group.name);

        let target_group_found = target_instance_groups.iter()
                                                    .find(|gg| gg.name == group.name);

        match target_group_found {
            Some(_) => {}
            None => {
                create_gitlab_private_group(&client, &target, &group.name, &group.path)
                                        .context("cannot create gitlab group on target instance")?;
            }
        }

    }

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    let target_projects = get_project_list(&client, &target)
        .context("cannot get gitlab project list from target instance")?;

    for source_project in source_projects {
        info!("source project '{}'", source_project.name);

        let source_group_found = &source_instance_groups.iter().find(|sg|sg.id == source_project.namespace.id);

        match source_group_found {
            Some(source_group) => {
                let target_project_exists = target_projects.iter()
                    .find(|tp|
                        tp.name == source_project.name && tp.namespace.name == source_group.name &&
                        tp.namespace.path == source_group.path
                    );

                match target_project_exists {
                    None => {

                        let target_group_found = target_instance_groups.iter()
                            .find(|tg| tg.name == source_group.name &&
                                tg.path == source_group.path);

                        match target_group_found {
                            Some(target_group) => {
                                create_gitlab_private_project(
                                    &client, &target, target_group.id,
                                    &source_project.name, &source_project.path
                                ).context("cannot create project on target instance")?;

                                copy_git_repo_from_one_remote_to_another(
                                    &source_project.name, &source.git_url,
                            &source_group.path, &target_group.path,
                                    &target.git_url
                                ).context("unable to clone repo from source to destination")?;

                            }
                            None => {}
                        }

                    }
                    Some(_) => info!("project '{}' already migrated, skip", source_project.name)
                }
            }
            None => {
                error!("source group wasn't found by id {}", sg.id)
            }
        }

    }

    Ok(())
}