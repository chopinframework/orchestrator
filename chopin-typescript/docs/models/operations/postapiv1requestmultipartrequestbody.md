# PostApiV1RequestMultipartRequestBody

## Example Usage

```typescript
import { PostApiV1RequestMultipartRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiV1RequestMultipartRequestBody = {
  body: "<value>",
  headers: {
    "key": "<value>",
  },
  method: "<value>",
  url: "https://bright-agreement.net/",
};
```

## Fields

| Field                              | Type                               | Required                           | Description                        |
| ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| `body`                             | *string*                           | :heavy_check_mark:                 | The request body content           |
| `headers`                          | Record<string, *string*>           | :heavy_check_mark:                 | HTTP headers                       |
| `method`                           | *string*                           | :heavy_check_mark:                 | HTTP method (GET, POST, PUT, etc.) |
| `url`                              | *string*                           | :heavy_check_mark:                 | Complete URL of the request        |