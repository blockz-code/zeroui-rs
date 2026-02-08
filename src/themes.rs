use super::{Application, config::AppConfig};



#[derive(Debug, Clone)]
pub struct Theme {
    name: String,
    data: serde_json::Value,
}

impl Theme {

    pub fn parse<T>(&self) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned + Send + Sync + 'static,
    {
        Ok(serde_json::from_value(self.data.clone())?)
    }

}



pub use include_dir::Dir;

pub trait Themes {
    fn theme(&self, name: &str) -> crate::Result<Theme>;
    fn load_themes(self, dir_path: &str) -> crate::Result<Vec<Theme>> where Self: Sized;
    fn load_themes_embed(self, themes_dir: Dir<'_>) -> crate::Result<Vec<Theme>> where Self: Sized;
}

impl Themes for Application {

    fn theme(&self, name: &str) -> crate::Result<Theme> {
        let theme = self.themes().iter().find(|x| x.name == name).cloned().ok_or(crate::Error::Unknown(name.to_string()))?;
        Ok(theme)
    }

    fn load_themes<'a>(self, dir_path: &'a str) -> crate::Result<Vec<Theme>> {
        let mut result = Vec::new();
        let themes_dir = std::fs::read_dir(dir_path)?;
        for theme in themes_dir {
            let themes_path = theme?.path();
            let filename = themes_path.file_name().unwrap().to_str().unwrap();
            result.push(Theme{
                name: filename.to_string().replace(".json", ""),
                data: serde_json::from_str(&std::fs::read_to_string(&themes_path)?)?,
            });
        }
        Ok(result)
    }

    fn load_themes_embed<'a>(self, themes_dir: Dir<'_>) -> crate::Result<Vec<Theme>> {
        let mut result = Vec::new();
        for theme in themes_dir.entries() {
            let themes_path = theme.path();
            let filename = themes_path.file_name().unwrap().to_str().unwrap();
            result.push(Theme{
                name: filename.to_string().replace(".json", ""),
                data: serde_json::from_str(theme.as_file().unwrap().contents_utf8().unwrap())?,
            });
        }
        Ok(result)
    }

}

impl Themes for AppConfig {

    fn theme(&self, name: &str) -> crate::Result<Theme> {
        let theme = self.themes().iter().find(|x| x.name == name).cloned().ok_or(crate::Error::Unknown(name.to_string()))?;
        Ok(theme)
    }

    fn load_themes<'a>(self, dir_path: &'a str) -> crate::Result<Vec<Theme>> {
        let mut result = Vec::new();
        let themes_dir = std::fs::read_dir(dir_path)?;
        for theme in themes_dir {
            let themes_path = theme?.path();
            let filename = themes_path.file_name().unwrap().to_str().unwrap();
            result.push(Theme{
                name: filename.to_string().replace(".json", ""),
                data: serde_json::from_str(&std::fs::read_to_string(&themes_path)?)?,
            });
        }
        Ok(result)
    }

    fn load_themes_embed<'a>(self, themes_dir: Dir<'_>) -> crate::Result<Vec<Theme>> {
        let mut result = Vec::new();
        for theme in themes_dir.entries() {
            let themes_path = theme.path();
            let filename = themes_path.file_name().unwrap().to_str().unwrap();
            result.push(Theme{
                name: filename.to_string().replace(".json", ""),
                data: serde_json::from_str(theme.as_file().unwrap().contents_utf8().unwrap())?,
            });
        }
        Ok(result)
    }

}