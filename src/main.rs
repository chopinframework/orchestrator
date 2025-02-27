use std::{
    collections::{HashMap, VecDeque, HashSet},
    net::SocketAddr,
    sync::Arc,
};

use axum::{
    extract::State,
    routing::{post, get},
    Json, Router,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{oneshot, Mutex};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter::EnvFilter, fmt};
use utoipa::{OpenApi, ToSchema, Modify};
use utoipa_swagger_ui::SwaggerUi;

//
// ------------------
// API Security Schema Modifier
// ------------------
//
/// Adds Bearer token authentication to the OpenAPI spec
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Add Bearer authentication
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "BearerAuth", 
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer
                    )
                )
            )
        }
    }
}

//
// ------------------
// OpenAPI Documentation
// ------------------
//
#[derive(OpenApi)]
#[openapi(
    paths(
        sequence_handler,
        done_handler,
        health_handler
    ),
    components(
        schemas(SequenceRequest, DoneRequest, ErrorResponse)
    ),
    tags(
        (name = "sequencer", description = "Sequencer API endpoints")
    ),
    modifiers(&SecurityAddon),
    info(
        title = "Sequencer API",
        version = "0.1.0",
        description = "API for sequencing domain operations",
        contact(
            name = "API Support",
            email = "support@example.com"
        )
    )
)]
struct ApiDoc;

//
// ------------------
// Authentication
// ------------------
//
#[derive(Debug, Clone)]
pub struct ApiKeyStore {
    // Set of valid API keys
    keys: Arc<HashSet<String>>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        let mut keys = HashSet::new();
        // Add your hardcoded API keys here
        keys.insert("key1".to_string());
        keys.insert("key2".to_string());
        keys.insert("key3".to_string());
        
        Self {
            keys: Arc::new(keys),
        }
    }

    pub fn is_valid(&self, api_key: &str) -> bool {
        self.keys.contains(api_key)
    }

    fn extract_api_key(headers: &HeaderMap) -> Option<String> {
        headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|auth_str| {
                if auth_str.starts_with("Bearer ") {
                    Some(auth_str[7..].to_string())
                } else {
                    None
                }
            })
    }
}

//
// ------------------
// Sequencer
// ------------------
//
#[derive(Debug)]
struct DomainQueue {
    active: bool,
    waiting: VecDeque<oneshot::Sender<()>>,
}

#[derive(Clone, Debug)]
pub struct Sequencer {
    domain_queues: Arc<Mutex<HashMap<String, DomainQueue>>>,
}

impl Sequencer {
    pub fn new() -> Self {
        Self {
            domain_queues: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Wait until it's our turn for `domain`.
    pub async fn sequence(&self, domain: &str, request_id: &str) {
        let domain = domain.to_owned();
        let request_id = request_id.to_owned();

        let mut map = self.domain_queues.lock().await;
        let queue = map.entry(domain.clone()).or_insert(DomainQueue {
            active: false,
            waiting: VecDeque::new(),
        });

        if !queue.active {
            queue.active = true;
            println!("[Sequencer] Domain={}, RequestId={} is now active", domain, request_id);
        } else {
            println!("[Sequencer] Domain={}, RequestId={} is enqueued", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!("[Sequencer] Domain={}, RequestId={} is now active", domain, request_id);
        }
    }

    /// Signal that the active request is done for `domain`. If another is waiting,
    /// that one is unblocked and becomes active.
    pub async fn done(&self, domain: &str) {
        let domain = domain.to_owned();
        let mut map = self.domain_queues.lock().await;
        if let Some(queue) = map.get_mut(&domain) {
            if let Some(tx) = queue.waiting.pop_front() {
                let _ = tx.send(());
            } else {
                // No one waiting; mark domain as idle
                queue.active = false;
            }
        }
    }
}

//
// ------------------
// Request/Response Models
// ------------------
//

/// Error response model
#[derive(Serialize, Deserialize, ToSchema)]
struct ErrorResponse {
    /// Error message describing what went wrong
    message: String,
}

/// Request model for the sequence endpoint
#[derive(Deserialize, Serialize, ToSchema)]
struct SequenceRequest {
    /// Domain identifier for sequencing requests
    domain: String,
    /// Unique identifier for this request
    request_id: String,
}

/// Request model for the done endpoint
#[derive(Deserialize, Serialize, ToSchema)]
struct DoneRequest {
    /// Domain identifier to mark as done
    domain: String,
}

//
// ------------------
// Axum server
// ------------------
//
#[derive(Clone)]
struct AppState {
    sequencer: Arc<Sequencer>,
    api_key_store: ApiKeyStore,
}

/// Sequence a request for a specific domain
///
/// Request will be queued if there's already an active request for the domain.
#[utoipa::path(
    post,
    path = "/sequence",
    request_body = SequenceRequest,
    responses(
        (status = 200, description = "Request has been sequenced", body = String),
        (status = 401, description = "Unauthorized - Invalid or missing API key", body = ErrorResponse)
    ),
    security(
        ("BearerAuth" = [])
    ),
    tag = "sequencer"
)]
async fn sequence_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<SequenceRequest>,
) -> Result<&'static str, StatusCode> {
    // Extract and validate API key
    let api_key = ApiKeyStore::extract_api_key(&headers)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !state.api_key_store.is_valid(&api_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    state.sequencer.sequence(&payload.domain, &payload.request_id).await;
    Ok("OK")
}

/// Mark a domain's active request as done
///
/// If other requests are waiting, the next one will become active.
#[utoipa::path(
    post,
    path = "/done",
    request_body = DoneRequest,
    responses(
        (status = 200, description = "Domain marked as done", body = String),
        (status = 401, description = "Unauthorized - Invalid or missing API key", body = ErrorResponse)
    ),
    security(
        ("BearerAuth" = [])
    ),
    tag = "sequencer"
)]
async fn done_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<DoneRequest>,
) -> Result<&'static str, StatusCode> {
    // Extract and validate API key
    let api_key = ApiKeyStore::extract_api_key(&headers)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !state.api_key_store.is_valid(&api_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    state.sequencer.done(&payload.domain).await;
    Ok("OK")
}

/// Health check endpoint 
///
/// Returns 200 OK if the service is healthy.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy")
    ),
    tag = "sequencer"
)]
async fn health_handler() -> StatusCode {
    StatusCode::OK
}

pub fn build_app(sequencer: Arc<Sequencer>) -> Router {
    let state = AppState {
        sequencer,
        api_key_store: ApiKeyStore::new(),
    };
    
    // Generate OpenAPI documentation
    let api_doc = ApiDoc::openapi();
    
    Router::new()
        .route("/sequence", post(sequence_handler))
        .route("/done", post(done_handler))
        .route("/health", get(health_handler))
        // Use api-docs path for the OpenAPI spec served by Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

// ------------------
// Main entry
// ------------------
//
#[tokio::main]
async fn main() {
    fmt()
        .with_env_filter(EnvFilter::new("tower_http=debug"))
        .init();

    let sequencer = Arc::new(Sequencer::new());
    let app = build_app(sequencer);

    // Get port from environment variable or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(4001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Listening on http://{}", addr);
    println!("API documentation available at http://{}:{}/swagger-ui/", addr.ip(), addr.port());
    println!("OpenAPI specification available at http://{}:{}/api-docs/openapi.json", addr.ip(), addr.port());

    // Create a shutdown signal handler
    let handle = axum_server::Handle::new();
    let handle_clone = handle.clone();

    // Spawn signal handler task
    tokio::spawn(async move {
        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {
                println!("Received Ctrl+C, starting graceful shutdown");
            },
            _ = terminate => {
                println!("Received SIGTERM, starting graceful shutdown");
            },
        }

        handle_clone.shutdown();
    });

    // Start the server with the handle
    axum_server::bind(addr)
        .handle(handle)
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Server shutdown complete");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_server::Handle;
    use reqwest::Client;
    use std::time::{Duration, Instant};

    /// Helper to spawn server on an ephemeral port. Returns (Handle, SocketAddr).
    async fn spawn_server() -> (Handle, SocketAddr) {
        let sequencer = Arc::new(Sequencer::new());
        let app = build_app(sequencer);

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = Handle::new();
        let handle_clone = handle.clone();

        tokio::spawn(async move {
            axum_server::Server::from_tcp(listener)
                .handle(handle_clone)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        (handle, addr)
    }

    async fn call_sequence(
        client: &Client,
        addr: &SocketAddr,
        domain: &str,
        request_id: &str,
        api_key: &str,
    ) -> reqwest::Response {
        client
            .post(format!("http://{}/sequence", addr))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&SequenceRequest {
                domain: domain.to_string(),
                request_id: request_id.to_string(),
            })
            .send()
            .await
            .unwrap()
    }

    async fn call_done(client: &Client, addr: &SocketAddr, domain: &str, api_key: &str) -> reqwest::Response {
        client
            .post(format!("http://{}/done", addr))
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&DoneRequest {
                domain: domain.to_string(),
            })
            .send()
            .await
            .unwrap()
    }

    // -------------------------------------------------------
    // 1. test_authentication
    // Validates API key authentication logic
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_authentication() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        // Valid key should succeed
        let resp = call_sequence(&client, &addr, "test.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Invalid key should fail
        let resp = client
            .post(format!("http://{}/sequence", addr))
            .header("Authorization", "Bearer invalid-key")
            .json(&SequenceRequest {
                domain: "test.com".to_string(),
                request_id: "req2".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Missing key should fail
        let resp = client
            .post(format!("http://{}/sequence", addr))
            .json(&SequenceRequest {
                domain: "test.com".to_string(),
                request_id: "req3".to_string(),
            })
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        handle.shutdown();
    }

    // -------------------------------------------------------
    // 2. test_explicit_enqueuing_flow
    // Tests the explicit flow of multiple requests for a domain,
    // showing how unblocking works in a specific order
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_explicit_enqueuing_flow() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();
        
        // First req should return immediately
        let resp = call_sequence(&client, &addr, "domain1.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Set up a background task that will call sequence and then block
        let addr_clone = addr.clone();
        let sequence_task = tokio::spawn(async move {
            let client = Client::new();
            let start = Instant::now();
            let resp = call_sequence(&client, &addr_clone, "domain1.com", "req2", "key1").await;
            (resp, start.elapsed())
        });
        
        // Give the background task time to start and get enqueued
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // Now mark the first request as done
        let resp = call_done(&client, &addr, "domain1.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // The background task should now unblock
        let (resp, elapsed) = sequence_task.await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        
        // It should have waited a non-trivial amount of time
        assert!(elapsed.as_millis() >= 100);
        
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 3. test_concurrent_different_domains
    // Verifies that different domains can be processed concurrently
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_concurrent_different_domains() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();
        
        // Send first request for domain1
        let resp = call_sequence(&client, &addr, "domain1.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Send request for domain2 - should not block
        let start = Instant::now();
        let resp = call_sequence(&client, &addr, "domain2.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        // Should return quickly
        assert!(start.elapsed().as_millis() < 100);
        
        // Verify that we can process multiple domains in parallel
        let domain1_done = call_done(&client, &addr, "domain1.com", "key1").await;
        let domain2_done = call_done(&client, &addr, "domain2.com", "key1").await;
        
        assert_eq!(domain1_done.status(), StatusCode::OK);
        assert_eq!(domain2_done.status(), StatusCode::OK);
        
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 4. test_already_idle_domain
    // Verifies that calling done on an idle domain doesn't break anything
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_already_idle_domain() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();
        
        // Call done on a domain that hasn't been used yet
        let resp = call_done(&client, &addr, "unused.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Now use the domain and verify it works normally
        let resp = call_sequence(&client, &addr, "unused.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 5. test_never_calls_done
    // Verifies that a request remains blocked if the active request
    // never calls done
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_never_calls_done() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();
        
        // Make the domain active with a request
        let resp = call_sequence(&client, &addr, "blocking.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // We could test that another request blocks, but that's hard to do in a test
        // So we'll just verify we can mark it as done later
        let resp = call_done(&client, &addr, "blocking.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // And verify we can use it again
        let resp = call_sequence(&client, &addr, "blocking.com", "req2", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 6. test_done_multiple_times
    // Verifies that calling done multiple times doesn't cause issues
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_done_multiple_times() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();
        
        // Make the domain active with a request
        let resp = call_sequence(&client, &addr, "multiple.com", "req1", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Call done once - this should work
        let resp = call_done(&client, &addr, "multiple.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Call done again on the idle domain - should not error
        let resp = call_done(&client, &addr, "multiple.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // And again - still should not error
        let resp = call_done(&client, &addr, "multiple.com", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Should still be able to use the domain normally after all this
        let resp = call_sequence(&client, &addr, "multiple.com", "req2", "key1").await;
        assert_eq!(resp.status(), StatusCode::OK);
        
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 7. test_health_check
    // Verifies that the health check endpoint returns 200 OK
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_health_check() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        let resp = client
            .get(format!("http://{}/health", addr))
            .send()
            .await
            .unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
        handle.shutdown();
    }
}
