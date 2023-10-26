#!/bin/bash

set -e

setup_cron() {
    echo "*/10 * * * * MEILI_HOST_URL=$MEILI_HOST_URL MEILI_INDEX_UID=$MEILI_INDEX_UID MEILI_API_KEY=$MEILI_API_KEY /app/live" > /etc/crontabs/root
    crond
}

if [ "$1" = 'init' ]; then
    echo "Initializing..."
    git clone --progress --depth=1 https://github.com/rust-lang/crates.io-index.git
    ./full_init
else
    echo "Starting..."
    setup_cron
    tail -f /dev/null
fi
