#!/usr/bin/env bash
# All frozen construction inputs after the six terminal representation attempts.
# Reuse the three already constructed regression inputs; no solver is run.
set -euo pipefail
if test "${1:-}" = --prime; then
  joint_out=$2
  joint_prime=$3
  export RAYON_NUM_THREADS=1
  ulimit -c 0
  while IFS=$'\t' read -r joint_id joint_p joint_seed joint_arm joint_q joint_public joint_sha; do
    [[ "$joint_id" =~ ^p[0-9]+-s[012]-(phi-l0-active|double-l0-active|l0-peel)-q[0-9]+$ ]]
    joint_dir="$joint_out/exports/$joint_id"
    test ! -e "$joint_dir"
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.started.txt"
    if test "$joint_p" = 37 && test "$joint_seed" = 0 && test "$joint_q" = 2; then
      joint_origin=$(cat "$joint_out/metadata/reuse-root.txt")
      cp -a "$joint_origin/$joint_arm" "$joint_dir"
      printf 'reused-existing-complete-export\n' > "$joint_out/metadata/$joint_id.mode.txt"
      printf '0\n' > "$joint_out/metadata/$joint_id.exit"
      printf '%s\n' "$joint_origin/$joint_arm" > "$joint_out/metadata/$joint_id.reused-from.txt"
    else
      joint_command=(prlimit --as=8589934592 -- timeout --kill-after=10s 1200s "$joint_out/runner" --joint-variant-export "$joint_public" "$joint_sha" "$joint_arm" "$joint_q" "$joint_dir")
      printf '%q ' "${joint_command[@]}" > "$joint_out/metadata/$joint_id.command.txt"
      printf 'executed-new-export\n' > "$joint_out/metadata/$joint_id.mode.txt"
      set +e
      "${joint_command[@]}" > "$joint_out/metadata/$joint_id.stdout" 2> "$joint_out/metadata/$joint_id.stderr"
      joint_rc=$?
      set -e
      printf '%s\n' "$joint_rc" > "$joint_out/metadata/$joint_id.exit"
    fi
    date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/$joint_id.finished.txt"
  done < <(jq -r --argjson p "$joint_prime" '.tasks[] | select(.p==$p) | [.id,.p,.seed,.arm,.query_index,.public_path,.public_sha256] | @tsv' "$joint_out/workload.json")
  exit 0
fi
joint_repo=$(git rev-parse --show-toplevel)
joint_study="$joint_repo/research/twist_index_calculus_20261020/joint_image_elimination"
joint_out=${1:?new output directory}
test ! -e "$joint_out"
test -f "$joint_study/runs/variant-regression-v1/complete.txt"
test -f "$joint_study/runs/verification-v8/complete.txt"
mkdir -p "$joint_out/metadata" "$joint_out/exports"
joint_out=$(cd "$joint_out" && pwd)
cp "$0" "$joint_out/metadata/executed-script.sh"
cp "$joint_study/runs/algebra-v1/workload.json" "$joint_out/workload.json"
cp "$joint_study/runs/verification-v8/runner" "$joint_out/runner"
chmod 500 "$joint_out/runner"
printf '%s\n' "$joint_study/runs/verification-v8/variants" > "$joint_out/metadata/reuse-root.txt"
git rev-parse HEAD > "$joint_out/metadata/orchestration-commit.txt"
cp "$joint_study/runs/verification-v8/metadata/source-commit.txt" "$joint_out/metadata/runner-source-commit.txt"
sha256sum "$joint_out/runner" "$joint_out/workload.json" "$joint_out/metadata/executed-script.sh" > "$joint_out/metadata/frozen.sha256"
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/started-utc.txt"
for joint_prime in 37 73 97 157 229 241 337; do
  # One reservation per field, rather than per short constructor process.
  # Every individual process still has its original 8 GiB/1,200 s cap.
  python3 "$joint_repo/tools/isolated_bench.py" busy -- bash "$joint_out/metadata/executed-script.sh" --prime "$joint_out" "$joint_prime"
done
date -u '+%Y-%m-%dT%H:%M:%SZ' > "$joint_out/metadata/finished-utc.txt"
printf 'Every workload position has a terminal construction or explicit reused-export receipt. Independent validation is still required; no complete-root or cost claim.\n' > "$joint_out/complete.txt"
