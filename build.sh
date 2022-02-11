#!/usr/bin/env bash

cargo build --target wasm32-unknown-unknown --release --package ic-cron-time-alignment && \
 ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/ic_cron_time_alignment.wasm -o ./target/wasm32-unknown-unknown/release/ic-cron-time-alignment-opt.wasm