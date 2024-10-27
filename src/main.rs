#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use alloy::providers::{Provider, ProviderBuilder, WsConnect};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

async fn connect() {
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "ws://127.0.0.1:8545";

    // Create the provider.
    info!("connecting");
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
    info!("connected");
}

#[component]
fn Home() -> Element {
    let mut future = use_resource(|| connect());

    match &*future.read_unchecked() {
        Some(()) => rsx! { div { "Dogs loaded" } },
        None => rsx! { div { "Loading dogs..." } },
    }
}
