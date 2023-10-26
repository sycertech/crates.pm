#!/bin/bash

set -e

export TZ=UTC

setup_cron() {
    echo "*/10 * * * * MEILI_HOST_URL=$MEILI_HOST_URL MEILI_INDEX_UID=$MEILI_INDEX_UID MEILI_API_KEY=$MEILI_API_KEY /app/live" >> /var/log/cron.log 2>&1
    echo "00 02 * * * MEILI_HOST_URL=$MEILI_HOST_URL MEILI_INDEX_UID=$MEILI_INDEX_UID MEILI_API_KEY=$MEILI_API_KEY /app/update_downloads" >> /var/log/cron.log 2>&1
    crond
}

if [ "$1" = 'init' ]; then
    echo "Initializing..."
    git clone --progress --depth=1 https://github.com/rust-lang/crates.io-index.git
    ./init
else
    echo "Starting..."
    setup_cron
    tail -f /var/log/cron.log
fi
