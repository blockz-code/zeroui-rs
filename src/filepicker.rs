use gpui::{Div, InteractiveElement, IntoElement, Stateful};





#[allow(unused)]
#[derive(Debug, Clone)]
pub struct FilePicker {
    path : String,
    filter : String,
}





impl FilePicker {

    pub fn new() -> Self {
        Self {
            path : String::new(),
            filter : String::new(),
        }
    }

    pub fn drives(&self) -> crate::Result<()> {
    
        Ok(())

    }

    pub fn list(&self, _path: &str) -> crate::Result<()> {

        Ok(())

    }


    pub fn layout(&self, callback: fn() -> Stateful<Div>) -> impl IntoElement + use<> {

        callback()

    }

}






#[allow(unused)]
fn testi() {
    let x = FilePicker::new();

    x.drives().unwrap();

    x.list("c:\\").unwrap();

    x.layout(|| {

        gpui::div()
            .id("id")

    });


}
