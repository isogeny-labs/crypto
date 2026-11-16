#!/usr/bin/env bash
# Replay the p <= 2053 cells of each study with the binary rebuilt from its
# frozen commit, then build val_dump (the same bin source + val_* fields) in the
# same tree (scripts/build_pair.sh) and rerun the same cells, then verify
# certificates independently.
# Usage: replay_all.sh STUDY...   (lucas la unit_t capped)
set -u
V=/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/validator
R=/home/user/crypto/research
export CARGO_TARGET_DIR=$V/target
P=128,256,512,1024,2048
run() { # bin label args...
  local b=$1 l=$2; shift 2
  echo "+ $b run $* --json $V/out/$l.json" >> $V/out/replay_commands.txt
  $b run "$@" --threads 3 --json $V/out/$l.json > $V/out/$l.log 2>&1
  echo "$l exit=$? $(grep all_certified $V/out/$l.log)"
}
for s in "$@"; do
  case $s in
    lucas) c=4feb188f47; arms=(CLMR); committed="$R/ic_positive_control_lucas_20261116/run.json";;
    la) c=caae5eca9f; arms=(MNSR); committed="$R/ic_positive_control_la_20261116/run.json";;
    unit_t) c=551590e3b7; arms=(SUVR); committed="$R/ic_positive_control_unit_t_20261116/run.json";;
    capped) c=81ddcdc450; arms=(W SR); committed="$R/ic_positive_control_capped_20261116/run_w.json $R/ic_positive_control_capped_20261116/run_sr.json";;
  esac
  B=$V/bin/ic_positive_control.$c
  [ -x $B ] || { echo "no frozen binary for $c"; continue; }
  for a in "${arms[@]}"; do run $B replay_${s}_$a --primes $P --curves 5 --seeds 4 --arms $a; done
  if [ $s = la ]; then
    for cc in 0.7 1.0 1.4 2.0; do run $B replay_la_train_cc$cc --primes 4096 --curves 2 --seeds 2 --arms SR --cc $cc --curve-seed0 5001; python3 -I $V/scripts/compare_runs.py $V/out/replay_la_train_cc$cc.json $R/ic_positive_control_la_20261116/train_cc$cc.json | tee -a $V/out/compare_la_train.txt; done
  fi
  # val_dump.$c is built by scripts/build_pair.sh from the same lib compilation as the frozen binary
  [ -x $V/bin/val_dump.$c ] || { echo "no val_dump for $c"; continue; }
  for a in "${arms[@]}"; do run $V/bin/val_dump.$c dump_${s}_$a --primes $P --curves 5 --seeds 4 --arms $a; done
  for a in "${arms[@]}"; do
    python3 -I $V/scripts/verify_certs.py $V/out/dump_${s}_$a.json $V/out/replay_${s}_$a.json $committed --out $V/out/certs_${s}_$a.json
  done
done
