use axum::{routing::get, Router};
use cautious_memory::routes::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::{Ipv4Addr, SocketAddr};
use std::{env, net::SocketAddrV4};
use tower_http::{
    compression::CompressionLayer,
    decompression::DecompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let database_url = env::var("DATABASE_URL").unwrap();

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        .pretty()
        .with_max_level(Level::INFO)
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    // let addr: SocketAddr = env::var("SERVER_ADDRESS")
    //     .to_owned()
    //     .parse()
    //     .expect("Unable to parse socket address");

    let addr = env::var("SERVER_ADDRESS")
        .unwrap()
        .parse::<SocketAddr>()
        .unwrap();

    let app = Router::new()
        .route("/health_check", get(health_check()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Micros),
                ),
        )
        .layer(CompressionLayer::new())
        .layer(DecompressionLayer::new())
        .with_state(pool);

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
