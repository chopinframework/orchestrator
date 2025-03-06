# Oracle
(*oracle*)

## Overview

Request context operations

### Available Operations

* [postApiContextJson](#postapicontextjson) - Create a new context entry by request nonce
* [postApiContextMultipart](#postapicontextmultipart) - Create a new context entry by request nonce
* [postApiContextRaw](#postapicontextraw) - Create a new context entry by request nonce

## postApiContextJson

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiContextJson({
    requestNonce: "<value>",
    value: "<value>",
  });


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { oraclePostApiContextJson } from "@chopinframework/sdk/funcs/oraclePostApiContextJson.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiContextJson(chopin, {
    requestNonce: "<value>",
    value: "<value>",
  });

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request`                                                                                                                                                                      | [operations.PostApiContextJsonRequestBody](../../models/operations/postapicontextjsonrequestbody.md)                                                                           | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiContextMultipart

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiContextMultipart({
    requestNonce: "<value>",
    value: "<value>",
  });


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { oraclePostApiContextMultipart } from "@chopinframework/sdk/funcs/oraclePostApiContextMultipart.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiContextMultipart(chopin, {
    requestNonce: "<value>",
    value: "<value>",
  });

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request`                                                                                                                                                                      | [operations.PostApiContextMultipartRequestBody](../../models/operations/postapicontextmultipartrequestbody.md)                                                                 | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiContextRaw

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiContextRaw(bytesToStream(new TextEncoder().encode("0x7aDCC5c134")));


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { oraclePostApiContextRaw } from "@chopinframework/sdk/funcs/oraclePostApiContextRaw.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiContextRaw(chopin, bytesToStream(new TextEncoder().encode("0xF9f3eD4997")));

  if (!res.ok) {
    throw res.error;
  }

  const { value: result } = res;

  
}

run();
```

### Parameters

| Parameter                                                                                                                                                                      | Type                                                                                                                                                                           | Required                                                                                                                                                                       | Description                                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `request`                                                                                                                                                                      | [ReadableStream<Uint8Array>](../../models/requestbody.md)                                                                                                                      | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |