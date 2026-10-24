#!/usr/bin/env bash
# Install or launch the existing clients; never provision a hub or lab worker.
set -euo pipefail
repo=$(cd "$(dirname "$0")/.." && pwd)
state=${ISOLAB_AGENT_STATE_DIR:-"$repo/target/isolab-agent"}

fail() { printf 'Iso Lab: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "missing $1; see docs/ISOLAB_AGENTS.md"; }
usage() { echo 'Usage: bash scripts/isolab-agent.sh {setup|check|mcp} {nats|eks}' >&2; exit 2; }
[[ $# == 2 ]] || usage
action=$1
backend=$2
case "$action:$backend" in
  setup:nats|setup:eks|check:nats|check:eks|mcp:nats|mcp:eks) ;;
  *) usage ;;
esac

# Installation is explicit, never a side effect of starting an MCP server.
if [[ "$action" == setup ]]; then
  case "${WORKSPACE_OFFLINE:-0}" in
    0|'') ;;
    1) fail 'offline mode: preinstall clients; setup downloads are disabled' ;;
    *) fail 'WORKSPACE_OFFLINE must be 0 or 1' ;;
  esac
  mkdir -p "$state"
  if [[ "$backend" == nats ]]; then
    need uv
    export UV_CACHE_DIR=${UV_CACHE_DIR:-"$state/uv-cache"}
    uv venv --allow-existing "$state/nats" >&2
    uv pip install --python "$state/nats/bin/python" --editable "$repo/isolab[mcp]" >&2
  else
    need cargo
    deploy="$repo/taskq/deploy/eks-karpenter"
    cargo build --locked --release --manifest-path "$deploy/mcp/Cargo.toml" \
      --target-dir "$state/eks-target" >&2
    install -d "$state/eks/bin"
    install -m 755 "$state/eks-target/release/isolab-eks-mcp" "$state/eks/bin/isolab-eks-mcp"
    install -m 755 "$deploy/submit-job" "$state/eks/bin/submit-job"
  fi
  printf 'Iso Lab %s client installed. Run: bash scripts/isolab-agent.sh check %s\n' "$backend" "$backend" >&2
  exit 0
fi

# A supplied binary always wins, including an invalid override: do not silently
# switch implementations or fall back to a different lab.
if [[ "$backend" == nats ]]; then
  binary=${ISOLAB_BIN:-"$state/nats/bin/isolab"}
  if [[ -z "${ISOLAB_BIN:-}" && ! -x "$binary" ]]; then
    binary=$(command -v isolab || true)
  fi
else
  binary=${ISOLAB_MCP_BIN:-"$state/eks/bin/isolab-eks-mcp"}
  if [[ -z "${ISOLAB_MCP_BIN:-}" && ! -x "$binary" ]]; then
    binary=$(command -v isolab-eks-mcp || true)
  fi
fi
[[ -n "$binary" && -x "$binary" ]] || fail "client missing; run bash scripts/isolab-agent.sh setup $backend or set the binary override"

if [[ "$backend" == nats ]]; then
  [[ -n "${ISOLAB_URL:-}" ]] || fail 'set ISOLAB_URL to the existing hub; supply ISOLAB_TOKEN or ISOLAB_CREDS through secret settings'
  [[ -z "${ISOLAB_CREDS:-}" || ( -r "$ISOLAB_CREDS" && -f "$ISOLAB_CREDS" ) ]] || fail 'ISOLAB_CREDS is not a readable file'
  # The project entry is stdio even if another application uses an HTTP server.
  export ISOLAB_MCP_TRANSPORT=stdio
else
  [[ -n "${KUBECONFIG:-}" && -r "$KUBECONFIG" && -f "$KUBECONFIG" ]] || fail 'set KUBECONFIG to the dedicated Iso Lab kubeconfig'
  [[ -n "${ISOLAB_KUBE_CONTEXT:-}" ]] || fail 'set ISOLAB_KUBE_CONTEXT explicitly; the current kubectl context is not used'
fi

if [[ "$action" == mcp ]]; then
  # No output before exec: stdout belongs to MCP, not setup or diagnostics.
  if [[ "$backend" == nats ]]; then exec "$binary" mcp; fi
  exec "$binary"
fi

need timeout
if [[ "$backend" == eks ]]; then
  for tool in aws kubectl jq; do need "$tool"; done
fi
# Libraries/CLIs can include credential-bearing URLs in errors. Keep probe
# output private and report only a fixed message, including on timeout.
umask 077
probe=$(mktemp)
trap 'rm -f "$probe"' EXIT
if [[ "$backend" == nats ]]; then
  timeout 30 "$binary" workers >"$probe" 2>&1 || fail 'NATS roster check failed; verify hub access, credentials and routing'
else
  timeout 60 "$binary" call isolab_k8s_overview '{}' >"$probe" 2>&1 || fail 'EKS overview check failed; verify AWS identity, kubeconfig and endpoint routing'
fi
printf 'Iso Lab %s connection check passed (read-only; no job submitted).\n' "$backend"
