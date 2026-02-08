use std::path::{Path, PathBuf};

use dirs;

use crate::Result;

#[derive(Debug, Clone)]
pub struct Dirs {
    app_dir: Option<PathBuf>,
    app_local_dir: Option<PathBuf>,
}

impl Default for Dirs {
    fn default() -> Self {
        Dirs{ app_dir: None, app_local_dir: None }
    }
}

impl Dirs {

    pub fn new(identifier: &str) -> Result<Dirs> {
        let config_dir = dirs::data_dir().ok_or(crate::Error::DirNotFound("data_dir".to_string()))?.join(identifier);
        let config_local_dir = dirs::data_local_dir().ok_or(crate::Error::DirNotFound("data_local_dir".to_string()))?.join(identifier);

        if std::fs::metadata(&config_dir).is_err() {
            std::fs::create_dir(&config_dir).unwrap();
        }

        if std::fs::metadata(&config_local_dir).is_err() {
            std::fs::create_dir(&config_local_dir).unwrap();
        }

        Ok(Dirs{ app_dir: Some(config_dir), app_local_dir: Some(config_local_dir) })
    }

    pub fn app_dir(&self) -> &Path {
        self.app_dir.as_ref().unwrap().as_path()
    }

    pub fn app_local_dir(&self) -> &Path {
        self.app_local_dir.as_ref().unwrap().as_path()
    }
    
}




pub trait ExportDirsExt {
    fn audio_dir() -> Result<String>;
    fn cache_dir() -> Result<String>;
    fn config_dir() -> Result<String>;
    fn config_local_dir() -> Result<String>;
    fn data_dir() -> Result<String>;
    fn data_local_dir() -> Result<String>;
    fn desktop_dir() -> Result<String>;
    fn document_dir() -> Result<String>;
    fn download_dir() -> Result<String>;
    fn executable_dir() -> Result<String>;
    fn font_dir() -> Result<String>;
    fn home_dir() -> Result<String>;
    fn picture_dir() -> Result<String>;
    fn preference_dir() -> Result<String>;
    fn public_dir() -> Result<String>;
    fn runtime_dir() -> Result<String>;
    fn state_dir() -> Result<String>;
    fn template_dir() -> Result<String>;
    fn video_dir() -> Result<String>;
}

impl ExportDirsExt for Dirs {

    fn audio_dir() -> Result<String> {
        Ok(dirs::audio_dir().ok_or(crate::Error::DirNotFound("audio_dir".to_string()))?.display().to_string())
    }

    fn cache_dir() -> Result<String> {
        Ok(dirs::cache_dir().ok_or(crate::Error::DirNotFound("cache_dir".to_string()))?.display().to_string())
    }

    fn config_dir() -> Result<String> {
        Ok(dirs::config_dir().ok_or(crate::Error::DirNotFound("config_dir".to_string()))?.display().to_string())
    }

    fn config_local_dir() -> Result<String> {
        Ok(dirs::config_local_dir().ok_or(crate::Error::DirNotFound("config_local_dir".to_string()))?.display().to_string())
    }

    fn data_dir() -> Result<String> {
        Ok(dirs::data_dir().ok_or(crate::Error::DirNotFound("data_dir".to_string()))?.display().to_string())
    }

    fn data_local_dir() -> Result<String> {
        Ok(dirs::data_local_dir().ok_or(crate::Error::DirNotFound("data_local_dir".to_string()))?.display().to_string())
    }

    fn desktop_dir() -> Result<String> {
        Ok(dirs::desktop_dir().ok_or(crate::Error::DirNotFound("desktop_dir".to_string()))?.display().to_string())
    }

    fn document_dir() -> Result<String> {
        Ok(dirs::document_dir().ok_or(crate::Error::DirNotFound("document_dir".to_string()))?.display().to_string())
    }

    fn download_dir() -> Result<String> {
        Ok(dirs::download_dir().ok_or(crate::Error::DirNotFound("download_dir".to_string()))?.display().to_string())
    }

    fn executable_dir() -> Result<String> {
        Ok(dirs::executable_dir().ok_or(crate::Error::DirNotFound("executable_dir".to_string()))?.display().to_string())
    }

    fn font_dir() -> Result<String> {
        Ok(dirs::font_dir().ok_or(crate::Error::DirNotFound("font_dir".to_string()))?.display().to_string())
    }

    fn home_dir() -> Result<String> {
        Ok(dirs::home_dir().ok_or(crate::Error::DirNotFound("home_dir".to_string()))?.display().to_string())
    }

    fn picture_dir() -> Result<String> {
        Ok(dirs::picture_dir().ok_or(crate::Error::DirNotFound("picture_dir".to_string()))?.display().to_string())
    }

    fn preference_dir() -> Result<String> {
        Ok(dirs::preference_dir().ok_or(crate::Error::DirNotFound("preference_dir".to_string()))?.display().to_string())
    }

    fn public_dir() -> Result<String> {
        Ok(dirs::public_dir().ok_or(crate::Error::DirNotFound("public_dir".to_string()))?.display().to_string())
    }

    fn runtime_dir() -> Result<String> {
        Ok(dirs::runtime_dir().ok_or(crate::Error::DirNotFound("runtime_dir".to_string()))?.display().to_string())
    }

    fn state_dir() -> Result<String> {
        Ok(dirs::state_dir().ok_or(crate::Error::DirNotFound("state_dir".to_string()))?.display().to_string())
    }

    fn template_dir() -> Result<String> {
        Ok(dirs::template_dir().ok_or(crate::Error::DirNotFound("template_dir".to_string()))?.display().to_string())
    }

    fn video_dir() -> Result<String> {
        Ok(dirs::video_dir().ok_or(crate::Error::DirNotFound("video_dir".to_string()))?.display().to_string())
    }
    
}
