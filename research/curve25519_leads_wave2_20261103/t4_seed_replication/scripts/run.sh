#!/usr/bin/env bash
# T4 seed-selection replication, PROTOCOL.md (frozen 2026-11-03).
# Runs the planned f4cost pairs strictly sequentially (one process at a time).
# Optional: SKIP_SECONDARY=1 omits the secondary cell (5a/5b), which is then reported "not run".
set -euo pipefail
D="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$D/data"
B=${BIN:-/home/user/ctarget/release/curve25519_family}
WANT=7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be
CENSUS=/home/user/crypto/research/curve25519_isogeny_seed_20261103/data
mkdir -p "$OUT"
LOG="$OUT/run.log"

have=$(sha256sum "$B" | cut -d' ' -f1)
[ "$have" = "$WANT" ] || { echo "binary hash mismatch: $have" >&2; exit 2; }
( cd "$CENSUS" && sha256sum -c --quiet - ) <<'SUMS'
3935c968587dd631c7cebfaaa4f530b03a5ed6da4045ab9cf9ba14504699d498  k16.jsonl
4e670176d073d90c7f7dd7a6cd951f25072330128794e981eeb48181696d84cf  k18.jsonl
bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc  k20.jsonl
11903b11c6d1f2eccfadd27b058e95fb2410999b21e22afc3a9854220c770e9e  k22.jsonl
39c7543607717def3ff68ef852151f482ea6ff0a13b33a9bd00bc34a2329eba0  k24.jsonl
SUMS
echo "binary $have" >> "$LOG"

TS=2026111506; NT=100; CAP=3600; M=2; NNZ=30000000

run() { # label k form t G
  local label=$1 k=$2 form=$3 t=$4 g=$5
  local cmd=("$B" f4cost "$k" "$form" "$t" "$g" "$NT" "$TS" "$CAP" "$M" full 0 "$NNZ")
  echo "$(date -u +%FT%TZ) START $label: ${cmd[*]}" >> "$LOG"
  set +e
  "${cmd[@]}" > "$OUT/$label.json" 2> "$OUT/$label.stderr"
  local rc=$?
  set -e
  echo "$(date -u +%FT%TZ) END $label rc=$rc" >> "$LOG"
}

# control C0 (k=20 reproduction of R4)
run k20_t6_g4_s0     20 isogeny      6 4
run k20_t6_g4_s1093  20 isogeny@1093 6 4
# primary cells
run k16_t6_g4_s0     16 isogeny      6 4
run k16_t6_g4_s1381  16 isogeny@1381 6 4
run k18_t6_g4_s0     18 isogeny      6 4
run k18_t6_g4_s1277  18 isogeny@1277 6 4
run k22_t6_g4_s0     22 isogeny      6 4
run k22_t6_g4_s1070  22 isogeny@1070 6 4
run k24_t6_g4_s0     24 isogeny      6 4
run k24_t6_g4_s1490  24 isogeny@1490 6 4
# secondary cell
if [ "${SKIP_SECONDARY:-0}" != 1 ]; then
  run k24_t5_g3_s0     24 isogeny     5 3
  run k24_t5_g3_s797   24 isogeny@797 5 3
else
  echo "$(date -u +%FT%TZ) secondary cell not run (SKIP_SECONDARY=1)" >> "$LOG"
fi

( cd "$OUT" && sha256sum *.json *.stderr > SHA256SUMS )
