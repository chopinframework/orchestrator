# Chopin Sequencing Orchestrator

This repository provides a per-domain sequential request handler built on Axum. The core goal is to ensure that only one request at a time is allowed to run for a given domain, while requests for other domains can run concurrently.

## Technologies

- Rust + Tokio for async runtime
- Axum + tower-http for the HTTP server and middleware (like TraceLayer)
- A custom Sequencer structure enforcing per-domain queuing

## How It Works

### 1. Sequencer
- Maintains an in-memory queue for each domain (e.g., foo.com, bar.com)
- A request calls `sequence(domain, request_id)` which blocks until it becomes active for that domain
- Once a request finishes, the client (or handler) calls `done(domain)`, which unblocks the next request in the queue

### 2. Axum HTTP Routes
- `/sequence`: Enqueues the incoming request (domain+request_id) on its domain queue. The HTTP response is returned only when the request is at the front of that domain's queue
- `/done`: Notifies the Sequencer that the active request for a given domain is finished, allowing the next queued request to become active

### 3. Parallel Domains, Sequential Per Domain
- If two requests arrive for the same domain, the second request is blocked until the first completes
- If two requests arrive for different domains, they can run in parallel

## Project Structure

```
sequencer/
├── Cargo.toml
└── src
    └── main.rs    # Contains:
                   # - Sequencer struct & logic
                   # - Axum server code (routes)
                   # - End-to-end integration tests
```

### Key files:
- `src/main.rs`:
  - The Sequencer struct and its logic
  - Axum routes (/sequence, /done)
  - The main `tokio::main` function to run the server
  - A test module with end-to-end tests using reqwest
- `Cargo.toml`:
  - Lists the project name, version, and dependencies (including axum, tower-http, reqwest, etc.)

## Getting Started

1. Install Rust (including Cargo) if you haven't already:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone or download this repository:
   ```bash
   git clone https://github.com/username/sequencer.git
   cd sequencer
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run the server:
   ```bash
   cargo run
   ```
   This will start the server listening on `127.0.0.1:3000`.

## Usage

Once the server is running, you can test your per-domain queueing:

### 1. Enqueue a request
With domain foo.com and request_id req-1:
```bash
curl -X POST -H "Content-Type: application/json" \
     -d '{"domain":"foo.com","request_id":"req-1"}' \
     http://127.0.0.1:3000/sequence
```
- If foo.com is idle, you get "OK" immediately
- If there's already an active request, this call will block until it's your turn

### 2. Unblock the next request
For foo.com:
```bash
curl -X POST -H "Content-Type: application/json" \
     -d '{"domain":"foo.com"}' \
     http://127.0.0.1:3000/done
```
- If there is a queued request for foo.com, the next one is made active
- If no one is waiting, foo.com goes idle again

### 3. Test concurrency with multiple domains
- Requests for foo.com do not block requests for bar.com
- Requests for the same domain queue up sequentially

## API Endpoints

### POST /sequence
- Body: `{ "domain": "<domain>", "request_id": "<id>" }`
- Behavior:
  - Blocks until this request is active for that domain
  - Responds with "OK" once it becomes active

### POST /done
- Body: `{ "domain": "<domain>" }`
- Behavior:
  - Marks the current request for `<domain>` as finished, unblocking the next queued request
  - If no requests are queued, `<domain>` goes idle

## Running Tests

The repository includes integration tests in `main.rs` (within `#[cfg(test)] mod tests`). These tests:
- Spawn the Axum server in-process on an ephemeral port
- Use reqwest to send real HTTP calls to `/sequence` and `/done`
- Check concurrency and queueing (same-domain blocking, different-domain concurrency, edge cases, etc.)

To run all tests:
```bash
cargo test
```

### Test Scenarios
- `test_explicit_enqueuing_flow`: Checks a specific manual order of unblocking
- `test_concurrent_different_domains`: Verifies domains do not block each other
- `test_already_idle_domain`: Calls /done on an idle domain
- `test_never_calls_done`: Confirms a second request remains blocked forever if the first never calls /done
- `test_done_multiple_times`: Ensures extra /done calls on an idle domain don't break anything

Example output:
```
running 5 tests
test tests::test_explicit_enqueuing_flow ... ok
test tests::test_concurrent_different_domains ... ok
test tests::test_already_idle_domain ... ok
test tests::test_never_calls_done ... ok
test tests::test_done_multiple_times ... ok
```