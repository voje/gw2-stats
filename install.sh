#!/bin/bash

cargo build --release
sudo install ./target/release/gw2-stats /usr/local/bin/.
