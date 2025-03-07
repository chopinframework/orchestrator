# PostApiV1RequestJsonRequestBody

## Example Usage

```typescript
import { PostApiV1RequestJsonRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiV1RequestJsonRequestBody = {
  url: "https://second-newsletter.name",
  method: "<value>",
  headers: {
    "key": "<value>",
  },
  body: "<value>",
};
```

## Fields

| Field                              | Type                               | Required                           | Description                        |
| ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| `url`                              | *string*                           | :heavy_check_mark:                 | Complete URL of the request        |
| `method`                           | *string*                           | :heavy_check_mark:                 | HTTP method (GET, POST, PUT, etc.) |
| `headers`                          | Record<string, *string*>           | :heavy_check_mark:                 | HTTP headers                       |
| `body`                             | *string*                           | :heavy_check_mark:                 | The request body content           |