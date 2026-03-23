#!/usr/bin/env bash

cargo build --release
cargo run --release -- norm ./stems/sub

# /Users/simondanielsson/dev/rust/reamake/target/release/reamake sort ./test/stems/
# /Users/simondanielsson/dev/rust/reamake/target/release/reamake sort

# /Users/simondanielsson/dev/rust/reamake/target/release/reamake --var client="overridden client!" --var project="overridden project!" --var stems="/Users/simondanielsson/Downloads/Michael Jackson – Thriller (1982)/" -f /Users/simondanielsson/Downloads/test.reamake

