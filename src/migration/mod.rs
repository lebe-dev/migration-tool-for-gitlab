use anyhow::Context;
use log::info;
use reqwest::blocking::ClientBuilder;

use crate::config::InstanceConfig;
use crate::migration::group::get_group_list;

pub mod domain;
pub mod group;

pub fn migrate_gitlab_instance(source: &InstanceConfig, target: &InstanceConfig) -> anyhow::Result<()> {
    info!("migrating groups and projects from '{}' to '{}'..", source.host, target.host);

    let client = ClientBuilder::new().build().unwrap();

    let source_instance_groups = get_group_list(&client, &source)
        .context("unable to get gitlab groups from source instance")?;

    info!("received groups from source instance:");
    info!("{:?}", source_instance_groups);

    for group in source_instance_groups {
        info!("- group (id {}): '{}'", group.id, group.name);
    }

    Ok(())
}