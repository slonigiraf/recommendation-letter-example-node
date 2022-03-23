# Substrate Node Template
The example node that uses [recommendation-leter](https://github.com/slonigiraf/recommendation-letter) pallet to manage onchain reputation.
Was built with [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

## Getting Started

### Rust Setup

Install Rust [setup instructions](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/).


### Build

```sh
git clone https://github.com/slonigiraf/recommendation-letter-example-node.git
cd recommendation-letter-example-node
cargo build --release
```

### Run

```sh
./target/release/node-template --dev --tmp
```

### Connect with Example UI Front-end

Open [UI](https://github.com/slonigiraf/recommendation-letter-example-ui).

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
rm -rf ~/.cargo/registry
touch .local
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
