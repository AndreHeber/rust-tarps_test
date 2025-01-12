use std::env;
use anyhow::Ok;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};

/// This is the service definition. It looks a lot like a trait definition.
/// It defines one RPC, hello, which takes one arg, name, and returns a String.
#[tarpc::service]
pub trait World {
	/// Returns a greeting for name.
	async fn hello(name: String) -> String;
}

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(service_name: &str) -> anyhow::Result<()> {
	env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");

	let tracer = opentelemetry_jaeger::new_agent_pipeline()
		.with_service_name(service_name)
		.with_max_packet_size(2usize.pow(13))
		.install_batch(opentelemetry::runtime::Tokio)?;

	tracing_subscriber::registry()
		.with(tracing_subscriber::EnvFilter::from_default_env())
		.with(tracing_subscriber::fmt::layer().pretty().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
		.with(tracing_opentelemetry::layer().with_tracer(tracer))
		.init();
	
	Ok(())
}