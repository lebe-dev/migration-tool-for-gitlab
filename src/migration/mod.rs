use std::collections::HashMap;

use anyhow::Context;
use log::info;
use reqwest::blocking::ClientBuilder;

use crate::config::InstanceConfig;
use crate::migration::group::{create_gitlab_private_group, get_group_list};

pub mod domain;
pub mod group;
pub mod project;

pub const PRIVATE_TOKEN_HEADER: &str = "PRIVATE-TOKEN";

pub struct GitLabGroupIdMap {
    pub source_id: u32,
    pub target_id: u32,
}

pub fn migrate_gitlab_instance(source: &InstanceConfig, target: &InstanceConfig) -> anyhow::Result<()> {
    info!("migrating groups and projects from '{}' to '{}'..", source.host, target.host);

    let client = ClientBuilder::new().build().unwrap();

    let source_instance_groups = get_group_list(&client, &source)
        .context("unable to get gitlab groups from source instance")?;

    let target_instance_groups = get_group_list(&client, &target)
        .context("unable to get gitlab groups from target instance")?;

    info!("received groups from source instance:");
    info!("{:?}", source_instance_groups);

    let mut group_map: HashMap<String, GitLabGroupIdMap> = HashMap::new();

    for group in source_instance_groups {
        info!("- group (id {}): '{}'", group.id, group.name);

        let target_group_found = target_instance_groups.iter().find(|gg| gg.name == group.name);

        match target_group_found {
            Some(target_group) => {
                group_map.insert(
                    group.name.to_string(),
                    GitLabGroupIdMap {
                        source_id: group.id,
                        target_id: target_group.id,
                    }
                );
            }
            None => {
                let groups_resp = create_gitlab_private_group(&client, &target,
                                            &group.name, &group.path)
                                        .context("cannot create gitlab group on target instance")?;

                if !groups_resp.is_empty() {
                    let target_group = groups_resp.first().unwrap();

                    group_map.insert(
                        group.name.to_string(),
                        GitLabGroupIdMap {
                            source_id: group.id,
                            target_id: target_group.id,
                        }
                    );
                }
            }
        }

        if group_map.contains_key(&group.name) {
            let group_id_mapping = group_map.get(&group.name).unwrap();

            unimplemented!()
        }

    }

    Ok(())
}