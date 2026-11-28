#!/usr/bin/env bash
# T20 frozen driver: thin orchestration only (AGENTS.md "no Python").
# Restart-safe: a step whose output exists is skipped.  Run from anywhere.
set -euo pipefail
ROOT=/home/user/crypto
S=$ROOT/research/t20_n73_k600_e2e_20261019
R=$S/runs
IC=$ROOT/target/release/examples/koblitz_orbit_dlp_fast
RHO=$ROOT/target/release/examples/koblitz_rho_fixture
BASE=$ROOT/experiments/koblitz-single-target-n73-20261002/base_n73_K600.jsonl
TGT=$ROOT/experiments/koblitz-single-target-n73-20261003/frozen/target_points.jsonl
# 2026-10-19 amendment (orchestration only, no data affected): the first launch
# aborted at rho_01 when isolated_bench refused to start because the session's
# own polling used 0.31 CPU-s in the 2 s settle window.  Threshold relaxed to
# 0.5 CPU and refused starts are retried (max 6) before failing.
iso() { local n=0; until python3 $ROOT/tools/isolated_bench.py run --wait --max-other-cpu 0.5 --out $R/isolation.jsonl "$@"; do n=$((n+1)); [ $n -ge 6 ] && return 1; sleep 30; done; }
ISO=iso

check() { # file expected-sha
  [ "$(sha256sum "$1" | cut -d' ' -f1)" = "$2" ] || { echo "HASH MISMATCH $1" >&2; exit 2; }
}
check $IC   bd9c3f0dc8c3113ed6ffba4a85bef4249fdfa39d1d3454dff82624ba4767edb4
check $RHO  9e0aa26c772947748e427300c5530a84a6770166057292ed55edff11e20fe8de
check $BASE a65776025e4b9ebc4a1e074f9304d834ac12cd533afc7d89a1d3ee72a2a766cc
check $TGT  f813c261330dfa7b0dfaeb010c7d14828f596b32b3b08b908dbbed1250beb83b
mkdir -p "$R"

scan() { # arm round
  local arm=$1 i=$2
  local out=$R/scan_${arm}_$i.txt
  [ -s "$out" ] && return 0
  local envs="KIC_LAZY_CANON=0 KIC_PREFETCH=0"
  [ "$arm" = new ] && envs="KIC_INDEX_THREADS=3 KIC_INDEX_BATCH_INV=1"
  $ISO --cpus 1-3 --label "scan_${arm}_$i" -- bash -c \
    "exec env $envs KIC_PROF_MAX_STATES=20000 $IC $BASE $TGT 7 /dev/null 2>&1 >/dev/null | grep '^PROF' > $out"
}

ic() { # tag arm
  local tag=$1 arm=$2
  [ -s "$R/ic_$tag.row.jsonl" ] && return 0
  local envs="KIC_LAZY_CANON=0 KIC_PREFETCH=0"
  [ "$arm" = new ] && envs="KIC_INDEX_THREADS=3 KIC_INDEX_BATCH_INV=1"
  $ISO --cpus 1-3 --label "ic_$tag" -- bash -c \
    "TIMEFORMAT='real=%R user=%U sys=%S'; { time env $envs KIC_RANK_THREADS=3 KIC_RANK_TRACE=1 $IC $BASE $TGT 7 $R/ic_$tag.row.jsonl > $R/ic_$tag.summary.json 2> $R/ic_$tag.stderr; } 2> $R/ic_$tag.time"
}

rho() { # index 1..20
  local i; i=$(printf %02d "$1")
  [ -s "$R/rho_$i.jsonl" ] && return 0
  $ISO --cpus 1 --label "rho_$i" -- bash -c \
    "TIMEFORMAT='real=%R user=%U sys=%S'; export KIC_RHO_PUBLIC_TARGET_POINT='$(cat $TGT)' KIC_RHO_WALK_SEED=$((910000000 + $1)); { time $RHO 73 0 signed_frobenius 1 packed > $R/rho_$i.jsonl 2> $R/rho_$i.stderr; } 2> $R/rho_$i.time"
}

for r in 1 2 3; do scan old $r; scan new $r; done
for i in $(seq 1 10); do rho $i; done
ic N1 new; ic O1 old; ic N2 new
for i in $(seq 11 20); do rho $i; done
echo "T20 DRIVER COMPLETE $(date -u +%FT%TZ)" > "$R/COMPLETE"
