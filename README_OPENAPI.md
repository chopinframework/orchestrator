# OpenAPI Documentation with utoipa

This project uses [utoipa](https://github.com/juhaku/utoipa), a library that generates OpenAPI specifications directly from your Rust code annotations. This ensures that your API documentation is always in sync with your code.

## How It Works

The OpenAPI spec generation is integrated directly into the code:

1. Code annotations on models, endpoints, and handlers define the API structure
2. The `utoipa` crate processes these annotations at compile time
3. The API server serves both the specification and a Swagger UI interface at runtime

This approach ensures that:
- Documentation is always in sync with the actual code
- Changes to endpoints or models automatically update the API documentation
- No separate documentation files need to be maintained

## Key Components

### 1. Model Annotations

Models are annotated with `#[derive(ToSchema)]` to make them visible in the OpenAPI spec:

```rust
/// Request model for the sequence endpoint
#[derive(Deserialize, Serialize, ToSchema)]
struct SequenceRequest {
    /// Domain identifier for sequencing requests
    domain: String,
    /// Unique identifier for this request
    request_id: String,
}
```

### 2. Endpoint Annotations

Endpoints are annotated with `#[utoipa::path(...)]` to define their OpenAPI specifications:

```rust
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
async fn sequence_handler(...) { ... }
```

### 3. API Documentation Definition

The overall API spec is defined using a struct with the `#[derive(OpenApi)]` attribute:

```rust
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
```

### 4. Swagger UI Integration

The API documentation is served at runtime through Swagger UI:

```rust
// Generate OpenAPI documentation
let api_doc = ApiDoc::openapi();

Router::new()
    // API routes...
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc.clone()))
    .route("/api-docs/openapi.json", get(|| async move { Json(api_doc) }))
```

## Accessing the Documentation

When the API server is running, you can access:

- Swagger UI: `http://localhost:4001/swagger-ui/`
- Raw OpenAPI JSON: `http://localhost:4001/api-docs/openapi.json`

## Best Practices for API Documentation

To ensure high-quality API documentation, follow these practices:

1. Add descriptive doc comments to handlers:
   ```rust
   /// Sequence a request for a specific domain
   ///
   /// This comment will appear in the OpenAPI documentation.
   /// It can include multiple lines of description.
   #[utoipa::path(...)]
   async fn some_handler(...) { ... }
   ```

2. Add doc comments to schema fields:
   ```rust
   #[derive(Serialize, Deserialize, ToSchema)]
   struct MyRequest {
       /// Description of this field
       /// Can be multiple lines
       my_field: String,
   }
   ```

3. Use meaningful status code descriptions:
   ```rust
   #[utoipa::path(
       responses(
           (status = 200, description = "Detailed description of success case"),
           (status = 400, description = "Explanation of what causes this error")
       )
   )]
   ```

4. Group related endpoints with tags:
   ```rust
   #[utoipa::path(
       tag = "group-name"
   )]
   ```

## Modifying the OpenAPI Documentation

To modify the OpenAPI documentation:

1. Edit the code annotations on models and handlers
2. Update the `ApiDoc` struct definition for global changes
3. Create custom modifiers (like `SecurityAddon`) for more complex customizations
4. Run `cargo check` to validate your changes
5. Start the server to see the updated documentation

## References

- [utoipa Documentation](https://docs.rs/utoipa/latest/utoipa/)
- [OpenAPI Specification](https://swagger.io/specification/)
- [Swagger UI](https://swagger.io/tools/swagger-ui/) 