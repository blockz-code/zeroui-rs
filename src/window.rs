use std::collections::HashMap;

use gpui::{ AppContext, Context, Div, IntoElement, Render, Stateful, Window };



use crate::{Application, Content, Result, Error};




#[derive(Clone)]
pub struct Win {
    app: Application,

    current: String,
    content: HashMap<String, Content>,

    layout: fn(win: &mut Win, app: &mut Application, window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div>
}

impl Win {

    pub fn new(
        app: Application,
        from_builder: (String, HashMap<String, Content>),
        layout: fn(win: &mut Win, app: &mut Application, window: &mut Window, cx: &mut Context<Self>) -> Stateful<Div>
    ) -> Self {
        Win {
            app,
            layout,
            current : from_builder.0,
            content : from_builder.1,
        }
    }
    
    pub fn get_app(self) -> Application {
        self.app
    }

    pub fn get_app_mut<'a>(&'a mut self) -> &'a mut Application {
        &mut self.app
    }

    pub fn get_content(&self, win: &mut Window, cx: &mut Context<Win>) -> Result<impl IntoElement + use<>> {
        let mut app = self.app.clone();
        let id = self.current.clone();
        let mut content = self.content.get(&id).ok_or(Error::ViewsLayoutNotFound(id)).cloned()?;
        Ok(content.render(&mut app, win, cx))
    }
    
    pub fn set_current(&mut self, label: &str) {
        self.current = label.to_string();
    }
    
    pub fn get_current(&mut self) -> &str {
        self.current.as_str()
    }

    pub fn create_window(&mut self, cx: &mut Context<'_, Win>, label: &str) -> Result<()> {
        let (w, options) = self.app.get_window(label, cx)?;
        #[cfg(feature = "gpui-component")]
        cx.open_window(options, move |win, cx| cx.new(|cx| gpui_component::Root::new(cx.new(|_| w), win, cx))).unwrap();
        #[cfg(not(feature = "gpui-component"))]
        cx.open_window(options, move |_win, cx| cx.new(|_| w)).unwrap();
        Ok(())
    }

}

impl Render for Win {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Win>) -> impl IntoElement {
        let mut app = self.app.clone();
        (self.layout)(self, &mut app, win, cx)
    }
}