#!/bin/bash
set -eu pipefail

DOMAIN=$1
SECRET_NAME=$2

export KUBECONFIG=/etc/rancher/k3s/k3s.yaml

# Remove TLS secret
sudo -E kubectl delete secret ${SECRET_NAME} \
  --namespace ingress-nginx \
  --ignore-not-found

# Uninstall nginx-ingress
sudo -E helm uninstall ingress-nginx \
  --namespace ingress-nginx \
  --ignore-not-found

# Uninstall metallb
sudo -E helm uninstall metallb \
  --namespace metallb-system \
  --ignore-not-found

# Remove namespaces
sudo -E kubectl delete namespace ingress-nginx --ignore-not-found
sudo -E kubectl delete namespace metallb-system --ignore-not-found