mod app;

use super::{ Database, database::Conn };


use env_logger::Builder;
use log::Level;
use gpui::{App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, size, px, prelude::*};
use tokio::runtime::Builder as AsyncRuntimeBuilder;
use std::path::PathBuf;





static INIT_SQL: &str = include_str!("_init.sql");

pub struct CrashReporter {
    db : Database,
    builder: Builder
}


impl CrashReporter {

    fn new(dir: PathBuf, name: &str) -> Self {
        Self {
            builder: Builder::from_default_env(),
            db: Database::new(dir.join("logs"), name).unwrap()
        }
    }

    pub fn builder(dir: PathBuf, name: &str) -> Self {
        let this = Self::new(dir, name);
        let conn = this.db.conn.lock().unwrap();
        conn.execute_batch(INIT_SQL).unwrap();
        drop(conn);
        this
    }

    pub fn close(&self) -> crate::Result<()> {
        self.db.clone().close();
        Ok(())
    }

    pub fn log(&self, level: Level, source: &str, code: &str, message: &str) {
        let conn = self.db.conn.lock().unwrap();
        match level {
            Level::Info => conn.execute("INSERT INTO logs (source, code, message) VALUES (?, ?, ?)", &[source, code, message]).unwrap(),
            Level::Error => conn.execute("INSERT INTO logs_error (source, code, message) VALUES (?, ?, ?)", &[source, code, message]).unwrap(),
            Level::Warn => conn.execute("INSERT INTO logs_warning (source, code, message) VALUES (?, ?, ?)", &[source, code, message]).unwrap(),
            Level::Trace => conn.execute("INSERT INTO logs_trace (source, code, message) VALUES (?, ?, ?)", &[source, code, message]).unwrap(),
            _ => 0
        };
        drop(conn);
    }

    pub fn clear_db(&self) {
        let conn = self.db.conn.lock().unwrap();
        conn.execute("DELETE FROM logs", []).unwrap();
        conn.execute("DELETE FROM logs_error", []).unwrap();
        conn.execute("DELETE FROM logs_warning", []).unwrap();
        conn.execute("DELETE FROM logs_trace", []).unwrap();
        drop(conn);
    }

    pub fn run(&mut self) {
        let guard = self.db.conn.clone();
        self.builder.format(move |_buf, record: &log::Record<'_>| {

            let conn = guard.lock().unwrap();

            //let timestamp = buf.timestamp();

            let message = format!("{:?}", record.args());
            
            match record.level() {
                Level::Info => conn.execute("INSERT INTO logs (source, code, message) VALUES (?, ?, ?)", &[record.target(), "1", &message]).unwrap(),
                Level::Error => conn.execute("INSERT INTO logs_error (source, code, message) VALUES (?, ?, ?)", &[record.target(), "1", &message]).unwrap(),
                Level::Warn => conn.execute("INSERT INTO logs_warning (source, code, message) VALUES (?, ?, ?)", &[record.target(), "1", &message]).unwrap(),
                Level::Trace => conn.execute("INSERT INTO logs_trace (source, code, message) VALUES (?, ?, ?)", &[record.target(), "1", &message]).unwrap(),
                _ => 0
            };
            //writeln!(buf, "{}", format!("[{}][{}] {}", record.level().as_str(), timestamp, record.args()))

            drop(conn);

            Ok(())

        });
        //self.builder.target(Target::Stdout);
        self.builder.init();
    }

    //
    //
    //
    //
    // View Window
    //
    //
    //

    pub fn open_reporter_window(&self) {

        let title = "Crash Reporter";

        let conn = self.db.conn.clone();

        Application::new().run(|acx: &mut App| {

            gpui_component::init(acx);

            let bounds = Bounds::centered(None, size(px(1000.), px(500.0)), acx);

            acx.open_window(
                WindowOptions {
                    titlebar: Some(TitlebarOptions {
                        title: Some(title.into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |win, cx| {

                    let view = cx.new(|_| app::CrashReporterUI::new(conn, title.into()));

                    cx.new(|ctx| gpui_component::Root::new(view, win, ctx))

                },
            ).unwrap();

            acx.activate(true);

        });

    }

    async fn open_async_reporter_window(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.open_reporter_window();
        Ok(())
    }

    pub fn open_crash_reporter(&self) -> crate::Result<()> {
        let art = AsyncRuntimeBuilder::new_current_thread().enable_all().build().unwrap();
        art.block_on(self.open_async_reporter_window()).map_err(|e| crate::Error::Unknown(format!("{}", e)))?;
        Ok(())
    }

}