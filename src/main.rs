use anyhow::{anyhow, Result};
use env_logger::{Builder, Target};
use log::{info, LevelFilter};

use std::fs::read_dir;
use std::io::Write;
use std::time::SystemTime;
use std::{
    fs::ReadDir,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::github::github_app::GitHubApp;

pub mod github;

const FRONTEND_PATH: &str = "";
const FRONTEND_PKG_PATH: &str = "";
const GITHUB_PAGES_REPOSITORY_PATH: &str = "";
const LIST_COMMAND: &str = "ls";
const LIST_COMMAND_ALL_FLAG: &str = "-a";
const DIST_FOLDER: &str = "dist";
const PKG_FOLDER: &str = "pkg";

const PERSEUS_COMMAND: &str = "perseus";
const PERSEUS_DEPLOY_COMMAND: &str = "deploy";
const PERSEUS_DEPLOY_EXPORT_FLAG: &str = "-e";

fn main() -> Result<()> {
    configure_logging()?;
    info!("lorerunner started");

    let github_app: GitHubApp = GitHubApp::new()?;

    github_app.ping_github()?;
    github_app.get_app_details()?;

    Ok(())
}

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

pub fn get_directory<PATH>(directory_path: PATH) -> Result<Directory>
where
    PATH: AsRef<Path>,
{
    let now: SystemTime = SystemTime::now();
    let directory_path: &Path = directory_path.as_ref();

    if !directory_path.exists() {
        return Err(anyhow!(
            "Cannot get directory. The provided path ({}) does not exist.",
            directory_path.to_str().unwrap_or("invalid path string")
        ));
    }

    if !directory_path.is_dir() {
        return Err(anyhow!(
            "Cannot get directory. The provided path ({}) is not a directory.",
            directory_path.to_str().unwrap_or("invalid path string")
        ));
    }

    let mut file_paths: Vec<String> = vec![];
    let mut subdirectory_paths: Vec<String> = vec![];

    for entry in read_dir(directory_path)? {
        let current_path: PathBuf = entry?.path();
        match current_path.is_dir() {
            true => {
                if let Some(path_string) = current_path.to_str() {
                    subdirectory_paths.push(path_string.to_string());
                }
            }
            false => {
                if let Some(path_string) = current_path.to_str() {
                    file_paths.push(path_string.to_string());
                }
            }
        }
    }

    let result: Directory = Directory {
        files_paths: file_paths,
        sub_directory_paths: subdirectory_paths,
    };

    info!(
        "Time elapsed (seconds): {}",
        now.elapsed().unwrap().as_secs()
    );
    Ok(result)
}

pub fn get_file_content_as_string(file_path: String) -> Result<String> {
    let target_path: &Path = Path::new(&file_path);

    if !target_path.exists() {
        panic!("Failed to get file contents -- file not found!");
    }

    if !target_path.is_file() || target_path.is_dir() || target_path.is_symlink() {
        panic!("Failed to get file contents -- specified path is not a file!");
    }

    Ok(std::fs::read_to_string(target_path)?)
}

pub fn copy_directory_to() -> Result<()> {
    Ok(())
}

pub struct Directory {
    pub files_paths: Vec<String>,
    pub sub_directory_paths: Vec<String>,
}

pub fn print_directory_contents(path: &Path) -> Result<()> {
    let read_directory: ReadDir = path.read_dir()?;
    println!("Reading files in root directory\n");
    for entry in read_directory {
        let path: PathBuf = entry?.path();
        if let Some(name) = path.file_name() {
            if let Some(name_string) = name.to_str() {
                let mut console_message: String = String::from(name_string);
                if path.is_dir() {
                    console_message.push_str(" directory");
                }
                if path.is_file() {
                    console_message.push_str(" file")
                }
                println!("{}", console_message);
            }
        }
    }

    Ok(())
}

pub fn delete_existing_folders() -> Result<()> {
    if Path::new(&format!("{FRONTEND_PATH}/{DIST_FOLDER}")).exists() {
        println!("dist folder exists, it will now be deleted");
        std::fs::remove_dir_all(DIST_FOLDER)?;
    }

    if Path::new(&format!("{FRONTEND_PATH}/{PKG_FOLDER}")).exists() {
        println!("pkg folder exists, it will now be deleted");
        std::fs::remove_dir_all(PKG_FOLDER)?;
    }

    Ok(())
}

pub fn configure_logging() -> Result<()> {
    const DEBUG: &str = "DEBUG";
    Builder::new()
        .target(Target::Stdout)
        .format(move |buf, record| -> Result<(), std::io::Error> {
            writeln!(
                buf,
                "[LORERUNNER_{}]: [{}] [{}] - {}",
                DEBUG,
                OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    Ok(())
}
