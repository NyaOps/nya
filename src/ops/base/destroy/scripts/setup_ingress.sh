#!/bin/bash
set -eu pipefail

DOMAIN=$1
SECRET_NAME=$2
IP_RANGE=$3

sudo helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
sudo helm repo add metallb https://metallb.github.io/metallb
sudo helm repo update
sudo helm upgrade --install metallb metallb/metallb \
  --namespace metallb-system \
  --create-namespace
cat <<EOF | sudo kubectl apply -f -
apiVersion: metallb.io/v1beta1
kind: IPAddressPool
metadata:
  name: default-pool
  namespace: metallb-system
spec:
  addresses:
  - ${IP_RANGE}
---
apiVersion: metallb.io/v1beta1
kind: L2Advertisement
metadata:
  name: default
  namespace: metallb-system
EOF
sudo kubectl rollout status deployment/metallb-controller -n metallb-system --timeout=120s
sudo helm upgrade --install ingress-nginx ingress-nginx/ingress-nginx \
  --namespace ingress-nginx \
  --create-namespace \
  --set controller.kind=DaemonSet \
  --set controller.service.type=LoadBalancer

sudo kubectl create secret tls ${SECRET_NAME} \
  --key /etc/nya/certs/${DOMAIN}-key.pem \
  --cert /etc/nya/certs/${DOMAIN}.pem \
  --kubeconfig=/etc/rancher/k3s/k3s.yaml \
  --namespace ingress-nginx