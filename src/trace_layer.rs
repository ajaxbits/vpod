use axum::{body::BoxBody, extract::ConnectInfo, response::Response};
use hyper::{Body, Request};
use std::{net::SocketAddr, time::Duration};
use tracing::Span;

pub(crate) fn trace_layer_make_span_with(req: &Request<Body>) -> Span {
    tracing::error_span!("request",
        uri = %req.uri(),
        method = %req.method(),
        source = request.extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|conn_info| tracing::field::display(conn_info))
            .unwrap_or_else(|| tracing::field::display("<unknown>".to_string())),
        status = tracing::field::Empty,
        latency = tracing::field::Empty,
    )
}

pub(crate) fn trace_layer_on_request(_req: &Request<Body>, _span: &Span) {
    tracing::trace!("Got request")
}

pub(crate) fn trace_layer_on_response(resp: &Response<BoxBody>, latency: Duration, span: &Span) {
    span.record(
        "latency",
        tracing::field::display(format!("{}us", latency.as_micros())),
    );
    span.record("status", tracing::field::display(resp.status()));
    tracing::trace!("Responded");
}
