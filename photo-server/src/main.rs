use std::{env, path::PathBuf};

use actix_cors::Cors;
use actix_web::{ middleware::Logger, web, App, HttpServer};
use actix_ratelimit::{RateLimiter, MemoryStore, MemoryStoreActor};
use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use lib::controllers::{hello::{echo, hello, test}, photo::{list_albums, list_photos, serve_photo}, ui::{serve_static, serve_static_file}};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Current working directory: {:?}", env::current_dir());
    println!("Serving static files from: {:?}", std::fs::canonicalize("../photos"));
    println!("Checking file metadata:");
    println!("{:?}", std::fs::metadata("../photos/Aquairium/DSCF0429.JPG"));

    // Setup Rate limiting
    let backend = InMemoryBackend::builder().build();
    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(60), 100) // 100 requests per minute
        .real_ip_key() // Use client IP for rate limiting
        .build();

    let middleware = RateLimiter::builder(backend.clone(), input)
        .add_headers() // Add rate limit headers to responses
        .build();

    // Setup Certs
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("server.key", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("thetoaster_ddns_net.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(10 * 1024))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin() // Allow any origin (for development, not recommended in production)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"]) // Allow specific HTTP methods
                    .allowed_headers(vec![header::CONTENT_TYPE])
                    .max_age(3600), // Cache the CORS response for 1 hour
            )
            .service(
                web::resource("/")
                    .route(web::get().to(|| async {
                        serve_static_file(PathBuf::from("index.html")).await
                    })),
            )
            .service(serve_static)
            .service(list_albums)
            .service(list_photos)
            .service(serve_photo)
            .service(hello)
            .service(echo)
            .service(test)
            .default_service(
                web::route().to(|| async {
                    // Forward unmatched requests to serve_static with index.html
                    serve_static_file(PathBuf::from("index.html")).await
                }),
            )
    })
    //.bind(("0.0.0.0", 80))?
    .bind_openssl("0.0.0.0:443", builder)?
    .run()
    .await
}