use wasm_bindgen::prelude::*;

use moxie_dom::{
    elements::{html::*, text_content::*},
    prelude::{event::*, *},
};

#[topo::nested]
pub fn app() -> Div {
    let (value, update) = moxie::state(|| 0);
    tracing::info!("RENDER with {}", value);

    let attr = format!("iter-{}", value);

    let input = moxie_dom::elements::forms::input()
        .class(&attr)
        .attribute("style", &attr)
        .attribute("placeholder", "test")
        .on(move |_ev: Input| {
            let new_value = (*value) + 1;
            tracing::info!("{}", new_value);
            update.set(new_value);
        })
        .build();

    div().child(text(attr)).child(input).build()
}

#[wasm_bindgen(start)]
pub fn boot() {
    tracing_wasm::set_as_global_default_with_config(tracing_wasm::WASMLayerConfig {
        report_logs_in_console: true,
        report_logs_in_timings: false,
        use_console_color: false,
    });
    std::panic::set_hook(Box::new(|info| {
        tracing::error!(?info, "crashed");
    }));

    tracing::info!("booting");

    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("app")
        .unwrap();

    moxie_dom::boot(root, || app());
}
