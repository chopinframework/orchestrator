# PostApiRequestJsonRequestBody

## Example Usage

```typescript
import { PostApiRequestJsonRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiRequestJsonRequestBody = {
  url: "https://second-newsletter.name",
  method: "<value>",
  headers: {},
  body: "<value>",
};
```

## Fields

| Field                                                                                        | Type                                                                                         | Required                                                                                     | Description                                                                                  |
| -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `url`                                                                                        | *string*                                                                                     | :heavy_check_mark:                                                                           | Complete URL of the request                                                                  |
| `method`                                                                                     | *string*                                                                                     | :heavy_check_mark:                                                                           | HTTP method (GET, POST, PUT, etc.)                                                           |
| `headers`                                                                                    | [operations.PostApiRequestJsonHeaders](../../models/operations/postapirequestjsonheaders.md) | :heavy_check_mark:                                                                           | HTTP headers                                                                                 |
| `body`                                                                                       | *string*                                                                                     | :heavy_check_mark:                                                                           | The request body content                                                                     |