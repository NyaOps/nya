#!/bin/bash
set -euo pipefail

DOMAIN=$1

LATEST=$(curl -fsSL https://api.github.com/repos/FiloSottile/mkcert/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
curl -fsSL "https://github.com/FiloSottile/mkcert/releases/download/${LATEST}/mkcert-${LATEST}-linux-amd64" -o /usr/local/bin/mkcert
chmod +x /usr/local/bin/mkcert
sudo mkcert -install
sudo mkdir -p /etc/nya/certs
mkcert \
  -cert-file /etc/nya/certs/${DOMAIN}.pem \
  -key-file /etc/nya/certs/${DOMAIN}-key.pem \
  "*.${DOMAIN}" ${DOMAIN}