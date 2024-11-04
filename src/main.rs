#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use alloy::providers::{Provider, ProviderBuilder, WsConnect};

use futures_util::StreamExt;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
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
fn BlockNumber() -> Element {
    let block_number = use_signal(|| 0u64);
    let block_task = use_coroutine(|rx: UnboundedReceiver<u64>| {
        let mut block_number = block_number.to_owned();
        async move {
            // Set up the WS transport which is consumed by the RPC client.
            let rpc_url = "ws://127.0.0.1:8545";

            // Create the provider.
            info!("connecting");
            let ws = WsConnect::new(rpc_url);
            let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
            info!("connected");
            let sub = provider.subscribe_blocks().await.unwrap();
            let mut stream = sub.into_stream();
            while let Some(block) = stream.next().await {
                block_number.set(block.header.number);
            }
        }
    });

    rsx! { "{block_number}" }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "navbar bg-base-100",
            a {
                class: "btn btn-ghost text-xl",
                "Acuity DEX",
            }
        },
        div {
            BlockNumber {}
        }
    }
}
