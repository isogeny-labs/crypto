"""Forward completed native certificates/replays into the protocol export; no algebra."""
from pathlib import Path
import copy,hashlib,json,shutil,sys
s=Path(__file__).resolve().parent;raw=Path(sys.argv[1]);e=s/'evidence/top'
read=lambda p:json.loads(p.read_text())
def art(rel,kind='raw'):
 p=s/rel;h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(4*1024*1024),b''):h.update(b)
 return {'path':rel,'kind':kind,'bytes':p.stat().st_size,'sha256':h.hexdigest()}
assert not e.exists();e.mkdir()
for f in ['terminal.json','status.json','replay.json','host.txt','manifest.sha256','producer.log','replay.log']:
 shutil.copy2(raw/f,e/f)
shutil.copy2(raw/'certificates/summary.json',e/'summary.json')
summary=read(e/'summary.json');replay=read(e/'replay.json');status=read(e/'status.json');proto=read(s/'protocol-top.json')
assert len(summary['rows'])==96
rep={r['key']:r for r in replay['rows']};verified=[]
for row in summary['rows']:
 r=copy.deepcopy(row);r['verified']=(status['producer_exit_code']==0 and status['replay_exit_code']==0 and rep.get(row['key'],{}).get('verified') is True and rep[row['key']].get('source_sha256')==row.get('source_sha256'))
 verified.append(r)
(e/'verified-summary.json').write_text(json.dumps({'schema':'n83.paired-top-verified-summary/v1','rows':verified,'prime_isomorphisms':replay['prime_isomorphisms'],'scope':summary['scope']},indent=2)+'\n')
(e/'isomorphisms').mkdir()
for i in replay['prime_isomorphisms']:shutil.copy2(raw/'certificates'/i['certificate'],e/'isomorphisms'/i['certificate'])
x=read(s/'experiment-degree3.export.json');template=copy.deepcopy(x['runs'][0]);x['export_id']=proto['protocol_id'];x['protocol']={'path':'protocol-top.json','protocol_id':proto['protocol_id'],'sha256':proto['freeze']['protocol_sha256'],'version':proto['version']}
x['source']={'repo':'crypto','commit':proto['freeze']['commit'],'dirty':False};x['producer']['session']='n83-paired-top-20261028-r1'
x['arms']=[{'id':a['id'],'role':a['role'],'commit':a['instrument']['commit'],'binary_sha256':a['instrument']['binary_sha256'],'image_digest':None} for a in proto['arms']]
x['host']['path']='evidence/top/host.txt';x['host']['sha256']=art(x['host']['path'])['sha256']
x['analysis']={'script':art('src/bin/top.rs','analysis'),'outputs':[art('evidence/top/verified-summary.json','summary')],'blinded':False,'blinding_key_sha256':None,'unmasked_at':None}
x['files']=[art(str(p.relative_to(s))) for p in sorted(e.rglob('*')) if p.is_file()]+[art('top-archives/runs.tar.xz'),art('TOP_FORMAT.json'),art('TOP_DESIGN.md')]
x['runs']=[]
for r in verified:
 a=copy.deepcopy(template);ok=r['verified'];a.update(run_id=r['key'],arm=r['arm']+'-'+r['family'],instance_id=str((r['m']-2)*2+int(r['target_kind']=='natural')),status='succeeded' if ok else 'failed',outcome_class='completed' if ok else 'not_completed',backend_ref=proto['extensions']['raw_prefix']+'certificates/'+r['key']+'.json',artifacts=[art('evidence/top/verified-summary.json','summary')]);a['verification']={'status':'verified' if ok else 'no_claim','verifier':'Fresh-process full source, projection, linear XOR and paired prime map recomputation; producer implementation, independent review pending'}
 a['measurements']['counted']={k:r[k] for k in ['dreg_lower_bound','dreg_upper_bound','top_quotient_dimension_lower_bound','linear_top_rank'] if ok and r.get(k) is not None};x['runs'].append(a)
complete=sum(r['verified'] for r in verified);x['flow']={'planned':96,'attempted':96,'completed':complete,'not_completed':96-complete,'interrupted':0,'resumed':0,'excluded':[]}
x['notes']=[{'by':'producer','kind':'observation','text':t} for t in ['All96 retained paired systems are included; only successful producer plus replay admits numeric bounds.','Six deterministic binary projections retain their failed surviving monomials; greedy maps are not claimed optimal.','Original linear top generators strengthen an upper bound without changing the ideal. Intersecting intervals do not rank bases.','Sixteen prime diagonal maps, when verified, identify graded top quotients only; no affine Boolean-selector ideal equivalence is asserted.','No new LFD, intermediate solving-degree, timing or universal FFD/LFD verdict is measured.']]
(s/'experiment-top.export.json').write_text(json.dumps(x,indent=2)+'\n')
idx={'schema':'n83.paired-top-storage/v1','raw_prefix':proto['extensions']['raw_prefix'],'archive':art('top-archives/runs.tar.xz'),'layout':'TOP_FORMAT.json','reconstruction':'Extract runs.tar.xz into a fresh directory. It contains raw paths directly; sha256sum -c manifest.sha256 checks the full run.','source_revision':proto['freeze']['commit'],'frozen_protocol_commit':'5af325f18cc','source_archive_sha256':'376fa9b2fc4b97f03f1a76e09291ed3886ba9581b06b9207e7a7f93e153a9bce','source_archive_uri':'s3://setec-astronomy-artifacts/runs/n83-paired-top-20261028-r1/source/frozen-source.tar.xz'}
(s/'evidence/top-storage.json').write_text(json.dumps(idx,indent=2)+'\n');print(json.dumps({'cases':96,'verified':complete,'prime_isomorphisms':len(replay['prime_isomorphisms'])}))
