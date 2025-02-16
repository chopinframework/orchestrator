use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::Arc,
};

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use tokio::sync::{oneshot, Mutex};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter::EnvFilter, fmt};

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
// Axum server
// ------------------
//
#[derive(Deserialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize)]
struct DoneRequest {
    domain: String,
}

pub fn build_app(sequencer: Arc<Sequencer>) -> Router {
    Router::new()
        .route("/sequence", post(sequence_handler))
        .route("/done", post(done_handler))
        .with_state(sequencer)
        .layer(TraceLayer::new_for_http())
}

async fn sequence_handler(
    State(sequencer): State<Arc<Sequencer>>,
    Json(payload): Json<SequenceRequest>,
) -> &'static str {
    sequencer.sequence(&payload.domain, &payload.request_id).await;
    "OK"
}

async fn done_handler(
    State(sequencer): State<Arc<Sequencer>>,
    Json(payload): Json<DoneRequest>,
) -> &'static str {
    sequencer.done(&payload.domain).await;
    "OK"
}

//
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//
// ------------------
// Tests
// ------------------
//
#[cfg(test)]
mod tests {
    use super::*;
    use axum_server::Handle;
    use reqwest::Client;
    use std::time::{Duration, Instant};
    use tokio::sync::Mutex;

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
    ) -> String {
        let resp = client
            .post(format!("http://{}/sequence", addr))
            .json(&serde_json::json!({ "domain": domain, "request_id": request_id }))
            .send()
            .await
            .expect("sequence call failed");
        resp.text().await.expect("failed to read sequence body")
    }

    async fn call_done(client: &Client, addr: &SocketAddr, domain: &str) {
        let resp = client
            .post(format!("http://{}/done", addr))
            .json(&serde_json::json!({ "domain": domain }))
            .send()
            .await
            .expect("done call failed");
        assert!(resp.status().is_success());
    }

    // -------------------------------------------------------
    // 1. test_explicit_enqueuing_flow
    // Demonstrates a specific manual order of unblocking: req-1-> req-2-> req-3
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_explicit_enqueuing_flow() {
        tracing_subscriber::fmt()
            .with_env_filter("tower_http=debug")
            .init();

        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        // We'll track finish order in a vector (like "done-req-1", etc.)
        let results = Arc::new(Mutex::new(vec![]));

        // Helper to spawn a request
        let spawn_req = |request_id: &'static str| {
            let client = client.clone();
            let addr = addr.clone();
            let results = results.clone();
            tokio::spawn(async move {
                let body = call_sequence(&client, &addr, "foo", request_id).await;
                let mut lock = results.lock().await;
                lock.push(format!("done-{}", request_id));
                body
            })
        };

        // Start "req-0" so the domain is busy
        let r0 = spawn_req("req-0");
        // Sleep a bit so req-0 is definitely active
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Now spawn req-1, req-2, req-3 => all will be enqueued behind "req-0".
        let r1 = spawn_req("req-1");
        let r2 = spawn_req("req-2");
        let r3 = spawn_req("req-3");

        // Unblock req-0 => that unblocks req-1
        call_done(&client, &addr, "foo").await;
        let out_r1 = r1;
        let out_r0 = r0.await.unwrap(); // "req-0" done
        assert_eq!(out_r0, "OK");

        // Now we call done again => unblocks req-2
        call_done(&client, &addr, "foo").await;
        let out_r1 = out_r1.await.unwrap();
        assert_eq!(out_r1, "OK");

        // Next done => unblocks req-3
        call_done(&client, &addr, "foo").await;
        let out_r2 = r2.await.unwrap();
        assert_eq!(out_r2, "OK");

        // Finally, one more done => though it's possible no one is left waiting,
        // let's do it anyway to fully ensure req-3 is done:
        call_done(&client, &addr, "foo").await;
        let out_r3 = r3.await.unwrap();
        assert_eq!(out_r3, "OK");

        {
            let lock = results.lock().await;
            println!("Finish order: {:?}", *lock);
            // Expect something like ["done-req-0","done-req-1","done-req-2","done-req-3"]
        }

        handle.shutdown();
    }

    // -------------------------------------------------------
    // 2. test_concurrent_different_domains
    // Verifies parallel requests on distinct domains do not block each other.
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_concurrent_different_domains() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        let start = Instant::now();

        // domain=foo
        let foo_req = {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = call_sequence(&client, &addr, "foo", "foo-req").await;
                resp
            })
        };

        // domain=bar
        let bar_req = {
            let client = client.clone();
            tokio::spawn(async move {
                let resp = call_sequence(&client, &addr, "bar", "bar-req").await;
                resp
            })
        };

        // Wait a bit
        tokio::time::sleep(Duration::from_millis(200)).await;

        // "Done" for foo
        call_done(&client, &addr, "foo").await;
        // "Done" for bar
        call_done(&client, &addr, "bar").await;

        let foo_body = foo_req.await.unwrap();
        let bar_body = bar_req.await.unwrap();
        assert_eq!(foo_body, "OK");
        assert_eq!(bar_body, "OK");

        let elapsed = start.elapsed().as_millis();
        println!("Different domains test took {elapsed} ms");

        handle.shutdown();
    }

    // -------------------------------------------------------
    // 3. test_already_idle_domain
    // Calls /done for a domain that isn't busy.
    // Ensures no panics and domain remains idle.
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_already_idle_domain() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        // 1) Immediately call /done on domain=idle.com, which has no active or waiting requests
        call_done(&client, &addr, "idle.com").await;

        // 2) Send a request to "idle.com" => should become active right away
        let out = call_sequence(&client, &addr, "idle.com", "req-1").await;
        assert_eq!(out, "OK");

        // 3) Now done -> should go idle again
        call_done(&client, &addr, "idle.com").await;

        handle.shutdown();
    }

    // -------------------------------------------------------
    // 4. test_never_calls_done
    // One request becomes active for domain=foo. Another
    // request to domain=foo is blocked forever because we
    // never call /done on the first request.
    // We confirm the second request times out or doesn't proceed.
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_never_calls_done() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        // Start the first request => becomes active
        let r1 = tokio::spawn({
            let client = client.clone();
            async move {
                let resp = call_sequence(&client, &addr, "foo", "req-1").await;
                resp
            }
        });

        // Wait a moment to ensure r1 is active
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Now start second request => This will be enqueued forever
        // We'll wait 1 second to see if it remains blocked
        let r2 = tokio::spawn({
            let client = client.clone();
            async move {
                // We'll artificially time out the request using a tokio::time::timeout
                match tokio::time::timeout(
                    Duration::from_secs(1),
                    call_sequence(&client, &addr, "foo", "req-2"),
                )
                .await
                {
                    Ok(resp) => resp, // if it completes, return
                    Err(_) => "TIMED OUT".into(),
                }
            }
        });

        // r1 has not called /done, so r2 should remain blocked
        // The test ensures r2 times out
        let resp2 = r2.await.unwrap();
        assert_eq!(resp2, "TIMED OUT");

        // If we want to confirm r1 is indeed still active, we can do so.
        // Right now, r1 won't complete unless we call done:
        // call_done(&client, &addr, "foo").await;

        // Then r1 eventually returns "OK"
        // But for this test, we prove that not calling /done => second request is stuck.

        // Optionally clean up:
        let _ = r1; // we never used its result
        handle.shutdown();
    }

    // -------------------------------------------------------
    // 5. test_done_multiple_times
    // Ensures extra /done calls on a domain that isn't queued
    // doesn't break anything. 
    // -------------------------------------------------------
    #[tokio::test]
    async fn test_done_multiple_times() {
        let (handle, addr) = spawn_server().await;
        let client = Client::new();

        // domain=foo, single request
        let r1 = tokio::spawn({
            let client = client.clone();
            async move {
                call_sequence(&client, &addr, "foo", "req-1").await
            }
        });

        // Sleep to let r1 become active
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Now call /done => unblocks r1
        call_done(&client, &addr, "foo").await;

        // Wait for r1
        let out_r1 = r1.await.unwrap();
        assert_eq!(out_r1, "OK");

        // domain=foo is now idle, so the next /done calls do nothing
        call_done(&client, &addr, "foo").await;
        call_done(&client, &addr, "foo").await;
        call_done(&client, &addr, "foo").await;

        // We can check a fresh request to "foo" still works
        let out2 = call_sequence(&client, &addr, "foo", "req-2").await;
        assert_eq!(out2, "OK");

        // done again
        call_done(&client, &addr, "foo").await;

        handle.shutdown();
    }
}