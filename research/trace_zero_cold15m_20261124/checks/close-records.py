#!/usr/bin/env python3
"""Benchmark metadata only; copy Rust counts without scientific analysis."""
from pathlib import Path
import json,subprocess
p=Path('research/trace_zero_cold15m_20261124');entry='benchmarks/20261124-ecc2k-cold15m';records=[]
def tool(*args):subprocess.run(['python3','tools/benchrec.py',*map(str,args)],check=True)
for b in [p,p/'candidate2',p/'candidate3']:
 for phase in ['preflight','primary']:
  r=(b/'checks'/f'{phase}-record.path').read_text().strip();records.append(r)
  rec=json.loads(Path(r).read_text());assert rec['status']=='open',(r,rec['status'])
  if phase=='primary':
   s=json.loads((b/'summary.json').read_text())['cells'];c=s['scale-n131-d12-uniform/elimination']['metrics'];a=s['scale-n131-d12-uniform/baseline']['metrics'];n='5'
   pairs=[('complete_cold_query_ir','instructions'),('complete_oracle_ir','oracle_instructions')]
   raw=[b/'rows.jsonl',b/'summary.json',b/'grid-audit.json',b/'polynomial-replay.json']
  else:
   s=json.loads((b/'preflight.json').read_text())['rows'];c=next(x['metrics'] for x in s if x['arm']=='elimination' and x['instance']=='scale-n131-d12-uniform');a=next(x['metrics'] for x in s if x['arm']=='baseline' and x['instance']=='scale-n131-d12-uniform');c={k:{'median':v,'min':v,'max':v} for k,v in c.items()};a={k:{'median':v} for k,v in a.items()};n='1';pairs=[('pilot_complete_cold_query_ir','instructions')];raw=[b/'preflight.json',b/'calibration.json',b/'preflight-gate.json']
  tool('add-raw',r,*raw,p/'RAW-ALL.tar.xz',p/'FROZEN-EXECUTABLES.tar.xz')
  for name,field in pairs:
   x=c[field];tool('metric',r,'--name',name,'--value',x['median'],'--unit','ir','--n',n,'--spread-low',x['min'],'--spread-high',x['max'],'--baseline',a[field]['median'],'--note','Exact copied Rust result; complete cold/oracle boundary. No native-time or instruction-rate claim. Remote raw custody pending, uploaded=false.')
  text={'': 'First candidate misses15M and one n83 oracle guard; all raw rows retained.', 'candidate2':'Second candidate misses15M; six control guards pass.', 'candidate3':'Third candidate primary satisfies strict15M and all six controls; pilot is provisional only.'}[b.name if b!=p else '']
  tool('close',r,'--status','measured','--report',p/'report.pdf','--svg',p/'instructions.svg','--png',p/'instructions.png','--entry',entry,'--summary',text+' Unsigned producer checks, remote custody and independent admission remain open.')
records.extend(['benchmarks/records/20261124T0341Z-ecc2k-cold15m-candidate3-preflight.json','benchmarks/records/20261124T0344Z-ecc2k-cold15m-candidate3-corrected-preflight.json'])
assert all(json.loads(Path(r).read_text())['status']!='open' for r in records)
(p/'checks/owned-record-closure.json').write_text(json.dumps({'records':records,'all_eight_closed':True,'remote_upload_established':False},indent=2)+'\n')
