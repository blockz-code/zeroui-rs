



use gpui::{ Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, Window, WindowControlArea, div, px, rgba, svg };



pub fn init(tb: crate::Titlebar, window: &mut Window, _cx: &mut Context<crate::Win>) -> Div {
        
    let btn_width = px(45.32);
    let height = tb.height.unwrap_or(px(35.0).into());

    let icon_size = tb.size.unwrap_or(px(14.0));

    let icon_color = tb.icon_color.unwrap_or(rgba(0xFFFFFFFF));
    let text_color = tb.text_color.unwrap_or(rgba(0xFFFFFFFF));
    let back_color = tb.back_color.unwrap_or(rgba(0xFFFFFF00));

    let btn_min = tb.min_bg_color.unwrap_or(rgba(0xFFFFFF00));
    let btn_min_h = tb.min_bg_hcolor.unwrap_or(rgba(0xFFFFFF11));

    let btn_max = tb.max_bg_color.unwrap_or(rgba(0xFFFFFF00));
    let btn_max_h = tb.max_bg_hcolor.unwrap_or(rgba(0xFFFFFF11));

    let btn_close = tb.close_bg_color.unwrap_or(rgba(0xFFFFFF00));
    let btn_close_h = tb.close_bg_hcolor.unwrap_or(rgba(0xE81123ff));

    tb.base()
        .bg(back_color)
        .h(height)
        .child(
                div()
                    .flex()
                    .items_center()
                    .justify_start()
                    .w(px(150.0))
                    .h_full()
                    .pl_3()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(text_color)
                    .child(tb.title.unwrap_or("No Title".to_string()))
                    .window_control_area(WindowControlArea::Drag)
        )
        .child(
                div()
                    .id("titlebar-space")
                    .flex()
                    .items_center()
                    .justify_center()
                    .w_full()
                    .h_full()
                    .child("")
                    .window_control_area(WindowControlArea::Drag)
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_end()
                .w(px(150.0))
                .h_full()
                .gap(px(0.5))
                .child(
                    div()
                        .id("act-min")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_width)
                        .h_full()
                        .bg(btn_min)
                        .hover(|s| s.bg(btn_min_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path("titlebar/w-min.svg")
                        )
                        .window_control_area(WindowControlArea::Min)
                )
                .child(
                    div()
                        .id("act-max")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_width)
                        .h_full()
                        .bg(btn_max)
                        .hover(|s| s.bg(btn_max_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path(format!("titlebar/{}", if window.is_maximized() { "w-unmax.svg" } else { "w-max.svg" }))
                        )
                        .window_control_area(WindowControlArea::Max)
                )
                .child(
                    div()
                        .id("act-close")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_width)
                        .h_full()
                        .bg(btn_close)
                        .hover(|s| s.bg(btn_close_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path("titlebar/w-close.svg")
                        )
                        .window_control_area(WindowControlArea::Close)
                )
        )

}