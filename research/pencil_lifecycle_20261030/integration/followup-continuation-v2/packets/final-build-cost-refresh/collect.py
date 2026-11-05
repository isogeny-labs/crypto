from pathlib import Path
import subprocess,concurrent.futures,json,datetime,hashlib,resource,time
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');D=R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/final-build-cost-refresh';O=D/'observations';O.mkdir()
def limits():resource.setrlimit(resource.RLIMIT_FSIZE,(16*1024*1024,)*2)
def run(item):
 name,a=item;start=datetime.datetime.now(datetime.timezone.utc).isoformat();t=time.monotonic()
 with (O/(name+'.stdout')).open('xb') as so,(O/(name+'.stderr')).open('xb') as se:
  p=subprocess.run(a,stdout=so,stderr=se,timeout=35,preexec_fn=limits)
 rec={'argv':a,'exit':p.returncode,'started_utc':start,'observed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'elapsed_seconds':time.monotonic()-t}
 for kind in ['stdout','stderr']:
  f=O/(name+'.'+kind);rec[kind+'_sha256']=hashlib.sha256(f.read_bytes()).hexdigest();rec[kind+'_bytes']=f.stat().st_size
 (O/(name+'.receipt.json')).write_text(json.dumps(rec,indent=2)+'\n');print(name,p.returncode,flush=True);return p.returncode
q=json.loads((R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-cost-refresh-v5/queries.json').read_text())
for a in q.values():a[1:2]=['-k','2s','30s']
q['caller']+=['--cli-connect-timeout','5','--cli-read-timeout','15']
(D/'queries.json').write_text(json.dumps(q,indent=2)+'\n')
assert run(('caller',q.pop('caller')))==0
assert json.loads((O/'caller.stdout').read_text())['Account']=='104186811093'
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:list(pool.map(run,q.items()))
if json.loads((O/'clusters.receipt.json').read_text())['exit']==0:
 names=json.loads((O/'clusters.stdout').read_text())['clusters'];assert len(names)<=5
 for i,n in enumerate(names):
  a=['timeout','-k','2s','30s','aws','eks','describe-cluster','--name',n,'--query','cluster.{Name:name,Status:status,Version:version,Created:createdAt,Platform:platformVersion,Upgrade:upgradePolicy}','--region','us-east-2','--cli-connect-timeout','5','--cli-read-timeout','15','--output','json'];run(('cluster-detail-'+str(i),a))
