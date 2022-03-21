#!/bin/bash
cargo build --target=i686-unknown-linux-gnu --release
scp ./target/release/mafi hrandom:/root/mafi

