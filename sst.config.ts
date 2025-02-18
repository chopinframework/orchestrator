/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "sequencer",
      removal: input?.stage === "production" ? "retain" : "remove",
      protect: ["production"].includes(input?.stage),
      home: "aws",
    };
  },
  async run() {
    const vpc = new sst.aws.Vpc("SequencerVpc");
    const cluster = new sst.aws.Cluster("SequencerCluster", { vpc });
    
    const service = cluster.addService("Sequencer", {
      image: {
        dockerfile: "Dockerfile",
      },
      cpu: "0.25 vCPU",
      memory: "0.5 GB",
      scaling: {
        min: 1,
        max: 1,
      },
      loadBalancer: {
        ports: [{
          listen: "80/http",
          forward: "4001/http"
        }],
        health: {
          "4001/http": {
            path: "/health",
            interval: "10 seconds",
            timeout: "5 seconds",
            healthyThreshold: 2,
            unhealthyThreshold: 3
          }
        }
      }
    });

    return {
      serviceUrl: service.url,
    };
  },
});
