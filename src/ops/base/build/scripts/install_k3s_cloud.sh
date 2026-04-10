#!/bin/bash
set -euo pipefail

curl -sfL https://get.k3s.io | INSTALL_K3S_TOKEN={{ k3s_token }} sh -s - server \
  --disable traefik \
  --disable servicelb \
  --disable local-storage \
  --advertise-address={{ control_plane_ip }} \
  --node-ip={{ control_plane_ip }} \
  --tls-san={{ control_plane_ip }}