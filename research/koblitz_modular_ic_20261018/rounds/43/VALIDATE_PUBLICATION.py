#!/usr/bin/env python3
"""Historical source-byte and terminal-receipt consistency, not math replay."""
from pathlib import Path
import hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
m=json.loads((h/'raw-meta.json').read_text());rev=m['source_revision'];pins=[]
for line in (h/'source.sha256').read_text().splitlines():
 digest,name=line.split(maxsplit=1);p=(h/name).resolve();assert hashlib.sha256(p.read_bytes()).hexdigest()==digest
 rel=str(p.relative_to(repo));r=subprocess.run(['git','show',rev+':'+rel],cwd=repo,capture_output=True)
 if r.returncode==0:
  assert hashlib.sha256(r.stdout).hexdigest()==digest;method='historical_git_blob'
 else:
  assert name=='../34/source-data/proof-cache-source-roots.txt'
  expected=dict((v,k) for k,v in (line.split(maxsplit=1) for line in (h/'../34/runs.sha256').read_text().splitlines()))
  assert expected['source-data/proof-cache-source-roots.txt']==digest;method='sealed_R34_input_manifest'
 pins.append({'path':name,'sha256':digest,'verified_against':method})
r=json.loads((h/'results.json').read_text());assert sum(x.get('audited_declarations',0) for x in r['runs'])==6
for run in r['runs']:
 p=h/'runs'/run['attempt']
 if run['status']=='accepted':
  v=subprocess.run(['sha256sum','-c',str(p/'source.sha256')],cwd=p,capture_output=True,text=True)
  assert v.returncode==0,(run['attempt'],v.stderr)
 if run['kind']=='Lean' and run['status']=='accepted':
  assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
  assert '0 failing, 0 trusted' in (p/'audit.log').read_text()
assert len(pins)==m['source_pins']
for stage in r['product_stages']:
 p=h/'runs/multiply43-01/output';assert (p/(stage['label']+'-coordinates.bin')).stat().st_size==400*stage['attempted_products']
 assert len((p/(stage['label']+'-basis.txt')).read_text().splitlines())==stage['output_rank']
metrics=json.loads((h/'metrics-validation.json').read_text());assert len(metrics['committed_ref_rows'])==8 and metrics['command_completed']
v=json.loads((h/'artifact-verification.json').read_text());assert v['expected_sha256']==m['raw_sha256'] and v['all_member_hashes_verified'] and v['deterministic_repack_verified']
out={'schema':'koblitz.publication-validation/v1','round':43,'source_revision':rev,'source_pins_verified':len(pins),'pins':pins,'accepted_attempt_source_hashes_verified':True,'terminal_receipt_consistency':True,'independent_scientific_replay':False,'independent_semantic_review':False}
(h/'publication-validation.json').write_text(json.dumps(out,indent=2)+'\n');print('Verified',len(pins),'historical source pins, all accepted attempt source hashes, and publication/receipt consistency.')
