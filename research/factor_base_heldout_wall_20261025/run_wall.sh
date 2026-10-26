#!/usr/bin/env bash
set -euo pipefail
# Run only from the repository root, after a fixed release build and validation.
study=research/factor_base_heldout_wall_20261025
[ "$PWD" = "$(realpath .)" ]
[ ! -e "$study/results/timed/processes.jsonl" ] || { echo 'timed schedule already exists' >&2; exit 2; }
[ ! -e "$study/results/isolation.jsonl" ] || { echo 'isolation receipt already exists' >&2; exit 2; }
printf '%s\n' \
  '550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339  research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst' \
  '08fbcf90d71fc38bbac30d421fd96b0b06f25033673da3ee10f1c833101729e2  research/factor_base_heldout_20261025/results/selection.json' \
  '55bcb7e54a30e0b73e107b7bc2c5c300a64e4de7078e63493ef1223c72d8960d  research/factor_base_heldout_20261025/results/evaluation-targets.jsonl' \
  '33968a365ca815b25de7d52ddde6749d10d4f2992819a550200ad7a768a42e03  research/factor_base_heldout_20261025/src/bin/heldout.rs' \
  'a283f799f171156d81bc85932ac44a280a71f911219b93eecb6ecfcb6d53585a  research/factor_base_heldout_20261025/src/bin/cost.rs' | sha256sum --check --status
[ -x "$study/target/release/heldout-wall" ]
[ -x "$study/target/release/wall-run" ]
sha256sum "$study/target/release/heldout-wall" "$study/target/release/wall-run" > "$study/results/binaries.sha256"
io_before=$(awk '/^some / {split($2,x,"="); print x[2]}' /proc/pressure/io)
if ! awk -v x="$io_before" 'BEGIN {exit !(x <= 5.0)}'; then
  printf '{"schema":"factor-base-heldout-wall-io/v1","before_some_avg10":%s,"after_some_avg10":null,"status":"preflight_rejected"}\n' "$io_before" > "$study/results/io-pressure.json"
  echo "I/O pressure preflight rejected: some avg10=$io_before > 5.0" >&2
  exit 3
fi
set +e
RAYON_NUM_THREADS=1 python3 tools/isolated_bench.py run --cpus 0,16 \
  --out "$study/results/isolation.jsonl" --label n12-heldout-paired-wall \
  -- prlimit --as=4294967296 -- timeout --signal=TERM --kill-after=10s 3600s \
  "$study/target/release/wall-run" "$(realpath "$study/target/release/heldout-wall")" "$study/results/timed"
status=$?
set -e
io_after=$(awk '/^some / {split($2,x,"="); print x[2]}' /proc/pressure/io)
printf '{"schema":"factor-base-heldout-wall-io/v1","before_some_avg10":%s,"after_some_avg10":%s,"status":"attempted","command_exit_code":%s}\n' "$io_before" "$io_after" "$status" > "$study/results/io-pressure.json"
exit "$status"
