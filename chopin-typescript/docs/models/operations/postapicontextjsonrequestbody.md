# PostApiContextJsonRequestBody

## Example Usage

```typescript
import { PostApiContextJsonRequestBody } from "@chopinframework/sdk/models/operations";

let value: PostApiContextJsonRequestBody = {
  requestNonce: "<value>",
  value: "<value>",
};
```

## Fields

| Field                  | Type                   | Required               | Description            |
| ---------------------- | ---------------------- | ---------------------- | ---------------------- |
| `requestNonce`         | *string*               | :heavy_check_mark:     | Nonce of the request   |
| `value`                | *string*               | :heavy_check_mark:     | Context value to store |