#!/usr/bin/env bash
# T10 (wave 3): joint (t, G, seed) sweep of the isogeny factor base, k = 20.
# Frozen with PROTOCOL.md. Executes exactly the 36 planned f4cost cells,
# strictly sequentially (one heavy process at a time), into ../data/.
# No git, no rebuilds, writes only inside this lead directory.
set -euo pipefail

LEAD="/home/user/crypto/research/curve25519_leads_wave3_20261103/t10_seed_and_guess"
BIN="/home/user/ctarget/release/curve25519_family"
FROZEN_BIN_SHA="ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef"
CENSUS="/home/user/crypto/research/curve25519_isogeny_seed_20261103/data/k20.jsonl"
FROZEN_CENSUS_SHA="bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc"
DATA="$LEAD/data"
CELLS="$DATA/cells"

K=20
TARGETS=100          # frozen for every cell (PROTOCOL.md, "Sizing")
CAP_S=3600
M=2
MODE=full
LIFTS=0
MAX_NNZ=30000000
# J/columns-selected seeds (selection/seeds.json, computed before any data).
declare -A SEL=([5]=1307 [6]=1308 [7]=1594)

# F1: instrument identity. Refuse to run on a different binary or census.
got=$(sha256sum "$BIN" | cut -d' ' -f1)
if [ "$got" != "$FROZEN_BIN_SHA" ]; then
  echo "F1: binary sha256 $got != frozen $FROZEN_BIN_SHA; not running (protocol addendum required)" >&2
  exit 3
fi
gotc=$(sha256sum "$CENSUS" | cut -d' ' -f1)
if [ "$gotc" != "$FROZEN_CENSUS_SHA" ]; then
  echo "F1: census sha256 $gotc != frozen $FROZEN_CENSUS_SHA; not running" >&2
  exit 3
fi

# Never overwrite measurements.
if [ -e "$DATA/ALL_DONE" ] || { [ -d "$CELLS" ] && [ -n "$(ls -A "$CELLS" 2>/dev/null)" ]; }; then
  echo "data/ already holds cells; refusing to overwrite" >&2
  exit 4
fi
mkdir -p "$CELLS"

{
  echo "start_utc $(date -u +%FT%TZ)"
  echo "host $(hostname)"
  echo "nproc $(nproc)"
  echo "uname $(uname -srm)"
  echo "binary $BIN"
  echo "binary_sha256 $got"
  echo "census_sha256 $gotc"
} > "$DATA/env.txt"
echo "$got  $BIN" > "$DATA/binary.sha256"
cp "$LEAD/selection/seeds.json" "$DATA/seeds.used.json"

run_cell() {  # t G seed
  local t=$1 g=$2 s=$3 form tseed name rc try
  if [ "$s" = 0 ]; then form="isogeny"; else form="isogeny@$s"; fi
  tseed=$((2026111500 + t))
  name="t${t}_g${g}_s${s}"
  for try in 1 2; do
    # Instrument must not change mid-run (F1).
    [ "$(sha256sum "$BIN" | cut -d' ' -f1)" = "$FROZEN_BIN_SHA" ] || { echo "F1: binary changed mid-run before $name" >&2; exit 3; }
    local start load
    start=$(date -u +%FT%TZ); load=$(cut -d' ' -f1 /proc/loadavg)
    set +e
    "$BIN" f4cost "$K" "$form" "$t" "$g" "$TARGETS" "$tseed" "$CAP_S" "$M" "$MODE" "$LIFTS" "$MAX_NNZ" \
      > "$CELLS/$name.json" 2> "$CELLS/$name.stderr"
    rc=$?
    set -e
    echo "cell=$name form=$form t=$t G=$g targets=$TARGETS target_seed=$tseed try=$try rc=$rc start=$start end=$(date -u +%FT%TZ) load1=$load" \
      | tee -a "$DATA/run.log" > "$CELLS/$name.status"
    if [ "$rc" = 0 ]; then return 0; fi
    # F2: keep the failed attempt, then one identical rerun.
    mv "$CELLS/$name.json" "$CELLS/$name.try$try.json"
    mv "$CELLS/$name.stderr" "$CELLS/$name.try$try.stderr"
  done
  echo "cell=$name FAILED twice (F2)" | tee -a "$DATA/run.log" >> "$CELLS/$name.status"
}

# Order: per t, from G = t-1 (cheapest) down to G = 0; seed 0 then the
# selected seed within each (t, G). Counts are deterministic, so order does
# not change them.
for t in 5 6 7; do
  for ((g = t - 1; g >= 0; g--)); do
    run_cell "$t" "$g" 0
    run_cell "$t" "$g" "${SEL[$t]}"
  done
done

echo "end_utc $(date -u +%FT%TZ)" >> "$DATA/env.txt"
( cd "$DATA" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS )
touch "$DATA/ALL_DONE"
echo "T10: all 36 cells attempted; see $DATA/run.log"
