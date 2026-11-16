#!/usr/bin/env bash
# T6: Edwards t=5 G=0 per-target outlier -- replication runs (PROTOCOL.md, frozen).
# Executes exactly the planned cells, strictly sequentially (one f4cost process at a time).
# Resumable: a cell whose .json already has targets_done == requested and capped == false is skipped.
# No git, no rebuild. Writes only under ../data/.
set -u
D=$(cd "$(dirname "$0")/.." && pwd)
OUT="$D/data"
BIN=/home/user/ctarget/release/curve25519_family
EXPECT_SHA=7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be
mkdir -p "$OUT"
ulimit -v 3500000

got=$(sha256sum "$BIN" | cut -d' ' -f1)
echo "$got  $BIN" > "$OUT/binary.sha256"
if [ "$got" != "$EXPECT_SHA" ]; then
  echo "binary sha256 $got != frozen $EXPECT_SHA; refusing to run (PROTOCOL.md, failure handling F1)" >&2
  exit 3
fi

ORIG=2026111505
FRESH="3110300001 3110300002 3110300003 3110300004 3110300005 3110300006"
FRESH3="3110300001 3110300002 3110300003"

# cell <name> <k> <form> <t> <G> <targets> <seed>
cell() {
  local name=$1 k=$2 form=$3 t=$4 g=$5 n=$6 seed=$7
  local c="$OUT/$name"
  if [ -s "$c.json" ] && python3 -I -c "
import json,sys
d=json.loads(open(sys.argv[1]).read().strip().splitlines()[-1])
sys.exit(0 if d.get('targets_done')==d.get('targets_requested') and not d.get('capped') else 1)" "$c.json" 2>/dev/null; then
    return 0
  fi
  { echo "cmd $BIN f4cost $k $form $t $g $n $seed 3600 2 full 0 30000000"; echo "start $(date -u +%FT%TZ)"; uptime; } > "$c.status"
  timeout 4000 "$BIN" f4cost "$k" "$form" "$t" "$g" "$n" "$seed" 3600 2 full 0 30000000 > "$c.json" 2> "$c.stderr"
  local rc=$?
  { echo "end $(date -u +%FT%TZ) rc=$rc"; uptime; } >> "$c.status"
  if [ $rc -ne 0 ]; then   # F2: one identical rerun, then record and move on
    mv "$c.json" "$c.try1.json"; mv "$c.stderr" "$c.try1.stderr"
    echo "rerun $(date -u +%FT%TZ)" >> "$c.status"
    timeout 4000 "$BIN" f4cost "$k" "$form" "$t" "$g" "$n" "$seed" 3600 2 full 0 30000000 > "$c.json" 2> "$c.stderr"
    rc=$?
    { echo "end2 $(date -u +%FT%TZ) rc=$rc"; uptime; } >> "$c.status"
  fi
}

# Block A: k=20 t=5 G=0, 100 targets; reproduction seed + 6 fresh seeds;
# forms interleaved bits / edwards-bits / bits@976503 (translation control).
for s in $ORIG $FRESH; do
  cell "A_k20_bits_t5_g0_s$s"        20 bits         5 0 100 "$s"
  cell "A_k20_edwards_t5_g0_s$s"     20 edwards-bits 5 0 100 "$s"
  cell "A_k20_bits976503_t5_g0_s$s"  20 bits@976503  5 0 100 "$s"
done

# Block B: other family curves, t=5 G=0, 100 targets, 3 fresh seeds.
for k in 16 18 22 24; do
  for s in $FRESH3; do
    cell "B_k${k}_bits_t5_g0_s$s"    "$k" bits         5 0 100 "$s"
    cell "B_k${k}_edwards_t5_g0_s$s" "$k" edwards-bits 5 0 100 "$s"
  done
done

# Block C: guess sweep at k=20 t=5, G in {1,2,3}, 100 targets, 3 fresh seeds.
for g in 1 2 3; do
  for s in $FRESH3; do
    cell "C_k20_bits_t5_g${g}_s$s"    20 bits         5 "$g" 100 "$s"
    cell "C_k20_edwards_t5_g${g}_s$s" 20 edwards-bits 5 "$g" 100 "$s"
  done
done

# Block D: neighbouring t at k=20, G=0, 100 targets: t=4 on 3 fresh seeds, t=6 on 1 fresh seed.
for s in $FRESH3; do
  cell "D_k20_bits_t4_g0_s$s"    20 bits         4 0 100 "$s"
  cell "D_k20_edwards_t4_g0_s$s" 20 edwards-bits 4 0 100 "$s"
done
cell "D_k20_bits_t6_g0_s3110300001"    20 bits         6 0 100 3110300001
cell "D_k20_edwards_t6_g0_s3110300001" 20 edwards-bits 6 0 100 3110300001

# Block E: per-target distribution at k=20 t=5 G=0: 40 single-target runs per form.
for i in $(seq 1 40); do
  s=$((3110301000 + i))
  cell "E_k20_bits_t5_g0_n1_s$s"    20 bits         5 0 1 "$s"
  cell "E_k20_edwards_t5_g0_n1_s$s" 20 edwards-bits 5 0 1 "$s"
done

echo "done $(date -u +%FT%TZ)" > "$OUT/ALL_DONE"
