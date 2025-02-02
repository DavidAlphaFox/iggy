use crate::http::{consumer_groups, messages, partitions, streams, system, topics};
use axum::http::Method;
use axum::Router;
use std::sync::Arc;
use streaming::system::System;
use tokio::sync::RwLock;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::server_config::{CorsConfig, HttpConfig};
use tracing::info;

pub async fn start(config: HttpConfig, system: Arc<RwLock<System>>) {
    info!("Starting HTTP API on: {:?}", config.address);
    let mut app = Router::new().nest(
        "/",
        system::router(system.clone()).nest(
            "/streams",
            streams::router(system.clone()).nest(
                "/:stream_id/topics",
                topics::router(system.clone())
                    .nest(
                        "/:topic_id/consumer-groups",
                        consumer_groups::router(system.clone()),
                    )
                    .nest("/:topic_id/messages", messages::router(system.clone()))
                    .nest("/:topic_id/partitions", partitions::router(system.clone())),
            ),
        ),
    );

    if config.cors.enabled {
        app = app.layer(configure_cors(config.cors));
    }

    axum::Server::bind(&config.address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn configure_cors(config: CorsConfig) -> CorsLayer {
    let allowed_origins = match config.allowed_origins {
        origins if origins.is_empty() => AllowOrigin::default(),
        origins if origins.first().unwrap() == "*" => AllowOrigin::any(),
        origins => AllowOrigin::list(origins.iter().map(|s| s.parse().unwrap())),
    };

    let allowed_headers = config
        .allowed_headers
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let exposed_headers = config
        .exposed_headers
        .iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let allowed_methods = config
        .allowed_methods
        .iter()
        .map(|s| match s.to_uppercase().as_str() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "CONNECT" => Method::CONNECT,
            "PATCH" => Method::PATCH,
            "TRACE" => Method::TRACE,
            _ => panic!("Invalid HTTP method: {}", s),
        })
        .collect::<Vec<_>>();

    CorsLayer::new()
        .allow_methods(allowed_methods)
        .allow_origin(allowed_origins)
        .allow_headers(allowed_headers)
        .expose_headers(exposed_headers)
        .allow_credentials(config.allow_credentials)
        .allow_private_network(config.allow_private_network)
}
