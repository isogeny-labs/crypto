#!/bin/bash
# Thin orchestration: runs the frozen specs of PROTOCOL.md into $D/sessions, audits each
# session with exact replays, and writes the §5a band report. Test B's test specs are
# generated from the tuning sessions by the frozen `pick` rule.
set -u
cd "$(dirname "$0")/../.."
D=research/structure_aware_ic_20261019
B=./target/release/ecbench
T=./target/release/examples/structure_aware_curves
mkdir -p $D/sessions $D/specs/B
run() { # name spec
  local name=$1 spec=$2 out=$D/sessions/$1
  [ -d "$out" ] && { echo "== $name exists"; return; }
  echo "== $name start $(date -u +%FT%TZ)"
  $B run --wait --allow-busy --spec "$spec" --out "$out" > "$D/sessions/$name.run.log" 2>&1
  echo "   run exit $? ; $(tail -n1 $D/sessions/$name.run.log | cut -c1-160)"
  $B verify --dir "$out" --replay 12 --out "$D/sessions/$name.audit.json" > "$D/sessions/$name.verify.log" 2>&1
  echo "   verify exit $?"
  $B table --dir "$out" > "$D/sessions/$name.table.txt" 2>&1
  $B band --dir "$out" --weights 0,0.1,1 --json "$D/sessions/$name.band.json" --ranks > "$D/sessions/$name.band.txt" 2>&1
}
case "${1:-all}" in
A|all)
  run e11b_a1 $D/specs/e11b_a1.json
  for p in "mitm-half9 mitm-default9" "f4-half9 f4-default9" "mitm-default9b mitm-default9"; do set -- $p
    $B compare --dir $D/sessions/e11b_a1 --a $2 --b $1 --save > $D/sessions/e11b_a1.compare.$1.vs.$2.txt 2>&1
  done;;&
B|all)
  for bits in ${BITS:-20 24 28 32}; do
    run tune_j0_$bits $D/specs/B/tune_j0_$bits.json
    run tune_rand_$bits $D/specs/B/tune_rand_$bits.json
    cp $D/sessions/tune_j0_$bits.band.json $D/sessions/tune_rand_$bits.band.json $D/specs/B/ 2>/dev/null
    if [ ! -f $D/specs/B/sizes_$bits.json ]; then
      sz=$($T pick $bits $D/sessions 2> $D/sessions/pick_$bits.log) && $T spec-test $bits $sz $D/specs/B >/dev/null
    fi
    for t in j0 randA randB; do run test_${t}_$bits $D/specs/B/test_${t}_$bits.json; done
  done;;
esac
echo "== done $(date -u +%FT%TZ)"
