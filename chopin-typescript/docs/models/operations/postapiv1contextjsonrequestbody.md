# PostApiV1ContextJsonRequestBody

## Example Usage

```typescript
import { PostApiV1ContextJsonRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiV1ContextJsonRequestBody = {
  requestNonce: "<value>",
  value: "<value>",
};
```

## Fields

| Field                  | Type                   | Required               | Description            |
| ---------------------- | ---------------------- | ---------------------- | ---------------------- |
| `requestNonce`         | *string*               | :heavy_check_mark:     | Nonce of the request   |
| `value`                | *string*               | :heavy_check_mark:     | Context value to store |