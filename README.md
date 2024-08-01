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
```
forc build --release
```
The `--release` flag is highly recommended, as you'd probably want to interact
with the optimized contract, even when testing. If you see any warnings during
compilation, you can ignore them, because they come from Fuel-maintained
dependencies, not the contract.

### Test the smart contract
Run the contract's unit tests:
```
forc t --release
```

Run the contract's integration tests (written in Rust):
