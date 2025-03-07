<!-- Start SDK Example Usage [usage] -->
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
<!-- End SDK Example Usage [usage] -->