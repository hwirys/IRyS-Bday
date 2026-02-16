use actix_cors::Cors;
use actix_files as fs;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::middleware::{Compress, DefaultHeaders};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::Serialize;
use std::path::PathBuf;
use std::time::Duration;

// Embed CSV at compile time — no file needed on disk at runtime
static CSV_DATA: &str = include_str!("../../data/messages.csv");

#[derive(Debug, Clone, Serialize)]
struct Message {
    name: String,
    message: String,
}

/// Pre-serialized JSON responses — avoids re-serializing on every request
struct CachedMessages {
    json: String,
}

struct CachedConfig {
    json: String,
}

fn env_or<T: std::str::FromStr>(key: &str, default: T) -> T {
    match std::env::var(key) {
        Ok(v) => match v.parse() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("Warning: failed to parse {}={:?}, using default", key, v);
                default
            }
        },
        Err(_) => default,
    }
}

const MAX_NAME_LEN: usize = 200;
const MAX_MESSAGE_LEN: usize = 5_000;

fn parse_embedded_messages() -> Vec<Message> {
    let mut rdr = csv::Reader::from_reader(CSV_DATA.as_bytes());
    let mut messages = Vec::new();
    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Skipping malformed CSV row: {}", e);
                continue;
            }
        };
        if record.len() >= 2 {
            let name = &record[0];
            let message = &record[1];
            if name.len() > MAX_NAME_LEN || message.len() > MAX_MESSAGE_LEN {
                eprintln!("Skipping CSV row: field exceeds length limit");
                continue;
            }
            messages.push(Message {
                name: name.to_string(),
                message: message.to_string(),
            });
        }
    }
    messages
}

#[get("/api/messages")]
async fn get_messages(cache: web::Data<CachedMessages>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Cache-Control", "public, max-age=300"))
        .body(cache.json.clone())
}

#[get("/api/config")]
async fn get_config(cache: web::Data<CachedConfig>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Cache-Control", "public, max-age=300"))
        .body(cache.json.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let messages = parse_embedded_messages();
    assert!(!messages.is_empty(), "No messages found in embedded CSV");
    println!("Loaded {} messages (embedded at compile time)", messages.len());

    // Pre-serialize once — zero serialization cost per request
    let cached = web::Data::new(CachedMessages {
        json: serde_json::to_string(&messages).expect("Failed to serialize messages"),
    });

    let powered_by_name = env_or("POWERED_BY_NAME", "HyunJa".to_string());
    let powered_by_link = env_or("POWERED_BY_LINK", "https://x.com/hyunja_0423".to_string());
    assert!(
        powered_by_link.starts_with("https://")
            && powered_by_link.len() > "https://".len()
            && !powered_by_link.chars().any(|c| c.is_control() || c == ' ')
            && powered_by_link.chars().filter(|&c| c == '/').count() >= 3,
        "POWERED_BY_LINK must be a valid HTTPS URL, got: {}", powered_by_link
    );
    let cached_config = web::Data::new(CachedConfig {
        json: serde_json::to_string(&serde_json::json!({
            "powered_by_name": powered_by_name,
            "powered_by_link": powered_by_link,
        })).expect("Failed to serialize config"),
    });

    // All tunables read from env — defaults sized for ~1000 concurrent users
    let bind_addr = env_or("BIND_ADDR".into(), "0.0.0.0:8080".to_string());
    let allowed_origin = env_or("ALLOWED_ORIGIN".into(), "http://localhost:5173".to_string());
    assert!(
        (allowed_origin.starts_with("http://") || allowed_origin.starts_with("https://"))
            && !allowed_origin.chars().any(|c| c.is_control() || c == ' '),
        "ALLOWED_ORIGIN must be a valid HTTP(S) URL, got: {}", allowed_origin
    );
    let static_dir: Option<PathBuf> = std::env::var("STATIC_DIR").ok().map(PathBuf::from);
    let rate_limit_secs: u64 = env_or("RATE_LIMIT_SECS", 1);
    let rate_limit_burst: u32 = env_or("RATE_LIMIT_BURST", 10);
    let max_connections: usize = env_or("MAX_CONNECTIONS", 4096);
    let keep_alive_secs: u64 = env_or("KEEP_ALIVE_SECS", 10);
    let request_timeout_secs: u64 = env_or("REQUEST_TIMEOUT_SECS", 5);
    let num_workers: usize = env_or("WORKERS", 4);
    let max_payload: usize = env_or("MAX_PAYLOAD_BYTES", 1024);

    let governor_config = GovernorConfigBuilder::default()
        .seconds_per_request(rate_limit_secs)
        .burst_size(rate_limit_burst)
        .finish()
        .expect("Failed to build rate limiter config");

    println!("Binding to: {} (workers: {}, max_conn: {})", bind_addr, num_workers, max_connections);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET"])
            .max_age(86400);

        let security_headers = DefaultHeaders::new()
            .add(("X-Content-Type-Options", "nosniff"))
            .add(("X-Frame-Options", "DENY"))
            .add(("X-XSS-Protection", "1; mode=block"))
            .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            .add(("Permissions-Policy", "camera=(), microphone=(), geolocation=()"))
            .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
            .add(("Content-Security-Policy",
                "default-src 'self'; \
                 script-src 'self'; \
                 style-src 'self' 'unsafe-inline'; \
                 font-src 'self'; \
                 img-src 'self'; \
                 connect-src 'self'; \
                 frame-src https://www.youtube.com; \
                 object-src 'none'; \
                 base-uri 'self'; \
                 form-action 'none'; \
                 navigate-to 'self' https://x.com",
            ));

        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(security_headers)
            .wrap(cors)
            .wrap(Governor::new(&governor_config))
            .app_data(cached.clone())
            .app_data(cached_config.clone())
            .app_data(web::PayloadConfig::new(max_payload))
            .service(get_messages)
            .service(get_config);

        // Serve static frontend files
        let dir = static_dir.clone().unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../frontend/dist")
        });
        if dir.exists() {
            app = app.service(
                fs::Files::new("/", &dir)
                    .index_file("index.html")
                    .use_etag(true)
                    .use_last_modified(true)
                    .prefer_utf8(true),
            );
        }

        app
    })
    .bind(&bind_addr)?
    .keep_alive(Duration::from_secs(keep_alive_secs))
    .client_request_timeout(Duration::from_secs(request_timeout_secs))
    .max_connections(max_connections)
    .workers(num_workers)
    .run()
    .await
}
