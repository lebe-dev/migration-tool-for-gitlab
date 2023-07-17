use std::{env, fs};
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Context};
use log::{debug, error};

const REPO_TMP_DIR: &str = "gmt-tmp";

pub fn copy_git_repo_from_one_remote_to_another(repo_name: &str,
                                                source_git_url: &str, source_group_path: &str,
                                                target_group_path: &str,
                                                target_git_url: &str) -> anyhow::Result<()> {

    if Path::new(REPO_TMP_DIR).exists() {
        fs::remove_dir_all(REPO_TMP_DIR)?;
    }

    fs::create_dir(REPO_TMP_DIR).context("cannot create temporary directory")?;

    let args = format!("clone --mirror {source_git_url}/{source_group_path}/{repo_name} {REPO_TMP_DIR}/.git");

    execute_git_command(&args).context("unable to clone source repository")?;

    let current_dir = env::current_dir()?;
    let cloned_repo_dir = current_dir.join(REPO_TMP_DIR);

    env::set_current_dir(&cloned_repo_dir)?;

    let args = format!("remote add --mirror=fetch secondary {target_git_url}/{target_group_path}/{repo_name}.git");
    execute_git_command(&args).context("unable to set remote repository for target git instance")?;

    execute_git_command("fetch origin").context("unable to fetch origin repo")?;

    execute_git_command("push secondary --all").context("unable to fetch origin repo")?;

    env::set_current_dir(current_dir)?;

    fs::remove_dir_all(&cloned_repo_dir)?;

    Ok(())
}

fn execute_git_command(args_row: &str) -> anyhow::Result<String> {
    debug!("args '{}'", args_row);

    let args: Vec<&str> = args_row.split(" ").collect();

    let output = Command::new("/usr/bin/git").args(args).output()?;

    if output.status.success() {
        let stdout = format!("{}", String::from_utf8_lossy(&output.stdout));

        debug!("<stdout>");
        debug!("{}", stdout);
        debug!("</stdout>");

        Ok(stdout)

    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);

        error!("<stderr>");
        error!("{}", stderr);
        error!("</stderr>");

        Err(anyhow!("git command error"))
    }
}