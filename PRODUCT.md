# Product Snapshot

`todolist-101` is the starting point for a server-rendered Rust TodoMVC application built with Leptos and Axum.

# Current State

The project now has a working Leptos SSR bootstrap with an Axum server entry point and a browser hydration entry point. It builds successfully in both server-side (`ssr`) and client hydration (`hydrate`) modes.

Today it only provides a placeholder UI:
- A placeholder `<App />` page for the future TodoMVC application
- An HTML shell for SSR responses
- An Axum server that loads `LeptosOptions`, generates Leptos routes, and serves the app
- A hydration entry point that mounts the app in the browser
- Environment-based startup configuration loaded from `.env` when present

There is still no Todo domain model, persistence layer, styling, or TodoMVC behavior yet. The current app is infrastructure plus a placeholder page.

# Key Decisions

- Rendering model: Leptos with an explicit split between `ssr` and `hydrate` feature flags
- Server stack: Axum and Tokio are included as SSR-only dependencies
- Configuration model: SSR startup loads environment variables with `dotenvy`
- Server bootstrap: the binary crate owns Axum startup, Leptos route generation, and fallback handling
- Client entry: WebAssembly hydration is implemented with `wasm-bindgen`, `console_error_panic_hook`, and `hydrate_body`
- Build contract: Leptos metadata is configured for `cargo-leptos` style builds, with output rooted at `target/site`
- Network convention: the intended server bind address is `0.0.0.0:8080`

# Conventions

- The library crate is the source of shared UI code
- The binary crate owns server startup and HTTP wiring
- The library crate owns the app view and SSR shell
- `.env.example` is the checked-in template for required runtime settings
- `.env` and `data/` stay untracked locally
- Browser-only behavior must stay behind `hydrate`
- Server-only behavior must stay behind `ssr`
- Documentation in this file should describe only what is merged on `main`
