#!/usr/bin/env bash
# Native Mac execution explicitly selected by the owner. One attempt per rung.
set -euo pipefail
control=$(cd "$(dirname "$0")" && pwd)
repo=/Volumes/SSD990-2/ic-boundary-local-20261115
export PATH=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin:$PATH
export CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=1 OMP_NUM_THREADS=1
export CARGO_TARGET_DIR="$repo/target"
prefix=mac-20261115T2220Z
out="$repo/research/sat_factor_base_review_20260908/autolab/runs/$prefix"
mkdir -p "$out"
cp "$control/local-plan.json" "$out/local-plan.json"
cp "$0" "$out/run-local.sh"
printf '%s\n' "$$" > "$out/pid.txt"
deadline=$(python3 -c 'import json,datetime,sys;print(int(datetime.datetime.fromisoformat(json.load(open(sys.argv[1]))["original_deadline_utc"]).timestamp()))' "$control/local-plan.json")
remaining() { local seconds=$((deadline-$(date +%s))); ((seconds>0)) || return 1; echo "$seconds"; }
# shellcheck disable=SC2329
finish() { local code=$?; trap - EXIT; printf '%s\n' "$code" > "$out/job.exit"; date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/stopped-at.txt"; exit "$code"; }
trap finish EXIT
trap 'exit 143' TERM
cd "$repo"
[[ $(git rev-parse HEAD) == a10cb7b4f8e48f094efe70c31d9a24b6d72aee4a ]]
git rev-parse HEAD > "$out/source_revision.txt"
rustc --version > "$out/rustc.txt"
uname -a > "$out/host.txt"
sysctl hw.memsize hw.logicalcpu hw.physicalcpu >> "$out/host.txt"
ps -axo pid,ppid,pcpu,pmem,etime,command > "$out/processes-before.txt"
python3 research/sat_factor_base_review_20260908/autolab/boundary_autolab.py preflight > "$out/preflight.json"
printf 'BUILDING\n' > "$out/status.txt"
build_seconds=$(remaining)
((build_seconds>3600)) && build_seconds=3600
set +e
gtimeout --signal=TERM --kill-after=45s "${build_seconds}s" cargo build --release --locked \
  --example koblitz_orbit_dlp_fast_online --example koblitz_rho_batch_ks_strong_online \
  --example koblitz_orbit_dlp_fast --example koblitz_rho_batch_ks_strong > "$out/build.log" 2>&1
code=$?
set -e
printf '%s\n' "$code" > "$out/build.exit"
if ((code)); then
  for n in 61 71 73 83; do printf 'BLOCKED_BUILD_FAILURE\n' > "$out/n${n}.status"; done
  printf 'BUILD_FAILED\n' > "$out/status.txt"
  exit "$code"
fi
shasum -a 256 target/release/examples/koblitz_orbit_dlp_fast{,_online} target/release/examples/koblitz_rho_batch_ks_strong{,_online} > "$out/binaries.sha256"
left=4
status=0
for n in 61 71 73 83; do
  if ! seconds=$(remaining); then printf 'NOT_ATTEMPTED_DEADLINE\n' > "$out/n${n}.status"; status=124; left=$((left-1)); continue; fi
  seconds=$(((seconds-120)/left))
  ((seconds>10800)) && seconds=10800
  if ((seconds<=0)); then printf 'NOT_ATTEMPTED_DEADLINE\n' > "$out/n${n}.status"; status=124; left=$((left-1)); continue; fi
  read -r workloads tune < <(python3 -c 'import json,sys;b=next(x for x in json.load(open(sys.argv[1]))["beats"] if x["n"]==int(sys.argv[2]));print(b["workloads"],b["tune_workloads_per_k"])' "$control/local-plan.json" "$n")
  run_id="${prefix}-n${n}"
  printf 'RUNNING_n%s\n' "$n" > "$out/status.txt"
  printf 'RUNNING timeout_seconds=%s\n' "$seconds" > "$out/n${n}.status"
  date -u '+%Y-%m-%dT%H:%M:%SZ' > "$out/n${n}.started-at.txt"
  set +e
  gtimeout --signal=TERM --kill-after=45s "${seconds}s" python3 research/sat_factor_base_review_20260908/autolab/boundary_autolab.py launch-single \
    --beat "koblitz.compact_orbit.n${n}_single_target" --workloads "$workloads" --tune-workloads "$tune" \
    --run-id "$run_id" > "$out/n${n}.log" 2>&1
  code=$?
  set -e
  printf '%s\n' "$code" > "$out/n${n}.exit"
  printf 'EXIT_%s\n' "$code" > "$out/n${n}.status"
  run="research/sat_factor_base_review_20260908/autolab/runs/$run_id"
  if [[ -d "$run" ]]; then
    [[ ! -f "$run/state.json" ]] || cp "$run/state.json" "$out/n${n}.state.json"
    for f in single_target_summary.json claim_check.json claim_draft.json; do
      [[ ! -f "$run/artifacts/$f" ]] || cp "$run/artifacts/$f" "$out/n${n}.$f"
    done
  fi
  ((code==0)) || status=1
  left=$((left-1))
done
printf 'FINISHED_WITH_STATUS_%s\n' "$status" > "$out/status.txt"
exit "$status"
