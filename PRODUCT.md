# Product Snapshot

`todolist-101` is the starting point for a server-rendered Rust TodoMVC application built with Leptos and Axum.

# Current State

The project is initialized as a Cargo workspace with a Leptos library crate and a binary entry point. It builds successfully in both server-side (`ssr`) and client hydration (`hydrate`) modes.

Today it only provides a placeholder UI:
- A minimal `<App />` component that renders the project name
- A stub `main` entry point behind the `ssr` feature
- A hydration entry point behind the `hydrate` feature

There is no running HTTP server, router setup, database integration, Todo domain model, or TodoMVC behavior yet.

# Key Decisions

- Rendering model: Leptos with an explicit split between `ssr` and `hydrate` feature flags
- Server stack: Axum and Tokio are included as SSR-only dependencies
- Client entry: WebAssembly hydration is prepared with `wasm-bindgen` and `console_error_panic_hook`
- Build contract: Leptos metadata is configured for `cargo-leptos` style builds, with output rooted at `target/site`
- Network convention: the intended server bind address is `0.0.0.0:8080`

# Conventions

- The library crate is the source of shared UI code
- The binary crate is reserved for server startup code
- Browser-only behavior must stay behind `hydrate`
- Server-only behavior must stay behind `ssr`
- Documentation in this file should describe only what is merged on `main`
