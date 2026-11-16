#!/usr/bin/env bash
# IsoLab command-job entry point for the frozen strong-rho protocol.
set -euo pipefail

[[ $# == 1 ]] || { echo "usage: $0 low|mid|high|top" >&2; exit 2; }
split=$1
case "$split" in low|mid|high|top) ;; *) echo "unknown split: $split" >&2; exit 2 ;; esac
: "${ISOLAB_RESULTS_URI:?}"

root=$(git rev-parse --show-toplevel)
study=$root/research/quartic_ghs_strong_rho_20261114
out=${ISOLAB_OUTPUT_DIR:-/work/results}
mkdir -p "$out"
revision=$(git -C "$root" rev-parse HEAD)
[[ -z $(git -C "$root" status --porcelain) ]] || { echo "dirty source checkout" >&2; exit 2; }

export AWS_DEFAULT_REGION=${AWS_DEFAULT_REGION:-${AWS_REGION:-us-east-2}}
export GHS_CALIB_TRIPLES=600
export GHS_ISOLATION=isolab-eks-standard

upload() {
  (
    flock 9
    aws s3 sync "$out" "$ISOLAB_RESULTS_URI" --exclude SHA256SUMS --exclude .upload.lock --only-show-errors
  ) 9> "$out/.upload.lock"
}

seal_upload() {
  (
    flock 9
    cd "$out"
    find . -type f ! -name SHA256SUMS ! -name .upload-list ! -name .upload.lock -printf '%P\n' \
      | LC_ALL=C sort > .upload-list
    if [[ -s .upload-list ]]; then
      xargs -d '\n' sha256sum < .upload-list > SHA256SUMS
    else
      : > SHA256SUMS
    fi
    rm -f .upload-list
    aws s3 sync "$out" "$ISOLAB_RESULTS_URI" --exclude .upload.lock --only-show-errors
  ) 9> "$out/.upload.lock"
}

syncer=
pids=()
finish() {
  rc=$?
  trap - EXIT TERM INT
  mapfile -t active < <(jobs -pr)
  if [[ ${#active[@]} -gt 0 ]]; then
    kill "${active[@]}" 2>/dev/null || true
    wait "${active[@]}" 2>/dev/null || true
  fi
  printf 'finish=%s rc=%s\n' "$(date -u +%FT%TZ)" "$rc" >> "$out/exit_codes.txt"
  seal_upload || rc=90
  exit "$rc"
}
trap finish EXIT
trap 'exit 143' TERM
trap 'exit 130' INT

{
  printf 'split=%s\nrevision=%s\nstart=%s\n' "$split" "$revision" "$(date -u +%FT%TZ)"
  printf 'nproc=%s\n' "$(nproc)"
  uname -a
  lscpu
  grep -E 'MemTotal|MemAvailable' /proc/meminfo
  rustc --version --verbose
  jq --version
  aws --version
} > "$out/host.txt" 2>&1

sha256sum "$study/PROTOCOL.md" "$study/main.rs" "$study/strong_rho.rs" \
  "$study/pod_run.sh" > "$out/source.sha256"
printf '%s\n' "$revision" > "$out/source_revision.txt"

binary=$out/ghs-strong-rho
GHS_SOURCE_REVISION=$revision rustc --edition 2021 -C opt-level=3 -C target-cpu=native \
  "$study/main.rs" -o "$binary"
sha256sum "$binary" > "$out/binary.sha256"
"$binary" selftest > "$out/selftest.out" 2>&1
grep -qx 'SELFTEST OK' "$out/selftest.out"
upload

(
  while sleep 300; do upload; done
) &
syncer=$!

run_cell() {
  local q=$1 seed=$2 repetitions=$3 arm=$4 tag=$5
  local dst=$out/q${q}_s${seed}_${tag}.jsonl
  "$binary" run "$q" "$seed" 0 "$repetitions" ic-repeat "$arm" \
    > "$dst" 2> "$out/q${q}_s${seed}_${tag}.stderr"
  jq -e -s --arg rev "$revision" --arg arm "$arm" --argjson n "$repetitions" '
    ([.[] | select(.rec=="instance") | .source_revision] | length >= 1) and
    all(.[] | select(.rec=="instance"); .source_revision==$rev and .target_policy=="ic-repeat") and
    ([.[] | select(.rec=="rho" and .arm==$arm)] | length)==$n and
    all(.[] | select(.rec=="rho");
      .ok==true and (.arm=="old-neg" or .k_found==.k) and .target_kind=="ic")
  ' "$dst" >/dev/null
}

run_seed() {
  local seed=$1
  case "$split" in
    low)
      for q in 4051 8191 16363 32719 65479 131071 262111; do
        run_cell "$q" "$seed" 5 strong-b8 primary
        case "$q" in
          4051|65479|262111)
            run_cell "$q" "$seed" 1 strong-b4 width-low-control
            run_cell "$q" "$seed" 1 strong-b16 width-high-control
            run_cell "$q" "$seed" 1 old-neg frozen-control
            ;;
        esac
      done
      ;;
    mid)
      run_cell 524287 "$seed" 3 strong-b8 primary
      run_cell 1048507 "$seed" 3 strong-b8 primary
      ;;
    high)
      run_cell 2097091 "$seed" 3 strong-b8 primary
      ;;
    top)
      run_cell 4194271 "$seed" 3 strong-b8 primary
      ;;
  esac
}

for seed in 1 2 3; do
  run_seed "$seed" > "$out/seed${seed}.log" 2>&1 &
  pids+=("$!")
done

failed=0
for i in 0 1 2; do
  seed=$((i + 1))
  if wait "${pids[$i]}"; then
    printf 'seed=%s rc=0\n' "$seed" >> "$out/exit_codes.txt"
  else
    rc=$?
    printf 'seed=%s rc=%s\n' "$seed" "$rc" >> "$out/exit_codes.txt"
    failed=1
  fi
  upload
done

if [[ $split == low && $failed == 0 ]]; then
  replay=$out/q4051_s1_replay.jsonl
  run_cell 4051 1 1 strong-b8 replay
  jq -S 'select(.rec=="rho" and .arm=="strong-b8" and .i==0)
    | del(.wall_s)' "$out/q4051_s1_primary.jsonl" > "$out/replay_expected.json"
  jq -S 'select(.rec=="rho" and .arm=="strong-b8" and .i==0)
    | del(.wall_s)' "$replay" > "$out/replay_observed.json"
  cmp "$out/replay_expected.json" "$out/replay_observed.json"
  printf 'replay=match\n' >> "$out/exit_codes.txt"
fi

[[ $failed == 0 ]]
