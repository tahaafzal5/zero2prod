#!/bin/bash

set -euo pipefail

# Configuration
PI_HOST="raspberrypi"
PI_USER="taha"
PI_PROJECT_DIR="/home/taha/swdev/zero2prod"
DATABSE_USER="postgres"
DATABASE_PASSWORD="password"

echo "Syncing code to Raspberry Pi..."
rsync -avz --delete --exclude target/ --exclude .git/ ./ ${PI_USER}@${PI_HOST}:${PI_PROJECT_DIR}/

echo "Installing sqlx-cli (if needed)..."
ssh ${PI_USER}@${PI_HOST} "
    source ~/.cargo/env
    if ! command -v sqlx >/dev/null 2>&1; then
        echo 'Installing sqlx-cli...'
        cargo install sqlx-cli --no-default-features --features rustls,postgres
    fi
"

echo "Building release on Raspberry Pi..."
ssh ${PI_USER}@${PI_HOST} "
    source ~/.cargo/env
    cd ${PI_PROJECT_DIR}
    cargo build --release
"

echo "Running database migrations..."
ssh ${PI_USER}@${PI_HOST} "
    source ~/.cargo/env
    cd ${PI_PROJECT_DIR}
    DATABASE_URL='postgres://${DATABSE_USER}:${DATABASE_PASSWORD}@${PI_HOST}:5432/newsletter' sqlx migrate run
"

echo "Stopping existing application (if running)..."
ssh ${PI_USER}@${PI_HOST} "
    # Kill any process using port 8000
    PID=\$(lsof -ti :8000 2>/dev/null || echo '')
    if [[ -n \"\$PID\" ]]; then
        echo 'Stopping process using port 8000:' \$PID
        kill \$PID
    else
        echo 'No process found using port 8000'
    fi
    rm -f ${PI_PROJECT_DIR}/app.pid
"

echo "Starting application..."
ssh ${PI_USER}@${PI_HOST} "
    cd ${PI_PROJECT_DIR}
    nohup env APP_ENVIRONMENT=production ./target/release/zero2prod > app.log 2>&1 &
    echo \$! > app.pid
    echo 'Application started with PID:' \$(cat app.pid)
"

echo "Performing health check..."
sleep 3
if curl -f -s "http://${PI_HOST}:8000/health_check" > /dev/null 2>&1; then
    echo "SUCCESS: Application is running at http://${PI_HOST}:8000"
else
    echo "WARNING: Health check failed. Check ${PI_PROJECT_DIR}/app.log"
    exit
fi

echo "Build completed successfully!"
echo "Binary location on Pi: ${PI_PROJECT_DIR}/target/release/"
