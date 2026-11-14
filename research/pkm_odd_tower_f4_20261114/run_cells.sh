#!/usr/bin/env bash
# EXP-PKM-ODD-001 runner (PROTOCOL.md §3). Runs every cell in the frozen order,
# one JSON row per system into runs/<cell>.jsonl. Writes binary.sha256,
# commit.txt and rustc.txt first. Cells already complete (their row count
# reached) are skipped, so an interrupted run resumes from the next cell; a
# partially written cell is rerun from the start under the same name
# (PROTOCOL.md §7) after moving the partial file to runs/partial/.
set -euo pipefail
cd "$(dirname "$0")"
ROOT="$(git rev-parse --show-toplevel)"
BIN="$ROOT/target/release/examples/pkm_odd_tower_f4"
ORACLE="$ROOT/target/release/examples/pkm_algebraic_oracle"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-4}"
P1=1399680001
P2=3265920001
SEED=20261114
BUDGET="${BUDGET:-1800}"
DMAX=64

sha256sum "$BIN" "$ORACLE" > binary.sha256
git -C "$ROOT" rev-parse HEAD > commit.txt
rustc -Vv > rustc.txt
mkdir -p runs runs/partial
echo "start $(date -u +%FT%TZ) threads=$RAYON_NUM_THREADS budget=$BUDGET" >> progress.txt

# cell name, expected rows, args
run() {
  local name="$1" rows="$2"; shift 2
  local f="runs/$name.jsonl"
  if [ -f "$f" ] && [ "$(wc -l < "$f")" -ge "$rows" ]; then return 0; fi
  if [ -f "$f" ]; then mv "$f" "runs/partial/$name.$(date -u +%s).jsonl"; echo "rerun $name (partial)" >> progress.txt; fi
  local t0=$(date +%s)
  "$BIN" --seed "$SEED" --budget "$BUDGET" --dmax "$DMAX" --cell "$name" --out "$f" "$@" > /dev/null
  echo "done $name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> progress.txt
}

deg() { local q="$1" n="$2" s=""; for _ in $(seq 1 "$n"); do s="$s,$q"; done; echo "${s#,}"; }
alt() { local a="$1" b="$2" n="$3" s="" i; for i in $(seq 1 "$n"); do if [ $((i % 2)) -eq 1 ]; then s="$s,$a"; else s="$s,$b"; fi; done; echo "${s#,}"; }

# Positive-control-of-correctness first: planted systems (tower), m = 2, small end.
for t in 4 5 6 7 8; do run "PL-A2-t$t" 1 --p $P1 --m 2 --degrees "$(deg 2 $t)" --kind planted --draws 1; done
for k in 3 4 5; do run "PL-A3-k$k" 1 --p $P1 --m 2 --degrees "$(deg 3 $k)" --kind planted --draws 1; done
for k in 2 3; do run "PL-A5-k$k" 1 --p $P1 --m 2 --degrees "$(deg 5 $k)" --kind planted --draws 1; done
for n in 4 5 6; do run "PL-A23-n$n" 1 --p $P1 --m 2 --degrees "$(alt 2 3 $n)" --kind planted --draws 1; done

# Main ladder, m = 2, interleaved by information size (log2 S ascending), tower then null.
# Order fixed now: for each size step, arms in the rotation A2, A3, A5, A23, A32.
L2_CELLS=(
  "A2-t4:2:4" "A23-n4:23:4" "A32-n4:32:4" "A5-k2:5:2" "A3-k3:3:3"
  "A2-t5:2:5" "A2-t6:2:6" "A23-n5:23:5" "A3-k4:3:4" "A32-n5:32:5"
  "A2-t7:2:7" "A5-k3:5:3" "A23-n6:23:6" "A32-n6:32:6" "A2-t8:2:8"
  "A3-k5:3:5" "A23-n7:23:7" "A2-t9:2:9"
)
degs_of() {
  case "$1" in
    2) deg 2 "$2" ;; 3) deg 3 "$2" ;; 5) deg 5 "$2" ;;
    23) alt 2 3 "$2" ;; 32) alt 3 2 "$2" ;;
  esac
}
for c in "${L2_CELLS[@]}"; do
  IFS=: read -r name q n <<< "$c"
  d="$(degs_of "$q" "$n")"
  run "M2-$name-tower" 3 --p $P1 --m 2 --degrees "$d" --control tower --kind refute --draws 3
  run "M2-$name-null" 3 --p $P1 --m 2 --degrees "$d" --control null --kind refute --draws 3
done

# Isogeny form (auxiliary curve, Vélu steps), m = 2: same degree sequences as A2/A3/A23 (lower end).
# The auxiliary-curve search is not charged (owner, 2026-11-14); rows record its count only.
for k in 3 4; do run "PL-I3-k$k" 1 --p $P1 --m 2 --form isogeny --degrees "$(deg 3 $k)" --kind planted --draws 1; done
run "PL-I23-n4" 1 --p $P1 --m 2 --form isogeny --degrees "$(alt 2 3 4)" --kind planted --draws 1
I2_CELLS=( "I2-t4:2:4" "I3-k3:3:3" "I23-n4:23:4" "I2-t5:2:5" "I2-t6:2:6" "I23-n5:23:5" "I3-k4:3:4" "I2-t7:2:7" "I23-n6:23:6" "I2-t8:2:8" "I3-k5:3:5" )
for c in "${I2_CELLS[@]}"; do
  IFS=: read -r name q n <<< "$c"
  d="$(degs_of "$q" "$n")"
  run "M2-$name-tower" 3 --p $P1 --m 2 --form isogeny --degrees "$d" --control tower --kind refute --draws 3
  run "M2-$name-null" 3 --p $P1 --m 2 --form isogeny --degrees "$d" --control null --kind refute --draws 3
done

# Secondary ladder, m = 3 (small S).
L3_CELLS=( "A2-t2:2:2" "A23-n2:23:2" "A2-t3:2:3" "A3-k2:3:2" "A23-n3:23:3" "A2-t4:2:4" "A5-k2:5:2" "A3-k3:3:3" "A2-t5:2:5" "A23-n4:23:4" )
for c in "${L3_CELLS[@]}"; do
  IFS=: read -r name q n <<< "$c"
  d="$(degs_of "$q" "$n")"
  run "M3-$name-tower" 3 --p $P1 --m 3 --degrees "$d" --control tower --kind refute --draws 3
  run "M3-$name-null" 3 --p $P1 --m 3 --degrees "$d" --control null --kind refute --draws 3
done

# Replication at P2 (tower, m = 2).
for c in "A2-t4:2:4" "A3-k3:3:3" "A2-t6:2:6" "A3-k4:3:4" "A5-k3:5:3" "A2-t8:2:8" "A3-k5:3:5"; do
  IFS=: read -r name q n <<< "$c"
  run "X2-$name-tower" 3 --p $P2 --m 2 --degrees "$(degs_of "$q" "$n")" --control tower --kind refute --draws 3
done

# A/A determinism: rerun one main cell under a new name; counts must be bit-identical.
run "AA-M2-A3-k5-tower" 3 --p $P1 --m 2 --degrees "$(deg 3 5)" --control tower --kind refute --draws 3

# Part B: algebraic oracle vs enumeration (2 planted + 2 random targets per cell).
runb() {
  local name="$1"; shift
  local f="runs/B-$name.jsonl"
  if [ -f "$f" ] && [ "$(wc -l < "$f")" -ge 4 ]; then return 0; fi
  if [ -f "$f" ]; then mv "$f" "runs/partial/B-$name.$(date -u +%s).jsonl"; echo "rerun B-$name (partial)" >> progress.txt; fi
  local t0=$(date +%s)
  "$ORACLE" --seed "$SEED" --budget "$BUDGET" --dmax "$DMAX" --cell "B-$name" --out "$f" --planted 2 --random 2 "$@" > /dev/null
  echo "done B-$name rows=$(wc -l < "$f") secs=$(( $(date +%s) - t0 ))" >> progress.txt
}
for c in "A2-t4:2:4" "A3-k3:3:3" "A23-n4:23:4" "A2-t6:2:6" "A23-n6:23:6" "A3-k5:3:5" "A2-t8:2:8"; do
  IFS=: read -r name q n <<< "$c"; runb "M2-$name" --p $P1 --m 2 --form kummer --degrees "$(degs_of "$q" "$n")"
done
for c in "I3-k3:3:3" "I23-n4:23:4" "I23-n5:23:5" "I3-k4:3:4"; do
  IFS=: read -r name q n <<< "$c"; runb "M2-$name" --p $P1 --m 2 --form isogeny --degrees "$(degs_of "$q" "$n")"
done
for c in "A2-t2:2:2" "A23-n2:23:2" "A2-t3:2:3" "A3-k2:3:2" "A23-n3:23:3" "A2-t4:2:4" "A3-k3:3:3" "A23-n4:23:4" "A2-t5:2:5"; do
  IFS=: read -r name q n <<< "$c"; runb "M3-$name" --p $P1 --m 3 --form kummer --degrees "$(degs_of "$q" "$n")"
done
for c in "I23-n2:23:2" "I23-n3:23:3"; do
  IFS=: read -r name q n <<< "$c"; runb "M3-$name" --p $P1 --m 3 --form isogeny --degrees "$(degs_of "$q" "$n")"
done

echo "end $(date -u +%FT%TZ)" >> progress.txt
