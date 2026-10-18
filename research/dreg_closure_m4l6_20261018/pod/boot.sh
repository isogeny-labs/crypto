#!/usr/bin/env bash
# Boot script for the RunPod hosts of this study (PROTOCOL.md §7) and of the
# PKM m = 3, N = 21 cell (research/pkm_lead1_m3n21_20261016/PROTOCOL.md).
# Plumbing only: installs a toolchain, clones the pinned branch, builds the
# two examples, runs the frozen run.sh, serves the study directory on :8000.
#   boot.sh closure|pkm|both     (env: BRANCH; PKM_SHARE=1|2 runs the two PKM targets one after the other or at once)
set -u
JOB=$1; BRANCH=${BRANCH:?BRANCH}
T0=$(date +%s); CAPS=${POD_MAX_SECONDS:-36000}
remaining() { echo $(( CAPS - ($(date +%s) - T0) )); }
OUT=/workspace/out; mkdir -p "$OUT"; ln -sf /root/boot.log "$OUT/boot.log"
status() { echo "$(date -u +%FT%TZ) $1" >> "$OUT/STATUS"; }
status BOOT
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq > "$OUT/apt.log" 2>&1
apt-get install -y -qq git build-essential curl pkg-config libssl-dev python3 util-linux time ca-certificates >> "$OUT/apt.log" 2>&1
(cd "$OUT" && nohup python3 -m http.server 8000 > /dev/null 2>&1 &)
curl -sSf https://sh.rustup.rs | sh -s -- -y -q --profile minimal > "$OUT/rustup.log" 2>&1
. "$HOME/.cargo/env"
cd /workspace
git clone -q --branch "$BRANCH" https://github.com/aburan28/crypto src >> "$OUT/boot.log" 2>&1 || { status CLONE_FAILED; exit 1; }
cd src
ln -sfn /workspace/src/research/dreg_closure_m4l6_20261018 "$OUT/closure"
ln -sfn /workspace/src/research/pkm_lead1_m3n21_20261016 "$OUT/pkm"
ln -sfn /workspace/src/research/pkm_m3_n21_msolve_20261018 "$OUT/pkm-msolve"
ln -sfn /workspace/src "$OUT/src"
{ echo "cgroup memory.max: $(cat /sys/fs/cgroup/memory.max 2>/dev/null || cat /sys/fs/cgroup/memory/memory.limit_in_bytes 2>/dev/null)"
  grep MemTotal /proc/meminfo; nproc; } > "$OUT/host-quick.txt"
status BUILDING
run_closure() {
  cargo build --release --example dreg_ladder --example dreg_closure > "$OUT/build.log" 2>&1 || { status BUILD_FAILED; return 1; }
  L=target/release/examples/dreg_ladder; C=target/release/examples/dreg_closure
  S=research/dreg_closure_m4l6_20261018/run.sh
  export FRESH="${FRESH:-4:15:6 4:16:6}" TMO4="${TMO4:-14400}" DEG5_DRAWS="${DEG5_DRAWS:-1}"
  $S $L $C host; status GENERATING; $S $L $C gen; status GEN_DONE
  # The draws are committed before any closure measurement (PROTOCOL.md §10):
  # the gate is the committed checksum file in this clone, which verifies
  # against the draws it names. (The tag push was refused by the authoring
  # session's git proxy; §10 names the anchor commit instead.)
  DR=research/dreg_closure_m4l6_20261018/draws
  until [ -s "$DR/sha256.txt" ] && (cd "$DR" && sha256sum -c --quiet sha256.txt); do git pull -q --ff-only origin "$BRANCH" || true; sleep 120; done
  status MEASURING
  $S $L $C measure; status A_DONE
  export TMO5=$(( $(remaining) - 2400 )); [ "$TMO5" -gt 600 ] && $S $L $C deg5
  $S $L $C control; status CLOSURE_DONE
}
run_pkm() {
  cargo build --release --example pkm_tower_pilot --example pkm_tower_check > "$OUT/build-pkm.log" 2>&1 || { status BUILD_FAILED; return 1; }
  S=research/pkm_lead1_m3n21_20261016/run.sh
  LIM=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || cat /sys/fs/cgroup/memory/memory.limit_in_bytes 2>/dev/null || echo max)
  MT=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
  if [ "$LIM" != max ] && [ -n "$LIM" ] && [ "$LIM" -lt "$MT" ]; then
    # PROTOCOL.md §2: A = 0.92 * MemTotal / SHARE. In a container the memory the
    # process can use is the cgroup limit, not the host's MemTotal; apply the
    # same rule to it. The copy is recorded beside the original.
    cp "$S" "${S%.sh}-cgroup.sh"
    sed -i "s|^mem_kib=.*|mem_kib=$(( LIM / 1024 ))  # cgroup memory limit, set by pod/boot.sh; /proc/meminfo MemTotal was $(( MT / 1024 )) KiB|" "${S%.sh}-cgroup.sh"
    S="${S%.sh}-cgroup.sh"
  fi
  export SHARE=${PKM_SHARE:-1}
  $S host; status IDENTITY; $S identity
  if [ "$SHARE" = 2 ]; then
    status "MAIN0+MAIN1 (SHARE=2)"; $S main 0 & p0=$!; sleep 60; $S main 1 & p1=$!; wait $p0 $p1
  else
    status MAIN0; $S main 0; status MAIN1; $S main 1
  fi
  for t in 0 1; do
    D=$(python3 - "$t" <<'PY'
import json, sys
t = sys.argv[1]; best = None
try:
    for line in open(f"research/pkm_lead1_m3n21_20261016/runs/M3-N21-t{t}.jsonl"):
        r = json.loads(line)
        if r.get("inconsistent") and r.get("solving_degree_max"):
            best = max(best or 0, int(r["solving_degree_max"]))
except Exception:
    pass
print(best if best else "")
PY
)
    [ -n "$D" ] && { status "CONFIRM$t D=$D"; $S confirm "$t" "$D"; }
  done
  status VERIFY; $S verify; status PKM_DONE
}
run_pkm_msolve() {
  apt-get install -y -qq msolve >> "$OUT/apt.log" 2>&1 || {
    apt-get install -y -qq autoconf automake libtool libgmp-dev libmpfr-dev libflint-dev >> "$OUT/apt.log" 2>&1
    (cd /workspace && git clone -q --depth 1 --branch v0.6.5 https://github.com/algebraic-solving/msolve && cd msolve \
       && ./autogen.sh && ./configure && make -j"$(nproc)" && make install) > "$OUT/msolve-build.log" 2>&1; }
  D=research/pkm_m3_n21_msolve_20261018; mkdir -p "$D/runs"
  { echo "date: $(date -u +%FT%TZ)"; echo "commit: $(git rev-parse HEAD)"; echo "msolve: $(command -v msolve) $(msolve -h 2>&1 | head -2 | tr '\n' ' ')"
    echo "msolve sha256: $(sha256sum "$(command -v msolve)" | cut -d' ' -f1)"; cat "$OUT/host-quick.txt"; lscpu; } > "$D/runs/host.txt"
  LIM=$(cat /sys/fs/cgroup/memory.max 2>/dev/null || echo max); [ "$LIM" = max ] && LIM=$(( $(awk '/^MemTotal:/ {print $2}' /proc/meminfo) * 1024 ))
  export CAP=$(( LIM * 85 / 100 ))
  status "MSOLVE control m3_t5"; WALL=2700 "$D/run.sh" pkm_kummer_p2013265921_m3_t5_x1289903598
  for s in pkm_kummer_p2013265921_m3_t7_x221026520 pkm_kummer_p2013265921_m3_t7_x704078485; do
    export WALL=$(( ($(remaining) - 3600) / 2 )); [ "$WALL" -gt 600 ] || break
    status "MSOLVE $s wall=$WALL"; "$D/run.sh" "$s"; done
  export WALL=$(( $(remaining) - 3000 )); [ "$WALL" -gt 1800 ] && { status "MSOLVE m3_t8 wall=$WALL"; "$D/run.sh" pkm_kummer_p2013265921_m3_t8_x1227422598; }
  status PKM_MSOLVE_DONE
}
case $JOB in
  closure) run_closure ;;
  pkm-msolve) run_pkm_msolve ;;
  pkm) run_pkm ;;
  both) run_closure; run_pkm ;;
esac
status DONE
# Keep serving results for 45 minutes, then end the container (budget); the
# launching session also terminates the pod. No API key lives on the pod.
sleep 2700
echo "$(date -u +%FT%TZ) serving window over" >> "$OUT/STATUS"
kill -TERM 1
