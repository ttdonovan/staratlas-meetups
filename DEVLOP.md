# Development

Notes on development of Star Atlas Meetups.

* https://docs.anza.xyz/cli/install

```
solana --version
solana-cli 2.1.5 (src:4da190bd; feat:288566304, client:Agave)

solana config get
...

# solana config set --url mainnet-beta
solana config set --url localhost

mkdir tmp
solana-keygen new -o .\tmp\id.json
solana config set --keypair .\tmp\id.json

solana-test-validator
solana airdrop 2
```

* https://www.anchor-lang.com/docs

```
cargo install --git https://github.com/coral-xyz/anchor avm --force

avm --version
avm 0.30.1

avm use 0.30.1

anchor --version
anchor-cli 0.30.1
```

```
anchor init meetups --template multiple
cd meetups

anchor build
```