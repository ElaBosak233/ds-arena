# ds-arena

A simple validator for Data Security challenge.

## How to use

Prepare a CSV file named `criteria.csv`, then prepare a Dockerfile like this:

```dockerfile
FROM elabosak233/ds-arena:latest

COPY ./criteria.csv /app/resources/criteria.csv

ENV FLAG=cdsctf{whats_your_prolem}
```

Then build and run your own image. The service listens on port `8889`.

## Local development

Install the Rust toolchain specified in `rust-toolchain.toml` and Node.js 26
(Node.js 24.15+ or 22.22.2+ also works).

From the repository root, start the backend:

```sh
cargo run --locked --bin dsa-server
```

The default criteria file is `resources/criteria.csv`. In another terminal, start
the frontend:

```sh
cd web
npm ci --include=dev
npm run dev
```

Vite forwards `/api` to `http://127.0.0.1:8889`. Set `VITE_DEV_API` to use another
backend. To serve the production frontend directly from the backend, build it
with `npm run build` in `web`, copy `web/dist` to `dist` in the repository root,
and start the backend from that root.

## Configuration

| Environment variable | Default | Purpose |
| --- | --- | --- |
| `SERVER_HOST` | `0.0.0.0` | Listen address |
| `SERVER_PORT` | `8889` | Listen port |
| `LOGGER_LEVEL` | `info` | Log filter |
| `CRITERIA_PATH` | `resources/criteria.csv` | Reference CSV path, relative to the working directory |
| `EXPECTED_ACCURACY` | `0.98` | Required score between 0 and 1 |
| `CAPTCHA_DIFFICULTY` | `2` | Proof of work difficulty |
| `FLAG` | Example value | Result message for a passing submission; set for your challenge |

## Validation

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cd web
npm run check
npm test
npm run build
```

The checks workflow runs these commands for pushes and pull requests. Both
dependency lockfiles are committed; use `cargo --locked` and `npm ci` for
reproducible installs.

## Docker

```sh
docker build -t ds-arena .
docker run --rm -p 8889:8889 -e FLAG='your-result-message' ds-arena
```

The image builds a static Rust executable and includes the production frontend.
Mount a custom criteria file at `/app/resources/criteria.csv` or set
`CRITERIA_PATH` to its location in the container.
