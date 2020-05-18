use crate::Component;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;

#[derive(Default)]
pub struct App {
    routes: BTreeMap<&'static str, Box<dyn Component>>,
    start: Option<&'static str>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: BTreeMap::new(),
            start: None,
        }
    }

    pub fn push(&mut self, route: &'static str, component: Box<dyn Component>) -> &mut Self {
        self.routes.insert(route, component);
        self.start = Some(route);
        self
    }

    pub fn start(&mut self, start: &'static str) -> &mut Self {
        self.start = Some(start);
        self
    }

    pub fn render(&mut self) {
        if let Some(x) = body().first_child() {
            body()
                .replace_child(
                    &self.routes.get_mut(self.start.unwrap()).unwrap().view(),
                    &x,
                )
                .unwrap();
        } else {
            body()
                .append_child(&self.routes.get_mut(self.start.unwrap()).unwrap().view())
                .unwrap();
        }
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("No global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("Should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("Document should have a body")
}
