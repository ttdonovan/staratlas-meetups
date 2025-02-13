# Development

Notes on development of Star Atlas Meetups.

* https://www.anchor-lang.com/docs/installation

## Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
rustc 1.84.1 (e71f9a9a9 2025-01-27)
```

## Bun

```
curl -fsSL https://bun.sh/install | bash
bun --version
1.2.2
```

## Solana CLI

```
source ~/.profile
solana --version
solana-cli 2.1.5 (src:4da190bd; feat:288566304, client:Agave)

solana config get
...

mkdir tmp
solana-keygen new -o ./tmp/id.json
solana config set --keypair ./tmp/id.json

# solana config set --url mainnet-beta
solana config set --url localhost

solana config get

solana-test-validator --limit-ledger-size 1000
solana airdrop 2

solana address
solana balance

# https://explorer.solana.com
```

## AVM

```
cargo install --git https://github.com/coral-xyz/anchor avm --force

avm --version
avm 0.30.1

avm use 0.30.1

anchor --version
anchor-cli 0.30.1
```

## Meetups (Anchor Program Template)

```
anchor init meetups --template multiple
mv meetups anchor
cd anchor
anchor build
```

### Anchor Bankrun

```
bun init .
bun add @coral-xyz/anchor @solana/web3.js
bun add --dev @types/jest anchor-bankrun jest ts-jest
bun install

cd anchor
mkdir -p tests/fixtures
touch tests/meetups.spec.ts
```

## Refernce: Foobaring (Solana Dapp Template)

```
cd tmp
bunx create-solana-dapp
┌  create-solana-dapp 4.1.2
│
◇  Enter project name
│  foobaring
│
◇  Select a framework
│  Next.js
│
◇  Select a template
│  next-tailwind-counter
...

cd foobaring
bun install
# bun run dev

cd anchor
anchor test --skip-local-validator --skip-deploy
```