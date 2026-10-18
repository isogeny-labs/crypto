#!/usr/bin/env bash
# Boot script for the RunPod hosts of this study (PROTOCOL.md §7) and of the
# PKM m = 3, N = 21 cell (research/pkm_lead1_m3n21_20261016/PROTOCOL.md).
# Plumbing only: installs a toolchain, clones the pinned branch, builds the
# two examples, runs the frozen run.sh, serves the study directory on :8000.
#   boot.sh closure|pkm     (env: BRANCH, RUNPOD_API_KEY, RUNPOD_POD_ID)
set -u
JOB=$1; BRANCH=${BRANCH:?BRANCH}
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
{ echo "cgroup memory.max: $(cat /sys/fs/cgroup/memory.max 2>/dev/null || cat /sys/fs/cgroup/memory/memory.limit_in_bytes 2>/dev/null)"
  grep MemTotal /proc/meminfo; nproc; } > "$OUT/host-quick.txt"
status BUILDING
case $JOB in
closure)
  cargo build --release --example dreg_ladder --example dreg_closure > "$OUT/build.log" 2>&1 || { status BUILD_FAILED; exit 1; }
  L=target/release/examples/dreg_ladder; C=target/release/examples/dreg_closure
  S=research/dreg_closure_m4l6_20261018/run.sh
  $S $L $C host; status GENERATING; $S $L $C gen; status GEN_DONE
  # The draws are committed before any closure measurement: wait for the tag.
  until git ls-remote --tags origin 2>/dev/null | grep -q 'refs/tags/draws-m4l6-20261018$'; do sleep 120; done
  status MEASURING
  $S $L $C control; $S $L $C measure; $S $L $C deg5 C2; status DONE ;;
pkm)
  cargo build --release --example pkm_tower_pilot --example pkm_tower_check > "$OUT/build.log" 2>&1 || { status BUILD_FAILED; exit 1; }
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
  $S host; status IDENTITY; $S identity; status MAIN0; $S main 0; status MAIN1; $S main 1
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
  status VERIFY; $S verify; status DONE ;;
esac
# Keep serving results for a day, then stop the pod (billing stops; disk kept).
sleep 86400
curl -s https://api.runpod.io/graphql -H 'Content-Type: application/json' -H "Authorization: Bearer ${RUNPOD_API_KEY:-}" \
  -d "{\"query\":\"mutation { podStop(input: {podId: \\\"${RUNPOD_POD_ID:-}\\\"}) { id } }\"}" > /dev/null
