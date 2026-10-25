#!/bin/bash
# The host and binaries of the run tree, written once (AGENTS.md section 10).
# Usage: manifest.sh <rule/v4 dir> <scratch bin dir> <crypto repo>
set -eu
ROOT=$1; BIN=$2; REPO=$3
OUT=$ROOT/runs/host.json
[ -e "$OUT" ] && { echo "$OUT exists; not overwritten"; exit 0; }
mkdir -p "$ROOT/runs"
sha() { sha256sum "$1" | cut -d' ' -f1; }
cpu=$(grep -m1 "model name" /proc/cpuinfo | cut -d: -f2- | sed 's/^ //')
flags=$(grep -m1 flags /proc/cpuinfo | tr ' ' '\n' | grep -E "^(gfni|avx512(f|bw|vbmi|vbmi2|ifma)|pclmulqdq|vpclmulqdq|bmi2|popcnt)$" | sort | tr '\n' ' ')
head=$(git -C "$REPO" rev-parse HEAD)
changed=$(git -C "$REPO" status --short -- src examples Cargo.toml | sed 's/^ *//' | tr '\n' ';')
cat > "$OUT" <<J
{
 "repository_head": "$head",
 "changed_source_paths": "$changed",
 "rustc": "$(rustc --version)",
 "cpu_model": "$cpu",
 "cpu_flags_checked": "$flags",
 "logical_cores": $(nproc),
 "kernel": "$(uname -r)",
 "memory": "$(grep MemTotal /proc/meminfo | sed 's/  */ /g')",
 "binaries_sha256": {
  "ic": "$(sha "$BIN/ic-lever")",
  "ic_rule_v4": "$(sha "$BIN/ic_rule_v4")",
  "isolated_bench": "$(sha "$BIN/isolated_bench")",
  "koblitz_rho_fixture": "$(sha "$BIN/koblitz_rho_fixture")"
 },
 "source_sha256": {
  "koblitz_index_calculus.rs": "$(sha "$REPO/src/cryptanalysis/koblitz_index_calculus.rs")",
  "price.rs": "$(sha "$REPO/src/bin/ic/price.rs")",
  "workflow.rs": "$(sha "$REPO/src/bin/ic/workflow.rs")",
  "ic_rule_v4.rs": "$(sha "$REPO/src/bin/ic_rule_v4.rs")"
 },
 "uptime": "$(uptime)",
 "note": "one L0 shared VM; timings are descriptive; no GFNI/VBMI on this host"
}
J
echo "wrote $OUT"
