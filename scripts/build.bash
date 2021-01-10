#!/bin/bash

set -e

cargo +nightly build --manifest-path ../Cargo.toml --target armv7a-none-eabi --release
cp ../target/armv7a-none-eabi/release/libv5_rust.a ../pros_package/firmware/
prosv5 build --project ../pros_package
