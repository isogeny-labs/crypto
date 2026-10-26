#!/bin/sh
set -eu

binary=${1:?pass the release ic binary}
out_dir=${2:?pass a new output directory}
if [ -e "$out_dir" ]; then
  echo "output directory already exists: $out_dir" >&2
  exit 2
fi
mkdir -p "$out_dir"

run_one() {
  sequence=$1
  arm=$2
  size=$3
  prefix="$out_dir/$sequence-$arm"
  if [ -e "$prefix.json" ] || [ -e "$prefix.stdout" ] || [ -e "$prefix.stderr" ]; then
    echo "raw result already exists: $prefix" >&2
    exit 2
  fi
  set +e
  RAYON_NUM_THREADS=1 gtimeout 600 /usr/bin/time -l "$binary" prime \
    --curve eccp-131 --size "$size" --trials 0 --controls 8 \
    --setup-candidates 128 --target-candidates 128 --seed 131 \
    --json --out "$prefix.json" > "$prefix.stdout" 2> "$prefix.stderr"
  status=$?
  set -e
  printf '%s\n' "$status" > "$prefix.exit"
  if [ "$status" -ne 0 ]; then
    echo "exact probe failed: $prefix (exit $status)" >&2
    exit "$status"
  fi
  if ! jq -e --argjson size "$size" '
      .status == "complete" and
      .report.controls == 8 and
      .report.controls_detected == 8 and
      .report.controls_recovered == 8 and
      .ic_probe.factor_base_abscissae == $size and
      .ic_probe.setup_candidates == 128 and
      .ic_probe.target_candidates == 128 and
      ((.ic_probe.target_log_determined | not) or .ic_probe.verified)
    ' "$prefix.json" > /dev/null; then
    echo "invalid exact-probe output: $prefix" >&2
    exit 3
  fi
}

run_one 01 B256 256
run_one 02 B512a 512
run_one 03 B1024 1024
run_one 04 B512b 512
run_one 05 B512b 512
run_one 06 B1024 1024
run_one 07 B512a 512
run_one 08 B256 256
run_one 09 B256 256
run_one 10 B512b 512
run_one 11 B1024 1024
run_one 12 B512a 512
run_one 13 B512a 512
run_one 14 B1024 1024
run_one 15 B512b 512
run_one 16 B256 256
run_one 17 B256 256
run_one 18 B512a 512
run_one 19 B1024 1024
run_one 20 B512b 512
