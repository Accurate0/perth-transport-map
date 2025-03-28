use http::{HeaderMap, HeaderValue};
use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{
    trace::BatchConfigBuilder,
    trace::{BatchSpanProcessor, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::resource::{
    DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, TELEMETRY_SDK_LANGUAGE, TELEMETRY_SDK_NAME,
    TELEMETRY_SDK_VERSION,
};
use reqwest::{Request, Response};
use reqwest_tracing::{default_on_request_end, reqwest_otel_span, ReqwestOtelSpanBackend};
use std::time::Duration;
use tokio::time::Instant;
use tracing::Span;
use tracing_subscriber::EnvFilter;

const INGEST_URL: &str = "https://api.axiom.co/v1/traces";

pub fn external_tracer(name: &'static str) -> Tracer {
    let token = std::env::var("AXIOM_TOKEN").expect("must have axiom token configured");
    let dataset_name = std::env::var("AXIOM_DATASET").expect("must have axiom dataset configured");

    let mut headers = HeaderMap::<HeaderValue>::with_capacity(3);
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    headers.insert(
        "X-Axiom-Dataset",
        HeaderValue::from_str(&dataset_name).unwrap(),
    );
    headers.insert(
        "User-Agent",
        HeaderValue::from_str(&format!("tracing-axiom/{}", env!("CARGO_PKG_VERSION"))).unwrap(),
    );

    let tags = vec![
        KeyValue::new(TELEMETRY_SDK_NAME, "external-tracer".to_string()),
        KeyValue::new(TELEMETRY_SDK_VERSION, env!("CARGO_PKG_VERSION").to_string()),
        KeyValue::new(TELEMETRY_SDK_LANGUAGE, "rust".to_string()),
        KeyValue::new(SERVICE_NAME, name),
        KeyValue::new(
            DEPLOYMENT_ENVIRONMENT_NAME,
            if cfg!(debug_assertions) {
                "development"
            } else {
                "production"
            },
        ),
    ];

    let resource = Resource::builder_empty().with_attributes(tags).build();

    let batch_config = BatchConfigBuilder::default()
        .with_max_queue_size(20480)
        .build();

    let span_exporter = opentelemetry_otlp::HttpExporterBuilder::default()
        .with_http_client(
            std::thread::spawn(|| {
                reqwest::blocking::ClientBuilder::new()
                    .default_headers(headers)
                    .build()
                    .unwrap()
            })
            .join()
            .unwrap(),
        )
        .with_endpoint(INGEST_URL)
        .with_timeout(Duration::from_secs(3))
        .build_span_exporter()
        .unwrap();

    let tracer_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_span_processor(
            BatchSpanProcessor::builder(span_exporter)
                .with_batch_config(batch_config)
                .build(),
        )
        .with_resource(resource)
        .build();

    let tracer = tracer_provider.tracer(name);
    global::set_tracer_provider(tracer_provider);

    tracer
}

#[cfg(not(debug_assertions))]
pub fn init_logger(name: &'static str) {
    use opentelemetry_sdk::propagation::TraceContextPropagator;
    use tracing::Level;
    use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

    let tracer = external_tracer(name);

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    tracing_subscriber::registry()
        .with(Targets::default().with_default(Level::INFO))
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_ids(true)
                .with_ansi(true),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();
}

#[cfg(debug_assertions)]
pub fn init_logger(_: &'static str) {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_thread_ids(true)
        .init()
}

pub struct TimeTrace;
impl ReqwestOtelSpanBackend for TimeTrace {
    fn on_request_start(req: &Request, extension: &mut http::Extensions) -> Span {
        extension.insert(Instant::now());
        reqwest_otel_span!(name = "http", req, time_elapsed = tracing::field::Empty)
    }

    fn on_request_end(
        span: &Span,
        outcome: &reqwest_middleware::Result<Response>,
        extension: &mut http::Extensions,
    ) {
        let time_elapsed = extension.get::<Instant>().unwrap().elapsed().as_millis() as i64;
        default_on_request_end(span, outcome);
        span.record("time_elapsed", time_elapsed);
    }
}
