"""Package frozen native V3 output and execution receipts; no algebra is computed."""
from pathlib import Path
import copy
import hashlib
import json
import shutil
import sys
s=Path(__file__).resolve().parent;raw=Path(sys.argv[1]);e=s/'evidence/degree3'
def read(p):return json.loads(p.read_text())
def art(rel,kind='raw'):
 p=s/rel;h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(4*1024*1024),b''):h.update(b)
 return dict(path=rel,kind=kind,bytes=p.stat().st_size,sha256=h.hexdigest())
assert not e.exists();assert read(raw/'terminal.json')['exit_code']==0
proto=read(s/'protocol-degree3.json');summary=read(raw/'summary.json');assert len(summary['rows'])==8
assert len(list((raw/'status').glob('*.json')))==8
e.mkdir()
for name in ['summary.json','terminal.json','host.txt','manifest.sha256','controls.json']:
 shutil.copy2(raw/name,e/name)
shutil.copytree(raw/'status',e/'status');shutil.copytree(raw/'replay',e/'replay')
(e/'results').mkdir()
for row in summary['rows']:
 p=raw/'cases'/row['key']/'result.json'
 if p.exists():shutil.copy2(p,e/'results'/f"{row['key']}.json")
out=read(s/'experiment-pdp.export.json');template=copy.deepcopy(out['runs'][0]);out['export_id']=proto['protocol_id']
out['protocol']=dict(path='protocol-degree3.json',protocol_id=proto['protocol_id'],sha256=proto['freeze']['protocol_sha256'],version=1)
out['source']=dict(repo='crypto',commit=proto['freeze']['commit'],dirty=False);out['producer']['session']='n83-degree3-paired-20261028-r1'
out['arms']=[dict(id=a['id'],role=a['role'],commit=a['instrument']['commit'],binary_sha256=a['instrument']['binary_sha256'],image_digest=None) for a in proto['arms']]
out['host']['path']='evidence/degree3/host.txt';out['host']['sha256']=art(out['host']['path'])['sha256']
out['analysis']=dict(script=art('src/bin/degree3.rs','analysis'),outputs=[art('evidence/degree3/summary.json','summary')],blinded=False,blinding_key_sha256=None,unmasked_at=None)
out['files']=[art(str(p.relative_to(s))) for p in sorted(e.rglob('*')) if p.is_file()]
archives=[art(str(p.relative_to(s))) for p in sorted((s/'degree3-archives').glob('*.tar.xz'))]
assert len(archives)==9
out['files']+=archives+[art('DEGREE3_FORMAT.json'),art('DEGREE3_DESIGN.md')]
out['runs']=[]
for r in summary['rows']:
 row=copy.deepcopy(template);ok=r['verified'];status=r['status'];timeout=124 in [status['run_exit_code'],status['replay_exit_code']]
 row.update(run_id=r['key'],arm=r['arm']+'-'+r['family'],instance_id=str(int(r['target_kind']=='natural')),status='succeeded' if ok else 'timeout' if timeout else 'failed',outcome_class='completed' if ok else 'not_completed',backend_ref=proto['extensions']['raw_prefix']+'cases/'+r['key'],artifacts=[art('evidence/degree3/status/'+r['key']+'.json')])
 row['verification']=dict(status='verified' if ok else 'no_claim',verifier='Full V2 and V3 source/row/queue replay, strict V2 nonmembership and global upper checks; producer implementation, independent research review pending')
 row['measurements']['counted']={k:r[k] for k in ['last_fall_degree','last_fall_lower_bound','last_fall_upper_bound','closure_rank','linear_rank'] if ok and r[k] is not None}
 out['runs'].append(row)
complete=sum(r['verified'] for r in summary['rows']);out['flow']=dict(planned=8,attempted=8,completed=complete,not_completed=8-complete,interrupted=0,resumed=0,excluded=[])
out['notes']=[dict(by='producer',kind='observation',text=t) for t in ['Eight preserved m2 systems; each production and replay first verifies the complete saved V2 trace.','Numerical degree claims are retained only for successful production plus fresh replay. Partial traces and errors remain archived.','LFD3 requires both a strict nonmembership witness and a global upper certificate. This stage measures no regularity or intermediate solving degree.','Three native algebra controls and four unit tests pass. Producer replay is not independent research review.']]
(s/'experiment-degree3.export.json').write_text(json.dumps(out,indent=2)+'\n')
index={'schema':'n83.degree-three-archive-index/v1','raw_prefix':proto['extensions']['raw_prefix'],'archives':archives,'reconstruction':'Extract every case archive and metadata.tar.xz into one fresh root, then sha256sum -c manifest.sha256. Case archives contain cases/KEY; metadata archive contains everything else.','format':'DEGREE3_FORMAT.json','source_uri':'s3://setec-astronomy-artifacts/runs/n83-degree3-paired-20261028-r1/source/frozen-source.tar.xz','source_sha256':'448bc2d5efb6d89e8273d470efdb0ff412cdacecfafd63f77de27ac95bb3b439','frozen_source_commit':'4955822af2f805eb6046dcfe2fb775bdf52a0293'}
(s/'evidence/degree3-storage.json').write_text(json.dumps(index,indent=2)+'\n')
print(json.dumps({'cases':8,'verified':complete,'not_completed':8-complete,'archives':len(archives)}))
