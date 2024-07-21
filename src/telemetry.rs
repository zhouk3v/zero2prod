use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Create layers to process emitted span events
    // Layer to filter out all spans above info level (which only consist of trace events)
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // Output span records in a bunyan-compatible JSON format
    let formatting_layer = BunyanFormattingLayer::new(
        name, // Output the spans to stdout (terminal)
        sink,
    );
    // Init a tracing subscriber to process emitted spans, print all info-level spans by default
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register a subscriber as global default to process span data
//
// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to the subscriber
    LogTracer::init().expect("Failed to set logger");
    // Set the subscriber to be used globally in the app
    set_global_default(subscriber).expect("Failed to set subscriber");
}
