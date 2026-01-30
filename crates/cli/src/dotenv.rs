use std::error::Error;
use std::path::PathBuf;

pub fn load_dotenv(no_dotenv: bool, dotenv: Option<&PathBuf>) -> Result<(), Box<dyn Error>> {
    if no_dotenv {
        return Ok(());
    }
    if let Some(path) = dotenv {
        dotenvy::from_path(path)?;
    } else {
        let _ = dotenvy::dotenv();
    }
    Ok(())
}
