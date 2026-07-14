use std::collections::HashMap;
use std::path::PathBuf;

// use std::sync::mpsc::{Receiver, Sender};

pub use gpui;
use gpui::{ KeyBinding, actions };
use gpui::{ AppContext, Context, Div, Global, SharedString, Stateful, TitlebarOptions, Window, WindowOptions };



pub mod components;

mod assets;
mod config;
mod content;
mod database;
mod dirs;
mod filepicker;
mod http;
mod themes;
mod titlebar;
mod updater;
mod window;

// CRASHREPOTER
mod crashreporter;
pub use crashreporter::CrashReporter;


pub use dirs::{ Dirs, ExportDirsExt };
pub use window::Win;
pub use assets::Assets;
pub use content::Content;
pub use updater::Updater;
pub use titlebar::Titlebar;


pub use http::{
    Method,
    FetchType,
    RequestBuilder,
    Fetch,
    FetchBuilder,
    FetchClient
};

pub use config::{
    AppConfig,
    Config,
    WindowConfig,
    WindowType,
    WindowConfigKind,
    WindowConfigDecorated
};

pub use themes::{
    Theme,
    Themes
};

pub use content::{ContentBuilder, ContentOutput};

use assets::InternalAssets;

use filepicker::FilePicker;

use std::sync::Arc;
use std::sync::Mutex;

use crate::database::Database;








actions!(
    zeroui,
    [
        CloseWindow,
    ]
);







#[derive(Clone)]
pub struct Application {
    version: Option<String>,
    config: Option<config::AppConfig>,

    database: Option<Database>,

    http: Fetch,
    updater: Option<Updater>,
    titlebar : Option<Titlebar>,

    windows: Arc<Mutex<HashMap<String, Win>>>,

    #[allow(unused)]
    filepicker: Option<FilePicker>,
}

impl Application {
    //
    pub fn new() -> Self {

        Self {
            config : None,
            updater : None,
            version : None,
            database : None,

            http : Fetch::new(),
            titlebar : Some(Titlebar::new()),

            windows : Arc::new(Mutex::new(HashMap::new())),

            filepicker : None,
        }
    }
    //
    
    pub fn themes(&self) -> Vec<Theme> {
        self.config.as_ref().unwrap().themes()
    }
    //
    pub fn set_theme(&mut self, name: &str) {
        self.config.as_mut().unwrap().set_active_theme(name);
    }
    //
    pub fn use_theme<T: for<'a> serde::Deserialize<'a> + Send + Sync + 'static>(&self) -> T {
        self.config.as_ref().unwrap().active_theme().parse::<T>().unwrap()
    }
    //
    pub fn get_version(&mut self) -> Option<&String> {
        self.version.as_ref()
    }
    //
    pub fn fetch(&self) -> &Fetch {
        &self.http
    }
    //
    pub fn fetch_builder(&self) -> FetchBuilder {
        self.fetch().builder()
    }
    //
    pub fn fetch_client(&self, typ: FetchType, url: &str, method: Method) -> RequestBuilder {
        self.fetch().req(typ, url, method)
    }
    //
    pub fn dirs(&self) -> Dirs {
        self.config.as_ref().unwrap().dirs()
    }
    //
    pub fn config(&self) -> Config {
        self.config.as_ref().unwrap().config()
    }
    //
    pub fn updater(&self) -> &Updater {
        self.updater.as_ref().unwrap()
    }
    //
    pub fn updater_mut(&mut self) -> &mut Updater {
        self.updater.as_mut().unwrap()
    }
    //
    pub fn database(&self) -> &Database {
        self.database.as_ref().unwrap()
    }
    //
    pub fn get_titlebar(&self) -> &Titlebar {
        self.titlebar.as_ref().unwrap()
    }
    //
    pub fn get_window(&mut self, label: &str, cx: &mut gpui::App) -> Result<(Win, WindowOptions)> {
        let config = self.config();
        let options = self.window_options(cx, SharedString::from(label.to_string()), &config.identifier)?;
        
        let layout_guard = self.windows.lock().unwrap();
        let layout = layout_guard.get(label).ok_or(Error::WindowNotFound(label.to_string()))?.clone();
        drop(layout_guard);
        Ok((layout, options))
    }
    //
    pub fn window_options(&mut self, cx: &mut gpui::App, label: SharedString, apid: &str) -> Result<WindowOptions> {
        let config = self.config();
        let win_config = config.windows.iter().find(|p| p.label == label).ok_or(Error::WindowNotFound(label.to_string()))?;
        let bounds = win_config.window_bounds(cx)?;
        Ok(WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some(win_config.label.clone().into()),
                appears_transparent: !win_config.decorated,
                ..Default::default()
            }),
            window_bounds: Some(bounds),
            window_min_size: Some(bounds.get_bounds().size),
            kind: win_config.kind(),
            app_id: Some(apid.to_string()),
            window_decorations: Some(win_config.decorated_kind()),
            focus: win_config.focus,
            show: win_config.show,
            is_movable: win_config.is_movable, 
            is_resizable: win_config.is_resizable,
            is_minimizable: win_config.is_minimizable,
            ..Default::default()
        })
    }
    //
    pub fn open_window(&mut self, cx: &mut gpui::App, w: Win, options: WindowOptions) -> Result<()> {
        #[cfg(feature = "gpui-component")]
        cx.open_window(options, move |win, ctx| ctx.new(|cx| gpui_component::Root::new(ctx.new(|_| w), win, cx)))?;
        #[cfg(not(feature = "gpui-component"))]
        cx.open_window(options, move |_win, ctx| ctx.new(|_| w))?;
        Ok(())
    }
    //
}

impl Global for Application {}












pub struct Builder {
    main: Application,
    window_name: String,

    assets: Option<Assets>,
    multi_assets: Option<Assets>,
}

impl Builder {
    //
    fn new() -> Self {
        let app = Application::new();
        Self {
            main: app,
            window_name: String::new(),

            assets: None,
            multi_assets: None,
        }
    }
    //
    pub fn builder(config: config::AppConfig) -> Result<Self> {
        Ok(Self::new().pre_init(config)?)
    }
    //
    fn pre_init(mut self, config: config::AppConfig) -> Result<Self> {
        let app = &mut self.main;
        app.config = Some(config);
        Ok(self)
    }
    //
    pub fn set_version(mut self, v: &str) -> Self {
        self.main.version = Some(v.to_string());
        self
    }
    //
    pub fn database(mut self, database: PathBuf, name: &str) -> Self {
        self.main.database = Some(Database::new(database, name).unwrap());
        self
    }
    pub fn database_in_memory(mut self) -> Self {
        self.main.database = Some(Database::new_memory().unwrap());
        self
    }
    //
    pub fn set_default_window(mut self, label: &str) -> Self {
        self.window_name = label.to_string();
        self
    }
    //
    pub fn set_assets(mut self, path: &str) -> Self {
        self.assets = Some(Assets::new(path));
        self
    }
    //
    pub fn set_multi_assets(mut self, path: Vec<&str>) -> Self {
        self.assets = Some(Assets::multi(path));
        self
    }
    //
    pub fn updater(mut self, updater: updater::Updater) -> Self {
        self.main.updater = Some(updater.into());
        self
    }
    //
    pub fn add_window(self, label: &str, content: ContentOutput, layout: fn(app: &mut Win, app: &mut Application, window: &mut Window, cx: &mut Context<Win>) -> Stateful<Div>) -> Self {
        let mut guard = self.main.windows.lock().unwrap();
        guard.insert(label.to_string(), Win::new(self.main.clone(), content, layout));
        drop(guard);
        self
    }
    


    pub fn run<'a>(mut self) -> Result<()> {

        gpui_platform::application()
            .with_assets(if self.assets.is_none() { Assets::empty() } else { self.assets.unwrap() })
            .with_assets(if self.multi_assets.is_none() { Assets::empty() } else { self.multi_assets.unwrap() })
            .with_assets(InternalAssets)
            .run(move |cx: &mut gpui::App| {
                
                #[cfg(feature = "gpui-component")]
                gpui_component::init(cx);

                cx.activate(true);
                cx.on_action(|_: &CloseWindow, cx| cx.quit());
                cx.bind_keys([KeyBinding::new("ctrl-q", CloseWindow, None)]);

                //cx.on_app_quit( move |app| on_app_quit(app, &mut main)).detach();

                cx.on_window_closed(|cx, _wid| {
                    if cx.windows().is_empty() {
                        cx.quit();
                    }
                }).detach();

                let (w, options) = self.main.get_window(&self.window_name, cx).unwrap();

                cx.spawn(async move |cx| {
                    #[cfg(feature = "gpui-component")]
                    cx.open_window(options, move |win, ctx| ctx.new(|cx| gpui_component::Root::new(view = ctx.new(|_| w), win, cx))).unwrap();
                    #[cfg(not(feature = "gpui-component"))]
                    cx.open_window(options, move |_win, ctx| ctx.new(|_| w)).unwrap();
                    Ok::<_, anyhow::Error>(())
                })
                .detach();
            });

        Ok(())

    }
    //
}











// Errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
    //
    #[error(transparent)]
    Io(#[from] std::io::Error),
    //
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    //
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    // rusqlite::Error
    #[error(transparent)]
    RuSqliteError(#[from] rusqlite::Error),
    //
    #[error(transparent)]
    CargoPackagerUpdaterError(#[from] cargo_packager_updater::Error),
    //
    #[error("app is not initialized error:[{0}]")]
    AppNotInitialized(String),
    //
    #[error("views layout [{0}] not found.")]
    ViewsLayoutNotFound(String),
    //
    #[error("window with label [{0}] not found.")]
    WindowNotFound(String),
    //
    #[error("no windows available.")]
    NoWindowsAvailable,
    //
    #[error("no config available.")]
    NoConfigAvailable,
    //
    #[error("no window config available.")]
    NoWindowConfigAvailable,
    //
    #[error("no startup window found.")]
    NoStartupWindowFound,
    //
    #[error("display not found.")]
    DisplayFound,
    //
    #[error("no startup window found.")]
    NoDisplayFound,
    //
    #[error("no action emitter found.")]
    NoActionEmitterFound,
    //
    #[error("titlebar not found")]
    TitlebarNotFound,
    //
    #[error("no window available for layout [{0}].")]
    NoWindowAvailable(String),
    //
    #[error("dir [{0}] not found.")]
    DirNotFound(String),
    //
    #[error("binary [{0}] not found in PATH")]
    LibBinaryPathError(String),
    //
    #[error("{0}")]
    CustomLibError(String),
    //
    #[error("unknown error: {0}")]
    Unknown(String),
    //
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
