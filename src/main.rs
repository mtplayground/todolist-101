#[cfg(feature = "ssr")]
use std::error::Error;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use axum::Router;
    use dotenvy::dotenv;
    use leptos::config::get_configuration;
    use leptos::logging::log;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use todolist_101::{shell, App};

    let _ = dotenv();

    let conf = get_configuration(None)?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // Hydration is provided by the wasm entry point in `src/lib.rs`.
}
