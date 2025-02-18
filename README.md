# Chopin Sequencing Orchestrator

This repository provides a per-domain sequential request handler built on Axum. The core goal is to ensure that only one request at a time is allowed to run for a given domain, while requests for other domains can run concurrently.

## Technologies

- Rust + Tokio for async runtime
- Axum + tower-http for the HTTP server and middleware (like TraceLayer)
- A custom Sequencer structure enforcing per-domain queuing
- Simple API key authentication
- Docker containerization
- SST for AWS Fargate deployment

## How It Works

### 1. Authentication
- Each request must include an `Authorization: Bearer <key>` header
- A fixed set of API keys is maintained in memory
- Authentication responses:
  - `401 Unauthorized`: Missing or invalid API key
  - `200 OK`: Valid API key

### 2. Sequencer
- Maintains an in-memory queue for each domain (e.g., foo.com, bar.com)
- A request calls `sequence(domain, request_id)` which blocks until it becomes active for that domain
- Once a request finishes, the client (or handler) calls `done(domain)`, which unblocks the next request in the queue

### 3. Axum HTTP Routes
- `/sequence`: Enqueues the incoming request (domain+request_id) on its domain queue. The HTTP response is returned only when the request is at the front of that domain's queue
- `/done`: Notifies the Sequencer that the active request for a given domain is finished, allowing the next queued request to become active
- `/health`: A health check endpoint that returns 200 OK when the service is running properly

### 4. Parallel Domains, Sequential Per Domain
- If two requests arrive for the same domain, the second request is blocked until the first completes
- If two requests arrive for different domains, they can run in parallel

## Project Structure

```
sequencer/
├── Dockerfile          # Docker container configuration
├── sst.config.ts      # SST deployment configuration
├── Cargo.toml
└── src
    └── main.rs        # Contains:
                       # - ApiKeyStore for authentication
                       # - Sequencer struct & logic
                       # - Axum server code (routes)
                       # - End-to-end integration tests
```

### Key files:
- `src/main.rs`:
  - The ApiKeyStore struct for authentication
  - The Sequencer struct and its logic
  - Axum routes (/sequence, /done)
  - The main `tokio::main` function to run the server
  - A test module with end-to-end tests using reqwest
- `Cargo.toml`:
  - Lists the project name, version, and dependencies (including axum, tower-http, reqwest, etc.)
- `Dockerfile`: Multi-stage build for creating a minimal production container
- `sst.config.ts`: AWS Fargate deployment configuration using SST

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
   This will start the server listening on `127.0.0.1:4001`.

### Docker Build

Build the Docker image:
```bash
docker build -t sequencer .
```

Run the container locally:
```bash
docker run -p 4001:4001 sequencer
```

### AWS Deployment (Experimental)

The service can be deployed to AWS Fargate using SST. First, ensure you have AWS credentials configured:

```bash
aws configure
```

Then deploy with:

```bash
npx sst deploy
```

This will:
1. Create a VPC and ECS cluster in your AWS account
2. Build and push the Docker image to Amazon ECR
3. Deploy the service to ECS Fargate with:
   - Exactly one container instance (no auto-scaling)
   - Load balancer for HTTP traffic
   - Health checks every 10 seconds
   - Resource allocation:
     - CPU: 0.25 vCPU
     - Memory: 0.5 GB
     - Storage: 20 GB

The deployment uses AWS best practices:
- Runs in a private subnet with a load balancer in the public subnet
- Automatic container health monitoring and replacement
- CloudWatch logs integration
- IAM roles with least privilege

After deployment, you can find your service URL in:
1. The SST Console
2. The deployment output
3. The AWS ECS Console

## Usage

Once the server is running, you can test your per-domain queueing. All requests require a valid API key.

### 1. Enqueue a request
With domain foo.com and request_id req-1:
```bash
curl -X POST \
     -H "Authorization: Bearer key1" \
     -H "Content-Type: application/json" \
     -d '{"domain":"foo.com","request_id":"req-1"}' \
     http://127.0.0.1:4001/sequence
```
- If foo.com is idle and the key is valid, you get "OK" immediately
- If there's already an active request, this call will block until it's your turn
- If the key is invalid or missing, you'll get a 401 response

### 2. Unblock the next request
For foo.com:
```bash
curl -X POST \
     -H "Authorization: Bearer key1" \
     -H "Content-Type: application/json" \
     -d '{"domain":"foo.com"}' \
     http://127.0.0.1:4001/done
```
- If there is a queued request for foo.com, the next one is made active
- If no one is waiting, foo.com goes idle again
- The same authentication rules apply as for /sequence

### 3. Test concurrency with multiple domains
- Requests for foo.com do not block requests for bar.com
- Requests for the same domain queue up sequentially
- Any valid API key can be used with any domain

## API Endpoints

### POST /sequence
- Headers:
  - `Authorization: Bearer <key>` (required)
  - `Content-Type: application/json`
- Body: `{ "domain": "<domain>", "request_id": "<id>" }`
- Responses:
  - `200 OK`: Request is now active
  - `401 Unauthorized`: Missing or invalid API key

### POST /done
- Headers:
  - `Authorization: Bearer <key>` (required)
  - `Content-Type: application/json`
- Body: `{ "domain": "<domain>" }`
- Responses:
  - `200 OK`: Current request marked as done
  - `401 Unauthorized`: Missing or invalid API key

### GET /health
- No authentication required
- Returns:
  - `200 OK`: Service is healthy and ready to accept requests

## Running Tests

The repository includes integration tests in `main.rs` (within `#[cfg(test)] mod tests`). These tests:
- Spawn the Axum server in-process on an ephemeral port
- Use reqwest to send real HTTP calls to `/sequence`, `/done`, and `/health`
- Check authentication (API key validation)
- Check concurrency and queueing
- Verify health check functionality

To run all tests:
```bash
cargo test
```

### Test Scenarios
- `test_authentication`: Verifies API key validation
- `test_explicit_enqueuing_flow`: Checks a specific manual order of unblocking
- `test_concurrent_different_domains`: Verifies domains do not block each other
- `test_already_idle_domain`: Calls /done on an idle domain
- `test_never_calls_done`: Confirms a second request remains blocked forever if the first never calls /done
- `test_done_multiple_times`: Ensures extra /done calls on an idle domain don't break anything
- `test_health_check`: Verifies the health check endpoint returns 200 OK

Example output:
```
running 7 tests
test tests::test_authentication ... ok
test tests::test_explicit_enqueuing_flow ... ok
test tests::test_concurrent_different_domains ... ok
test tests::test_already_idle_domain ... ok
test tests::test_never_calls_done ... ok
test tests::test_done_multiple_times ... ok
test tests::test_health_check ... ok
```