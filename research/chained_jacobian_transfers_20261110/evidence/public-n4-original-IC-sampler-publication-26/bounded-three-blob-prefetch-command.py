"""Hydrate only three exact known Git blobs; preserve refs, HEAD and working status."""
from pathlib import Path
from datetime import datetime,timezone
import base64,hashlib,json,os,subprocess
r=Path('/workspace/crypto');out=Path('/workspace/scratch/ct99-handoff26-bounded-object-prefetch-01');out.mkdir()
env=dict(os.environ,GIT_NO_LAZY_FETCH='1');before={k:subprocess.check_output(['git',*a],cwd=r,env=env)for k,a in [('refs',['show-ref']),('head',['rev-parse','HEAD']),('status',['status','--porcelain'])]}
fetch=dict(env,GIT_NO_LAZY_FETCH='0');n=int(fetch.get('GIT_CONFIG_COUNT','0'));fetch['GIT_CONFIG_COUNT']=str(n+1);fetch['GIT_CONFIG_KEY_'+str(n)]='http.https://github.com/.extraheader';fetch['GIT_CONFIG_VALUE_'+str(n)]='AUTHORIZATION: basic '+base64.b64encode(('x-access-token:'+fetch['GH_TOKEN']).encode()).decode();fetch['GIT_TERMINAL_PROMPT']='0'
prefix='research/chained_jacobian_transfers_20261110/evidence/'
rows=[]
for label,commit,name,oid in [('actual-manifest','b7bc54fcba9ad9a7ac901037c263c0b8e8a300a8',prefix+'public-n4-original-IC-sampler-handoff-26/.artifacts.json','fd48440eba7eed3f8dfa1e5f22c13cf8b8ca0a91'),('actual-ignore','b7bc54fcba9ad9a7ac901037c263c0b8e8a300a8',prefix+'public-n4-original-IC-sampler-handoff-26/.gitignore','8fbb1f15adbecdbe48be934413dd6a8de00e55a2'),('unchanged-prior-PDF','99cb2b204da7adc85345d3a7e052a9b95f09a1b5',prefix+'public-n4-norm-halving-handoff-16/report.pdf','eb82275a354a599a70a6fac3d5492902e9248e7c')]:
 tree=subprocess.check_output(['git','ls-tree',commit,'--',name],cwd=r,env=env).decode().split();assert tree[2]==oid
 q=subprocess.run(['git','show',commit+':'+name],cwd=r,env=fetch,capture_output=True,timeout=120);(out/(label+'.blob')).write_bytes(q.stdout);(out/(label+'.stderr')).write_bytes(q.stderr);(out/(label+'.exit')).write_text(str(q.returncode)+'\n');assert q.returncode==0
 assert subprocess.check_output(['git','hash-object','--stdin'],cwd=r,input=q.stdout,env=env).decode().strip()==oid
 rows.append(dict(commit=commit,path=name,exact_blob=oid,bytes=len(q.stdout),sha256=hashlib.sha256(q.stdout).hexdigest(),exit=q.returncode));print(label,len(q.stdout),flush=True)
for k,a in [('refs',['show-ref']),('head',['rev-parse','HEAD']),('status',['status','--porcelain'])]:assert subprocess.check_output(['git',*a],cwd=r,env=env)==before[k]
(out/'outcome.json').write_text(json.dumps(dict(at=datetime.now(timezone.utc).isoformat(),only_three_exact_named_blobs=rows,all_refs_HEAD_status_unchanged=True,no_history_enumeration=True,original_FF01_failure='GIT_NO_LAZY_FETCH blocked actual new manifest during rename detection before merge; source HEAD unchanged',scope='bounded metadata hydration only'),indent=2)+'\n')
