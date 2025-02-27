# Sequencer
(*sequencer*)

## Overview

Sequencer API endpoints

### Available Operations

* [sequence](#sequence) - Sequence operation
* [done](#done) - Done operation
* [health](#health) - Health operation
* [openapiJson](#openapijson) - Openapi_json operation

## sequence

Wait until it's our turn for `domain`.
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
            "openapi": "3.0.3",
            "info": {
                "title": "Sequencer API",
                "description": "API for sequencing domain operations",
                "version": "0.1.0",
                "contact": {
                    "name": "API Support",
                    "email": "support@example.com"
                }
            },
            "paths": {
                "/sequence": {
                    "post": {
                        "summary": "Sequence operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain",
                        "operationId": "sequence",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/SequenceRequest"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful sequence operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/done": {
                    "post": {
                        "summary": "Done operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const result = await chopin.sequencer.sequence({
    domain: "accurate-eternity.info",
    requestId: "<id>",
  });

  // Handle the result
  console.log(result);
}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerSequence } from "@chopinframework/sdk/funcs/sequencerSequence.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerSequence(chopin, {
    domain: "accurate-eternity.info",
    requestId: "<id>",
  });

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  // Handle the result
  console.log(result);
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request`                                                                                                                                                                      | [components.SequenceRequest](../../models/components/sequencerequest.md)                                                                                                       | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<[string](../../models/.md)\>**

### Errors

| Error Type           | Status Code          | Content Type         |
| -------------------- | -------------------- | -------------------- |
| errors.ErrorResponse | 401                  | application/json     |
| errors.APIError      | 4XX, 5XX             | \*/\*                |

## done

Wait until it's our turn for `domain`.
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
            "openapi": "3.0.3",
            "info": {
                "title": "Sequencer API",
                "description": "API for sequencing domain operations",
                "version": "0.1.0",
                "contact": {
                    "name": "API Support",
                    "email": "support@example.com"
                }
            },
            "paths": {
                "/sequence": {
                    "post": {
                        "summary": "Sequence operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain",
                        "operationId": "sequence",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/SequenceRequest"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful sequence operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/done": {
                    "post": {
                        "summary": "Done operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain
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
    Ok(\"OK\")
}

/// Handler to mark a domain's active request as done",
                        "operationId": "done",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/DoneRequest"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful done operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/health": {
                    "get": {
                        "summary": "Health operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain
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
    Ok(\"OK\")
}

/// Handler to mark a domain's active request as done

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const result = await chopin.sequencer.done({
    domain: "jittery-blossom.name",
  });

  // Handle the result
  console.log(result);
}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerDone } from "@chopinframework/sdk/funcs/sequencerDone.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerDone(chopin, {
    domain: "jittery-blossom.name",
  });

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  // Handle the result
  console.log(result);
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request`                                                                                                                                                                      | [components.DoneRequest](../../models/components/donerequest.md)                                                                                                               | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<[string](../../models/.md)\>**

### Errors

| Error Type           | Status Code          | Content Type         |
| -------------------- | -------------------- | -------------------- |
| errors.ErrorResponse | 401                  | application/json     |
| errors.APIError      | 4XX, 5XX             | \*/\*                |

## health

Wait until it's our turn for `domain`.
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
            "openapi": "3.0.3",
            "info": {
                "title": "Sequencer API",
                "description": "API for sequencing domain operations",
                "version": "0.1.0",
                "contact": {
                    "name": "API Support",
                    "email": "support@example.com"
                }
            },
            "paths": {
                "/sequence": {
                    "post": {
                        "summary": "Sequence operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain",
                        "operationId": "sequence",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/SequenceRequest"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful sequence operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/done": {
                    "post": {
                        "summary": "Done operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain
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
    Ok(\"OK\")
}

/// Handler to mark a domain's active request as done",
                        "operationId": "done",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/DoneRequest"
                                    }
                                }
                            }
                        },
                        "responses": {
                            "200": {
                                "description": "Successful done operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/health": {
                    "get": {
                        "summary": "Health operation",
                        "description": "Wait until it's our turn for `domain`.
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
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
        } else {
            println!(\"[Sequencer] Domain={}, RequestId={} is enqueued\", domain, request_id);
            let (tx, rx) = oneshot::channel();
            queue.waiting.push_back(tx);
            drop(map);

            // Wait until unblocked by .done()
            let _ = rx.await;
            println!(\"[Sequencer] Domain={}, RequestId={} is now active\", domain, request_id);
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

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Deserialize, Serialize)]
struct SequenceRequest {
    domain: String,
    request_id: String,
}

#[derive(Deserialize, Serialize)]
struct DoneRequest {
    domain: String,
}

//
// ------------------
// OpenAPI Specification
// ------------------
//

/// Returns the OpenAPI specification as a JSON object
async fn openapi_json() -> Json<serde_json::Value> {
    let openapi_spec = serde_json::json!({
        \"openapi\": \"3.0.3\",
        \"info\": {
            \"title\": \"Sequencer API\",
            \"description\": \"API for sequencing domain operations\",
            \"version\": \"0.1.0\",
            \"contact\": {
                \"name\": \"API Support\",
                \"email\": \"support@example.com\"
            }
        },
        \"paths\": {
            \"/sequence\": {
                \"post\": {
                    \"summary\": \"Sequence a request for a domain\",
                    \"description\": \"Request will be queued if there's already an active request for the domain\",
                    \"operationId\": \"sequence\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/SequenceRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Request has been sequenced\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/done\": {
                \"post\": {
                    \"summary\": \"Mark a domain's active request as done\",
                    \"description\": \"If other requests are waiting, the next one will become active\",
                    \"operationId\": \"done\",
                    \"tags\": [\"sequencer\"],
                    \"security\": [{\"BearerAuth\": []}],
                    \"requestBody\": {
                        \"required\": true,
                        \"content\": {
                            \"application/json\": {
                                \"schema\": {
                                    \"$ref\": \"#/components/schemas/DoneRequest\"
                                }
                            }
                        }
                    },
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Domain marked as done\",
                            \"content\": {
                                \"text/plain\": {
                                    \"schema\": {
                                        \"type\": \"string\",
                                        \"example\": \"OK\"
                                    }
                                }
                            }
                        },
                        \"401\": {
                            \"description\": \"Unauthorized - Invalid or missing API key\",
                            \"content\": {
                                \"application/json\": {
                                    \"schema\": {
                                        \"$ref\": \"#/components/schemas/ErrorResponse\"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            \"/health\": {
                \"get\": {
                    \"summary\": \"Health check endpoint\",
                    \"description\": \"Returns 200 OK if the service is healthy\",
                    \"operationId\": \"health\",
                    \"tags\": [\"sequencer\"],
                    \"responses\": {
                        \"200\": {
                            \"description\": \"Service is healthy\"
                        }
                    }
                }
            }
        },
        \"components\": {
            \"securitySchemes\": {
                \"BearerAuth\": {
                    \"type\": \"http\",
                    \"scheme\": \"bearer\"
                }
            },
            \"schemas\": {
                \"SequenceRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\", \"request_id\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier for sequencing requests\"
                        },
                        \"request_id\": {
                            \"type\": \"string\",
                            \"description\": \"Unique identifier for this request\"
                        }
                    }
                },
                \"DoneRequest\": {
                    \"type\": \"object\",
                    \"required\": [\"domain\"],
                    \"properties\": {
                        \"domain\": {
                            \"type\": \"string\",
                            \"description\": \"Domain identifier to mark as done\"
                        }
                    }
                },
                \"ErrorResponse\": {
                    \"type\": \"object\",
                    \"required\": [\"message\"],
                    \"properties\": {
                        \"message\": {
                            \"type\": \"string\",
                            \"description\": \"Error message\"
                        }
                    }
                }
            }
        },
        \"tags\": [
            {
                \"name\": \"sequencer\",
                \"description\": \"Sequencer API endpoints\"
            }
        ]
    });

    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain
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
    Ok(\"OK\")
}

/// Handler to mark a domain's active request as done
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
    Ok(\"OK\")
}

/// Health check endpoint handler",
                        "operationId": "health",
                        "tags": [
                            "sequencer"
                        ],
                        "responses": {
                            "200": {
                                "description": "Successful health operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                "/openapi.json": {
                    "get": {
                        "summary": "Openapi_json operation",
                        "description": "openapi_json operation",
                        "operationId": "openapi_json",
                        "tags": [
                            "sequencer"
                        ],
                        "security": [
                            {
                                "BearerAuth": []
                            }
                        ],
                        "responses": {
                            "200": {
                                "description": "Successful openapi_json operation",
                                "content": {
                                    "text/plain": {
                                        "schema": {
                                            "type": "string",
                                            "example": "OK"
                                        }
                                    }
                                }
                            },
                            "401": {
                                "description": "Unauthorized - Invalid or missing API key",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "$ref": "#/components/schemas/ErrorResponse"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "SequenceRequest": {
                        "type": "object",
                        "properties": {
                            "domain": {
                                "type": "string",
                                "description": "domain for the SequenceRequest"
                            },
                            "request_id": {
                                "type": "string",
                                "description": "request_id for the SequenceRequest"
                            }
                        },
                        "required": [
                            "domain",
                            "request_id"
                        ]
                    },
                    "DoneRequest": {
                        "type": "object",
                        "properties": {
                            "domain": {
                                "type": "string",
                                "description": "domain for the DoneRequest"
                            }
                        },
                        "required": [
                            "domain"
                        ]
                    },
                    "ErrorResponse": {
                        "type": "object",
                        "properties": {
                            "message": {
                                "type": "string",
                                "description": "Error message"
                            }
                        },
                        "required": [
                            "message"
                        ]
                    }
                },
                "securitySchemes": {
                    "BearerAuth": {
                        "type": "http",
                        "scheme": "bearer"
                    }
                }
            },
            "tags": [
                {
                    "name": "sequencer",
                    "description": "Sequencer API endpoints"
                }
            ]
        });
    
    Json(openapi_spec)
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

/// Handler to sequence a request for a specific domain
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

/// Handler to mark a domain's active request as done
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

/// Health check endpoint handler

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const result = await chopin.sequencer.health();

  // Handle the result
  console.log(result);
}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerHealth } from "@chopinframework/sdk/funcs/sequencerHealth.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerHealth(chopin);

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  // Handle the result
  console.log(result);
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<[string](../../models/.md)\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## openapiJson

openapi_json operation

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const result = await chopin.sequencer.openapiJson();

  // Handle the result
  console.log(result);
}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerOpenapiJson } from "@chopinframework/sdk/funcs/sequencerOpenapiJson.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerOpenapiJson(chopin);

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  // Handle the result
  console.log(result);
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<[string](../../models/.md)\>**

### Errors

| Error Type           | Status Code          | Content Type         |
| -------------------- | -------------------- | -------------------- |
| errors.ErrorResponse | 401                  | application/json     |
| errors.APIError      | 4XX, 5XX             | \*/\*                |