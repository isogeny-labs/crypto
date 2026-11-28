#!/usr/bin/env bash
# One build and one attempt per registered rung. No retry or resume loop.
set -euo pipefail
export RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1 CARGO_BUILD_JOBS=4
out=${ISOLAB_OUTPUT_DIR:?}
mkdir -p "$out"
cp /opt/tournament/request.json "$out/request.json"
deadline=$(date -u -d "$(jq -r .original_deadline_utc /opt/tournament/request.json)" +%s)
remaining() { local seconds=$((deadline-$(date -u +%s))); ((seconds>0)) || return 1; echo "$seconds"; }
upload() {
  # SHA256SUMS is explicitly excluded from find's input.
  # shellcheck disable=SC2094
  (cd "$out" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 -r sha256sum > SHA256SUMS)
  aws s3 sync "$out/" "${ISOLAB_RESULTS_URI:?}" --only-show-errors
}
# Invoked by the EXIT trap.
# shellcheck disable=SC2329
finish() { local code=$?; trap - EXIT; printf '%s\n' "$code" > "$out/job.exit"; upload || exit 91; exit "$code"; }
trap finish EXIT
trap 'exit 143' TERM
cp -a /opt/source /work/repo
cd /work/repo
git config --global --add safe.directory /work/repo
rustc --version > "$out/rustc.txt"
git rev-parse HEAD > "$out/algorithm_revision.txt"
uname -a > "$out/uname.txt"
lscpu > "$out/lscpu.txt"
cat /proc/self/status > "$out/process-status.txt"
cat /proc/cmdline > "$out/kernel-cmdline.txt"
cp /opt/tournament/run.sh "$out/run.sh"
# Build and validate before taking the timing reservation. Every failure is terminal for this build.
build_seconds=$(remaining)
((build_seconds>3600)) && build_seconds=3600
set +e
timeout --signal=TERM --kill-after=45s "${build_seconds}s" cargo build --release --locked \
  --example koblitz_orbit_dlp_fast_online --example koblitz_rho_batch_ks_strong_online \
  --example koblitz_orbit_dlp_fast --example koblitz_rho_batch_ks_strong > "$out/build.log" 2>&1
build_status=$?
set -e
printf '%s\n' "$build_status" > "$out/build.exit"
if ((build_status)); then
  for n in 61 71 73 83; do printf 'BLOCKED_BUILD_FAILURE\n' > "$out/n${n}.status"; done
  exit "$build_status"
fi
sha256sum target/release/examples/koblitz_orbit_dlp_fast{,_online} target/release/examples/koblitz_rho_batch_ks_strong{,_online} > "$out/binaries.sha256"
upload
cpu=$(python3 -c 'import os; print(sorted(os.sched_getaffinity(0))[-1])')
cpus=$(cat "/sys/devices/system/cpu/cpu${cpu}/topology/thread_siblings_list")
printf '%s\n' "$cpu" > "$out/pinned_cpu.txt"
status=0
for n in 61 71 73 83; do
  if ! seconds=$(remaining); then printf 'NOT_ATTEMPTED_DEADLINE\n' > "$out/n${n}.status"; status=124; continue; fi
  # Leave a minute for final durable upload before the original absolute deadline.
  seconds=$((seconds-60)); ((seconds>10800)) && seconds=10800
  if ((seconds<=0)); then printf 'NOT_ATTEMPTED_DEADLINE\n' > "$out/n${n}.status"; status=124; continue; fi
  workloads=$(jq -r --argjson n "$n" '.beats[]|select(.n==$n)|.workloads' /opt/tournament/request.json)
  tune=$(jq -r --argjson n "$n" '.beats[]|select(.n==$n)|.tune_workloads_per_k' /opt/tournament/request.json)
  run_id="${ISOLAB_JOB_ID}-n${n}"
  printf 'RUNNING\n' > "$out/n${n}.status"
  set +e
  timeout --signal=TERM --kill-after=45s "${seconds}s" python3 tools/isolated_bench.py reserve \
    --cpus "$cpus" --max-psi 5 --out "$out/n${n}.isolation.jsonl" --label "$run_id" -- \
    python3 research/sat_factor_base_review_20260908/autolab/boundary_autolab.py launch-single \
      --beat "koblitz.compact_orbit.n${n}_single_target" --workloads "$workloads" \
      --tune-workloads "$tune" --run-id "$run_id" --cpu "$cpu" > "$out/n${n}.log" 2>&1
  code=$?
  set -e
  printf '%s\n' "$code" > "$out/n${n}.exit"
  printf 'EXIT_%s\n' "$code" > "$out/n${n}.status"
  run="research/sat_factor_base_review_20260908/autolab/runs/$run_id"
  if [[ -d "$run" ]]; then
    tar -czf "$out/n${n}.run.tar.gz" -C "$(dirname "$run")" "$run_id"
    [[ ! -f "$run/state.json" ]] || cp "$run/state.json" "$out/n${n}.state.json"
    for f in single_target_summary.json claim_check.json claim_draft.json; do
      [[ ! -f "$run/artifacts/$f" ]] || cp "$run/artifacts/$f" "$out/n${n}.$f"
    done
  fi
  ((code==0)) || status=1
  upload
 done
exit "$status"
