#!/usr/bin/env bash
# Lead T3, m = 3 frame selection in the F4 engine. Executes exactly the runs frozen in
# ../PROTOCOL.md, sequentially (one process at a time), into ../data/.
# No rebuild, no git, no writes outside this lead directory.
#   Step S  : regenerate the torsion census at k = 20 with its frozen script, check its
#             sha256 against the published manifest, select L3*(t), L2*(t) by the frozen rules.
#   Step E2 : m = 2 controls and comparators (seed 2026111500 + t, 100 targets).
#   Step E3 : m = 3 pairs bits vs bits@L3* (seed 2026111300 + t; 100 targets t = 4, 30 at t = 5).
#   Step V  : m = 3 coverage-only rows (targets = 0).
set -euo pipefail
LEAD=/home/user/crypto/research/curve25519_leads_wave2_20261103/t3_m3_engine
TORSION=/home/user/crypto/research/curve25519_torsion_leads_20261103
BIN=${BIN:-/home/user/ctarget/release/curve25519_family}
D=$LEAD/data
mkdir -p "$D/census" "$D/cells"
export PYTHONDONTWRITEBYTECODE=1        # never write __pycache__ into the torsion study

# ---- provenance -------------------------------------------------------------
sha256sum "$BIN" > "$D/binary.sha256"
{ python3 --version; python3 -c 'import numpy; print("numpy", numpy.__version__)'; uname -a; nproc; date -u +%FT%TZ; } > "$D/env.txt" 2>&1

# ---- Step S: census regeneration + selection -------------------------------
CENSUS_SHA=dc08cc8ea25f6ff8598603e5751394d7a4448c4353819cc49952937d71d1099f   # torsion data/.artifacts.json, k20.json
python3 "$TORSION/scripts/torsion_census.py" --k 20 --p 1048573 --A 926 --n 1049624 \
  --out "$D/census/k20.json" 2> "$D/census/k20.log"
got=$(sha256sum "$D/census/k20.json" | cut -d' ' -f1)
echo "$got  census/k20.json" > "$D/census/SHA256SUMS"
if [ "$got" != "$CENSUS_SHA" ]; then
  echo "ABORT: regenerated census sha256 $got != published $CENSUS_SHA (PROTOCOL.md F1)" | tee "$D/ABORTED"
  exit 3
fi
python3 "$LEAD/scripts/select_frames.py" "$D/census/k20.json" "$TORSION/analysis/summary.json" \
  "$D/selection.json" > "$D/selection.log" 2>&1 || { echo "ABORT: selection check failed (F1)" | tee "$D/ABORTED"; exit 4; }
L3_4=$(python3 -c "import json;print(json.load(open('$D/selection.json'))['cells']['4']['L3'])")
L3_5=$(python3 -c "import json;print(json.load(open('$D/selection.json'))['cells']['5']['L3'])")
L2_4=797820; L2_5=233902    # checked equal to the frozen rule's output by select_frames.py
echo "L3*(4)=$L3_4 L3*(5)=$L3_5 L2*(4)=$L2_4 L2*(5)=$L2_5" | tee "$D/frames.txt"

# ---- engine cells ------------------------------------------------------------
ulimit -v 6000000     # 6 GB address space per process (shared 15 GB machine)
cell() { # tag form t G targets seed m
  local tag=$1 form=$2 t=$3 g=$4 n=$5 seed=$6 m=$7
  local out="$D/cells/$tag.json" err="$D/cells/$tag.stderr"
  echo "# $(date -u +%FT%TZ) $tag: f4cost 20 $form $t $g $n $seed 3600 $m full 0 30000000" | tee -a "$D/run.log"
  if timeout 4200 "$BIN" f4cost 20 "$form" "$t" "$g" "$n" "$seed" 3600 "$m" full 0 30000000 > "$out" 2> "$err" \
     && [ -s "$out" ]; then :; else
    echo "{\"tag\":\"$tag\",\"form\":\"$form\",\"t\":$t,\"guess\":$g,\"m\":$m,\"failed\":true,\"exit\":$?}" > "$out"
  fi
}
# Step E2: m = 2 (seed 2026111500 + t, the frame/torsion studies' seeds)
cell m2_t4_lo0      bits          4 3 100 2026111504 2   # control: Edwards study bits_t4_g3 (8884000)
cell m2_t4_L2       bits@$L2_4    4 3 100 2026111504 2   # control: torsion bits_797820_t4 (8884000)
cell m2_t4_L3       bits@$L3_4    4 3 100 2026111504 2
cell m2_t5_lo0      bits          5 3 100 2026111505 2   # control: frame study bits_t5_g3 (303691176)
cell m2_t5_L2       bits@$L2_5    5 3 100 2026111505 2   # control: torsion bits_233902_t5 (303690989)
cell m2_t5_L3       bits@$L3_5    5 3 100 2026111505 2
# Step E3: m = 3 pairs (seed 2026111300 + t, the m = 3 study's seeds); arm order alternates
cell m3_t4_lo0      bits          4 3 100 2026111304 3   # control: m3 study bits t4 G3 full (119824176893)
cell m3_t4_L3       bits@$L3_4    4 3 100 2026111304 3
cell m3_t5_L3       bits@$L3_5    5 3  30 2026111305 3
cell m3_t5_lo0      bits          5 3  30 2026111305 3
# Step V: m = 3 coverage only (targets = 0)
cell m3cov_t4_L2    bits@$L2_4    4 3   0 2026111304 3
cell m3cov_t5_L2    bits@$L2_5    5 3   0 2026111305 3

(cd "$D" && sha256sum binary.sha256 env.txt selection.json frames.txt census/k20.json cells/*.json > SHA256SUMS)
echo "done $(date -u +%FT%TZ)" | tee -a "$D/run.log"
