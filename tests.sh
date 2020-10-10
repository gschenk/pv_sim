#!/bin/sh

# paths to crate
paths=("pv solarize meter")

for p in ${paths}
do
  echo "Test Crate ${p}"
  cargo test --manifest-path=${p}/Cargo.toml
done
