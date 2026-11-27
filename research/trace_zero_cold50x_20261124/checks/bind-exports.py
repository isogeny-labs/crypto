#!/usr/bin/env python3
"""Bind observed run/source identities; copy frozen Rust counts without analysis."""
from pathlib import Path
import json,importlib.util,sys
root=Path.cwd();p=Path('research/trace_zero_cold50x_20261124')
spec=importlib.util.spec_from_file_location('experiment_protocol','/workspace/scratch/protocol-tools/tools/experiment_protocol.py');m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)
for base,rows,name,outputs in [(p,'v2-rows.jsonl','v2-export.json',['v2-summary.json','v2-grid-audit.json','v2-polynomial-replay.json','checks/v2-source-final.json']),(p/'inline','rows.jsonl','export.json',['summary.json','grid-audit.json','polynomial-replay.json','checks/source-final.json'])]:
 q=json.loads((base/'protocol.json').read_text());e=m.from_rows(base/rows,q,'protocol.json',q['protocol_id'],{'instance_id':'instance'})
 for a,b in zip(e['arms'],q['arms']):
  assert a['id']==b['id'];a.update({k:b['instrument'][k] for k in ['commit','binary_sha256']})
 e['source'].update(commit=q['arms'][0]['instrument']['commit'],dirty=False);e['producer'].update(session='codex-ecc2k-cold50x-20261124',runtime='Codex',role='executor');e['backend']='manual';e['host']['summary']['cpu_model']='INTEL(R) XEON(R) PLATINUM 8573C'
 a=q['analysis_plan']['script'];e['analysis']['script']=m.file_entry(base,a['path'],'analysis');assert e['analysis']['script']['sha256']==a['sha256'];e['analysis']['outputs']=[m.file_entry(base,f,'summary') for f in outputs]
 e['files'] += [m.file_entry(base,f,'summary') for f in outputs]
 e['notes']=[{'kind':'observation','text':'Verification fields record producer native-output and separate schoolbook replay checks, not independently signed admission. All120 planned rows retained, no exclusions/retries; whole cold boundary includes parsing/setup/admission/oracle/replay/output/exit. Source commit names the exact numeric/harness freeze; later metadata-only publication is documented in the journal.50x target remains unmet; no native-clock or warmed-throughput result.','by':'codex-ecc2k-cold50x-20261124'}]
 out=base/name
 if out.exists():raise SystemExit('refuse replacing existing final export: '+str(out))
 out.write_text(json.dumps(e,indent=2)+'\n')
