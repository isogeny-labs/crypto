#!/bin/bash
# Run this whole bounded native correctness qualifier under timeout 120.
set -eu
study=research/prime_conductor_gap80_20261031/fixture-conductor-audit-20261118
gp=$1
out=$2
test ! -e "$out"
mkdir -p "$out"
CONDUCTOR_FIXTURES="$study/inputs.gpvec" "$gp" -s 128M -fq "$study/check-v2.gp" </dev/null > "$out/positive.jsonl" 2> "$out/positive.stderr"
test ! -s "$out/positive.stderr"
for label in degree_is_not_gap composite_degree wrong_kernel wrong_codomain wrong_image supersingular; do
 if CONDUCTOR_FIXTURES="$study/negative-$label.gpvec" "$gp" -s 128M -fq "$study/check-v2.gp" </dev/null > "$out/$label.stdout" 2> "$out/$label.stderr"; then code=0; else code=$?; fi
 printf '%s\n' "$code" > "$out/$label.exit"
 test "$code" -eq 2
 test ! -s "$out/$label.stderr"
 rg -q '^FAIL: ' "$out/$label.stdout"
done
python3 - "$out" <<'PY'
# Serialization/receipt checks only; all new algebra ran in native GP.
import json,pathlib,sys
out=pathlib.Path(sys.argv[1]);rows=[json.loads(l) for l in (out/'positive.jsonl').read_text().splitlines()]
assert [r['case'] for r in rows]==['degree19','degree10009','field202-degree2','field96-degree5']
assert [r['conductor_ratio'] for r in rows]==['1','1','2','1']
assert all(r['ordinary'] and r['kernel_intersection_trivial'] and r['runtime_measurements'] is None and not r['ECDLP_solved'] for r in rows)
assert rows[1]['source_end_conductor'] in ['1','3'] and rows[1]['destination_end_conductor']==rows[1]['source_end_conductor']
assert rows[2]['source_end_conductor']=='1' and rows[2]['destination_end_conductor']=='2'
print(json.dumps({'scope':'four retained fixtures; native correctness/certification only','rows':rows,'direct_map_inputs_verified':sum(r['direct_map_inputs_verified'] for r in rows),'matched_scalars_verified':sum(r['matched_scalars_verified'] for r in rows),'negative_admissions':6,'largest_certified_prime_conductor_ratio_in_this_panel':'2','original_80bit_crossing_complete':False,'world_record_established':False,'runtime_measurements':None,'new_ECDLP_solve':False,'independent_signed_verdict':False}))
PY
