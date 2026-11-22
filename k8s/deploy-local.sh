#!/usr/bin/env bash
# Build the images, import them into k3s, and deploy the local overlay.
#
# Usage (from WSL):
#   ./k8s/deploy-local.sh              # build + import + deploy
#   ./k8s/deploy-local.sh --no-build   # re-apply manifests only
#
# Run it as your normal user (not with sudo): it prompts for sudo only for the
# k3s image import, which needs root.
set -euo pipefail

cd "$(dirname "$0")/.."
export KUBECONFIG="${KUBECONFIG:-/etc/rancher/k3s/k3s.yaml}"

# Use k3s's bundled kubectl. Docker Desktop's WSL-integration kubectl runs from a
# Docker-managed mount and crashes with SIGBUS when Docker Desktop updates it.
if command -v k3s >/dev/null; then
  kubectl() { k3s kubectl "$@"; }
fi

OVERLAY=k8s/overlays/local
NS=mtmulch
IMAGES=(mtmulch-api:local mtmulch-web:local mtmulch-migrate:local)

build=1
[[ "${1:-}" == "--no-build" ]] && build=0

# --- secrets -----------------------------------------------------------------
if [[ ! -f "$OVERLAY/secrets.env" ]]; then
  echo "==> Generating $OVERLAY/secrets.env"
  pg_pw="$(openssl rand -hex 16)"
  cat > "$OVERLAY/secrets.env" <<EOF
POSTGRES_PASSWORD=$pg_pw
DATABASE_URL=postgres://postgres:$pg_pw@postgres:5432/rust_playground
PASETO_SECRET_KEY=$(openssl rand -hex 32)
BLOB_ENCRYPTION_KEY=$(openssl rand -hex 32)
RESEND_API_KEY=re_local_placeholder
EOF
  chmod 600 "$OVERLAY/secrets.env"
fi

# --- images ------------------------------------------------------------------
if (( build )); then
  echo "==> Building images"
  docker build -t mtmulch-api:local .
  docker build -t mtmulch-web:local frontend
  docker build -f k8s/migrate.Dockerfile -t mtmulch-migrate:local .

  echo "==> Importing images into k3s (sudo)"
  docker save "${IMAGES[@]}" | sudo k3s ctr images import -
fi

# --- deploy ------------------------------------------------------------------
echo "==> Applying $OVERLAY"
# Jobs are immutable; remove the previous migration run so it re-runs.
kubectl -n "$NS" delete job migrate --ignore-not-found
kubectl apply -k "$OVERLAY"

echo "==> Waiting for Postgres and migrations"
kubectl -n "$NS" rollout status statefulset/postgres --timeout=180s
kubectl -n "$NS" wait --for=condition=complete job/migrate --timeout=300s

if (( build )); then
  # The tag stays ":local", so restart to pick up the newly imported images.
  kubectl -n "$NS" rollout restart deployment/api deployment/web
fi
kubectl -n "$NS" rollout status deployment/azurite --timeout=180s
kubectl -n "$NS" rollout status deployment/api --timeout=180s
kubectl -n "$NS" rollout status deployment/web --timeout=180s

echo
kubectl -n "$NS" get pods
echo
echo "Open the site with:"
echo "  k3s kubectl -n kube-system port-forward svc/traefik 8088:80"
echo "then browse to http://localhost:8088 (works from Windows too)."
