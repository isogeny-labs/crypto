"""Package native paired results and execution receipts; no algebra is computed here."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
import sys
study=Path(__file__).resolve().parent
raw=Path(sys.argv[1]); evidence=study/'evidence/pdp'
assert not evidence.exists()
def read(p):return json.loads(p.read_text())
def write(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def art(rel,kind='raw'):
 p=study/rel; h=hashlib.sha256()
 with p.open('rb') as stream:
  for b in iter(lambda:stream.read(4*1024*1024),b''):h.update(b)
 return dict(path=rel,kind=kind,bytes=p.stat().st_size,sha256=h.hexdigest())
assert read(raw/'terminal.json')['exit_code']==0
proto=read(study/'protocol-pdp.json')
paired=read(raw/'paired-summary.json');assert paired['systems']==96 and len(paired['rows'])==48
for arm in ['baseline','spike40']:
 statuses=list((raw/arm/'status').glob('*.json'));assert len(statuses)==80
 for p in statuses:
  status=read(p)
  for key in (['exit_code'] if p.name.endswith('.first-fall.json') else ['closure_exit_code','replay_exit_code']):assert status[key]==0,(p,key,status)
evidence.mkdir()
for rel in ['terminal.json','host.txt','manifest.sha256','paired-summary.json','first-fall-controls.json']:
 shutil.copy2(raw/rel,evidence/rel)
for arm in ['baseline','spike40']:
 (evidence/arm).mkdir()
 for source,target in [('summary.json','summary.json'),('inputs/inputs.json','inputs.json'),('catalog/index.json','catalog-index.json')]:shutil.copy2(raw/arm/source,evidence/arm/target)
 shutil.copytree(raw/arm/'status',evidence/arm/'status')
old=read(study.parent/'n83_fixed_fall_catalog_20261028/experiment.export.json');template=copy.deepcopy(old['runs'][0]);out=old
out.update(export_id=proto['protocol_id'],source=dict(commit=proto['freeze']['commit'],dirty=False,repo='crypto'),protocol=dict(path='protocol-pdp.json',protocol_id=proto['protocol_id'],version=1,sha256=proto['freeze']['protocol_sha256']))
out['producer']['session']='n83-candidate-paired-falls-20261028-r1'
out['host']['path']='evidence/pdp/host.txt';out['host']['sha256']=art(out['host']['path'])['sha256']
out['arms']=[dict(id=a['id'],role=a['role'],commit=a['instrument']['commit'],binary_sha256=a['instrument']['binary_sha256'],image_digest=None) for a in proto['arms']]
out['files']=[art(str(p.relative_to(study))) for p in sorted(evidence.rglob('*')) if p.is_file()]
out['files'] += [art('paired-runs.tar.xz'),art('PDP_FORMAT.json'),art('PDP_COMPONENT_FORMAT.json')]
out['analysis']=dict(script=art('src/bin/compare.rs','analysis'),outputs=[art('evidence/pdp/paired-summary.json','summary')],blinded=False,blinding_key_sha256=None,unmasked_at=None)
out['runs']=[]
for arm in ['baseline','spike40']:
 for r in read(evidence/arm/'summary.json')['rows']:
  row=copy.deepcopy(template);instance=(r['m']-2)*2+(r['target_kind']=='natural')
  row.update(arm=arm+'-'+r['family'],instance_id=str(instance),run_id=arm+'-'+r['id'],backend_ref=proto['extensions']['raw_prefix']+arm+'/first-fall/'+r['id']+'.json',artifacts=[art('evidence/pdp/'+arm+'/summary.json','summary')])
  row['measurements']['counted']={k:v for k,v in {'first_fall_degree':r['first_fall']['first_fall_degree'],'first_fall_lower_bound':r['first_fall']['first_fall_lower_bound'],'last_fall_degree':r['last_fall_degree'],'closure_rank':r['closure_rank'],'linear_rank':r['linear_rank']}.items() if v is not None}
  out['runs'].append(row)
out['flow']=dict(planned=96,attempted=96,completed=96,not_completed=0,interrupted=0,resumed=0,excluded=[])
out['notes']=[dict(by='producer',kind='observation',text=t) for t in ['96 first-fall certificates and96 fresh replays;64 binary V2 closures and64 fresh replays;96 equation catalogs. Every recorded exit code is zero.','Paired native summary verifies identical targets, tuples and witnesses for each48-case pair. Natural baseline source hashes match original fixed-base systems.','All degree assertions are input-presentation-specific. FFD/LFD do not substitute for regularity or solving degree. No elite or ECDLP claim.','Common-support planted tuples are restricted to low26 coordinates; no uniform full-factor-base planted distribution is claimed.']]
write(study/'experiment-pdp.export.json',out)
print('Packaged96 case rows with160 successful producer replays; all native fields forwarded')
