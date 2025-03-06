# PostApiRequestMultipartRequestBody

## Example Usage

```typescript
import { PostApiRequestMultipartRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiRequestMultipartRequestBody = {
  body: "<value>",
  headers: {},
  method: "<value>",
  url: "https://punctual-humor.org/",
};
```

## Fields

| Field                                                                                                  | Type                                                                                                   | Required                                                                                               | Description                                                                                            |
| ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `body`                                                                                                 | *string*                                                                                               | :heavy_check_mark:                                                                                     | The request body content                                                                               |
| `headers`                                                                                              | [operations.PostApiRequestMultipartHeaders](../../models/operations/postapirequestmultipartheaders.md) | :heavy_check_mark:                                                                                     | HTTP headers                                                                                           |
| `method`                                                                                               | *string*                                                                                               | :heavy_check_mark:                                                                                     | HTTP method (GET, POST, PUT, etc.)                                                                     |
| `url`                                                                                                  | *string*                                                                                               | :heavy_check_mark:                                                                                     | Complete URL of the request                                                                            |