



use gpui::{ Context, Div, FontWeight, InteractiveElement, MouseButton, ParentElement, StatefulInteractiveElement, Styled, Window, WindowControlArea, div, px, rgba, svg };



pub fn init(tb: crate::Titlebar, window: &mut Window, cx: &mut Context<crate::Win>) -> Div {
        
    let btn_width = px(40.0);

    let height = tb.height.unwrap_or(px(35.0).into());

    let icon_size = tb.size.unwrap_or(px(16.0));

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
                    .child(tb.title.unwrap_or("No Title.".to_string()))
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        window.start_window_move();
                    })
        )
        .child(
                div()
                    .id("titlebar-space")
                    .flex()
                    .items_center()
                    .justify_center() 
                    .w_full()
                    .h_full()
                    .child("")// or any draggable area
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| {
                        window.start_window_move();
                    })
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_end()
                .w(px(150.0))
                .gap_1()
                .p_1()
                .h_full()
                .child(
                    div()
                        .id("act-min")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_width)
                        .h_full()
                        .bg(btn_min)
                        .rounded_md()
                        .hover(|s| s.bg(btn_min_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path("titlebar/l-min.svg")
                        )
                        .on_click(cx.listener(|_, _, window, ctx| {
                            window.minimize_window(); 
                            ctx.notify();
                        }))
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
                        .rounded_md()
                        .hover(|s| s.bg(btn_max_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path(format!("titlebar/{}", if window.is_maximized() { "l-unmax.svg" } else { "l-max.svg" }))
                        )
                        .on_click(cx.listener(|_, _, window, ctx| {
                            window.toggle_fullscreen();
                            ctx.notify();
                        }))
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
                        .rounded_md()
                        .hover(|s| s.bg(btn_close_h))
                        .child(
                            svg()
                                .w(icon_size)
                                .h(icon_size)
                                .text_color(icon_color)
                                .path("titlebar/l-close.svg")
                        )
                        .on_click(cx.listener(|w, evt, ww, ctx| {
                            ww.remove_window();
                            ctx.notify();
                        }))
                )
        )

}