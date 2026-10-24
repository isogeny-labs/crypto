#!/usr/bin/env bash
# C1 pod job (PROTOCOL.md Amendment 2). Plumbing only, after
# research/dreg_closure_m4l6_20261018/pod/boot.sh run_c1: build the dense
# closure, run degree 5 on the four unsatisfiable C1 draws 0, 3, 5, 7 one
# after another, then the satisfiable control draw 1 with the time left.
# Serves /workspace/out on :8000. No API key lives on the pod.
set -u
BRANCH=${BRANCH:?BRANCH}
T0=$(date +%s); CAPS=${POD_MAX_SECONDS:-93600}
remaining() { echo $(( CAPS - ($(date +%s) - T0) )); }
OUT=/workspace/out; mkdir -p "$OUT"; ln -sf /root/boot.log "$OUT/boot.log"
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
status BOOT
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq > "$OUT/apt.log" 2>&1
apt-get install -y -qq git build-essential curl pkg-config libssl-dev python3 util-linux time bc ca-certificates >> "$OUT/apt.log" 2>&1
(cd "$OUT" && nohup python3 -m http.server 8000 > /dev/null 2>&1 &)
curl -sSf https://sh.rustup.rs | sh -s -- -y -q --profile minimal > "$OUT/rustup.log" 2>&1
. "$HOME/.cargo/env"
cd /workspace
git clone -q --depth 1 --single-branch --branch "$BRANCH" https://github.com/aburan28/crypto src >> "$OUT/boot.log" 2>&1 || { status CLONE_FAILED; exit 1; }
cd src
ln -sfn /workspace/src/research/dreg_closure_m4l6_20261018 "$OUT/closure"
status BUILDING
cargo build --release --example dreg_ladder --example dreg_closure > "$OUT/build.log" 2>&1 || { status BUILD_FAILED; exit 1; }
L=target/release/examples/dreg_ladder; C=target/release/examples/dreg_closure
S=research/dreg_closure_m4l6_20261018/run.sh
F=research/dreg_closure_scale_20261016/draws/m3-19-7.jsonl
$S $L $C host
export DEG5_DRAWS=4 DEG5_MIN_LIMIT=280000000000 TMO5=${TMO5:-18000}
status "DEG5 C1 draws=4 TMO5=$TMO5"; $S $L $C deg5; status C1_UNSAT_DONE
# Satisfiable control (draw 1): must never refute; a timeout is expected.
TC=$(( $(remaining) - ${SERVE_SECONDS:-2700} - 900 ))
if [ "$TC" -gt 1800 ]; then
  status "CONTROL d1 tmo=$TC"
  R=research/dreg_closure_m4l6_20261018/runs
  LIMIT=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo max)
  [ "$LIMIT" = max ] && LIMIT=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
  prlimit --as=$(( LIMIT * 92 / 100 )) timeout "$TC" /usr/bin/time -v \
    "$C" --to 5 --max-cols 1100000 --draw 1 --satisfiable "$F" > "$R/control5-m3-19-7.d1.jsonl" 2> "$R/control5-m3-19-7.d1.log"
  echo "$(date -u +%FT%TZ) control5 d1: exit $?" >> "$R/queue.log"
fi
status DONE
sleep "${SERVE_SECONDS:-2700}"
echo "$(date -u +%FT%TZ) serving window over" >> "$OUT/STATUS"
kill -TERM 1
