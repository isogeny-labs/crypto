#!/bin/bash
# First hardware baseline of gpu/semaev and gpu/rootfind, per
# research/gpu_ic_baseline_20261027/PROTOCOL.md.  Container contract of
# ecc2k130/modal_job.py: the gpu/ tree is at /work/gpu, results go to $RESULTS,
# nvcc and nvidia-smi are on PATH.  Stage order is the protocol's gate order:
#   0 host facts   1 host tests in this container   2 builds   3 device selftests
#   4 ptxas / occupancy   5 throughput (only if every selftest passed)
# A failing selftest is the result: stage 5 is then skipped, never partial.
set -uo pipefail
cd /work/gpu || exit 1
export DEBIAN_FRONTEND=noninteractive
R=${RESULTS:-/results}; mkdir -p "$R"
ARCH=${ARCH:-sm_120}
LS=${SEM_LS:-"12 14 16"}            # semaev subspace dimensions; each is its own build
REPS=${REPS:-5}                     # interleaved throughput rounds per arm
TOKEN=${TOKEN:-$(date +%s)}
fail=0
note() { echo "[$(date -u +%H:%M:%S)] $*" | tee -a "$R/stages.log"; }

note "stage 0: host facts"
{
  nvidia-smi --query-gpu=name,driver_version,clocks.max.sm,power.limit,ecc.mode.current,memory.total --format=csv,noheader
  nvidia-smi -q -d CLOCK,PERFORMANCE | head -60
  nvcc --version | tail -2
  grep -m1 "model name" /proc/cpuinfo; nproc
  echo "source: ${SOURCE_REV:-unknown}"; echo "arch: $ARCH  semaev l: $LS  reps: $REPS"
} > "$R/host.txt" 2>&1
cat "$R/host.txt" | head -8

note "stage 1: host tests in this container (same sources as the device build)"
for l in $LS; do
  make -s -C semaev clean >/dev/null; make -s -C semaev test L=$l > "$R/hosttest-semaev-l$l.log" 2>&1 \
    && note "semaev host test l=$l PASS" || { note "semaev host test l=$l FAIL"; fail=1; }
done
make -s -C rootfind clean >/dev/null; make -s -C rootfind test > "$R/hosttest-rootfind.log" 2>&1 \
  && note "rootfind host test PASS" || { note "rootfind host test FAIL"; fail=1; }

note "stage 2: builds"
mkdir -p "$R/bin"
build_semaev() {  # name l extra-flags
  local name=$1 l=$2 extra=$3
  make -s -C semaev clean >/dev/null
  make -s -C semaev bench2 L=$l ARCH=$ARCH NVCCFLAGS="-O3 -std=c++17 -arch=$ARCH -lineinfo $extra" \
    > "$R/build-$name.log" 2>&1 && cp semaev/bench2 "$R/bin/$name" || { note "BUILD FAILED $name"; fail=1; }
}
for l in $LS; do
  build_semaev "semaev-l$l-clmad" $l ""
  build_semaev "semaev-l$l-soft" $l "-DGF2N_CLMAD=0"
done
l0=$(echo $LS | cut -d' ' -f1)
build_semaev "semaev-l$l0-clmad-AA" $l0 ""      # separate build of the same source: A/A arm
make -s -C rootfind clean >/dev/null
make -s -C rootfind bench ARCH=$ARCH > "$R/build-rootfind.log" 2>&1 && cp rootfind/bench "$R/bin/rootfind" \
  || { note "BUILD FAILED rootfind"; fail=1; }

note "stage 3: device selftests (gate)"
for b in "$R"/bin/semaev-*; do
  n=$(basename "$b")
  timeout 600 "$b" selftest > "$R/selftest-$n.log" 2>&1 && note "selftest $n PASS" || { note "selftest $n FAIL"; fail=1; }
done
if [ -x "$R/bin/rootfind" ]; then
  ( cd rootfind
    timeout 900 "$R/bin/rootfind" selftest 61 1024 4096 > "$R/selftest-rootfind-61.log" 2>&1 \
      && note "selftest rootfind 61 PASS" || { note "selftest rootfind 61 FAIL"; echo 1 > "$R/.fail"; }
    timeout 900 "$R/bin/rootfind" p256 256 1024 > "$R/selftest-rootfind-p256.log" 2>&1 \
      && note "selftest rootfind p256 PASS" || { note "selftest rootfind p256 FAIL"; echo 1 > "$R/.fail"; } )
  [ -f "$R/.fail" ] && fail=1
fi

note "stage 4: ptxas and occupancy (recorded, not tuned)"
make -s -C rootfind ptxas-info ARCH=$ARCH > "$R/ptxas-rootfind.log" 2>&1 || note "ptxas-info rootfind failed"
for b in "$R"/bin/semaev-*-clmad "$R"/bin/semaev-*-soft; do
  [ -x "$b" ] && "$b" occupancy > "$R/occupancy-$(basename "$b").log" 2>&1
done

if [ "$fail" -ne 0 ]; then
  note "GATE FAILED: no throughput is reported (protocol stop rule)"
  echo "$fail" > "$R/gate-failed"; exit 1
fi

note "stage 5: throughput, $REPS interleaved rounds, order from sha256(token, round)"
nvidia-smi --query-gpu=timestamp,clocks.sm,clocks.max.sm,power.draw,temperature.gpu --format=csv,noheader -lms 500 \
  > "$R/clocks.csv" 2>/dev/null &
SMI=$!
arms=$(ls "$R"/bin | tr '\n' ' ')
for r in $(seq 1 "$REPS"); do
  order=$(python3 - "$TOKEN" "$r" $arms <<'PY'
import hashlib, random, sys
token, rnd, arms = sys.argv[1], sys.argv[2], sys.argv[3:]
random.Random(int(hashlib.sha256(f"{token}:{rnd}".encode()).hexdigest(), 16)).shuffle(arms)
print(" ".join(arms))
PY
)
  note "round $r order: $order"
  for a in $order; do
    case $a in
      rootfind) ( cd rootfind; "$R/bin/rootfind" throughput 61 65536 65536 ) > "$R/throughput-r$r-$a.log" 2>&1 ;;
      *)        "$R/bin/$a" throughput > "$R/throughput-r$r-$a.log" 2>&1 ;;
    esac
    echo "$r $a $?" >> "$R/throughput-exit.txt"
  done
done
kill $SMI 2>/dev/null
note "done"
exit 0
