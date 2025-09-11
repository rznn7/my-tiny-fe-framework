use leptos_reactive::{ReadSignal, SignalGet, SignalUpdate, WriteSignal, create_signal};

use crate::core::{
    component::{Component, ComponentContext},
    el::El,
};

// ============= Components =============

pub struct DependencyInjectionExample;
impl Component for DependencyInjectionExample {
    fn render(&self) -> El {
        El::new("div")
            .attr(
                "style",
                r#"
                    display: flex;
                    width: 100vw;
                    height: 100vh;
                    background-color: #f2f2f2;
                    font-family: monospace;
                "#,
            )
            .child(
                El::new("div")
                    .child(El::new("h1").text("Dependency Injection Example"))
                    .child(
                        RootCounterProvider {
                            title: "Root Provider (A)",
                        }
                        .mount(),
                    ),
            )
    }
}

pub struct RootCounterProvider {
    title: &'static str,
}
impl Component for RootCounterProvider {
    fn render(&self) -> El {
        ComponentContext::provide(CounterService::new());
        let counter_service = ComponentContext::inject::<CounterService>().unwrap();
        let counter_increment = counter_service.clone();
        let counter_display = counter_service.clone();

        El::new("div")
            .attr(
                "style",
                r#"
                         display: flex;
                         flex-direction: column;
                         gap: 5px;
                         box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                         border-radius: 5px;
                         background-color: #fff;
                         padding: 5px;
                "#,
            )
            .child(El::new("h2").text(self.title))
            .child(El::new("span").text("I am providing counter service to my children."))
            .child(
                El::new("div")
                    .attr("style", "display: flex; flex-direction: column; gap: 10px")
                    .child(El::new("button").text("+1").on("click", move |_| {
                        counter_increment.borrow().increment();
                    }))
                    .child(El::new("span").dyn_text(move || {
                        format!(
                            "Count is: {}",
                            counter_display.borrow().get_count_signal().get()
                        )
                    })),
            )
            .child(
                MiddleCounterConsumer {
                    title: "Service Consumer (B)",
                }
                .mount(),
            )
    }
}

struct MiddleCounterConsumer {
    title: &'static str,
}
impl Component for MiddleCounterConsumer {
    fn render(&self) -> El {
        let counter_service = ComponentContext::inject::<CounterService>().unwrap();
        let counter_decrement = counter_service.clone();

        El::new("div")
            .attr(
                "style",
                r#"
                         display: flex;
                         flex-direction: column;
                         gap: 5px;
                         box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                         border-radius: 5px;
                         background-color: #f9d4ff;
                         padding: 5px;
                "#,
            )
            .child(El::new("h2").text(self.title))
            .child(El::new("span").text("I am using counter service from my parent provider."))
            .child(
                El::new("button")
                    .text("-1")
                    .on("click", move |_| counter_decrement.borrow().decrement()),
            )
            .child(
                NestedCounterProvider {
                    title: "Nested Provider (C)",
                }
                .mount(),
            )
    }
}

pub struct NestedCounterProvider {
    title: &'static str,
}
impl Component for NestedCounterProvider {
    fn render(&self) -> El {
        ComponentContext::provide(CounterService::new());
        let counter_service = ComponentContext::inject::<CounterService>().unwrap();
        let counter_increment = counter_service.clone();
        let counter_display = counter_service.clone();

        El::new("div")
            .attr(
                "style",
                r#"
                         display: flex;
                         flex-direction: column;
                         gap: 5px;
                         box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
                         border-radius: 5px;
                         background-color: #ffc8ba;
                         padding: 5px;
                "#,
            )
            .child(El::new("h2").text(self.title))
            .child(
                El::new("span").text("I am providing a NEW counter service (shadowing parent's)."),
            )
            .child(
                El::new("div")
                    .attr("style", "display: flex; flex-direction: column; gap: 10px")
                    .child(El::new("button").text("+1").on("click", move |_| {
                        counter_increment.borrow().increment();
                    }))
                    .child(El::new("span").dyn_text(move || {
                        format!(
                            "Count is: {}",
                            counter_display.borrow().get_count_signal().get()
                        )
                    })),
            )
    }
}

// ============= Services =============

struct CounterService {
    count: (ReadSignal<i32>, WriteSignal<i32>),
    total_clicks: (ReadSignal<i32>, WriteSignal<i32>),
}
impl CounterService {
    fn new() -> Self {
        let scope = ComponentContext::scope().unwrap();

        Self {
            count: create_signal(scope, 0),
            total_clicks: create_signal(scope, 0),
        }
    }

    fn increment(&self) {
        self.count.1.update(|n| *n += 1);
        self.total_clicks.1.update(|n| *n += 1);
    }

    fn decrement(&self) {
        self.count.1.update(|n| *n = n.saturating_sub(1));
        self.total_clicks.1.update(|n| *n += 1);
    }

    fn get_count_signal(&self) -> ReadSignal<i32> {
        self.count.0
    }
}
