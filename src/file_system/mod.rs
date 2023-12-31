use anyhow::{anyhow, Result};
use log::info;
use std::fs::read_dir;
use std::time::SystemTime;
use std::{
    fs::ReadDir,
    path::{Path, PathBuf},
};

use crate::configuration::constants::project_file_paths::{DIST_FOLDER, FRONTEND_PATH, PKG_FOLDER};

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
