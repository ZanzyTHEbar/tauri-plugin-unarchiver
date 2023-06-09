extern crate unrar;

use log::warn;
use serde::{ser::Serializer, Serialize};
use unrar::Archive;

use std::path::PathBuf;
use tauri::{
    command,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};
use zip_extract::ZipExtractError;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<ZipExtractError> for Error {
    fn from(e: ZipExtractError) -> Self {
        Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

/// Checks whether the path exists.
#[command]
async fn exists(path: PathBuf) -> bool {
    path.exists()
}

/// TODO: refactor to use tauri::fs and tauri::path
/// Unzips an archive to a target directory
/// # Arguments
/// * `archive_path` - The path to the archive to unzip
/// * `target_dir` - The path to the directory to unzip the archive to (must exist)
/// * `erase_when_done` - Whether to erase the archive when done
#[command]
async fn unarchive(
    archive_path: PathBuf,
    target_dir: Option<PathBuf>,
    erase_when_done: Option<bool>,
) -> Result<String> {
    let erase_when_done = erase_when_done.unwrap_or(false);

    // check in the target directory is passed in
    let target_dir = match target_dir {
        Some(dir) => dir,
        None => {
            warn!("No target directory passed in, using parent directory of archive");
            archive_path
                .parent()
                .expect("Failed to get parent directory of archive")
                .to_path_buf()
        }
    };

    // check if the target directory was passed in
    if !target_dir.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Target directory does not exist",
        )));
    }

    // check if the archive exists
    if !archive_path.exists() {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Archive does not exist",
        )));
    }

    let archive_path_str = match archive_path.to_str() {
        Some(path) => path,
        None => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Archive path is not valid UTF-8",
            )))
        }
    };

    // split the archive path to get the extension
    let archive_path_split: Vec<&str> = archive_path_str.split('.').collect();

    match archive_path_split.last() {
        Some(ext) => {
            match *ext {
                "zip" => {
                    // extract the archive
                    // The third parameter allows you to strip away toplevel directories.
                    // If `archive` contained a single directory, its contents would be extracted instead.
                    let archive = std::fs::read(&archive_path)?;
                    zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true)?;
                }
                "tar" | "gz" | "tar.gz" => {
                    // extract the archive
                    let archive = std::fs::File::open(&archive_path)?;
                    let mut archive = tar::Archive::new(archive);
                    archive.unpack(&target_dir)?;
                }
                "rar" => {
                    // convert the path to a string
                    let archive_path = match archive_path.to_str() {
                        Some(path) => path,
                        None => {
                            return Err(Error::Io(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Archive path is not valid UTF-8",
                            )))
                        }
                    };

                    let target_dir = match target_dir.to_str() {
                        Some(path) => path,
                        None => {
                            return Err(Error::Io(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "Target directory path is not valid UTF-8",
                            )))
                        }
                    };

                    // extract the archive
                    // Need to refactor this to remove unwrap
                    Archive::new(archive_path.to_string())
                        .extract_to(target_dir.to_string())
                        .unwrap()
                        .process()
                        .expect("Failed to extract archive");
                }
                _ => {
                    return Err(Error::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Archive is not a supported type",
                    )));
                }
            }
        }
        None => {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "File is not an archive",
            )));
        }
    }

    // erase the archive if the flag is set
    if erase_when_done {
        std::fs::remove_file(&archive_path)?;
    }

    Ok("Archive extracted successfully".to_string())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("unarchiver")
        .invoke_handler(tauri::generate_handler![exists, unarchive])
        .build()
}
