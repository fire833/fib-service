#!/bin/bash

curl -f http://localhost:8080/v1/fibonacci/50 | jq || { echo "Failed"; exit 1; }

curl -f http://localhost:8080/healthz | jq || { echo "Healthcheck Failed"; exit 1; }
curl -f http://localhost:8080/readyz | jq || { echo "Readiness check Failed"; exit 1; }
curl http://localhost:8080/unknown_endpoint | jq || { echo "Not found Failed"; exit 1; }
