




pub fn quit_app(win: &mut gpui::Window, cx: &mut gpui::Context<'_, gpui::App>) {
    cx.quit();
}

pub fn close_window(win: &mut gpui::Window, cx: &mut gpui::Context<'_, gpui::App>) {
    win.remove_window();
}