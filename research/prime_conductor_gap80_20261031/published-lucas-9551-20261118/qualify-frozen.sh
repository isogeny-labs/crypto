#!/bin/sh
# Fixed published proof qualification only; no benchmark or parameter search.
set -eu
repo=$1
oracle=$2
gp=$3
out=$4
cd "$repo"
study=research/prime_conductor_gap80_20261031/published-lucas-9551-20261118
test ! -e "$out"
mkdir -p "$out"
timeout 30 "$oracle" > "$out/ring.jsonl" 2> "$out/ring.stderr"
test ! -s "$out/ring.stderr"
cd "$out"
if timeout 300 "$gp" -s 512M -fq < "$repo/$study/check-fixed.gp" > certificate.stdout 2> certificate.stderr; then certificate_status=0; else certificate_status=$?; fi
printf '%s\n' "$certificate_status" > certificate.exit
test "$certificate_status" -eq 0
test ! -s certificate.stderr
test -s conductor.cert
if timeout 60 "$gp" -s 512M -fq < "$repo/$study/replay.gp" > replay.stdout 2> replay.stderr; then replay_status=0; else replay_status=$?; fi
printf '%s\n' "$replay_status" > replay.exit
test "$replay_status" -eq 0
test ! -s replay.stderr
python3 - "$repo/$study" <<'PY'
# Metadata equality only: all mathematical work was done in native GP/C++.
import json,pathlib,sys
study=pathlib.Path(sys.argv[1]);out=pathlib.Path.cwd()
rows=[json.loads(line) for line in (out/'ring.jsonl').read_text().splitlines()]
a=json.loads((out/'arithmetic.json').read_text());done=json.loads((out/'certificate.stdout').read_text());replay=json.loads((out/'replay.stdout').read_text())
assert [row['n'] for row in rows]==[4,7,571,7351,9551]
assert rows[-1]['trace']==a['trace'] and rows[-1]['frobenius_conductor']==a['frobenius_conductor']
assert rows[-1]['conductor_bits']==a['conductor_bits']
prior=json.loads((study.parent/'published-lucas-20261118/attempt-a/arithmetic.json').read_text())
assert rows[-2]['trace']==prior['trace'] and rows[-2]['frobenius_conductor']==prior['frobenius_conductor']
assert a['n']==9551 and a['p']==2 and a['D_K']==-7
assert a['recurrence_matrix_agree'] and a['discriminant_identity']
assert all(done[key] for key in ('arithmetic_complete','certificate_generated','certificate_valid'))
assert all(replay[key] for key in ('subject_matches','certificate_replay_accept','changed_subject_reject'))
assert all(a[key] is None for key in ('quotient','point_images','endpoint_timings'))
print(json.dumps({'qualification_passed':True,'fixed_published_index':9551,'conductor_bits':a['conductor_bits'],'decimal_digits':len(a['frobenius_conductor']),'certificate_generated':True,'certificate_replayed':True,'changed_subject_reject':True,'native_oracles_match':True,'source_endomorphism_conductor':1,'lower_endpoint':None,'kernel':None,'isogeny':None,'point_images':None,'endpoint_timings':None,'benchmark_session_executed':False,'world_record_claim':None,'independent_signed_execution':False}))
PY
