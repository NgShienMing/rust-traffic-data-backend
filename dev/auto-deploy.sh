#!/bin/bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

## Stop all running docker containers
echo "==============================================="
echo "Stopping all running docker containers"
echo "==============================================="
"$SCRIPT_DIR"/stop-server.sh | 2>/dev/null

# echo "Update to latest Git changes"
# echo "==============================================="
# git pull | 2>/dev/null

echo "==============================================="
echo "Docker build the new images"
echo "==============================================="
"$SCRIPT_DIR"/build-docker.sh

echo "==============================================="
echo "Docker up all the containers"
echo "==============================================="
"$SCRIPT_DIR"/run-server.sh

echo "[+] --- Finished deploying the dockers"
