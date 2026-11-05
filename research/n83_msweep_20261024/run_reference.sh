#!/usr/bin/env bash
# Thin orchestration only: the repository's m=4 rootfinder baseline (two
# root-table layouts, K=1107) and the matched signed-Frobenius strong rho,
# on the same exposed public point, alternated, one fresh process each.
# Usage: run_reference.sh <ic_binary> <rho_binary> <out_dir>
set -euo pipefail
ic=$1; rho=$2; out=$3
here=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$out"
common="KIC_PACKED_ROOT_TABLE=1 KIC_PACKED_ROOT_TABLE_DENSE=0 KIC_PACKED_ROOT_TABLE_LOAD75=0 KIC_STATE28=0 KIC_STATE26=0 KIC_ROOT_ONLY21=0 KIC_INDEXED_ROOT5=1 KIC_INDEXED_ROOT4=0 KIC_SYMMETRIC_S3=1 KIC_INDEXED4_FIXED31=0 KIC_SPARSE_CANONICAL=1 KIC_RANK_THREADS=4 KIC_TARGET_THREADS=1"
root11="KIC_ROOT_ONLY22=0 KIC_ROOT_ONLY11=1 KIC_ROOT_BLOOM=1 KIC_INDEXED5_LOAD75=1 KIC_INDEXED5_FIXED30=0"
root22="KIC_ROOT_ONLY22=1 KIC_ROOT_ONLY11=0 KIC_ROOT_BLOOM=0 KIC_INDEXED5_LOAD75=1 KIC_INDEXED5_FIXED30=0"
run_rho() {
  local tag=$1 seed=$2 t0
  t0=$(date +%s.%N)
  timeout 600 "$rho" "$here/point.jsonl" "$seed" "$here/rho_config.json" "$out/$tag.receipt.json" > "$out/$tag.stdout" 2> "$out/$tag.stderr" || echo "$tag rc=$?"
  echo "$(date +%s.%N) - $t0" | bc > "$out/$tag.wall"
}
run_ic() {
  local tag=$1 layout=$2 t0
  t0=$(date +%s.%N)
  mkdir -p "$out/$tag"
  env -i PATH=/usr/bin:/bin $common $layout KIC_DUMP_BASE="$out/$tag/base_dump.jsonl" \
    timeout 900 "$ic" construct:83:1:1107 "$here/point.jsonl" 29 "$out/$tag/targets_result.jsonl" > "$out/$tag.stdout" 2> "$out/$tag.stderr" || echo "$tag rc=$?"
  echo "$(date +%s.%N) - $t0" | bc > "$out/$tag.wall"
  gzip -f "$out/$tag/base_dump.jsonl" 2>/dev/null || true
}
run_rho rho_a 2026102401
run_ic ic_root11_k1107 "$root11"
run_rho rho_b 2026102402
run_ic ic_root22_k1107 "$root22"
run_rho rho_c 2026102403
echo done
