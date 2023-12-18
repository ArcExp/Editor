use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

// The following functions are async for performance reasons
pub async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
    .set_title("Choose a text file...")
    .pick_file()
    .await
    .ok_or(Error::DialogClosed)?;

    load_file(handle.path().to_owned()).await
}

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
   let contents = tokio::fs::read_to_string(&path)
    .await
    .map(Arc::new)
    .map_err(|error|error.kind())
    .map_err(Error::IOFailed)?;

    Ok((path, contents))
}

pub async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path { path } else {
        rfd::AsyncFileDialog::new()
        .set_title("Choose a file name...")
        .save_file()
        .await
        .ok_or(Error::DialogClosed).map(|handle| handle.path().to_owned())?
    };

    tokio::fs::write(&path, text)
    .await
    .map_err(|error| Error::IOFailed(error.kind()))?;

    Ok(path)
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IOFailed(io::ErrorKind), // Error handling for when a file fails to be loaded or saved by this program
}
