#!/usr/bin/env bash

set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)"

cd "$DIR"

cargo build --release
cp ./target/release/devpost-cli ~/.local/bin/devpost
