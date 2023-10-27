#!/bin/bash
set -ex

if [ "$1" = 'init' ]; then
    echo "Initializing..."
    git clone --progress --depth=1 https://github.com/rust-lang/crates.io-index.git
    ./search-crates-pm init
else
    echo "Starting..."
    ./search-crates-pm
fi
