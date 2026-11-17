#!/usr/bin/env bash
# L10 (wave 4): scaling of the joint-optimum F4 cost with r, isogeny factor base.
# Frozen with PROTOCOL.md. Executes exactly the 40 planned f4cost cells, strictly
# sequentially (one heavy process at a time), into ../data/. Engine runs need no
# /tmp/crypto-bench.lock. No git, no rebuilds, writes only inside this lead directory.
set -euo pipefail

LEAD="/home/user/crypto/research/curve25519_leads_wave4_20261104/l10_scaling_exponent"
BIN="/home/user/ctarget/release/curve25519_family"
FROZEN_BIN_SHA="ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef"
CENSUS_DIR="/home/user/crypto/research/curve25519_isogeny_seed_20261103/data"
declare -A FROZEN_CENSUS_SHA=(
  [16]=3935c968587dd631c7cebfaaa4f530b03a5ed6da4045ab9cf9ba14504699d498
  [18]=4e670176d073d90c7f7dd7a6cd951f25072330128794e981eeb48181696d84cf
  [20]=bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc
  [22]=11903b11c6d1f2eccfadd27b058e95fb2410999b21e22afc3a9854220c770e9e
  [24]=39c7543607717def3ff68ef852151f482ea6ff0a13b33a9bd00bc34a2329eba0
)
SEEDS_JSON="$LEAD/selection/seeds.json"
FROZEN_SEEDS_SHA="1e6195046fbd445a3551a6726ac4df7e3ee08976f3814590ac904909e3338f7d"
DATA="$LEAD/data"
CELLS="$DATA/cells"

TARGETS=100          # frozen for every cell (PROTOCOL.md §2.4: no cell sized as too slow)
CAP_S=3600
M=2
MODE=full
LIFTS=0
MAX_NNZ=30000000
# J/columns-selected seeds S*(k,t) from selection/seeds.json (computed before any data).
declare -A SEL=(
  [16,4]=1786 [16,5]=1597 [16,6]=1381 [16,7]=278
  [18,4]=1074 [18,5]=997  [18,6]=1236 [18,7]=1955
  [20,4]=1193 [20,5]=1307 [20,6]=1308 [20,7]=1594
  [22,4]=1002 [22,5]=1034 [22,6]=284  [22,7]=454
  [24,4]=1717 [24,5]=978  [24,6]=1340 [24,7]=702
)

# F1: instrument identity. Refuse to run on a different binary, census or seed file.
got=$(sha256sum "$BIN" | cut -d' ' -f1)
[ "$got" = "$FROZEN_BIN_SHA" ] || { echo "F1: binary sha256 $got != frozen; not running (addendum required)" >&2; exit 3; }
for k in 16 18 20 22 24; do
  gotc=$(sha256sum "$CENSUS_DIR/k$k.jsonl" | cut -d' ' -f1)
  [ "$gotc" = "${FROZEN_CENSUS_SHA[$k]}" ] || { echo "F1: census k$k sha256 $gotc != frozen; not running" >&2; exit 3; }
done
gots=$(sha256sum "$SEEDS_JSON" | cut -d' ' -f1)
[ "$gots" = "$FROZEN_SEEDS_SHA" ] || { echo "F1: seeds.json sha256 $gots != frozen; not running" >&2; exit 3; }

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
  for k in 16 18 20 22 24; do echo "census_k${k}_sha256 ${FROZEN_CENSUS_SHA[$k]}"; done
  echo "seeds_sha256 $gots"
} > "$DATA/env.txt"
echo "$got  $BIN" > "$DATA/binary.sha256"
cp "$SEEDS_JSON" "$DATA/seeds.used.json"

run_cell() {  # k t G seed
  local k=$1 t=$2 g=$3 s=$4 form tseed name rc try
  if [ "$s" = 0 ]; then form="isogeny"; else form="isogeny@$s"; fi
  tseed=$((2026111500 + t))
  name="k${k}_t${t}_g${g}_s${s}"
  for try in 1 2; do
    [ "$(sha256sum "$BIN" | cut -d' ' -f1)" = "$FROZEN_BIN_SHA" ] || { echo "F1: binary changed mid-run before $name" >&2; exit 3; }
    local start load
    start=$(date -u +%FT%TZ); load=$(cut -d' ' -f1 /proc/loadavg)
    set +e
    "$BIN" f4cost "$k" "$form" "$t" "$g" "$TARGETS" "$tseed" "$CAP_S" "$M" "$MODE" "$LIFTS" "$MAX_NNZ" \
      > "$CELLS/$name.json" 2> "$CELLS/$name.stderr"
    rc=$?
    set -e
    echo "cell=$name k=$k form=$form t=$t G=$g targets=$TARGETS target_seed=$tseed try=$try rc=$rc start=$start end=$(date -u +%FT%TZ) load1=$load" \
      | tee -a "$DATA/run.log" > "$CELLS/$name.status"
    if [ "$rc" = 0 ]; then return 0; fi
    # F2: keep the failed attempt, then one identical rerun.
    mv "$CELLS/$name.json" "$CELLS/$name.try$try.json"
    mv "$CELLS/$name.stderr" "$CELLS/$name.try$try.stderr"
  done
  echo "cell=$name FAILED twice (F2)" | tee -a "$DATA/run.log" >> "$CELLS/$name.status"
}

# Order: k ascending (cheapest curves first), then t = 4..7. Per (k, t):
#   G = t-1 at S*(k,t)            (all t)            -> 20 cells
#   G = t-1 at seed 0 (control)   (t = 5, 6)         -> 10 cells
#   G = t-2 at S*(k,t)            (t = 5, 6)         -> 10 cells
# 40 cells. Counts are deterministic, so order and host load do not change them.
for k in 16 18 20 22 24; do
  for t in 4 5 6 7; do
    s="${SEL[$k,$t]}"
    run_cell "$k" "$t" $((t - 1)) "$s"
    if [ "$t" = 5 ] || [ "$t" = 6 ]; then
      run_cell "$k" "$t" $((t - 1)) 0
      run_cell "$k" "$t" $((t - 2)) "$s"
    fi
  done
done

echo "end_utc $(date -u +%FT%TZ)" >> "$DATA/env.txt"
( cd "$DATA" && find . -type f ! -name SHA256SUMS -print0 | sort -z | xargs -0 sha256sum > SHA256SUMS )
touch "$DATA/ALL_DONE"
echo "L10: all 40 cells attempted; see $DATA/run.log"
