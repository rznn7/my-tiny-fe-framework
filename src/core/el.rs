use std::ops::Deref;

use leptos_reactive::create_effect;
use web_sys::{Element, Event, window};

use wasm_bindgen::JsCast;

use crate::core::dependency_injection::ComponentScope;

#[derive(Debug, Clone)]
pub struct El(Element);
impl El {
    pub fn new(tag_name: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element(tag_name).unwrap();

        Self(el)
    }

    pub fn on(self, event_name: &str, callback: impl FnMut(Event) + 'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let callback = Closure::wrap(Box::new(callback) as Box<dyn FnMut(Event)>);
        self.0
            .add_event_listener_with_callback(event_name, callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();
        self
    }

    pub fn attr(self, attr: &str, val: &str) -> Self {
        self.0.set_attribute(attr, val).unwrap();
        self
    }

    pub fn text(self, data: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node(data);
        self.0.append_child(&node).unwrap();
        self
    }

    pub fn child(self, node: El) -> Self {
        self.append_child(&node).unwrap();
        self
    }

    pub fn dyn_text(self, f: impl Fn() -> String + 'static) -> Self {
        let scope =
            ComponentScope::reactive_scope().expect("dyn_text called outside component context");
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node("");

        self.0.append_child(&node).unwrap();

        create_effect(scope, move |_| {
            let value = f();
            node.set_data(&value);
        });
        self
    }

    pub fn dyn_child(self, f: impl Fn() -> Option<El> + 'static) -> Self {
        let current_scope = ComponentScope::current_scope().unwrap();
        let scope = current_scope.scope();
        let window = window().unwrap();
        let document = window.document().unwrap();

        let container = document.create_element("div").unwrap();
        container
            .set_attribute("style", "display: contents")
            .unwrap();
        container.set_attribute("data-dyn-child", "").unwrap();

        self.0.append_child(&container).unwrap();

        let child_ctx = current_scope.create_child();

        create_effect(scope, move |_| {
            child_ctx.with(|| {
                container.set_inner_html("");
                if let Some(value) = f() {
                    let _ = container.append_child(&value);
                }
            });
        });

        self
    }
}

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
