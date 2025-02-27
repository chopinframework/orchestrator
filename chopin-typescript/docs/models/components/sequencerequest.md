# SequenceRequest

## Example Usage

```typescript
import { SequenceRequest } from "@chopinframework/sdk/models/components";

let value: SequenceRequest = {
  domain: "negligible-provider.name",
  requestId: "<id>",
};
```

## Fields

| Field                              | Type                               | Required                           | Description                        |
| ---------------------------------- | ---------------------------------- | ---------------------------------- | ---------------------------------- |
| `domain`                           | *string*                           | :heavy_check_mark:                 | domain for the SequenceRequest     |
| `requestId`                        | *string*                           | :heavy_check_mark:                 | request_id for the SequenceRequest |