#!/usr/bin/env python3
"""Run candidate FFD cases serially with frozen 60s/case and 8GiB limits."""
import hashlib,json,os,shutil,subprocess,time
from pathlib import Path
ROOT=Path('/tmp/n83-candidate-inputs/spike28')
OUT=Path('/tmp/n83-candidate-fall/spike28')
BIN=Path('research/n83_candidate_pdp_20261028/target/release/fall').resolve()
S3='s3://isolab-results-104186811093-us-east-2/runs/n83-pdp-top-factorbase-20261031-v2/spike28/first-fall-attempt-01'
OUT.mkdir(parents=True,exist_ok=False)
inputs=json.loads((ROOT/'inputs.json').read_text())
records=[]
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
for row in inputs:
 case=row['id']; d=OUT/'cases'/case; root=d/'input'; dest=d/'result';root.mkdir(parents=True); (root/'inputs.json').write_text(json.dumps([row],separators=(',',':'))+'\n');(root/case).mkdir();os.symlink(ROOT/case/'system.sing',root/case/'system.sing')
 env=os.environ.copy();env['ISOLAB_RESULTS_URI']=S3
 cmd=['timeout','--kill-after=5s','60s','prlimit','--as=8589934592','--',str(BIN),'run',str(root),str(dest)]
 start=time.time();proc=subprocess.run(cmd,text=True,capture_output=True,env=env);elapsed=time.time()-start
 status='completed' if proc.returncode==0 and (dest/f'{case}.json').exists() else ('timeout' if proc.returncode==124 else 'failed')
 rec={'id':case,'family':row['family'],'m':row['m'],'target_kind':row['target_kind'],'status':status,'returncode':proc.returncode,'elapsed_seconds':round(elapsed,3),'command':cmd,'stdout':proc.stdout,'stderr':proc.stderr,'input_sha256':sha(root/'inputs.json'),'system_sha256':sha(ROOT/case/'system.sing')}
 if (dest/f'{case}.json').exists():
  cert=dest/f'{case}.json'; shutil.copyfile(cert,OUT/'certificates'/f'{case}.json') if (OUT/'certificates').exists() else None
  (OUT/'certificates').mkdir(exist_ok=True);shutil.copyfile(cert,OUT/'certificates'/f'{case}.json');rec['certificate_sha256']=sha(cert);c=json.loads(cert.read_text());rec['first_fall_degree']=c.get('first_fall_degree');rec['first_fall_lower_bound']=c.get('first_fall_lower_bound');rec['last_fall_degree']=c.get('last_fall_degree');rec['closure_rank']=c.get('closure_rank')
  replaycmd=[str(BIN),'replay',str(root),str(cert)]; replay=subprocess.run(replaycmd,text=True,capture_output=True); rec['replay']={'returncode':replay.returncode,'stdout':replay.stdout,'stderr':replay.stderr,'verified':replay.returncode==0}
  (d/'replay.json').write_text(json.dumps(rec['replay'],indent=2)+'\n')
  if replay.returncode!=0: rec['status']='replay_failed'
 (d/'attempt.json').write_text(json.dumps(rec,indent=2,sort_keys=True)+'\n'); records.append(rec)
 print(json.dumps({'id':case,'status':rec['status'],'returncode':proc.returncode,'elapsed_seconds':rec['elapsed_seconds']},separators=(',',':')),flush=True)
rows=[{'id':r['id'],'family':r['family'],'m':r['m'],'target_kind':r['target_kind'],'status':r['status'],'first_fall_degree':r.get('first_fall_degree'),'first_fall_lower_bound':r.get('first_fall_lower_bound'),'last_fall_degree':r.get('last_fall_degree'),'closure_rank':r.get('closure_rank'),'source_sha256':r['system_sha256'],'certificate_sha256':r.get('certificate_sha256'),'replay_verified':r.get('replay',{}).get('verified',False)} for r in records]
summary={'schema':'n83.candidate-first-fall-run/v1','candidate_id':'spike28','protocol_id':'N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2','protocol_sha256':'50cd6f8213789de2dfd54a3b33a067ed87cc0def761b7c98769be3e19aee47a9','inputs_sha256':sha(ROOT/'inputs.json'),'planned_cases':48,'attempted_cases':len(records),'completed_certificates':sum(r.get('certificate_sha256') is not None for r in records),'replay_verified':sum(r.get('replay',{}).get('verified',False) for r in records),'rows':rows,'scope':'First-fall evidence and separate closure fields only; null LFD is not promoted to exact. Per-case 60 second timeout and 8 GiB address-space limit; timeouts/failures retained.'}
(OUT/'summary.json').write_text(json.dumps(summary,indent=2,sort_keys=True)+'\n');print(json.dumps({'terminal':True,'attempted':len(records),'completed':summary['completed_certificates'],'replayed':summary['replay_verified']}),flush=True)
