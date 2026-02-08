#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
mod linux;
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
pub use linux::init;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::init;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::init;

use gpui::{ Context, Div, IntoElement, Length, Pixels, Rgba, Styled, Window, div };

#[derive(Default, Debug, Clone)]
pub struct Titlebar {
    title : Option<String>,
    size : Option<Pixels>,
    height : Option<Length>,

    text_color : Option<Rgba>,
    icon_color : Option<Rgba>,
    back_color : Option<Rgba>,

    min_bg_color  : Option<Rgba>,
    min_bg_hcolor : Option<Rgba>,
    max_bg_color  : Option<Rgba>,
    max_bg_hcolor : Option<Rgba>,
    close_bg_color  : Option<Rgba>,
    close_bg_hcolor : Option<Rgba>,
}

impl Titlebar {
    
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn icon_size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn height(mut self, height: impl std::clone::Clone + Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn text_color(mut self, color: Rgba) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn icon_color(mut self, color: Rgba) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn bg(mut self, color: Rgba) -> Self {
        self.back_color = Some(color);
        self
    }
    
    pub fn button_min(mut self, color: Rgba, hover_color: Option<Rgba>) -> Self {
        self.min_bg_color = Some(color);
        self.min_bg_hcolor = hover_color;
        self
    }
    
    pub fn button_max(mut self, color: Rgba, hover_color: Option<Rgba>) -> Self {
        self.max_bg_color = Some(color);
        self.max_bg_hcolor = hover_color;
        self
    }
    
    pub fn button_close(mut self, color: Rgba, hover_color: Option<Rgba>) -> Self {
        self.close_bg_color = Some(color);
        self.close_bg_hcolor = hover_color;
        self
    }

    pub fn render(self, window: &mut Window, cx: &mut Context<super::Win>) -> impl IntoElement + use<> {
        init(self, window, cx)
    }

    fn base(&self) -> Div {
        div()
            .flex()
            .flex_row()
            .w_full()
    }

}