#!/bin/bash
set -e
cargo build
exec ./target/debug/rust-playground
