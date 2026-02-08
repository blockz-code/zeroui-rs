use super::{Dirs, Theme, Themes};



use include_dir::Dir;
use schemars::{schema_for, JsonSchema};
use gpui::{Bounds, DisplayId, SharedString, WindowBounds, WindowDecorations, WindowKind, point, px, size};

#[derive(Debug, Clone)]
pub struct AppConfig {
    config: Config,
    dirs: Option<Dirs>,
    themes: Option<Vec<Theme>>,
    active_theme: Option<Theme>,
}

impl AppConfig {

    fn new(app_config: &[u8]) -> crate::Result<Self> {
        Ok(Self {
            config: serde_json::from_slice::<Config>(&app_config)?,
            themes: None, dirs: None, active_theme: None
        })
    }

    pub fn builder(app_config: &[u8], themes_efs: Dir<'_>) -> crate::Result<Self> {
        let mut this = Self::new(app_config)?;
        let themes = this.clone().load_themes_embed(themes_efs)?;
        this.themes = Some(themes.clone());
        this.dirs = Some(Dirs::new(&this.config.identifier)?);
        this.active_theme = Some(this.theme(&this.config.default_theme)?);
        Ok(this)
    }

    #[allow(unused)]
    fn pretty_print(self) -> String {
        let schema = schema_for!(Config);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    pub fn config(&self) -> Config {
        self.config.clone()
    }
    pub fn dirs(&self) -> Dirs {
        self.dirs.clone().unwrap()
    }
    pub fn themes(&self) -> Vec<Theme> {
        self.themes.clone().unwrap()
    }
    pub fn active_theme(&self) -> Theme {
        self.active_theme.clone().unwrap()
    }

    pub fn set_active_theme(&mut self, name: &str) {
        self.active_theme = Some(self.theme(name).unwrap());
    }
}

//
//
//

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default)]
    pub name: SharedString,
    #[serde(default)]
    pub identifier: String,
    #[serde(default = "default_theme")]
    pub default_theme: String,
    #[serde(default)]
    pub build: BuildConfig,
    #[serde(default)]
    pub windows: Vec<WindowConfig>,
}

//
//
//

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct BuildConfig {
    #[serde(default = "default_before_build_cmd")]
    pub before_build_cmd: String,
    #[serde(default = "default_after_build_cmd")]
    pub after_build_cmd: String,
    #[serde(default = "default_out_dir")]
    pub out_dir: String,
    #[serde(default = "default_icons")]
    pub icons: Vec<String>,
}

//
//
//

fn default_before_build_cmd() -> String {
    "".to_string()
}

fn default_after_build_cmd() -> String {
    "".to_string()
}

fn default_out_dir() -> String {
    "../out".to_string()
}


// GENERATOR FOR ICO, ICNS
fn default_icons() -> Vec<String> {
    vec![
        "32x32.png".to_string(),
        "64x64.png".to_string(),
        "128x128.png".to_string(),
    ]
}

//
//
//

impl Config {

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

}

//
//
//

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum WindowType {
    #[default]
    #[serde(rename = "center")]
    Centered,
    #[serde(rename = "window")]
    Windowed,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum WindowConfigKind {
    #[default]
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "popup")]
    PopUp,
    #[serde(rename = "floating")]
    Floating,
}

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum WindowConfigDecorated {
    #[default]
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
}

#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub enum WindowBackgroundAppearance {
    #[default]
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "blurred")]
    Blurred,
    #[serde(rename = "micabackdrop")]
    MicaBackdrop,
    #[serde(rename = "micaltbackdrop")]
    MicaAltBackdrop,
}




#[derive(Debug, Default, Clone, JsonSchema, serde::Deserialize, serde::Serialize)]
pub struct WindowConfig {
    pub label: String,
    pub width: f32,
    pub height: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub on_start: bool,
    #[serde(default)]
    pub decorated: bool,
    #[serde(default)]
    pub display_id: usize,
    #[serde(default, rename = "type")]
    pub typ: WindowType,

    #[serde(default)]
    pub focus: bool,
    #[serde(default = "true_bool")]
    pub show: bool,
    #[serde(default = "true_bool")]
    pub is_movable: bool,
    #[serde(default = "true_bool")]
    pub is_resizable: bool,
    #[serde(default = "true_bool")]
    pub is_minimizable: bool,
    
    #[serde(default)]
    pub kind: WindowConfigKind,

    #[serde(default)]
    pub decorated_kind: WindowConfigDecorated,

    #[serde(default)]
    pub window_background: WindowBackgroundAppearance,
}

//
//
//

impl WindowConfig {

    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn kind(&self) -> WindowKind {
        match self.kind {
            WindowConfigKind::Normal => WindowKind::Normal,
            WindowConfigKind::PopUp => WindowKind::PopUp,
            WindowConfigKind::Floating => WindowKind::Floating,                
        }
    }

    pub fn decorated_kind(&self) -> WindowDecorations {
        match self.decorated_kind {
            WindowConfigDecorated::Client => WindowDecorations::Client,
            WindowConfigDecorated::Server => WindowDecorations::Server,              
        }
    }

    pub fn display_id(&self, cx: &gpui::App) -> crate::Result<DisplayId> {
        match cx.displays().iter().enumerate().find(|v| v.0 == self.display_id) {
            Some(display) => Ok(display.1.id()),
            None => Err(crate::Error::DisplayFound),
        }
    }

    pub fn window_bounds(&self, cx: &gpui::App) -> crate::Result<WindowBounds> {
        let size = size(px(self.width), px(self.height));
        match self.typ {
            WindowType::Centered => {
                let display_id = self.display_id(cx)?;
                let bounds = Bounds::centered(Some(display_id), size, cx);
                Ok(WindowBounds::Windowed(bounds))
            },
            WindowType::Windowed => {
                let origin = point(px(self.x), px(self.y));
                let bounds = Bounds::new(origin, size);
                Ok(WindowBounds::Windowed(bounds))
            },
            WindowType::Maximized => {
                let display_id = self.display_id(cx)?;
                let bounds = Bounds::centered(Some(display_id), size, cx);
                Ok(WindowBounds::Maximized(bounds))
            },
            WindowType::Fullscreen => {
                let display_id = self.display_id(cx)?;
                let bounds = Bounds::centered(Some(display_id), size, cx);
                Ok(WindowBounds::Fullscreen(bounds))
            },
        }
    }

}

//
//
//

fn true_bool() -> bool {
    true
}

fn default_theme() -> String {
    "light".to_string()
}