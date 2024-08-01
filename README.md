# Guild pin contract for Fuel
This repo contains an implementation of the Guild pin smart contract for the
Fuel chain, written in [sway](https://docs.fuel.network/docs/sway/).
Additionally, it contains tests and examples to interact with the contract.

## Interacting with the code
### [Install Rust](https://www.rust-lang.org/tools/install)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### [Install Fuelup](https://install.fuel.network/master/installation/index.html)
```
curl -fsSL https://install.fuel.network/ | sh
```

### Setup custom toolchain

Unfortunately, I ran into this
[issue](https://github.com/FuelLabs/fuels-rs/issues/1449) when running tests,
so as a temporary workaround, you'll need to add the following components to a
custom toolchain. Feel free to give a different name than `custom-toolchain`.

```
fuelup toolchain new custom-toolchain
fuelup component add fuel-core@0.28.0
fuelup component add forc@0.60.0
```

Maybe in the future, this won't be necessary, but the code, in its current
state, runs only with this setup.

```
fuelup show
```
should output something like

```
installed toolchains
--------------------
latest-x86_64-unknown-linux-gnu
custom-toolchain (default)

active toolchain
----------------
custom-toolchain (default)
  forc : 0.60.0
    - forc-client
      - forc-deploy : 0.60.0
      - forc-run : 0.60.0
    - forc-crypto : 0.60.0
    - forc-debug : 0.60.0
    - forc-doc : 0.60.0
    - forc-explore : not found
    - forc-fmt : 0.60.0
    - forc-lsp : 0.60.0
    - forc-tx : 0.60.0
    - forc-wallet : not found
  fuel-core : 0.28.0
  fuel-core-keygen : 0.28.0

fuels versions
--------------
forc : 0.62.0
```

### Build the smart contract

**NOTE** it is recommended to use the `--release` flag with every command (even
`clippy`) because the contract is compiled to either `target/release` or
`target/debug`, depending on the optimization level. If you run into errors,
check that you're using the `--release` flag, because the issue probably
emanates from the compiler not finding the contract and its ABI.

```
forc build --release
```
The `--release` flag is highly recommended, as you'd probably want to interact
with the optimized contract, even when testing. If you see any warnings during
compilation, you can ignore them, because they come from Fuel-maintained
dependencies, not the contract.

This will build the contract and the respective ABI in the `target/release`
directory. If you omit the `--release` flag, the output will be created in
`target/debug`. You should pay close attention to this, because the integration
tests will look for the contract in `target/release` when tests are run with
the `--release` flag and vice versa.

### Test the smart contract
Run the contract's unit tests:
```
forc t --release
```

Run the contract's integration tests (written in Rust):

```
cargo t --release
```

The `--release` flag is required, because the tests will look for the optimized
contract binary and ABI in the `target/release` directory.

### Deploy and interact with the smart contract
You can interact with the contract via `examples/pin.rs`.

To see the available commands, run
```
cargo run --release --example pin -- --help
```

1) Deploy the contract

First, add the secret keys of the backend signer, the deployer and the treasury
to a given path. The signer seed should be 32 bytes in the form of `[0, 1, 2,
3,...]`. The deployer and treasury secret keys should be a 64 characters long
hex encoded string, i.e. 32 bytes hex encoded to a string.
```
cargo run --release --example pin \\
-- \\
--url <mainnet-url> \\
--signer <path-to-signer-seed> \\
--deployer <path-to-deployer-sk> \\
--treasury <path-to-treasury-sk> \\
deploy
```
You might ask, why do we need the signer seed and the treasury seed here? Well,
in order to be usable for tests, we definitely need the signer seed, however
the treasury is indeed not necessary. Feel free to change the code accordingly.

2) Set the backend signer address
The first thing you should do as an admin after deploying/testing the contract
is setting the backend signer address. The default value for the signer address
is already set to the current Guild backend signer, so it should be overridden
only if it changes on the backend.

```
cargo run --release --example pin \\
-- \\
--url <mainnet-url> \\
--deployer <path-to-deployer-sk> \\
set-signer
```

2) Set the treasury fee

```
cargo run --release --example pin \\
-- \\
--url <mainnet-url> \\
--deployer <path-to-deployer-sk> \\
set-fee <fee>
```

2) Set the treasury address

```
cargo run --release --example pin \\
-- \\
--url <mainnet-url> \\
--deployer <path-to-deployer-sk> \\
set-treasury <treasury-address>
```

3) Fetch a pin's metadata
```
cargo run --release --example pin \\
-- \\
--url <mainnet-url> \\
--deployer <path-to-deployer-sk> \\
metadata -p <pin-id>
```
