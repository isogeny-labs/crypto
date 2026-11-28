#!/usr/bin/env bash
set -eu
repo=$1
binary=$2
out=$3
cd "$repo"
study=research/prime_conductor_gap80_20261031/construction-audit-20261118
input=research/prime_conductor_gap80_20261031/wide-harness-20261118/inputs/target80-source.txt
mkdir "$out"
sha256sum -c "$study/pre-call.sha256" > "$out/pre-call.stdout" 2> "$out/pre-call.stderr"
if timeout 120 "$binary" < "$input" > "$out/positive.json" 2> "$out/positive.stderr"; then positive_status=0; else positive_status=$?; fi
printf '%s\n' "$positive_status" > "$out/positive.exit"
test "$positive_status" -eq 0
test ! -s "$out/positive.stderr"
if timeout 120 "$binary" --dependent-pair < "$input" > "$out/dependent.stdout" 2> "$out/dependent.stderr"; then dependent_status=0; else dependent_status=$?; fi
printf '%s\n' "$dependent_status" > "$out/dependent.exit"
test "$dependent_status" -eq 2
test ! -s "$out/dependent.stdout"
test "$(cat "$out/dependent.stderr")" = "dependent torsion pair rejected"
python3 - "$out" <<'PY'
import json,pathlib,sys
p=pathlib.Path(sys.argv[1]);d=json.loads((p/'positive.json').read_text())
assert d['weil_pairing_exact_prime_order'] and d['kernel_cm_stable'] is False
assert len(d['kernel_basis'])==2 and len(d['cm_branches'])==2
for b in d['cm_branches']:
    assert len(b['samples'])==32 and {r['scalar'] for r in b['samples']}==set(range(1,33))
assert [r['m'] for r in d['full_two_power_torsion_orders']]==list(range(1,81))
assert all(r['matrix_check'] for r in d['full_two_power_torsion_orders'])
assert all(d[k] is None for k in ['target_quotient','target_non_kernel_images','endpoint_timings'])
print(json.dumps({'positive_complete':True,'dependent_pair_rejected':True,'cm_branch_sample_rows':64,'native_pari_cm_images':128,'two_power_order_levels':80,'target_map_evaluated':False}))
PY
sha256sum -c "$study/pre-call.sha256" > "$out/post-call.stdout" 2> "$out/post-call.stderr"
