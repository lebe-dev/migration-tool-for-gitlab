use anyhow::Context;
use log::info;
use reqwest::blocking::Client;

use crate::config::InstanceConfig;
use crate::migration::domain::GitLabProject;
use crate::migration::project::{get_all_projects, get_project_branches};

pub fn get_empty_projects(client: &Client, instance: &InstanceConfig) -> anyhow::Result<Vec<GitLabProject>> {
    info!("get repositories without branches from instance '{}'", instance.git_url);

    let projects = get_all_projects(&client, &instance)
        .context("cannot get project list from gitlab instance")?;

    let mut empty_projects: Vec<GitLabProject> = vec![];

    for project in projects {
        let branches = get_project_branches(&client, &instance, project.id)
            .context("cannot get branch list from project")?;

        if branches.is_empty() {
            info!("empty project '{}'", project.path);
            empty_projects.push(project.clone());
        }
    }

    Ok(empty_projects)
}