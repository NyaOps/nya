#!/bin/bash
set -euo pipefail

K3S_TOKEN=$1
CONTROL_PLANE_IP=$2

sudo docker run -d \
  -p 5000:5000 \
  -e REGISTRY_HTTP_TLS_ENABLE=false \
  -e REGISTRY_STORAGE_DELETE_ENABLED=true \
  --restart always \
  --name registry \
  registry:2

curl -sfL https://get.k3s.io | INSTALL_K3S_TOKEN=${K3S_TOKEN} sh -s - server \
  --disable traefik \
  --disable servicelb \
  --disable local-storage \
  --advertise-address=${CONTROL_PLANE_IP} \
  --node-ip=${CONTROL_PLANE_IP}