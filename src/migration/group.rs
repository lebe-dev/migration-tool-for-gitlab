use anyhow::{anyhow, Context};
use log::{debug, error, info};
use reqwest::blocking::Client;

use crate::config::InstanceConfig;
use crate::migration::domain::GitLabGroup;

const PRIVATE_TOKEN_HEADER: &str = "PRIVATE-TOKEN";

pub fn get_group_list(client: &Client, instance: &InstanceConfig) -> anyhow::Result<Vec<GitLabGroup>> {
    info!("get group list for instance '{}'..", instance.host);

    let url = format!("{}/api/v4/groups", instance.host);

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