//! Example of how to use the network as a standalone component
//!
//! Run with
//!
//! ```sh
//! cargo run --release -p network
//! ```

#![warn(unused_crate_dependencies)]

use futures::StreamExt;
use reth_ethereum::{
    network::{
        config::rng_secret_key, NetworkConfig, NetworkEventListenerProvider, NetworkManager,
    },
    provider::test_utils::NoopProvider,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // This block provider implementation is used for testing purposes.
    let client = NoopProvider::default();

    // The key that's used for encrypting sessions and to identify our node.
    let local_key = rng_secret_key();

    // Configure the network
    let config = NetworkConfig::builder(local_key).mainnet_boot_nodes().build(client);

    // create the network instance
    let network = NetworkManager::eth(config).await?;

    // get a handle to the network to interact with it
    let handle = network.handle().clone();

    // spawn the network
    tokio::task::spawn(network);

    // interact with the network
    let mut events = handle.event_listener();
    while let Some(event) = events.next().await {
        println!("Received event: {event:?}");
    }

    Ok(())
}
