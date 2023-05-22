use dioxus::prelude::*;

use crate::{hooks::use_route, routable::Routable};

#[derive(Clone)]
pub(crate) struct OutletContext {
    pub current_level: usize,
}

pub(crate) fn use_outlet_context(cx: &ScopeState) -> &OutletContext {
    let outlet_context = use_context(cx).unwrap();
    outlet_context
}

impl OutletContext {
    pub(crate) fn render<R: Routable + Clone>(cx: &ScopeState) -> Element<'_> {
        let outlet = use_outlet_context(cx);
        let current_level = outlet.current_level;
        cx.provide_context({
            OutletContext {
                current_level: current_level + 1,
            }
        });

        use_route::<R>(cx)
            .expect("Outlet must be inside of a router")
            .render(cx, current_level)
    }
}
