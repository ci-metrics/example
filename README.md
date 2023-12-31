# cimetrics-hello-world

**This is a volatile experiment, you shouldn't use this, instead use https://github.com/bencherdev/bencher.**

A repo presenting an example system for tracking generic metrics in CI like [codecov](https://about.codecov.io).

![Example PR comment](./pr_comment_example.webp)

This is from https://github.com/JonathanWoollett-Light/example/pull/1.

![Example display](./display_example.webp)

This is from https://cimetrics.io/display/6546b543a35b7d5af8c93a7b/206925525691777734527727329171694843736.

## Setup

1. Create account by running `curl -X POST https://cimetrics.io/users`.
   Note down the `public_key` and `private_key`.
2. Copy [pull_request.yml](./.github/workflows/pull_request.yml) and [push.yml](./.github/workflows/push.yml) to your repo.
3. Update `PUBLIC_KEY` and use repository secrets for `PRIVATE_KEY`.

## Visualize metrics

Visit

```
https://cimetrics.io/display/<your public key>/<your private key>
```

## Display metrics on the same chart

You can display metrics on the same chart.

Set metrics to display on the same chart with:

```bash
curl -X POST https://cimetrics.io/metrics/sets -d "{ \
    \"user\": { \
        \"public_key\": \"<your public key>\", \
        \"private_key\": <your private key> \
    }, \
    \"metric_sets\": [ \
        [\"metric_one\",\"metric_two\"], \
        [\"metric_three\",\"metric_four\"] \
    ] \
}"
```

This sets:

- `metric_one` and `metric_two` to display on the same chart.
- `metric_three` and `metric_four` to display on the same chart.

*Any metric will only ever display on 1 chart.*

Get metrics you display on the same chart with:

```bash
curl -X GET https://cimetrics.io/metrics/sets -d "{ \
    \"public_key\": \"<your public key>\", \
    \"private_key\": <your private key> \
}"
```

## Billing

**Billing is currently free. It will only be charged on stable release**

You add credits to your account with
`https://cimetrics.io/payment/<your public key>/<your private key>/<amount in USD cents>`.

Billing will be based on a cost per each type of API request and overall storage used.