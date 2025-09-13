use leptos_reactive::{create_runtime, create_scope};
use web_sys::window;

use crate::core::{component::Component, dependency_injection::ComponentScope};

pub fn start_app<T: Component + 'static>(root_component: T) {
    let runtime = create_runtime();
    let _scope = create_scope(runtime, |scope| {
        let root_context = ComponentScope::new(scope);

        root_context.with(|| {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();

            let root_el = root_component.mount();
            body.append_child(&root_el).unwrap();
        });
    });
}
