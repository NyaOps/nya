#!/bin/bash
set -eu pipefail

# Stop and remove the registry container
sudo docker stop registry || true
sudo docker rm registry || true

# Uninstall k3s server
if [ -f /usr/local/bin/k3s-uninstall.sh ]; then
  sudo /usr/local/bin/k3s-uninstall.sh
else
  echo "k3s uninstall script not found, may already be uninstalled"
fi