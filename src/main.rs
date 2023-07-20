use anyhow::Result;
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use std::fmt::format;
use std::io::Write;
use std::{
    fs::ReadDir,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const FRONTEND_PATH: &str = "/Users/sean/GitHub/sean/loremaster/application/frontend";
const FRONTEND_PKG_PATH: &str = "/Users/sean/GitHub/sean/loremaster/application/frontend/pkg";
const GITHUB_PAGES_REPOSITORY_PATH: &str = "/Users/sean/GitHub/chronilore/chronilore.github.io";
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

    if !pkg_path.exists() {
        panic!("pkg directory doesn't exist! Exiting...");
    }

    let pkg_read_directory = pkg_path.read_dir()?;
    for possible_entry in pkg_read_directory {
        info!("moving ");
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
