# Sequencer
(*sequencer*)

## Overview

Request sequencing operations

### Available Operations

* [postApiV1RequestJson](#postapiv1requestjson) - Sequence an HTTP request
* [postApiV1RequestMultipart](#postapiv1requestmultipart) - Sequence an HTTP request
* [postApiV1RequestRaw](#postapiv1requestraw) - Sequence an HTTP request

## postApiV1RequestJson

Process an HTTP request through the sequencer

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.sequencer.postApiV1RequestJson({
    url: "https://possible-seagull.com",
    method: "<value>",
    headers: {

    },
    body: "<value>",
  });


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerPostApiV1RequestJson } from "@chopinframework/sdk/funcs/sequencerPostApiV1RequestJson.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerPostApiV1RequestJson(chopin, {
    url: "https://possible-seagull.com",
    method: "<value>",
    headers: {
  
    },
    body: "<value>",
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
| `request`                                                                                                                                                                      | [operations.PostApiV1RequestJsonRequestBody](../../models/operations/postapiv1requestjsonrequestbody.md)                                                                       | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiV1RequestMultipart

Process an HTTP request through the sequencer

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.sequencer.postApiV1RequestMultipart({
    body: "<value>",
    headers: {

    },
    method: "<value>",
    url: "https://possible-seagull.com",
  });


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerPostApiV1RequestMultipart } from "@chopinframework/sdk/funcs/sequencerPostApiV1RequestMultipart.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerPostApiV1RequestMultipart(chopin, {
    body: "<value>",
    headers: {
  
    },
    method: "<value>",
    url: "https://possible-seagull.com",
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
| `request`                                                                                                                                                                      | [operations.PostApiV1RequestMultipartRequestBody](../../models/operations/postapiv1requestmultipartrequestbody.md)                                                             | :heavy_check_mark:                                                                                                                                                             | The request object to use for the request.                                                                                                                                     |
| `options`                                                                                                                                                                      | RequestOptions                                                                                                                                                                 | :heavy_minus_sign:                                                                                                                                                             | Used to set various options for making HTTP requests.                                                                                                                          |
| `options.fetchOptions`                                                                                                                                                         | [RequestInit](https://developer.mozilla.org/en-US/docs/Web/API/Request/Request#options)                                                                                        | :heavy_minus_sign:                                                                                                                                                             | Options that are passed to the underlying HTTP request. This can be used to inject extra headers for examples. All `Request` options, except `method` and `body`, are allowed. |
| `options.retries`                                                                                                                                                              | [RetryConfig](../../lib/utils/retryconfig.md)                                                                                                                                  | :heavy_minus_sign:                                                                                                                                                             | Enables retrying HTTP requests under certain failure conditions.                                                                                                               |

### Response

**Promise\<void\>**

### Errors

| Error Type      | Status Code     | Content Type    |
| --------------- | --------------- | --------------- |
| errors.APIError | 4XX, 5XX        | \*/\*           |

## postApiV1RequestRaw

Process an HTTP request through the sequencer

### Example Usage

```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  await chopin.sequencer.postApiV1RequestRaw(bytesToStream(new TextEncoder().encode("0x233E5fC9a8")));


}

run();
```

### Standalone function

The standalone function version of this method:

```typescript
import { ChopinCore } from "@chopinframework/sdk/core.js";
import { sequencerPostApiV1RequestRaw } from "@chopinframework/sdk/funcs/sequencerPostApiV1RequestRaw.js";

// Use `ChopinCore` for best tree-shaking performance.
// You can create one instance of it to use across an application.
const chopin = new ChopinCore({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const res = await sequencerPostApiV1RequestRaw(chopin, bytesToStream(new TextEncoder().encode("0x313e7CC0FF")));

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