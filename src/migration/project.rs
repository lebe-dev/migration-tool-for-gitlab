use anyhow::{anyhow, Context};
use log::{debug, error, info};
use reqwest::blocking::Client;

use crate::config::InstanceConfig;
use crate::migration::domain::{GitLabProject, GitLabRepositoryBranch};
use crate::migration::PRIVATE_TOKEN_HEADER;

pub fn get_project_list(client: &Client, instance: &InstanceConfig) -> anyhow::Result<Vec<GitLabProject>> {
    info!("get project list for instance '{}'..", instance.public_url);

    let url = format!("{}/api/v4/projects?per_page=100", instance.public_url);

    debug!("url: {url}");

    let response = client.get(url)
        .header(PRIVATE_TOKEN_HEADER, instance.token.to_string())
        .send().context("gitlab api communication error")?;

    let response_status = response.status();

    if response_status == reqwest::StatusCode::OK {
        let groups = response.json().context("unable to decode server response")?;

        debug!("---[HTTP RESPONSE]----");
        debug!("{:?}", groups);
        debug!("---[/HTTP RESPONSE]----");

        Ok(groups)

    } else {
        error!("unexpected server response code {}", response_status);
        Err(anyhow!("unexpected server response"))
    }
}

pub fn get_project_branches(client: &Client, instance: &InstanceConfig,
                            project_id: u32) -> anyhow::Result<Vec<String>> {
    info!("get project (id {project_id}) branches, for instance '{}'..", instance.public_url);

    let url = format!("{}/api/v4/projects/{project_id}/repository/branches", instance.public_url);

    debug!("url: {url}");

    let response = client.get(url)
        .header(PRIVATE_TOKEN_HEADER, instance.token.to_string())
        .send().context("gitlab api communication error")?;

    let response_status = response.status();

    if response_status == reqwest::StatusCode::OK {
        let branches: Vec<GitLabRepositoryBranch> = response.json().context("unable to decode server response")?;

        let branch_names: Vec<String> = branches.clone().into_iter()
                                    .map(|b| b.name).collect::<Vec<String>>();

        debug!("---[HTTP RESPONSE]----");
        debug!("{:?}", branches);
        debug!("---[/HTTP RESPONSE]----");

        Ok(branch_names)

    } else {
        error!("unexpected server response code {}", response_status);
        Err(anyhow!("unexpected server response"))
    }
}

pub fn create_gitlab_private_project(client: &Client, instance: &InstanceConfig,
                     group_id: u32, name: &str, path: &str) -> anyhow::Result<Vec<GitLabProject>> {
    info!("create project '{name}' with group-id {group_id} at instance '{}'..", instance.public_url);

    let url = format!("{}/api/v4/projects?name={}&path={}&visibility=private&namespace_id={}",
                      instance.public_url, name, path, group_id);

    debug!("url: {url}");

    let response = client.post(url)
        .header(PRIVATE_TOKEN_HEADER, instance.token.to_string())
        .send().context("gitlab api communication error")?;

    let response_status = response.status();

    if response_status == reqwest::StatusCode::CREATED {
        let projects = response.json().unwrap_or(vec![]);

        debug!("---[HTTP RESPONSE]----");
        debug!("{:?}", projects);
        debug!("---[/HTTP RESPONSE]----");

        info!("project '{name}' has been created");

        Ok(projects)

    } else {
        error!("unexpected server response code {}", response_status);
        Err(anyhow!("unexpected server response"))
    }
}