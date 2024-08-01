# Install Fuelup
# Setup custom toolchain

fuelup toolchain new custom-toolchain
fuelup component add fuel-core@0.28.0
fuelup component add forc@0.60.0


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
