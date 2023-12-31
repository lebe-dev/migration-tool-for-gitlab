use anyhow::Context;
use log::{error, info};
use reqwest::blocking::{Client, ClientBuilder};

use crate::config::{ErrorHandlersConfig, InstanceConfig, MigrationConfig};
use crate::git::clone::copy_git_repo_from_one_remote_to_another;
use crate::migration::domain::GitLabGroup;
use crate::migration::group::{create_gitlab_private_group, get_all_groups};
use crate::migration::project::{create_gitlab_private_project, get_all_projects, get_project_branches, remove_gitlab_project};

pub mod domain;
pub mod group;
pub mod project;

pub const PRIVATE_TOKEN_HEADER: &str = "PRIVATE-TOKEN";

pub fn migrate_gitlab_instance(source: &InstanceConfig, target: &InstanceConfig,
                               migration_config: &MigrationConfig,
                               error_handlers: &ErrorHandlersConfig) -> anyhow::Result<()> {
    info!("migrating groups and projects from '{}' to '{}'..", source.public_url, target.public_url);

    let client = ClientBuilder::new().build().unwrap();

    let source_instance_groups = get_all_groups(&client, &source)
        .context("unable to get gitlab groups from source instance")?;

    let source_projects = get_all_projects(&client, &source)
        .context("cannot get gitlab project list from source instance")?;

    create_groups_on_target_instance(&client, &source_instance_groups, &target)?;

    info!("received groups from source instance:");
    info!("{:?}", source_instance_groups);

    let target_instance_groups = get_all_groups(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    let target_projects = get_all_projects(&client, &target)
        .context("cannot get gitlab project list from target instance")?;

    let mut progress = 0;

    for source_project in &source_projects {
        info!("source project '{}'", source_project.name);

        let source_group_found = &source_instance_groups.iter()
            .find(|sg|sg.id == source_project.namespace.id);

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

                                if is_migration_allowed(migration_config.ignore_empty_repos,
                                        source_project_branches.is_empty()) {

                                    let new_projects = create_gitlab_private_project(
                                        &client, &target, target_group.id,
                                        &source_project.name, &source_project.path
                                    ).context("cannot create project on target instance")?;

                                    if !source_project_branches.is_empty() {
                                        match copy_git_repo_from_one_remote_to_another(
                                            &source_project.path, &source.git_url,
                                            &source_group.full_path, &target_group.full_path,
                                            &target.git_url
                                        ) {
                                            Ok(_) => {}
                                            Err(e) => {
                                                error!("repo copy error: {}", e);
                                                error!("{}", e.root_cause());

                                                if error_handlers.remove_target_repo_after_clone_error {
                                                    let target_project = new_projects.first().unwrap();
                                                    info!("removing target repo '{}' after git clone/push error(s)..", target_project.path);
                                                    remove_gitlab_project(&client, &target, target_project.id)
                                                        .context("unable to remove repository on target instance")?;
                                                }

                                                break;
                                            }
                                        }
                                    }

                                } else {
                                    info!("migrate is not allowed for empty repo '{}'", source_project.path)
                                }

                            }
                            None => error!("unexpected error, target group wasn't found")
                        }

                    }
                    Some(_) => info!("project '{}' already migrated, skip", source_project.path)
                }
            }
            None => error!("source group wasn't found by id {}", source_project.namespace.id)
        }

        progress += 1;
        info!("migration progress: {progress}/{}", source_projects.len())
    }

    Ok(())
}

fn is_migration_allowed(ignore_empty_repos: bool, source_project_is_empty: bool) -> bool {
    (ignore_empty_repos && !source_project_is_empty) || !ignore_empty_repos
}

fn create_groups_on_target_instance(client: &Client, source_groups: &Vec<GitLabGroup>,
                                    target: &InstanceConfig) -> anyhow::Result<()> {
    info!("creating groups on target instance..");

    let target_instance_groups = get_all_groups(&client, &target)
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

    let target_instance_groups = get_all_groups(&client, &target)
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

#[cfg(test)]
mod tests_is_migration_allowed {
    use crate::migration::is_migration_allowed;

    #[test]
    fn ignore_empty_repos_flag_deny_to_migrate_project() {
        assert!(!is_migration_allowed(true, true));
    }

    #[test]
    fn tolerate_empty_repos_without_force_flag() {
        assert!(is_migration_allowed(false, true));
    }

    #[test]
    fn allow_non_empty_repos() {
        assert!(is_migration_allowed(true, false));
        assert!(is_migration_allowed(false, false));
    }
}