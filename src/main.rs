use crate::{core::runtime::start_app, examples::dependency_injection::DependencyInjectionExample};

mod core;
mod examples;

fn main() {
    start_app(DependencyInjectionExample);
}
