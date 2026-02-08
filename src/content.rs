use std::collections::HashMap;

use gpui::{ Context, Div, IntoElement, Stateful, Window };

use crate::{Application, Win};


#[derive(Debug, Clone)]
pub struct Content {
    layout: fn(win: &mut Content, app: &mut Application, window: &mut Window, cx: &mut Context<Win>) -> Stateful<Div>
}

impl Content {

    pub fn new(layout: fn(win: &mut Content, app: &mut Application, window: &mut Window, cx: &mut Context<Win>) -> Stateful<Div>) -> Self {
        Content { layout }
    }

    pub fn render(&mut self, app: &mut Application, window: &mut Window, cx: &mut Context<Win>) -> impl IntoElement + use<> {
        (self.layout)(self, app, window, cx)
    }

}



pub type ContentOutput = (String, HashMap<String, Content>);

pub struct ContentBuilder {
    current: String,
    content: HashMap<String, Content>,
}

impl ContentBuilder {
    pub fn new(default: &str) -> Self {
        ContentBuilder {
            current: default.to_string(),
            content: HashMap::new(),
        }
    }

    pub fn add(mut self, href: &str, layout: fn(win: &mut Content, app: &mut Application, window: &mut Window, cx: &mut Context<Win>) -> Stateful<Div>) -> Self {
        self.content.insert(href.to_string(), Content::new(layout));
        self
    }

    pub fn build(&self) -> ContentOutput {
        (self.current.clone(), self.content.clone())
    }

}