from pathlib import Path
import subprocess,json,hashlib,time,os,shutil
D=Path(__file__).parent
PIN='public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39'
env=os.environ.copy();cleared=[]
for k in ['DOCKER_HOST','DOCKER_CONTEXT','DOCKER_TLS','DOCKER_TLS_VERIFY','DOCKER_CERT_PATH']:
 if k in env:cleared.append(k);env.pop(k)
commands=[['sudo','-n','docker','--host','unix:///var/run/docker.sock','version','--format','{{.Server.Version}}'],['sudo','-n','docker','--host','unix:///var/run/docker.sock','image','inspect',PIN,'--format','{"Id":{{json .Id}},"RepoDigests":{{json .RepoDigests}},"Architecture":{{json .Architecture}},"Os":{{json .Os}},"User":{{json .Config.User}}}']]
receipts=[]
for i,cmd in enumerate(commands):
 start=time.time()
 try:
  r=subprocess.run(cmd,env=env,capture_output=True,text=True,timeout=20);entry={'argv':cmd,'started':start,'finished':time.time(),'exit_code':r.returncode,'stdout':r.stdout,'stderr':r.stderr,'timeout':False}
 except subprocess.TimeoutExpired as e:entry={'argv':cmd,'started':start,'finished':time.time(),'exit_code':None,'stdout':str(e.stdout or ''),'stderr':str(e.stderr or ''),'timeout':True}
 receipts.append(entry);(D/('command-'+str(i)+'.json')).write_text(json.dumps(entry,indent=2)+'\n')
 # An unavailable endpoint makes image absence unknowable; no second permission probe.
 if i==0 and (entry['exit_code']!=0 or entry['stderr'].strip() or not entry['stdout'].strip()):break
v=receipts[0];ok=v['exit_code']==0 and not v['stderr'].strip() and bool(v['stdout'].strip());image=receipts[1] if len(receipts)>1 else None
status={'schema':'runtime.local-route-inspection/v1','route':'existing sudo -n docker','explicit_endpoint':'unix:///var/run/docker.sock','selectors_cleared_names':cleared,'daemon_available':ok,'server_version':v['stdout'].strip() if ok else None,'pinned_upstream_image':PIN,'pinned_upstream_present':(image['exit_code']==0) if image else None,'final_ecbench_image_id':None,'final_runtime_image_id':None,'scope':'read-only existing route and single relevant pinned upstream image; final image bindings remain unset','actions_not_performed':['daemon changes','group changes','container actions','build','pull','push','cloud commands'],'commands':len(receipts)}
(D/'status.json').write_text(json.dumps(status,indent=2)+'\n')
refs=['/var/tmp/pencil-runtime-readiness-20261031-v1/manifest.json','/var/tmp/pencil-runtime-qualification-draft-20261031-v1/manifest.json','/home/ec2-user/crypto-pencil-signed-graph-20261029/taskq/deploy/eks-karpenter/Dockerfile.ecbench']
(D/'source-bindings.json').write_text(json.dumps({p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in refs},indent=2)+'\n')
print(json.dumps({'status':status,'receipts':receipts},indent=2))
