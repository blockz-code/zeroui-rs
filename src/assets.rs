use std::{borrow::Cow, path::PathBuf};



use gpui::{AssetSource, SharedString};
pub use rust_embed::Embed as EmbedDerive;



#[derive(rust_embed::Embed)]
#[folder = "assets"]
#[include = "**/*.svg"]
pub struct InternalAssets;

impl AssetSource for InternalAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Ok(Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| super::Error::Unknown("could not find asset at path \"{path}\"".to_string()))?)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}







enum AssetType {
    None,
    Static,
    StaticMulti,
}



pub struct Assets {
    base: PathBuf,
    multi: Vec<PathBuf>,
    t: AssetType,
}

impl Assets {

    #[allow(unused)]
    pub fn new(assets_path: &str) -> Self {
        Self { base: PathBuf::from(assets_path), multi: vec![], t: AssetType::Static }
    }

    #[allow(unused)]
    pub fn multi(assets_paths: Vec<&str>) -> Self {
        Self { base: PathBuf::from(""), multi: assets_paths.iter().map(PathBuf::from).collect(), t: AssetType::StaticMulti }
    }

    #[allow(unused)]
    pub fn empty() -> Self {
        Self { base: PathBuf::from(""), multi: vec![], t: AssetType::None }
    }
    
    fn load(&self, path: &str) -> std::result::Result<Option<Cow<'static, [u8]>>, anyhow::Error> {
        std::fs::read(self.base.join(path))
            .map(|data| Some(Cow::Owned(data)))
            .map_err(|err| err.into())
    }

    pub fn list(&self, path: &str) -> std::result::Result<Vec<SharedString>, anyhow::Error> {
        let assets_dir = self.base.join(path);
        std::fs::read_dir(assets_dir)
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|err| err.into())
    }

}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> std::result::Result<Option<Cow<'static, [u8]>>, anyhow::Error> {
        match self.t {
            AssetType::None => Ok(None),
            AssetType::Static => self.load(path),
            AssetType::StaticMulti => {
                for m in &self.multi {
                    if std::fs::metadata(m).ok().is_some() {
                        return self.load(path);
                    }
                }
                Err(crate::Error::Unknown(format!("assets multi load error. path [{}] not found", path)).into())
            },
        }
    }

    fn list(&self, path: &str) -> std::result::Result<Vec<SharedString>, anyhow::Error> {
        match self.t {
            AssetType::None => Ok(vec![]),
            AssetType::Static => self.list(path),
            AssetType::StaticMulti => {
                for m in &self.multi {
                    if std::fs::metadata(m).ok().is_some() {
                        return self.list(path);
                    }
                }
                Err(crate::Error::Unknown(format!("assets multi list error. path [{}] not found", path)).into())
            },
        }
    }
}