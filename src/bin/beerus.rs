use clap::Parser;

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eyre::Result<()> {
    tokio::runtime::Builder::new_multi_thread().enable_all().build()?.block_on(
        async {
            let _ = run().await;
        },
    );

    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn run() -> eyre::Result<()> {
    use beerus::config::Config;
    use std::{sync::Arc, time::Duration};
    use tokio::sync::RwLock;

    const RPC_SPEC_VERSION: &str = "0.6.0";

    tracing_subscriber::fmt::init();

    let config = if let Some(path) = Args::parse().config.as_ref() {
        Config::from_file(path)?
    } else {
        Config::from_env()
    };

    config.check().await?;

    let beerus = beerus::client::Client::new(&config).await?;
    beerus.start().await?;

    let rpc_spec_version = beerus.spec_version().await?;
    if rpc_spec_version != RPC_SPEC_VERSION {
        eyre::bail!("RPC spec version mismatch: expected {RPC_SPEC_VERSION} but got {rpc_spec_version}");
    }

    let state = beerus.get_state().await?;
    tracing::info!(?state, "initialized");

    let state = Arc::new(RwLock::new(state));

    {
        let state = state.clone();
        let period = Duration::from_secs(config.poll_secs);
        tokio::spawn(async move {
            let mut tick = tokio::time::interval(period);
            loop {
                tick.tick().await;
                match beerus.get_state().await {
                    Ok(update) => {
                        *state.write().await = update;
                        tracing::info!(?state, "updated");
                    }
                    Err(e) => {
                        tracing::error!(error=?e, "state update failed");
                    }
                }
            }
        });
    }

    let server =
        beerus::rpc::serve(&config.starknet_rpc, &config.rpc_addr, state)
            .await?;

    tracing::info!(port = server.port(), "rpc server started");
    server.done().await;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(short = 'c', long)]
    config: Option<String>,
}
