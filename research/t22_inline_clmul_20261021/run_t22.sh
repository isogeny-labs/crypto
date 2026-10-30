#!/usr/bin/env bash
# T22 frozen driver: scan A/B of the old (T21 head) binary against the binary with an
# inlined clmul_u64, 7 rotated rounds with an A/A arm, then callgrind Ir.
# Thin orchestration only.  Restart-safe.
set -euo pipefail
ROOT=/home/user/crypto
S=$ROOT/research/t22_inline_clmul_20261021
R=$S/runs
NEW=$ROOT/target/release/examples/koblitz_orbit_dlp_fast
OLD=/home/user/wt/t22/old_bin
BASE=$ROOT/experiments/koblitz-single-target-n73-20261002/base_n73_K600.jsonl
TGT=$ROOT/experiments/koblitz-single-target-n73-20261003/frozen/target_points.jsonl
iso() { local n=0; until python3 $ROOT/tools/isolated_bench.py run --wait --max-other-cpu 0.5 --out $R/isolation.jsonl "$@"; do n=$((n+1)); [ $n -ge 6 ] && return 1; sleep 30; done; }
check() { [ "$(sha256sum "$1" | cut -d' ' -f1)" = "$2" ] || { echo "HASH MISMATCH $1" >&2; exit 2; }; }
check $NEW  9988639a57a7c7d6c98d35ac78ede1aa95f2017ecd9f16e5aabaa2fb53c0fc67
check $OLD  ea4faa51604db1330805bbc0329016999edea8f89b5cbe09ef766b60427745f0
check $BASE a65776025e4b9ebc4a1e074f9304d834ac12cd533afc7d89a1d3ee72a2a766cc
check $TGT  f813c261330dfa7b0dfaeb010c7d14828f596b32b3b08b908dbbed1250beb83b
mkdir -p "$R"; : > $R/empty.txt
ARMS=(old new old2)
bin_of() { case $1 in new) echo $NEW;; *) echo $OLD;; esac; }
for r in 1 2 3 4 5 6 7; do
  for k in 0 1 2; do
    arm=${ARMS[$(( (k + r - 1) % 3 ))]}
    out=$R/scan_${arm}_$r.txt
    [ -s "$out" ] && continue
    iso --cpus 1-3 --label "scan_${arm}_$r" -- bash -c \
      "exec env KIC_ROOT_ALGEBRA=0 KIC_SPARSE_REDUCE=0 KIC_INDEX_THREADS=3 KIC_INDEX_BATCH_INV=1 KIC_PROF_MAX_STATES=20000 $(bin_of $arm) $BASE $TGT 7 /dev/null 2>&1 >/dev/null | grep '^PROF' > $out"
  done
done
for arm in old new; do
  out=$R/cg_$arm.txt
  [ -s "$out" ] && continue
  iso --cpus 1 --label "cg_$arm" -- bash -c \
    "env KIC_ROOT_ALGEBRA=0 KIC_SPARSE_REDUCE=0 KIC_PROF_MAX_STATES=2000 valgrind --tool=callgrind --callgrind-out-file=$R/cg_$arm.out $(bin_of $arm) construct:73:1:40 $R/empty.txt 1 /dev/null 2>&1 | grep -E 'PROF|Collected' > $out; callgrind_annotate $R/cg_$arm.out 2>/dev/null | grep -E 'reduce_fold|mul_fold|clmul|extract128|RootBatch128|build_index128_impl|Gf2_128>::sqr' >> $out"
done
echo "T22 COMPLETE $(date -u +%FT%TZ)" > "$R/COMPLETE"
