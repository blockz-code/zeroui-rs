use gpui::{InteractiveElement, Styled, div};




pub fn chart() -> impl InteractiveElement + Styled {
    div()
        .id("id")
        .w_full()
        .h_full()
}