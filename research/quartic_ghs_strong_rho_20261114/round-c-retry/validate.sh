#!/bin/sh
set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

jq -e -s '
  [ .[] | select(.rec == "rho") ] as $r |
  ($r | length == 2) and
  ($r[0].ok and $r[1].ok) and
  (["q","seed","i","target_kind","ell","k","k_found","steps","dps","restarts","lookahead","cycles","cycle_adds","batch"] |
    all(. as $k | $r[0][$k] == $r[1][$k])) and
  (["setup.fq_mul","setup.fq_inv","setup.fq_add","setup.f3_mul","setup.f3_sq","setup.f3_inv","setup.batch_inv","setup.group_add",
    "verify.fq_mul","verify.fq_inv","verify.fq_add","verify.f3_mul","verify.f3_sq","verify.f3_inv","verify.batch_inv","verify.group_add",
    "setup_native.rng","setup_native.hash","setup_native.canon","setup_native.lookahead_test","setup_native.dp","setup_native.table","setup_native.cycle_cmp","setup_native.scalar_add","setup_native.scalar_mul","setup_native.scalar_inv","setup_native.band_ops",
    "walk_native.rng","walk_native.hash","walk_native.canon","walk_native.lookahead_test","walk_native.dp","walk_native.table","walk_native.cycle_cmp","walk_native.scalar_add","walk_native.scalar_mul","walk_native.scalar_inv","walk_native.band_ops"] |
    all(. as $k | $r[0][$k] == $r[1][$k])) and
  ($r[0]["walk.batch_inv"] == 231) and
  ($r[1]["walk.batch_inv"] == 188) and
  ($r[1]["walk.group_add"] - $r[0]["walk.group_add"] == 1437) and
  ($r[1]["walk.fq_mul"] - $r[0]["walk.fq_mul"] == 50381) and
  ($r[1]["walk.fq_inv"] - $r[0]["walk.fq_inv"] == -43)
' "$here/q223.jsonl"

grep -qx 'SELFTEST OK' "$here/selftest.txt"
printf '%s\n' 'ROUND C VALIDATION OK'
