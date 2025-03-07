# Oracle
(*oracle*)

## Overview

Request context operations

### Available Operations

* [postApiV1ContextJson](#postapiv1contextjson) - Create a new context entry by request nonce
* [postApiV1ContextMultipart](#postapiv1contextmultipart) - Create a new context entry by request nonce
* [postApiV1ContextRaw](#postapiv1contextraw) - Create a new context entry by request nonce

## postApiV1ContextJson

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiV1ContextJson({
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
import { oraclePostApiV1ContextJson } from "@chopinframework/sdk/funcs/oraclePostApiV1ContextJson.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiV1ContextJson(chopin, {
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
| `request`                                                                                                                                                                      | [operations.PostApiV1ContextJsonRequestBody](../../models/operations/postapiv1contextjsonrequestbody.md)                                                                       | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiV1ContextMultipart

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiV1ContextMultipart({
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
import { oraclePostApiV1ContextMultipart } from "@chopinframework/sdk/funcs/oraclePostApiV1ContextMultipart.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiV1ContextMultipart(chopin, {
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
| `request`                                                                                                                                                                      | [operations.PostApiV1ContextMultipartRequestBody](../../models/operations/postapiv1contextmultipartrequestbody.md)                                                             | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiV1ContextRaw

Creates a new context entry for a request identified by its nonce

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.oracle.postApiV1ContextRaw(bytesToStream(new TextEncoder().encode("0x29DB7fEcED")));


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { oraclePostApiV1ContextRaw } from "@chopinframework/sdk/funcs/oraclePostApiV1ContextRaw.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await oraclePostApiV1ContextRaw(chopin, bytesToStream(new TextEncoder().encode("0x5BFf91eFb0")));

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