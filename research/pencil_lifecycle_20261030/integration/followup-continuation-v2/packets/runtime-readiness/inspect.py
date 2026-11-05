import pathlib,json,hashlib,subprocess,datetime,os
p=pathlib.Path(__file__).parent
r=pathlib.Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
f=r/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2'
h=lambda x:hashlib.sha256(pathlib.Path(x).read_bytes()).hexdigest()
commands=[['docker','--version'],['docker','--host=unix:///var/run/docker.sock','info','--format','{{json .ServerVersion}} {{json .OSType}} {{json .Architecture}}'],['id'],['rustup','target','list','--installed','--toolchain','1.99.1'],['stat','-c','%a %U %G %n','/var/run/docker.sock'],['file','/var/tmp/pencil-musl-private-20261031/prefix/bin/musl-gcc']]
env=os.environ.copy()
for k in ['DOCKER_HOST','DOCKER_CONTEXT','DOCKER_TLS','DOCKER_TLS_VERIFY','DOCKER_CERT_PATH']:env.pop(k,None)
receipts=[]
for cmd in commands:
 t=datetime.datetime.now(datetime.timezone.utc).isoformat();a=subprocess.run(cmd,env=env,capture_output=True,text=True,timeout=20)
 receipts.append(dict(command=cmd,started_at=t,exit_code=a.returncode,stdout=a.stdout,stderr=a.stderr))
(p/'local-observations.json').write_text(json.dumps(receipts,indent=2)+'\n')
s=json.loads((f/'packets/runtime-publication-checklist/status.json').read_text())
for v in s['retained_artifacts'].values():
 q=pathlib.Path(v['path']);v['observed_sha256']=h(q);v['observed_bytes']=q.stat().st_size;v['matches']=v['sha256']==v['observed_sha256'] and v['bytes']==v['observed_bytes']
(p/'retained-artifacts-readback.json').write_text(json.dumps(s['retained_artifacts'],indent=2)+'\n')
files=[r/'AGENTS.md',f/'packets/runtime-publication-checklist/CHECKLIST.md',f/'packets/runtime-publication-checklist/status.json',f/'packets/musl-build-supervision/READINESS.md',f/'packets/musl-compiler-qualification/build-receipt.json',r/'taskq/deploy/eks-karpenter/Dockerfile.ecbench',r/'taskq/deploy/eks-karpenter/resume/Dockerfile.runtime',r/'research/pencil_lifecycle_20261030/integration/continuation-planning-20261030/authorization.json',pathlib.Path('/home/ec2-user/pencil-controller-review-20261029/c1-takeover-20261030/runtime-archive-qualification-v1/REVIEW.md')]
(p/'source-bindings.json').write_text(json.dumps([dict(path=str(x),sha256=h(x)) for x in files],indent=2)+'\n')
print(json.dumps({'retained_artifacts_match':all(v['matches'] for v in s['retained_artifacts'].values()),'commands':receipts},indent=2))
