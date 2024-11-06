#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    sol,
};

use futures_util::StreamExt;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

struct BlockNumberState(u64);

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

enum EvmCommand {}

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    AcuityDexIntrachain,
    "abi/AcuityDexIntrachain.json"
);

async fn evm_service(rx: UnboundedReceiver<EvmCommand>) {
    let mut block_number = use_context::<Signal<BlockNumberState>>();
    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = "ws://127.0.0.1:8545";
    // Create the provider.
    info!("connecting");
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
    // Create a contract instance.
    let contract = AcuityDexIntrachain::new(
        address!("5FbDB2315678afecb367f032d93F642f64180aa3"),
        provider.clone(),
    );
    // do stuff
    let sub = provider.subscribe_blocks().await.unwrap();
    let mut stream = sub.into_stream();
    while let Some(block) = stream.next().await {
        block_number.set(BlockNumberState(block.header.number));
    }
}

fn App() -> Element {
    use_context_provider(|| Signal::new(BlockNumberState(0)));

    let evm = use_coroutine(evm_service);

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn BlockNumber() -> Element {
    let block_number = use_context::<Signal<BlockNumberState>>().read().0;

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
