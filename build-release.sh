#!/usr/bin/env bash

set -exu

cargo build --release --target x86_64-unknown-linux-musl
