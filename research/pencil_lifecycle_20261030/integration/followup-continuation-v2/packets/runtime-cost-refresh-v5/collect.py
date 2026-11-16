from pathlib import Path
import concurrent.futures,subprocess,json,datetime,hashlib
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029');D=R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/runtime-cost-refresh-v5';D.mkdir(exist_ok=True);O=D/'observations';O.mkdir()
(D/'PLAN.md').write_text('Read-only cost refresh2026-11-03 T1484. Reuse prior exact budget,price,resource reads with30second command bounds; broaden regional instance/request inventory to detect new commitments. No credentials printed, no resources changed. Preserve prior2400 worker seconds and58USD reservations. Potential additional600 seconds/1USD is a proposal only. Billing delayed, shared daily budgets nonadditive; require finite conservative footprint reserve and explicit unknowns.\n')
old=Path('/var/tmp/runtime-dollar-observation-jzfkejlk')
queries={p.name.split('.')[0]:json.loads(p.read_text())['argv'] for p in old.glob('*.receipt.json')}
for key,flag in [('instances','--instance-ids'),('spotrequests','--spot-instance-request-ids')]:
 a=queries[key];i=a.index(flag);end=a.index('--query');del a[i:end]
queries['caller']=['timeout','30s','aws','sts','get-caller-identity','--region','us-east-2','--output','json']
(D/'queries.json').write_text(json.dumps(queries,indent=2)+'\n')
def run(item):
 name,a=item;start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (O/(name+'.stdout')).open('wb') as so,(O/(name+'.stderr')).open('wb') as se:
  p=subprocess.run(a,stdout=so,stderr=se,timeout=35)
 rec={'argv':a,'exit':p.returncode,'started_utc':start,'observed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'stdout_sha256':hashlib.sha256((O/(name+'.stdout')).read_bytes()).hexdigest()}
 (O/(name+'.receipt.json')).write_text(json.dumps(rec,indent=2)+'\n');return name,p.returncode
with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
 for x in pool.map(run,queries.items()):print(x,flush=True)
