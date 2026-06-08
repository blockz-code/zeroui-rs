use gpui::{
    Context,
    InteractiveElement, 
    SharedString, 
    StatefulInteractiveElement, 
    Window, 
    prelude::*, 
    div, px, rgb, rgba
};

use gpui_component::StyledExt;
use gpui_component::scroll::ScrollableElement;

pub struct CrashReporterUI {
    pub conn: super::Conn,

    #[allow(unused)]
    pub text: SharedString,

    current: i32,
    logs: Vec<LogResult>,
}

impl CrashReporterUI {

    fn prepare(conn: super::Conn, text: SharedString) -> Self {
        Self {
            conn: conn,
            text: text,

            current: 0,
            logs: Vec::new(),
        }
    }

    pub fn new(conn: super::Conn, text: SharedString) -> Self {
        let mut this = Self::prepare(conn, text);
        this.update_logs("logs");
        this
    }
    
    pub fn update_logs(&mut self, table: &str) {
        let conn = self.conn.lock().unwrap();

        let query = format!("SELECT id, timestamp, source, message FROM {} ORDER BY id DESC", table);

        let mut stmt = conn.prepare(&query).unwrap();

        let rows = stmt.query_map([], |row| {
            Ok(LogResult {
                id: row.get(0)?,
                timestamp: row.get(1)?,
                message: row.get(3)?
            })
        }).unwrap();

        self.logs.clear();

        for row in rows {
            let row = row.unwrap();
            self.logs.push(row);
        }
    }

    pub fn clear_db(&self) {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM logs", []).unwrap();
        conn.execute("DELETE FROM logs_error", []).unwrap();
        conn.execute("DELETE FROM logs_warning", []).unwrap();
        conn.execute("DELETE FROM logs_trace", []).unwrap();
        drop(conn);
    }

    pub fn send_report(&self) {}

}

impl Render for CrashReporterUI {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {

        let btn_bg = rgb(0xBBBBBBBB);
        let base_bg = rgb(0xEEEEEEEE);

        div()
            .flex()
            .flex_col()
            .bg(rgb(0xFFFFFFFF))
            .text_color(rgb(0x00000000))
            .w_full()
            .h_full()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_start()
                    .w_full()
                    .h_9()
                    .bg(base_bg)
                    .text_sm()
                    .font_semibold()
                    .child(
                        div()
                            .id("update-logs")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .hover(|s| s.bg(btn_bg))
                            .bg(if self.current == 0 { btn_bg } else { base_bg })
                            .child("Logs")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.update_logs("logs");
                                app.current = 0;
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
                    .child(
                        div()
                            .id("update-logs-warning")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .hover(|s| s.bg(btn_bg))
                            .bg(if self.current == 1 { btn_bg } else { base_bg })
                            .child("Warnings")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.update_logs("logs_warning");
                                app.current = 1;
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
                    .child(
                        div()
                            .id("update-logs-trace")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .hover(|s| s.bg(btn_bg))
                            .bg(if self.current == 2 { btn_bg } else { base_bg })
                            .child("Tracing")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.update_logs("logs_trace");
                                app.current = 2;
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
                    .child(
                        div()
                            .id("update-logs-error")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .hover(|s| s.bg(btn_bg))
                            .bg(if self.current == 3 { btn_bg } else { base_bg })
                            .child("Errors")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.update_logs("logs_error");
                                app.current = 3;
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
                    .child(div().flex().w_2().h_9().mx_1().child(""))
                    .child(
                        div()
                            .id("send-report")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .text_color(rgb(0xFFFFFFFF))
                            .hover(|s| s.bg(rgb(0x213448)))
                            .bg(rgb(0x547792))
                            .child("Send Report")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.send_report();
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
                    .child(div().flex().w_2().h_9().mx_1().child(""))
                    .child(
                        div()
                            .id("clear-db")
                            .flex()
                            .items_center()
                            .justify_center()
                            .px_5()
                            .h_9()
                            .cursor_pointer()
                            .text_color(rgb(0xFFFFFFFF))
                            .hover(|s| s.bg(rgb(0x9E3B3B)))
                            .bg(rgb(0xD25353))
                            .child("Clean Database")
                            .on_click(cx.listener(move |app,  _evt, _win, ctx| {
                                app.clear_db();
                                ctx.refresh_windows();
                                ctx.notify();
                            }))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .h_full()
                    .max_h_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .h_full()
                            .overflow_x_hidden()
                            .overflow_y_scrollbar()
                            .children({
                                let mut items = Vec::new();
                                for row in self.logs.iter() {
                                    let id = SharedString::from(format!("log-{}", row.id));
                                    items.push(
                                        div()
                                            .id(id)
                                            .flex()
                                            .w_full()
                                            .h_7()
                                            .text_sm()
                                            .hover(|s| s.bg(rgb(0xEEEEEEEE)))
                                            .border_b(px(1.0))
                                            .border_color(rgb(0xCCCCCCCC))
                                            .child(
                                                div()
                                                    .flex()
                                                    .px_5()
                                                    .bg(rgba(0xDDDDDD55))
                                                    .child(row.timestamp.clone())
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_full()
                                                    .pl_1()
                                                    .child({
                                                        let raw = row.message.clone();
                                                        let mut parts = raw.split("\n");
                                                        let msg = parts.next().unwrap();

                                                        format!("{}{}", msg, if parts.count() > 1 { "..." } else { "" })
                                                    })
                                            )
                                    );
                                }
                                if items.len() == 0 {
                                    items.push(
                                        div()
                                            .id("no-logs")
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .w_full()
                                            .h_full()
                                            .text_xl()
                                            .font_semibold()
                                            .child(match self.current {
                                                0 => "No Logs".to_string(),
                                                1 => "No Warnings".to_string(),
                                                2 => "No Traces".to_string(),
                                                3 => "No Errors".to_string(),
                                                _ => "No Logs".to_string()
                                            })
                                    );
                                }
                                items
                            })
                    )
            )
    }
}

#[derive(Debug)]
pub struct LogResult {
    id: i32,
    timestamp: String,
    message: String
}