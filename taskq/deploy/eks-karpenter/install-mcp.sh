#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
[[ $# == 1 ]] || { echo 'Usage: install-mcp.sh ABSOLUTE_INSTALL_DIRECTORY' >&2; exit 2; }
[[ "$1" == /* ]] || { echo 'Install directory must be absolute' >&2; exit 2; }
for tool in cargo kubectl aws jq timeout; do command -v "$tool" >/dev/null; done
cargo build --locked --release --manifest-path mcp/Cargo.toml --target-dir "$PWD/mcp/target"
install -d -m 755 "$1"
install -m 755 mcp/target/release/isolab-eks-mcp "$1/isolab-eks-mcp"
install -m 755 submit-job "$1/submit-job"
printf 'Installed %s/isolab-eks-mcp\nSet KUBECONFIG and ISOLAB_KUBE_CONTEXT before connecting.\n' "$1"
