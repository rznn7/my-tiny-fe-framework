use std::{cell::RefCell, rc::Rc};

use leptos_reactive::Scope;

use crate::core::service::ServiceContainer;

thread_local! {
    static SCOPE_STACK: RefCell<Vec<ComponentScope>> = const { RefCell::new(Vec::new()) };
}

#[derive(Clone)]
pub struct ComponentScope {
    scope: Scope,
    services: Rc<RefCell<ServiceContainer>>,
}

impl ComponentScope {
    pub fn new(scope: Scope) -> Self {
        Self {
            scope,
            services: Rc::new(RefCell::new(ServiceContainer::new())),
        }
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }

    pub fn with<R>(&self, f: impl FnOnce() -> R) -> R {
        SCOPE_STACK.with(|stack| {
            stack.borrow_mut().push(self.clone());
        });

        let result = f();

        SCOPE_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });

        result
    }

    pub fn current_scope() -> Option<Self> {
        SCOPE_STACK.with(|stack| stack.borrow().last().cloned())
    }

    pub fn provide<T: std::any::Any + 'static>(service: T) {
        if let Some(ctx) = Self::current_scope() {
            ctx.services.borrow_mut().register(service);
        }
    }

    pub fn inject<T: std::any::Any + 'static>() -> Option<Rc<RefCell<T>>> {
        Self::current_scope()?.services.borrow().get::<T>()
    }

    pub fn reactive_scope() -> Option<Scope> {
        Self::current_scope().map(|ctx| ctx.scope)
    }

    pub fn create_child(&self) -> Self {
        let mut new_container = ServiceContainer::new();
        for (type_id, service_rc) in self.services.borrow().services() {
            new_container
                .services_mut()
                .insert(*type_id, service_rc.clone());
        }
        Self {
            scope: self.scope,
            services: Rc::new(RefCell::new(new_container)),
        }
    }
}
