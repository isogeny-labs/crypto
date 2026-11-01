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
r=json.loads((h/'results.json').read_text());assert sum(x.get('audited_declarations',0) for x in r['runs'])==11
for run in r['runs']:
 p=h/'runs'/run['attempt']
 if run['status'] in ('accepted','partial'):
  v=subprocess.run(['sha256sum','-c',str(p/'source.sha256')],cwd=p,capture_output=True,text=True)
  assert v.returncode==0,(run['attempt'],v.stderr)
 if run['kind']=='Lean' and run['status']=='accepted':
  assert (p/'compile.exit').read_text().strip()=='0' and (p/'audit.exit').read_text().strip()=='0'
  assert '0 failing, 0 trusted' in (p/'audit.log').read_text()
assert len(pins)==m['source_pins']
assert len(r['source_blocks'])==6 and len(r['independent_trace_cases'])==3 and len(r['retained_source_filter_controls'])==20
partial=h/'runs/point436-03';completion=h/'runs/resume-twist436-01'
assert(partial/'exit.txt').read_text().strip()=='124'
assert 'twist=1 stage=full_source_checks_complete' in(partial/'native.log').read_text()
for name in ['input.sing','native.log','exit.txt','source.sha256','source-revision.txt','started-utc.txt','finished-utc.txt']:
 assert(partial/name).read_bytes()==(completion/'prior-source-receipts'/name).read_bytes()
for name in ['dependency-receipts.sha256','prior-source-receipts.sha256']:
 for p in(h/'runs').glob('*/'+name):
  check=subprocess.run(['sha256sum','-c',str(p)],capture_output=True,text=True);assert check.returncode==0,(str(p),check.stderr)
metrics=json.loads((h/'metrics-validation.json').read_text());assert len(metrics['committed_ref_rows'])==4 and metrics['head_ref_exit']==0 and metrics['default_ref_exit']==0 and metrics['pre_existing_rows_byte_equivalent']==604
v=json.loads((h/'artifact-verification.json').read_text());assert v['expected_sha256']==m['raw_sha256'] and v['all_member_hashes_verified'] and v['deterministic_repack_verified']
out={'schema':'koblitz.publication-validation/v1','round':44,'source_revision':rev,'source_pins_verified':len(pins),'pins':pins,'accepted_attempt_source_hashes_verified':True,'terminal_receipt_consistency':True,'independent_scientific_replay':False,'independent_semantic_review':False}
(h/'publication-validation.json').write_text(json.dumps(out,indent=2)+'\n');print('Verified',len(pins),'historical source pins, all accepted attempt source hashes, and publication/receipt consistency.')
