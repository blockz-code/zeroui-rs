use zeroui_rs::gpui::Rgba;
use std::time::{ SystemTime, UNIX_EPOCH };

use zeroui_rs::{
    Builder, AppConfig, ContentBuilder, Win, gpui::{ Context, Div, FontWeight, InteractiveElement, ParentElement, SharedString, Stateful, StatefulInteractiveElement, Styled, div, px, rgb, rgba }
};





#[derive(Debug, serde::Deserialize)]
pub struct JsonTheme {
    pub app_bg : Rgba,
    pub app_text : Rgba,
    pub app_border : Rgba,
    pub app_content : Rgba,

    pub menu_item_bg : Rgba,
    pub menu_item_bg_h : Rgba,
    pub menu_item_text : Rgba,
    pub menu_item_icon : Rgba,
    pub menu_item_border : Rgba,

    pub footer_bg : Rgba,
    pub footer_text : Rgba,
    pub footer_border : Rgba
}





fn menu(items: &'static [(&str, &str)], win: &mut Win, cx: &mut Context<Win>) -> Stateful<Div> {
    div()
        .id("id")
        .flex()
        .flex_col()
        .w(px(200.0))
        .h_full()
        .p_1()
        .gap_1()
        .children(
            items
                .iter()
                .map(|(href, label)| {
                    let id = format!("nav-item-{}", href);
                    div()
                        .id(SharedString::new(id))
                        .flex()
                        .items_center()
                        .pl_3()
                        .w_full()
                        .h_10()
                        .child(*label)
                        .rounded_md()
                        .cursor_pointer()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .bg(if win.get_current() == *href { rgba(0x55555511) } else { rgba(0x00000000) })
                        .hover(|c| c.bg(rgba(0x55555511)))
                        .on_click(
                            cx.listener(move |w, _evt, _win, ctx| {
                                w.set_current(&href);
                                ctx.notify();
                            })
                        )
                })
        )
}


 
fn main() -> Result<(), anyhow::Error> {

    let content = ContentBuilder::new("/home")
        .add("/home", |_w, _app, _window, _ctx| {
            div()
                .id("content-home")
                .w_full()
                .h_full()
                .child("Home")
        })
        .add("/flash", |_w, _app, _window, _ctx| {
            div()
                .id("content-flash")
                .w_full()
                .h_full()
                .child("Flash")
        })
        .add("/download", |_w, _app, _window, _ctx| {
            div()
                .id("content-download")
                .w_full()
                .h_full()
                .child("Download")
        })
        .add("/settings", |_w, _app, _window, _ctx| {
            div()
                .id("content-settings")
                .w_full()
                .h_full()
                .child("Settings")
        })
        .build();

    let app_config = AppConfig::builder(
        include_bytes!("app.json"), 
        include_dir::include_dir!("examples/themes")
    )?;

    Builder::builder(app_config)?
        .set_default_window("main")
        .add_window("main", content, |w, app, win, ctx| {

            let theme  = app.use_theme::<JsonTheme>();

            div()
                .id("id")
                .w_full()
                .h_full()
                .bg(theme.app_bg)
                //.text_color(theme.app_text.parse_rgb())
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .h_full()
                        .child(
                            app
                                .get_titlebar().clone()
                                .title("Flasher")
                                .render(win, ctx)
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .w_full()
                                .h_full()
                                .child(
                                    menu(
                                        &[
                                            ("/home", "Home"),
                                            ("/flash", "Flash"),
                                            ("/download", "Download"),
                                            ("/settings", "Settings")
                                        ],
                                        w,
                                        ctx
                                    )
                                )
                                .child(w.get_content(win, ctx).unwrap())
                        )
                )
        })
        .run()?;

    Ok(())

}




fn _clock_div() -> Stateful<Div> {
    div()
        .id("id")
        .w_full()
        .h_full()
        .bg(rgb(0x111111))
        .text_color(rgb(0xEEEEEE))
        .child(
            div()
                .w_full()
                .flex()
                .items_center()
                .justify_start()
                .child("clock:")
                .child(_clock())
        )
}


fn _clock() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let hours = (now / 3600) % 24;
    let minutes = (now / 60) % 60;
    let seconds = now % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}