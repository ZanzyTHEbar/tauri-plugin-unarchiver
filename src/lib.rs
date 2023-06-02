use log::warn;
use serde::{ser::Serializer, Serialize};

use tauri::{
    command,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};

use std::path::PathBuf;

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

    // The third parameter allows you to strip away toplevel directories.
    // If `archive` contained a single directory, its contents would be extracted instead.
    let archive = std::fs::read(&archive_path).expect("Failed to read archive");
    zip_extract::extract(std::io::Cursor::new(archive), &target_dir, true)
        .expect("Failed to extract archive");

    // erase the archive if the flag is set
    if erase_when_done {
        std::fs::remove_file(&archive_path).expect("Failed to remove archive");
    }
    Ok("Archive extracted successfully".to_string())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("unarchiver")
        .invoke_handler(tauri::generate_handler![exists, unarchive])
        .build()
}
