use async_std::fs::File;
use async_std::io::{prelude::*, BufReader};

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

// The following functions are async to avoid app UI freezes
pub async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let pick_file = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file();
    let handle = pick_file.await.ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let file = File::open(&path)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok((path, Arc::new(contents)))
}

pub async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name...")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(|handle| handle.path().to_owned())?
    };

    tokio::fs::write(&path, text)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok(path)
}

pub async fn monitor_file_changes(path: PathBuf) -> Result<(), Error> {
    let file = File::open(&path)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;
    let mut reader = BufReader::new(file);
    let mut current_content = String::new();

    loop {
        let mut buffer = String::new();
        reader
            .read_to_string(&mut buffer)
            .await
            .map_err(|error| Error::IOFailed(error.kind()))?;

        if buffer != current_content {
            // Changes detected, perform actions for undo/redo
            println!("File content changed!");

            // Store 'buffer' as a new state for undo/redo purposes
            // Implement your logic here for handling undo/redo actions
            // You might store versions of the content or use a history mechanism
            // to track changes for undo/redo functionality

            current_content = buffer; // Update the current content for the next iteration
        }

        // Add a delay or use other mechanisms to control the frequency of monitoring
        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    }
}

async fn load_file_content(path: &PathBuf) -> Result<String, Error> {
    let file = File::open(path)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;
    let mut reader = BufReader::new(file);

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .await
        .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok(contents)
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind), // Error handling for when a file fails to be loaded or saved by this program
}
