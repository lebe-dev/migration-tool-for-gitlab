use anyhow::{anyhow, Context};
use log::{debug, error, info};
use reqwest::blocking::Client;

use crate::config::InstanceConfig;
use crate::migration::domain::GitLabGroup;
use crate::migration::PRIVATE_TOKEN_HEADER;

pub fn get_group_list(client: &Client, instance: &InstanceConfig) -> anyhow::Result<Vec<GitLabGroup>> {
    info!("get group list for instance '{}'..", instance.public_url);

    let url = format!("{}/api/v4/groups?per_page=100", instance.public_url);

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

pub fn create_gitlab_private_group(client: &Client, instance: &InstanceConfig,
   group_name: &str, path: &str, parent_id: Option<u32>) -> anyhow::Result<Vec<GitLabGroup>> {
    info!("create group '{group_name}' at instance '{}'..", instance.public_url);

    let parent_id_param = if let Some(value) = parent_id {
        format!("&parent_id={value}")

    } else {
        "".to_string()
    };

    let url = format!("{}/api/v4/groups?name={}&path={}&visibility=private{parent_id_param}",
                      instance.public_url, group_name, path);

    debug!("url: {url}");

    let response = client.post(url)
        .header(PRIVATE_TOKEN_HEADER, instance.token.to_string())
        .send().context("gitlab api communication error")?;

    let response_status = response.status().clone();

    if response_status == reqwest::StatusCode::CREATED {

        let groups: &Vec<GitLabGroup> = &response.json().unwrap_or(vec![]);

        debug!("---[HTTP RESPONSE]----");
        debug!("{:?}", groups);
        debug!("---[/HTTP RESPONSE]----");

        info!("group '{group_name}' has been created");

        let groups = vec![];

        Ok(groups.clone())

    } else {
        error!("unexpected server response code {}", response_status);
        Err(anyhow!("unexpected server response"))
    }
}

#[cfg(test)]
mod create_group_tests {
    use log::{error, info};
    use reqwest::blocking::ClientBuilder;

    use crate::config::InstanceConfig;
    use crate::migration::group::create_gitlab_private_group;
    use crate::tests::init_logging;

    #[ignore]
    #[test]
    fn group_have_to_be_created() {
        init_logging();

        let client = ClientBuilder::new().build().unwrap();

        let config = InstanceConfig {
            public_url: "http://localhost:28080".to_string(),
            git_url: "ssh://localhost:2222".to_string(),
            token: "CHANGE-ME".to_string(),
        };

        match create_gitlab_private_group(&client, &config, "g5000", "g5000", None) {
            Ok(groups) => {
                info!("groups: {:?}", groups);
            }
            Err(e) => {
                error!("{}", e);
                error!("{}", e.root_cause());
                assert!(false)
            }
        }
    }
}