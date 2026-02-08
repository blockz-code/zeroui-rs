



use gpui::{ Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, Window, WindowControlArea, div, px, rgba };



pub fn init(tb: crate::Titlebar, _window: &mut Window, _cx: &mut Context<crate::Win>) -> Div {
        
    let btn_size = px(15.0);
    let height = tb.height.unwrap_or(px(25.0).into());
    
    let text_color = tb.text_color.unwrap_or(rgba(0xFFFFFFFF));
    let back_color = tb.back_color.unwrap_or(rgba(0xFFFFFF00));

    let btn_min = tb.min_bg_color.unwrap_or(rgba(0x44cc45ff));
    let btn_min_h = tb.min_bg_hcolor.unwrap_or(rgba(0x48f048ff));

    let btn_max = tb.max_bg_color.unwrap_or(rgba(0xf8bb3aff));
    let btn_max_h = tb.max_bg_hcolor.unwrap_or(rgba(0xffb81fff));

    let btn_close = tb.close_bg_color.unwrap_or(rgba(0xf35d5bff));
    let btn_close_h = tb.close_bg_hcolor.unwrap_or(rgba(0xfd4542ff));

    tb.base()
        .bg(back_color)
        .h(height)
        .child(
            div()
                .flex()
                .items_center()
                .justify_end()
                .w(px(90.0))
                .h_full()
                .px(px(10.0))
                .gap(px(5.5))
                .child(
                    div()
                        .id("act-close")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_size)
                        .h(btn_size)
                        .bg(btn_close)
                        .hover(|s| s.bg(btn_close_h))
                        .child(" ")
                        .rounded_full()
                        .window_control_area(WindowControlArea::Close)
                )
                .child(
                    div()
                        .id("act-max")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_size)
                        .h(btn_size)
                        .bg(btn_max)
                        .hover(|s| s.bg(btn_max_h))
                        .child(" ")
                        .rounded_full()
                        .window_control_area(WindowControlArea::Max)
                )
                .child(
                    div()
                        .id("act-min")
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(btn_size)
                        .h(btn_size)
                        .bg(btn_min)
                        .hover(|s| s.bg(btn_min_h))
                        .child(" ")
                        .rounded_full()
                        .window_control_area(WindowControlArea::Min)
                )
        )
        .child(
            div()
                .flex()
                .items_center()
                .justify_start()
                .w(px(150.0))
                .h_full()
                .pl_2()
                .text_sm()
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

}