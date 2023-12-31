use crate::configuration::constants::project_file_paths::{
    FRONTEND_PATH, FRONTEND_PKG_PATH, GITHUB_PAGES_REPOSITORY_PATH,
};
use anyhow::Result;
use log::info;

use crate::configuration::constants::command_line::{
    PERSEUS_COMMAND, PERSEUS_DEPLOY_COMMAND, PERSEUS_DEPLOY_EXPORT_FLAG,
};
use std::{
    fs::ReadDir,
    path::Path,
    process::{Command, Output},
};

pub fn deploy_loremaster_static_site() -> Result<()> {
    let source_path: &Path = Path::new(FRONTEND_PATH);
    let target_path: &Path = Path::new(GITHUB_PAGES_REPOSITORY_PATH);
    let pkg_path: &Path = Path::new(FRONTEND_PKG_PATH);

    if !target_path.exists() {
        panic!("Target directory doesn't exist!");
    }

    if !source_path.exists() {
        panic!("Target directory doesn't exist!");
    }

    std::env::set_current_dir(source_path)?;

    let mut binding = Command::new(PERSEUS_COMMAND);
    let deploy_command: &mut Command = binding
        .arg(PERSEUS_DEPLOY_COMMAND)
        .arg(PERSEUS_DEPLOY_EXPORT_FLAG);

    info!("executing: {:?}", deploy_command);

    let deploy_output: Output = deploy_command.output()?;

    match deploy_output.status.success() {
        true => info!("output: {}", String::from_utf8(deploy_output.stdout)?),
        false => {
            info!("{}", String::from_utf8(deploy_output.stderr)?);
            panic!("Failed to deploy frontend files. Exiting...");
        }
    }

    if !source_path.read_dir().is_ok_and(|mut reader: ReadDir| {
        reader.any(|item| {
            item.is_ok_and(|entry| {
                entry.file_name().eq_ignore_ascii_case("pkg") && entry.path().is_dir()
            })
        })
    }) {
        panic!("pkg thing directory doesn't exist! Exiting...");
    }

    if !pkg_path.exists() {
        panic!("pkg directory doesn't exist! Exiting...");
    }

    let pkg_read_directory = pkg_path.read_dir()?;
    for possible_entry in pkg_read_directory {
        let Ok(entry) = possible_entry else {
            continue;
        };

        let directory_path_string = target_path.to_str().unwrap();
        if let Some(current_file_name) = entry.file_name().to_str() {
            let new_file_path = format!("{}/{}", directory_path_string, current_file_name);

            let new_path = Path::new(new_file_path.as_str());

            info!(
                "moving \n\t{} \n\tto \n\t{} \n\tIs new: {}",
                entry.path().to_str().unwrap(),
                new_file_path,
                !new_path.exists()
            );

            // std::fs::copy(entry.path(), new_path)?;
        }
    }

    // let ls_output: Output = Command::new(LIST_COMMAND)
    //     .arg(LIST_COMMAND_ALL_FLAG)
    //     .output()?;

    // match ls_output.status.success() {
    //     true => println!("{}", String::from_utf8(ls_output.stdout)?),
    //     false => println!("{}", String::from_utf8(ls_output.stderr)?),
    // }
    Ok(())
}
