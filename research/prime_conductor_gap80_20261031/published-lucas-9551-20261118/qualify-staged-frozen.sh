#!/bin/sh
set -eu
repo=$1
gp=$2
out=$3
cd "$repo"
study=research/prime_conductor_gap80_20261031/published-lucas-9551-20261118
test ! -e "$out"
mkdir -p "$out"
cd "$out"
if timeout 600 "$gp" -s 512M -fq < "$repo/$study/check-staged.gp" > certificate.stdout 2> certificate.stderr; then certificate_status=0; else certificate_status=$?; fi
printf '%s\n' "$certificate_status" > certificate.exit
test "$certificate_status" -eq 0
test ! -s certificate.stderr
test -s conductor.cert
if timeout 60 "$gp" -s 512M -fq < "$repo/$study/replay.gp" > replay.stdout 2> replay.stderr; then replay_status=0; else replay_status=$?; fi
printf '%s\n' "$replay_status" > replay.exit
test "$replay_status" -eq 0
test ! -s replay.stderr
python3 - "$repo/$study" <<'PY'
import json,pathlib,sys
study=pathlib.Path(sys.argv[1]);out=pathlib.Path.cwd()
prior=json.loads((study/'attempt-a/arithmetic.json').read_text())
a=json.loads((out/'arithmetic.json').read_text())
assert a==prior
rows=[json.loads(line) for line in (study/'attempt-a/ring.jsonl').read_text().splitlines()]
assert rows[-1]['trace']==a['trace'] and rows[-1]['frobenius_conductor']==a['frobenius_conductor']
assert rows[-1]['conductor_bits']==a['conductor_bits']
done=json.loads((out/'certificate.stdout').read_text());replay=json.loads((out/'replay.stdout').read_text())
assert all(done[key] for key in ('arithmetic_complete','certificate_generated','certificate_valid'))
assert all(replay[key] for key in ('subject_matches','certificate_replay_accept','changed_subject_reject'))
stages=[json.loads(line) for line in (out/'stages.jsonl').read_text().splitlines()]
assert [row['threshold_bits'] for row in stages]==[4000,3000,2000,1000]
assert all(row['partial_saved'] and not row['full_prime_claim'] for row in stages)
print(json.dumps({'qualification_passed':True,'fixed_published_index':9551,'conductor_bits':a['conductor_bits'],'decimal_digits':len(a['frobenius_conductor']),'certificate_generated':True,'certificate_replayed':True,'changed_subject_reject':True,'native_oracles_match':True,'source_endomorphism_conductor':1,'lower_endpoint':None,'kernel':None,'isogeny':None,'point_images':None,'endpoint_timings':None,'benchmark_session_executed':False,'world_record_claim':None,'independent_signed_execution':False}))
PY
