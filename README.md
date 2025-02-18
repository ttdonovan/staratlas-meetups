# Star Atlas Meetups

## Quick Start

Required dependencies.

```bash
$ rustc --version
rustc 1.84.1 (e71f9a9a9 2025-01-27)

$ solana --version
solana-cli 2.1.14 (src:de749b63; feat:3271415109, client:Agave)

$ avm --version
avm 0.30.1

$ avm use 0.30.1
Now using anchor version 0.30.1.

$ bun --version
1.2.2
```

### Anchor Bankrun (Specs)

Run a local Solana Test Validator (localhost).

```bash
$ mkdir tmp
$ solana-keygen new --outfile ./tmp/id.json --no-bip39-passphrase
$ solana config set --url localhost --keypair ./tmp/id.json
$ solana-test-validator
```

Build and run the Anchor program.

```bash
$ bun install
$ bun run anchor-build
$ bun run anchor-specs
```

### Cargo LiteSVM (Tests)

```
$ bun run anchor-tests
# or
# $ cd anchor && anchor build && cargo test
```