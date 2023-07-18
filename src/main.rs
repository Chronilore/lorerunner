use anyhow::Result;
use std::{
    fs::ReadDir,
    path::{Path, PathBuf},
    process::{Command, Output},
};

const GITHUB_PAGES_REPOSITORY_PATH: &str = "/Users/sean/GitHub/chronilore/chronilore.github.io";
const LIST_COMMAND: &str = "ls";
const LIST_COMMAND_ALL_FLAG: &str = "-a";

fn main() -> Result<()> {
    println!("lorerunner started\n");

    let target_path: &Path = Path::new(GITHUB_PAGES_REPOSITORY_PATH);

    if !target_path.exists() {
        panic!("Root directory doesn't exist!");
    }

    std::env::set_current_dir(target_path)?;

    let ls_output: Output = Command::new(LIST_COMMAND)
        .arg(LIST_COMMAND_ALL_FLAG)
        .output()?;
    if ls_output.status.success() {
        println!("{}", String::from_utf8(ls_output.stdout)?);
    }

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
