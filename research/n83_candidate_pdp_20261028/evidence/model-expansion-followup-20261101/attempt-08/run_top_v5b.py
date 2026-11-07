#!/usr/bin/env python3
"""Compute and replay top-form interval certificates for the frozen full-domain panel."""
import hashlib
import json
import subprocess
import time
from pathlib import Path

study=Path(__file__).resolve().parents[3]
attempt=Path(__file__).resolve().parent
protocol=study/'protocol-model-full-domain-v5.json'
protocol_sha='3a041db64c7d6d70aa99200fe46500770998de5faffaf2acb7a2e366c855eb57'
binary=study/'target/release/top'
binary_sha='4749b49ba7e98cd916c9d36d4b14c0396041fd6ff31e3708e17e7a91749fe53a'
assert hashlib.sha256(protocol.read_bytes()).hexdigest()==protocol_sha
assert hashlib.sha256(binary.read_bytes()).hexdigest()==binary_sha
toproot=attempt/'top'
if toproot.exists():raise SystemExit(f'preserving existing top output: {toproot}')
toproot.mkdir()
records=[]
for bit in (35,36,37):
 candidate=f'spike{bit}'; inputs=attempt/'inputs'/candidate; certdir=toproot/candidate/'certificates'
 certdir.parent.mkdir()
 rec={'candidate_id':candidate,'coordinate_bit':bit,
      'input_index_sha256':hashlib.sha256((inputs/'inputs.json').read_bytes()).hexdigest(),
      'input_rows':48}
 for mode,cmd in [('produce',[str(binary),'single',str(inputs),str(certdir)]),
                  ('replay',[str(binary),'single-replay',str(inputs),str(certdir)])]:
  log=certdir.parent/f'{mode}.log'; t=time.time()
  result=subprocess.run(cmd,cwd=study.parent.parent,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
  log.write_bytes(result.stdout)
  rec[f'{mode}_exit_code']=result.returncode
  rec[f'{mode}_elapsed_seconds_diagnostic_only']=time.time()-t
  rec[f'{mode}_log_sha256']=hashlib.sha256(log.read_bytes()).hexdigest()
  if result.returncode!=0:break
 if rec.get('produce_exit_code')==0 and rec.get('replay_exit_code')==0:
  summary=json.loads((certdir/'summary.json').read_text())
  replay=json.loads((certdir.parent/'replay-summary.json').read_text())
  assert len(summary['rows'])==48 and len(replay['rows'])==48 and replay['all_verified']
  byid={x['id']:x for x in summary['rows']}
  for item in json.loads((inputs/'inputs.json').read_text()):
   cert=json.loads((certdir/f"{item['id']}.json").read_text())
   assert cert['source_sha256']==hashlib.sha256((inputs/item['id']/'system.sing').read_bytes()).hexdigest()
   assert byid[item['id']]['source_sha256']==cert['source_sha256']
  rec.update({'certificates':48,'all_replays_verified':True,
              'summary_sha256':hashlib.sha256((certdir/'summary.json').read_bytes()).hexdigest(),
              'replay_summary_sha256':hashlib.sha256((certdir.parent/'replay-summary.json').read_bytes()).hexdigest()})
 else:
  rec['failure']='producer or replay failed; outputs and logs retained; this candidate will not be overwritten'
 records.append(rec)
 (certdir.parent/'run.json').write_text(json.dumps(rec,indent=2)+'\n')
 print(json.dumps(rec),flush=True)
 if 'failure' in rec:break
rows=[]
for rec in records:
 if rec.get('all_replays_verified'):
  rows.extend(json.loads(((toproot/rec['candidate_id'])/'certificates/summary.json').read_text())['rows'])
result={'schema':'n83.full-domain-new-base-top-labels/v1','protocol_sha256':protocol_sha,
        'top_binary_sha256':binary_sha,'planned':144,'completed':len(rows),
        'replayed_and_verified':sum(r.get('all_replays_verified',False) for r in records)*48,
        'all_verified':len(records)==3 and len(rows)==144 and all(r.get('all_replays_verified') for r in records),
        'records':records,
        'scope':'Top-form degree interval certificates only; not exact degree of regularity, affine solving degree, FFD, or LFD.'}
(toproot/'top-summary.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k!='records'},indent=2))
if not result['all_verified']:raise SystemExit(1)
