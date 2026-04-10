#!/bin/bash
set -euo pipefail

export KUBECONFIG=/etc/rancher/k3s/k3s.yaml

sudo -E helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
sudo -E helm repo update

sudo -E helm upgrade --install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace \
  --set controller.kind=DaemonSet \
  --set controller.service.type=LoadBalancer

sudo -E kubectl create secret tls {{ secret_name }} \
  --key /etc/nya/certs/{{ domain }}-key.pem \
  --cert /etc/nya/certs/{{ domain }}.pem \
  --namespace ingress-nginx
