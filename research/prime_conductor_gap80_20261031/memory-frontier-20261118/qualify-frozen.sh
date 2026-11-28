#!/bin/sh
set -eu
repo=$1
audit=$2
layout=$3
out=$4
cd "$repo"
study=research/prime_conductor_gap80_20261031/memory-frontier-20261118
test ! -e "$out"
mkdir -p "$out"
"$audit" --self-test > "$out/self-test.json" 2> "$out/self-test.stderr"
test ! -s "$out/self-test.stderr"
"$layout" research/prime_conductor_gap80_20261031/evaluator-20261118/inputs > "$out/layout.json" 2> "$out/layout.stderr"
test ! -s "$out/layout.stderr"
ell=729845429276318409687041
cap=137438953472
for label in balanced zero-leftover; do
 if [ "$label" = balanced ]; then set --; else set -- 414464344064; fi
 "$audit" "$ell" 6000000 "$cap" "$@" > "$out/$label.json" 2> "$out/$label.stderr"
 test ! -s "$out/$label.stderr"
done
"$audit" 10009 64 808 > "$out/root-only-counterexample.json" 2> "$out/root-only-counterexample.stderr"
"$audit" 5 2 40 > "$out/small-positive.json" 2> "$out/small-positive.stderr"
for label in even degree block cap window; do
 case $label in even) set -- 6 1 40;; degree) set -- 3 1 40;; block) set -- 5 0 40;; cap) set -- 5 1 0;; window) set -- 5 1 40 0;; esac
 if "$audit" "$@" > "$out/negative-$label.stdout" 2> "$out/negative-$label.stderr"; then result=0; else result=$?; fi
 printf '%s\n' "$result" > "$out/negative-$label.exit"
 test "$result" -eq 2
 test ! -s "$out/negative-$label.stdout"
done
python3 - "$out" <<'PY'
# Equality of serialized native fields only; no research arithmetic.
import json,pathlib,sys
out=pathlib.Path(sys.argv[1]);t=json.loads((out/'self-test.json').read_text());l=json.loads((out/'layout.json').read_text())
assert t['optimizer_equals_exhaustive'] and t['all_bounds_hold'] and t['half_kernel_sizes']==2047
assert l['native_layout_checks']==9 and l['slot_bytes']==8
rows=[json.loads((out/(label+'.json')).read_text()) for label in ('balanced','zero-leftover')]
for d in rows:
 assert d['ell']=='729845429276318409687041' and d['slot_bytes']==8
 assert not d['launch_admitted'] and d['runtime_measurements'] is None
assert rows[0]['all_window_single_rectangle_minimum_bytes']==rows[1]['all_window_single_rectangle_minimum_bytes']
assert rows[0]['all_window_contexts_minimum_under_cap']==rows[1]['all_window_contexts_minimum_under_cap']
r=json.loads((out/'root-only-counterexample.json').read_text());p=json.loads((out/'small-positive.json').read_text())
assert r['baby_root_phase_slots']=='101' and r['giant_product_phase_slots']=='152' and not r['chosen_lower_bound_fits_cap']
assert p['chosen_lower_bound_fits_cap'] and not p['launch_admitted']
print(json.dumps({'native_integer_controls':t,'native_layout_checks':l,'target_cases':rows,'counterexample_pass':True,'small_positive_is_not_admission':True,'negative_cli_controls':5,'benchmark_session_executed':False,'target_quotient':None,'point_images':None,'endpoint_times':None}))
PY
