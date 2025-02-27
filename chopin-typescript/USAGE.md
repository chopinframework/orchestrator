<!-- Start SDK Example Usage [usage] -->
```typescript
import { Chopin } from "@chopinframework/sdk";

const chopin = new Chopin({
  serverURL: "https://api.example.com",
  bearerAuth: process.env["CHOPIN_BEARER_AUTH"] ?? "",
});

async function run() {
  const result = await chopin.sequencer.sequence({
    domain: "accurate-eternity.info",
    requestId: "<id>",
  });

  // Handle the result
  console.log(result);
}

run();

```
<!-- End SDK Example Usage [usage] -->