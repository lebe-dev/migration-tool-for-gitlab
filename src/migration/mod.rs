use anyhow::Context;
use log::{error, info};
use reqwest::blocking::{Client, ClientBuilder};

use crate::config::InstanceConfig;
use crate::git::clone::copy_git_repo_from_one_remote_to_another;
use crate::migration::domain::GitLabGroup;
use crate::migration::group::{create_gitlab_private_group, get_group_list};
use crate::migration::project::{create_gitlab_private_project, get_project_branches, get_project_list};

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

    create_groups_on_target_instance(&client, &source_instance_groups, &target)?;

    info!("received groups from source instance:");
    info!("{:?}", source_instance_groups);

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    let target_projects = get_project_list(&client, &target)
        .context("cannot get gitlab project list from target instance")?;

    let mut progress = 0;

    for source_project in &source_projects {
        info!("source project '{}'", source_project.name);

        let source_group_found = &source_instance_groups.iter().find(|sg|sg.id == source_project.namespace.id);

        match source_group_found {
            Some(source_group) => {
                let target_project_exists = target_projects.iter()
                    .find(|tp|
                        tp.name == source_project.name && tp.namespace.name == source_group.name &&
                        tp.namespace.full_path == source_group.full_path
                    );

                match target_project_exists {
                    None => {
                        info!("project '{}' wasn't found on target instance", source_project.name);

                        let target_group_found = target_instance_groups.iter()
                            .find(|tg| tg.name == source_group.name &&
                                tg.full_path == source_group.full_path);

                        match target_group_found {
                            Some(target_group) => {
                                let source_project_branches = get_project_branches(
                                    &client, &source, source_project.id)
                                    .context("unable to get source project branches")?;

                                create_gitlab_private_project(
                                    &client, &target, target_group.id,
                                    &source_project.name, &source_project.path
                                ).context("cannot create project on target instance")?;

                                if !source_project_branches.is_empty() {
                                    copy_git_repo_from_one_remote_to_another(
                                        &source_project.name, &source.git_url,
                                        &source_group.full_path, &target_group.full_path,
                                        &target.git_url
                                    ).context("unable to clone repo from source to destination")?;
                                }

                            }
                            None => error!("unexpected error, target group wasn't found")
                        }

                    }
                    Some(_) => info!("project '{}' already migrated, skip", source_project.name)
                }
            }
            None => error!("source group wasn't found by id {}", source_project.namespace.id)
        }

        progress += 1;
        info!("migration progress: {progress}/{}", source_projects.len())
    }

    Ok(())
}

fn create_groups_on_target_instance(client: &Client, source_groups: &Vec<GitLabGroup>,
                                    target: &InstanceConfig) -> anyhow::Result<()> {
    info!("creating groups on target instance..");

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    let parent_groups: Vec<&GitLabGroup> = source_groups.iter()
        .filter(|sg|sg.parent_id.is_none()).collect();

    for parent_group in parent_groups.iter() {

        let group_found = target_instance_groups.iter()
            .find(|tg|tg.full_path == parent_group.full_path);

        match group_found {
            None => {
                create_gitlab_private_group(
                    &client, &target, &parent_group.name,
                    &parent_group.path, None
                ).context("cannot create parent group")?;
            }
            Some(_) => {}
        }
    }

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    let non_parent_groups: Vec<&GitLabGroup> = source_groups.iter()
        .filter(|sg|sg.parent_id.is_some()).collect();

    for non_parent_group in non_parent_groups.iter() {

        let group_found = target_instance_groups.iter()
            .find(|tg|tg.full_path == non_parent_group.full_path);

        match group_found {
            None => {
                let parent_group_found = parent_groups.iter()
                    .find(|spg|
                        spg.id == non_parent_group.parent_id.unwrap_or(0));

                if let Some(parent_group) = parent_group_found {

                    let target_group_found = target_instance_groups.iter()
                        .find(|tig|tig.full_path == parent_group.full_path);

                    match target_group_found {
                        Some(target_group) => {

                            create_gitlab_private_group(
                                &client, &target, &non_parent_group.name,
                                &non_parent_group.path, Some(target_group.id)
                            ).context("cannot create gitlab group on target instance")?;

                        }
                        None => {}
                    }

                } else {
                    error!("parent group wasn't found by id {}", non_parent_group.parent_id.unwrap_or(0))
                }
            }
            Some(_) => {}
        }
    }

    Ok(())
}