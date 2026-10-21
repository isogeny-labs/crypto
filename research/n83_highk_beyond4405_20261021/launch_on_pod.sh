#!/usr/bin/env bash
# Produce the sealed fixed31 screen after the external watchdog is ready.
set -uo pipefail

fixed31_root=/workspace/n83_fixed31
fixed31_repo=/workspace/crypto
fixed31_allocation=/tmp/n83_fixed31_allocation.json
fixed31_watchdog_ready=/tmp/n83_fixed31_watchdog_ready.json
mkdir -p "$fixed31_root"
cd "$fixed31_repo" || exit 70
export PATH=/root/.cargo/bin:$PATH
export PYTHONDONTWRITEBYTECODE=1
export CARGO_TARGET_DIR=/workspace/n83_fixed31_target

python3 -c 'import subprocess,sys; from pathlib import Path; from research.n83_highk_beyond4405_20261021.runpod_runner import read_allocation; from research.n83_highk_beyond4405_20261021.run import digest; p=read_allocation(Path(sys.argv[1])); assert subprocess.check_output(["git","rev-parse","HEAD"],text=True).strip()==p["source_head_sha"]; assert digest(Path("examples/koblitz_orbit_dlp_fast.rs"))==p["source_sha256"]' "$fixed31_allocation" || exit 77
python3 -c 'import datetime as d,json,sys; from pathlib import Path; from research.n83_highk_beyond4405_20261021.run import digest; from research.n83_highk_beyond4405_20261021.runpod_runner import read_allocation,timestamp; a=Path(sys.argv[1]); p=read_allocation(a); w=json.loads(Path(sys.argv[2]).read_text()); t=d.datetime.now(d.timezone.utc); assert w["schema"]=="n83.fixed31-watchdog-ready/v1" and w["pod_id"]==p["pod_id"] and w["allocation_sha256"]==digest(a) and timestamp(w["recorded_utc"])<=t and (t-timestamp(w["recorded_utc"])).total_seconds()<=1800 and timestamp(w["retire_at_utc"])==timestamp(p["stop_after_utc"])-d.timedelta(minutes=5)' "$fixed31_allocation" "$fixed31_watchdog_ready" || exit 79
cp "$fixed31_watchdog_ready" "$fixed31_root/watchdog_ready.json" || exit 80

python3 -m research.n83_highk_beyond4405_20261021.run --fixture-preflight \
  > "$fixed31_root/fixture_preflight.log" 2>&1 || exit 71
rustc -V > "$fixed31_root/rustc_version.txt"
cargo -V > "$fixed31_root/cargo_version.txt"
git rev-parse HEAD > "$fixed31_root/git_head.txt"
sha256sum examples/koblitz_orbit_dlp_fast.rs Cargo.lock \
  > "$fixed31_root/source_hashes.txt"

fixed31_cpus=$(python3 -c 'import json; p=json.load(open("/tmp/n83_fixed31_allocation.json")); ids=p["cpu_affinity_ids"]; assert len(ids)==4 and ids==sorted(set(ids)) and all(isinstance(i,int) and not isinstance(i,bool) and i>=0 for i in ids); print(",".join(map(str,ids)))') || exit 72
fixed31_remaining=$(python3 -c 'import datetime as d,json; p=json.load(open("/tmp/n83_fixed31_allocation.json")); end=d.datetime.fromisoformat(p["stop_after_utc"].replace("Z","+00:00")); print(max(0,int((end-d.datetime.now(d.timezone.utc)).total_seconds())-1200))') || exit 73
test "$fixed31_remaining" -gt 300 || exit 74

timeout --signal=TERM --kill-after=30s "$fixed31_remaining" \
  cargo build --release --locked --example koblitz_orbit_dlp_fast \
  --example n83_rho_optimization_eval \
  > "$fixed31_root/build.log" 2>&1
fixed31_build_code=$?
if [ "$fixed31_build_code" -ne 0 ]; then
  printf 'DONE build=%s %s\n' "$fixed31_build_code" "$(date -u +%FT%TZ)" \
    > "$fixed31_root/run.status"
  exit "$fixed31_build_code"
fi

fixed31_remaining=$(python3 -c 'import datetime as d,json; p=json.load(open("/tmp/n83_fixed31_allocation.json")); end=d.datetime.fromisoformat(p["stop_after_utc"].replace("Z","+00:00")); print(max(0,int((end-d.datetime.now(d.timezone.utc)).total_seconds())-1200))') || exit 75
test "$fixed31_remaining" -gt 300 || exit 76
printf 'START %s\n' "$(date -u +%FT%TZ)" > "$fixed31_root/run.status"
timeout --signal=TERM --kill-after=30s "$fixed31_remaining" \
  taskset -c "$fixed31_cpus" \
  python3 -m research.n83_highk_beyond4405_20261021.runpod_runner \
  --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
  --rho-binary "$CARGO_TARGET_DIR/release/examples/n83_rho_optimization_eval" \
  --provider-receipt "$fixed31_allocation" \
  --output "$fixed31_root/results" > "$fixed31_root/screen_run.log" 2>&1
fixed31_screen_code=$?
printf 'SCREEN_PRODUCER_EXIT %s %s\n' "$fixed31_screen_code" "$(date -u +%FT%TZ)" \
  > "$fixed31_root/run.status"
fixed31_verify_code=99
fixed31_holdout_code=98
fixed31_holdout_verify_code=99
if [ -d "$fixed31_root/results" ]; then
  timeout --signal=TERM --kill-after=30s 900 \
    python3 -m research.n83_highk_beyond4405_20261021.verify_screen \
    "$fixed31_root/results" > "$fixed31_root/screen_verify.log" 2>&1
  fixed31_verify_code=$?
  if [ "$fixed31_verify_code" -eq 0 ] && python3 -c 'import json,sys; p=json.load(open(sys.argv[1])); sys.exit(0 if p.get("status")=="SELECTED" else 1)' "$fixed31_root/results/screen_selection.json"; then
    fixed31_remaining=$(python3 -c 'import datetime as d,json; p=json.load(open("/tmp/n83_fixed31_allocation.json")); end=d.datetime.fromisoformat(p["stop_after_utc"].replace("Z","+00:00")); print(max(0,int((end-d.datetime.now(d.timezone.utc)).total_seconds())-1200))') || exit 78
    if [ "$fixed31_remaining" -gt 300 ]; then
      timeout --signal=TERM --kill-after=30s "$fixed31_remaining" \
        taskset -c "$fixed31_cpus" \
        python3 -m research.n83_highk_beyond4405_20261021.holdout \
        --provider-receipt "$fixed31_allocation" \
        --output "$fixed31_root/results" \
        > "$fixed31_root/holdout_run.log" 2>&1
      fixed31_holdout_code=$?
      if [ -d "$fixed31_root/results/holdout" ]; then
        timeout --signal=TERM --kill-after=30s 900 \
          python3 -m research.n83_highk_beyond4405_20261021.verify_holdout \
          "$fixed31_root/results" \
          > "$fixed31_root/holdout_verify.log" 2>&1
        fixed31_holdout_verify_code=$?
      fi
    fi
  fi
  fixed31_archive_items=(results fixture_preflight.log rustc_version.txt
    cargo_version.txt git_head.txt source_hashes.txt build.log screen_run.log
    screen_verify.log watchdog_ready.json)
  if [ -f "$fixed31_root/holdout_run.log" ]; then
    fixed31_archive_items+=(holdout_run.log)
  fi
  if [ -f "$fixed31_root/holdout_verify.log" ]; then
    fixed31_archive_items+=(holdout_verify.log)
  fi
  tar -C "$fixed31_root" -czf "$fixed31_root/raw_results.tar.gz" \
    "${fixed31_archive_items[@]}"
  sha256sum "$fixed31_root/raw_results.tar.gz" \
    > "$fixed31_root/raw_results.sha256"
fi
printf 'DONE screen=%s verify=%s holdout=%s holdout_verify=%s %s\n' "$fixed31_screen_code" \
  "$fixed31_verify_code" "$fixed31_holdout_code" \
  "$fixed31_holdout_verify_code" "$(date -u +%FT%TZ)" \
  > "$fixed31_root/run.status"
if [ "$fixed31_screen_code" -ne 0 ] || [ "$fixed31_verify_code" -ne 0 ] || [ "$fixed31_holdout_code" -ne 0 ] || [ "$fixed31_holdout_verify_code" -ne 0 ]; then
  exit 2
fi
exit 0
