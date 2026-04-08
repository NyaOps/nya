#!/bin/bash
set -euo pipefail

DOMAIN=$1

# Remove certs
sudo rm -f /etc/nya/certs/${DOMAIN}.pem
sudo rm -f /etc/nya/certs/${DOMAIN}-key.pem

# Remove mkcert CA
mkcert -uninstall

# Remove mkcert binary
sudo rm -f /usr/local/bin/mkcert

# Remove certs directory if empty
sudo rmdir --ignore-fail-on-non-empty /etc/nya/certs