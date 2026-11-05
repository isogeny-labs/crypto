#!/usr/bin/env bash
# The 17 frozen cells of PROTOCOL.md. Sequential, interleaved arms, frozen binary only.
set -u
D=$(cd "$(dirname "$0")" && pwd)
BIN=/home/user/frozen-bin/curve25519_family_leadE
mkdir -p "$D/cells"
ulimit -v 3500000
export F4COST_PLANTED=20
run() { # name form t G seed
  c="$D/cells/$1"
  { echo "start $(date -u +%FT%TZ)"; uptime; } > "$c.status"
  timeout 900 "$BIN" f4cost 20 "$2" "$3" "$4" 100 "$5" 600 2 full 0 30000000 > "$c.json" 2> "$c.stderr"
  echo "end $(date -u +%FT%TZ) rc=$?" >> "$c.status"; uptime >> "$c.status"
}
lo_of() { case $1 in 4) echo 0;; 5) echo 1;; 6) echo 34;; esac; }
for t in 4 5 6; do
  seed=$((2026110200 + t))
  for g in 0 3; do
    run plain_t${t}_g$g bits $t $g $seed
    run t0_t${t}_g$g "t0-bits@$(lo_of $t)" $t $g $seed
    case "$t:$g" in
      4:0) run x_t5_g0 "t0-bits@0" 5 0 $seed;;
      4:3) run x_t5_g3 "t0-bits@0" 5 3 $seed;;
      5:0) run x_t6_g0 "t0-bits@68" 6 0 $seed;;
      5:3) run x_t6_g3 "t0-bits@68" 6 3 $seed;;
      6:3) run x_t7_g3 "t0-bits@68" 7 3 $seed;;
    esac
  done
done
echo done > "$D/cells/ALL_DONE"
