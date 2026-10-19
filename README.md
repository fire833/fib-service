
# Fib-service

This is a small toy service that serves the Fibonacci numbers via an API. 

### Running

You can either run this program directly with `cargo`:

```bash
# Look at CLI args:
cargo run --release -- -h

# Run with the built-in test config.
cargo run --release -- -c config/test.yaml
```

or build/run with `docker` or `buildah`:

```
docker build . -t fib-service:latest

docker run -it -v ./config/:/app -e RUST_LOG=debug -e FIB_SERVICE_CONFIG_LOCATION=/app/test.yaml fib-service:latest
```

### Deploying

This application is meant to be run in something like Kubernetes. 
It is worth noting that the service exposes the `/readyz` and `/healthz` (currently both follow the same code path, but this could change over time) that will only be served once the service is up, and defaults to returning 200. 
You could add probes to your deployment to only route traffic to copies of this service once it is actually ready/alive. 

### Design Decisions

#### Pre-computed fibonacci numbers

I made the decision to pre-cache the numbers for the purposes of simplicity. This was a very intentional tradeoff between service complexity, memory, and amortizing compute costs for end users. 
In this case, I chose to optimize for making sure each user has the fastest API call possible with all values within a range precomputed. 
If you were scaling this kind of service up a substantial amount, you would probably have multiple instances and it would be worth caching values in an external cache like Redis/memcached.
You are able to specify the max fibonacci number to precompute in config, and an additional feature could be to not exclude overly large numbers by computing any large value by hand. But, that is a potential avenue for a DoS attack, which is part of why I chose to put the hard cap on the max length of the sequence.

#### Authentication/Authorization

I chose to omit any kind of authentication/authorization system for this service. 
In theory, you could easily add a JWKS set verifier and only allow incoming requests with a valid oauth access tokens with a particular scope in the scopes claim, but this seemed out of scope. You could also have this service sit behind a load balancer (such as a Gateway API) and have the load balancer authenticate requests, or you could use the [Oauth2 Proxy](https://oauth2-proxy.github.io/oauth2-proxy/).
