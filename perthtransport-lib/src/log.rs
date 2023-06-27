use reqwest::{Request, Response};
use reqwest_tracing::{default_on_request_end, reqwest_otel_span, ReqwestOtelSpanBackend};
use task_local_extensions::Extensions;
use tokio::time::Instant;
use tracing::Span;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

#[cfg(not(debug_assertions))]
pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::ENTER)
        .json()
        .with_thread_ids(true)
        .with_ansi(false)
        .init()
}

#[cfg(debug_assertions)]
pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::EXIT)
        .with_thread_ids(true)
        .init()
}

pub struct TimeTrace;
impl ReqwestOtelSpanBackend for TimeTrace {
    fn on_request_start(req: &Request, extension: &mut Extensions) -> Span {
        extension.insert(Instant::now());
        reqwest_otel_span!(name = "http", req, time_elapsed = tracing::field::Empty)
    }

    fn on_request_end(
        span: &Span,
        outcome: &reqwest_middleware::Result<Response>,
        extension: &mut Extensions,
    ) {
        let time_elapsed = extension.get::<Instant>().unwrap().elapsed().as_millis() as i64;
        default_on_request_end(span, outcome);
        span.record("time_elapsed", time_elapsed);
    }
}
