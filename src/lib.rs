use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"todolist-101"</h1>
        </main>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    let _ = view! { <App /> };
}

