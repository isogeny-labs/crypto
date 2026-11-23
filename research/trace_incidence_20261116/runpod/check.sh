#!/usr/bin/env bash
# Native correctness only; do not extract performance timings from this call.
set -euo pipefail
here=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
check_dir=$(mktemp -d "${TMPDIR:-/tmp}/trace-runpod-check.XXXXXX")
printf '%s\n' "$check_dir"
make -C "$here" > "$check_dir/build.log" 2>&1
cp "$here/build/benchmark" "$check_dir/qualifier"
chmod 0555 "$check_dir/qualifier"
sha256sum "$here/benchmark.cpp" "$here/incidence_lazy.cpp" "$here/pair_control_callable.cpp" "$here/auxiliary.gp" "$check_dir/qualifier" > "$check_dir/build.sha256"
set -o noclobber
cp "$check_dir/build.sha256" "$check_dir/pre-call.sha256"
for arm in dense trace pullback dense-isogeny trace-isogeny pullback-isogeny flint; do
  "$check_dir/qualifier" --check-only root "$arm" 1 >> "$check_dir/checks.jsonl"
done
"$check_dir/qualifier" --check-only datum 7 20261116 "$here/auxiliary.gp" 0 1 >> "$check_dir/checks.jsonl"
for arm in known pairs trace dense pullback pullback-isogeny; do
  "$check_dir/qualifier" --check-only incidence "$arm" "$here/nonlinear-control.json" 20261116 "$here/auxiliary.gp" >> "$check_dir/checks.jsonl"
done
for kind in generic collision exception_plus exception_minus; do
  for arm in trace dense; do
    "$check_dir/qualifier" --check-only translation "$arm" "$here/../historical/fast_trace/results_large/fixtures/p1000003-d256-$kind.json" >> "$check_dir/checks.jsonl"
  done
done
sha256sum "$here/benchmark.cpp" "$here/incidence_lazy.cpp" "$here/pair_control_callable.cpp" "$here/auxiliary.gp" "$check_dir/qualifier" > "$check_dir/post-call.sha256"
cmp "$check_dir/pre-call.sha256" "$check_dir/post-call.sha256"
jq -se 'length==22 and all(.status=="passed" or .status=="relation") and all(.timings_collected==false)' "$check_dir/checks.jsonl" >/dev/null
printf '%s\n' '22 native no-timer checks passed'
