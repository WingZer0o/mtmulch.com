#!/usr/bin/env bash
# One-time Flux setup for the home-lab k3s cluster. Run from WSL on the dev PC.
# See clusters/homelab/README.md for the steps before and after this.
#
#   KUBECONFIG=~/.kube/homelab.yaml ./clusters/homelab/bootstrap.sh
#
# What it does:
#   1. Installs the Flux controllers (from gotk-components.yaml in this repo).
#   2. Loads the SOPS age private key into the cluster as flux-system/sops-age.
#   3. Creates an SSH deploy key for GitHub (flux-system/flux-system) and prints
#      the public half — add it to the repo with write access.
#   4. Applies gotk-sync.yaml, after which Flux manages itself and the site from Git.
set -euo pipefail
cd "$(dirname "$0")/../.."
export PATH="$HOME/.local/bin:$PATH"

REPO_SSH=ssh://git@github.com/WingZer0o/mtmulch.com
AGE_KEY="${SOPS_AGE_KEY_FILE:-$HOME/.config/sops/age/keys.txt}"
k() { if command -v k3s >/dev/null; then k3s kubectl "$@"; else kubectl "$@"; fi; }

: "${KUBECONFIG:?Set KUBECONFIG to the home-lab cluster, e.g. KUBECONFIG=~/.kube/homelab.yaml}"
server="$(k config view --minify -o jsonpath='{.clusters[0].cluster.server}')"
echo "Target cluster: $server"
if [[ "$server" == *127.0.0.1* || "$server" == *localhost* ]]; then
  echo "Refusing: that's a local cluster, not the home lab." >&2
  exit 1
fi
[[ -f "$AGE_KEY" ]] || { echo "Missing age key at $AGE_KEY" >&2; exit 1; }

echo "==> Checking prerequisites"
flux check --pre

echo "==> Installing Flux controllers"
k apply --server-side -f clusters/homelab/flux-system/gotk-components.yaml
k -n flux-system wait --for=condition=available deployment --all --timeout=180s

echo "==> Loading SOPS age key"
k -n flux-system create secret generic sops-age \
  --from-file=age.agekey="$AGE_KEY" \
  --dry-run=client -o yaml | k apply -f -

if ! k -n flux-system get secret flux-system >/dev/null 2>&1; then
  echo "==> Creating GitHub deploy key"
  flux create secret git flux-system --url="$REPO_SSH" >/dev/null
fi
echo
echo "Deploy key (add at GitHub → repo Settings → Deploy keys, tick 'Allow write access'):"
k -n flux-system get secret flux-system -o jsonpath='{.data.identity\.pub}' | base64 -d
echo
read -rp "Press Enter once the deploy key is added on GitHub... "

echo "==> Applying Git sync"
k apply -f clusters/homelab/flux-system/gotk-sync.yaml
flux reconcile source git flux-system
flux reconcile kustomization flux-system --with-source

echo
flux get all -A
