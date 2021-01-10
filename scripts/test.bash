#!/bin/bash

cargo +nightly build --manifest-path ../Cargo.toml --target armv7a-none-eabi --release --features v5_test
cp ../target/armv7a-none-eabi/release/libv5_rust.a ../pros_package/firmware/
prosv5 build --project ../pros_package
prosv5 upload --project ../pros_package --name aros_test --slot 8
prosv5 v5 run 8
