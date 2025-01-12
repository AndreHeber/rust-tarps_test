use clap::Parser;
use service::{init_tracing, WorldClient};
use std::{net::SocketAddr, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::time::sleep;
use tracing::Instrument;

#[derive(Parser)]
struct Flags {
	#[clap(long)]
	server_addr: SocketAddr,
	#[clap(long)]
	name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let flags = Flags::parse();
	init_tracing("Tarpc Example Client")?;

	let transport = tarpc::serde_transport::tcp::connect(flags.server_addr, Json::default);

	// WorldClient is generated by the service attribute. It has a constructor `new` that takes a
	// config and any Transport as input.
	let client = WorldClient::new(client::Config::default(), transport.await?).spawn();

	let hello = async move {
		// Send the request twice, just to be safe! ;)
		tokio::select! {
			hello1 = client.hello(context::current(), format!("{}1", flags.name)) => { hello1 }
			hello2 = client.hello(context::current(), format!("{}2", flags.name)) => { hello2 }
		}
	}
	.instrument(tracing::info_span!("Two Hellos"))
	.await;

	tracing::info!("{:?}", hello);

	// Let the background span processor finish.
	sleep(Duration::from_micros(1)).await;
	opentelemetry::global::shutdown_tracer_provider();

	Ok(())
}