#!/usr/bin/env bash
# T21 frozen driver (stage A scan, stage P process start, stage C instruction
# counts).  Thin orchestration only.  Restart-safe: finished outputs are kept.
set -euo pipefail
ROOT=/home/user/crypto
S=$ROOT/research/t21_wide_producer_arithmetic_20261020
R=$S/runs
NEW=$ROOT/target/release/examples/koblitz_orbit_dlp_fast
OLD=/home/user/wt/target-t20/release/examples/koblitz_orbit_dlp_fast
BASE=$ROOT/experiments/koblitz-single-target-n73-20261002/base_n73_K600.jsonl
TGT=$ROOT/experiments/koblitz-single-target-n73-20261003/frozen/target_points.jsonl
iso() { local n=0; until python3 $ROOT/tools/isolated_bench.py run --wait --max-other-cpu 0.5 --out $R/isolation.jsonl "$@"; do n=$((n+1)); [ $n -ge 6 ] && return 1; sleep 30; done; }
check() { [ "$(sha256sum "$1" | cut -d' ' -f1)" = "$2" ] || { echo "HASH MISMATCH $1" >&2; exit 2; }; }
check $NEW  cb9502a2b2767cacdbea20587cab4f6d052e6eb20b2d9c08def482bd78889bbc
check $OLD  bd9c3f0dc8c3113ed6ffba4a85bef4249fdfa39d1d3454dff82624ba4767edb4
check $BASE a65776025e4b9ebc4a1e074f9304d834ac12cd533afc7d89a1d3ee72a2a766cc
check $TGT  f813c261330dfa7b0dfaeb010c7d14828f596b32b3b08b908dbbed1250beb83b
mkdir -p "$R"
: > $R/empty.txt
arm_env() { case $1 in
  ctl|ctl2) echo "KIC_ROOT_ALGEBRA=0 KIC_SPARSE_REDUCE=0";;
  sparse)   echo "KIC_ROOT_ALGEBRA=0 KIC_SPARSE_REDUCE=1";;
  algebra)  echo "KIC_ROOT_ALGEBRA=1 KIC_SPARSE_REDUCE=0";;
  all)      echo "KIC_ROOT_ALGEBRA=1 KIC_SPARSE_REDUCE=1";;
esac; }
ARMS=(ctl sparse algebra all ctl2)

# Stage A: scan at the real K=600 base, 7 rounds, arm order rotated per round
for r in 1 2 3 4 5 6 7; do
  for k in 0 1 2 3 4; do
    arm=${ARMS[$(( (k + r - 1) % 5 ))]}
    out=$R/scan_${arm}_$r.txt
    [ -s "$out" ] && continue
    iso --cpus 1-3 --label "scan_${arm}_$r" -- bash -c \
      "exec env $(arm_env $arm) KIC_INDEX_THREADS=3 KIC_INDEX_BATCH_INV=1 KIC_PROF_MAX_STATES=20000 $NEW $BASE $TGT 7 /dev/null 2>&1 >/dev/null | grep '^PROF' > $out"
  done
done

# Stage P: process start (factorise cut): old = T20 head binary, new = this binary,
# K=40 construct, exit after one scanned state; 5 rounds alternating, one core
for r in 1 2 3 4 5; do
  if [ $(( r % 2 )) -eq 0 ]; then order="new old"; else order="old new"; fi
  for arm in $order; do
    out=$R/proc_${arm}_$r.time
    [ -s "$out" ] && continue
    bin=$NEW; [ $arm = old ] && bin=$OLD
    iso --cpus 1 --label "proc_${arm}_$r" -- bash -c \
      "TIMEFORMAT='real=%R user=%U sys=%S'; { time env KIC_ROOT_ALGEBRA=0 KIC_SPARSE_REDUCE=0 KIC_PROF_MAX_STATES=1 $bin construct:73:1:40 $R/empty.txt 1 /dev/null > /dev/null 2>&1; } 2> $out"
  done
done

# Stage C: deterministic instruction counts (callgrind), K=40, 2000 states
for arm in ctl sparse algebra all; do
  out=$R/cg_$arm.txt
  [ -s "$out" ] && continue
  iso --cpus 1 --label "cg_$arm" -- bash -c \
    "env $(arm_env $arm) KIC_PROF_MAX_STATES=2000 valgrind --tool=callgrind --callgrind-out-file=$R/cg_$arm.out $NEW construct:73:1:40 $R/empty.txt 1 /dev/null 2>&1 | grep -E 'PROF|Collected' > $out; callgrind_annotate $R/cg_$arm.out 2>/dev/null | grep -E 'reduce_fold|mul_fold|extract128|RootBatch128|build_index128_impl|Gf2_128>::sqr' >> $out"
done
echo "T21 STAGES A P C COMPLETE $(date -u +%FT%TZ)" > "$R/COMPLETE"
