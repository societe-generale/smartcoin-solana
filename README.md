# Getting started

## Install

- Install [Rustup](https://rustup.rs/)
- Install dependencies
```bash
yarn install
```
- Install solana
```bash
curl -sSfLO https://github.com/anza-xyz/agave/releases/download/v2.0.14/solana-release-x86_64-unknown-linux-gnu.tar.bz2
bunzip2 solana-release-x86_64-unknown-linux-gnu.tar.bz2
tar xvf solana-release-x86_64-unknown-linux-gnu.tar
cd solana-release/
bin/agave-install-init v1.18.26
cd solana-release/
solana --version
```

## Build
- Get correct transfer-hook contract address
```bash
yarn keys-list
```
- Set contract address in Anchor.toml and uncomment this line
```
#transfer_hook = "<transfer hook address returned by 'yarn anchor keys list'>"
```
- Set contract address in programs/transfer-hook/src/lib.rs
```rust
declare_id!("<transfer hook address returned by 'yarn anchor keys list'>");
```
- Build project
```bash
yarn build
```

## Test

- Define needed keys in the environment
```bash
export SOLANA_REGISTRAR_PRIV=<private key for registrar role>
export SOLANA_TECHNICAL_PRIV=<private key for technical role>
export SOLANA_OPERATIONS_PRIV=<private key for operations role> 
```
- Start local blockchain
```bash
solana-test-validator -r > validator_log &
```
- Run integration tests
```bash
yarn test
```

## Deploy transfer hook on solana devnet
- First write the buffer
```bash
solana program write-buffer ./target/deploy/transfer_hook.so --use-rpc -ud
```
- Get the buffer address from the output of previous command.
e.g.
```bash
$ solana program write-buffer ./target/deploy/transfer_hook.so --use-rpc -ud
Buffer: Cg7aThjZFxUNpUHrn11BcfJh795Yt5mpHXAB1G2f1o5k
```
- Then deploy the program
```bash
solana program deploy --buffer <address from previous command> -ud --program-id ./target/deploy/transfer_hook-keypair.json --use-rpc
```

## Close program account to reclaim SOL
```bash
solana program close <ProgramId> -ud --use-rpc
```

## Deploy Smartcoin token on solana devnet
