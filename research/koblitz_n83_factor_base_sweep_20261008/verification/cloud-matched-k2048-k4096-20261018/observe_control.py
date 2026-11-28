from pathlib import Path
import sys,subprocess,threading,time,json,hashlib,os
stage,k=sys.argv[1],int(sys.argv[2]);root=Path(__file__).resolve().parent
repo=Path('/home/ec2-user/crypto-n83-resume');study=repo/'research/koblitz_n83_factor_base_sweep_20261008'
out=root/(stage+'-k'+str(k));done=threading.Event();errors=[]
def observe():
 while not done.is_set():
  config=out/'config.json'
  if config.exists():
   try:
    name=json.loads(config.read_text())['container_name']
    p=subprocess.run(['docker','inspect',name],capture_output=True,text=True,timeout=5)
    if p.returncode==0:
     obj=json.loads(p.stdout)[0]
     if obj['State']['Running']:
      fields=['Memory','MemorySwap','NanoCpus','CpuPeriod','CpuQuota','NetworkMode','ReadonlyRootfs','CapDrop','SecurityOpt','PidsLimit']
      safe={'Id':obj['Id'],'Image':obj['Image'],'State':{n:obj['State'].get(n) for n in ['Status','Running','StartedAt']},'HostConfig':{n:obj['HostConfig'].get(n) for n in fields},'Mounts':[{n:m.get(n) for n in ['Type','Source','Destination','RW']} for m in obj['Mounts']]}
      (out/'container-inspect.json').write_text(json.dumps(safe,indent=2)+'\n');return
   except Exception as e:errors.append(str(e))
  done.wait(0.02)
args=[sys.executable,str(study/('chain_capacity_supervisor.py' if stage=='capacity' else 'chain_search_supervisor.py')),'--panel',str(root/'inputs'/('k'+str(k))),'--columns',str(k),'--policy','public_x_hash','--seed','2026100801','--summands','5','--max-variables','150000','--max-domain-clauses','2000000','--wall-seconds','180' if stage=='capacity' else '180','--memory-mib','4096','--binary','/home/ec2-user/scratch/n83-resume-20261018/reconstruction-target/release/examples/koblitz_n83_factor_base_export','--image','python:3.11-slim','--orbit-domain','--output-dir',str(out)]
if stage=='capacity':args+=['--identity-mask','0']
else:args+=['--fixtures',str(root/'inputs/fixtures'),'--max-trials','1','--max-models','32','--conflict-budget','1000']
thread=threading.Thread(target=observe,daemon=True);thread.start();start=time.monotonic()
with (root/(stage+'-k'+str(k)+'.supervisor.stdout')).open('wb') as stdout,(root/(stage+'-k'+str(k)+'.supervisor.stderr')).open('wb') as stderr:
 p=subprocess.run(args,cwd=repo,stdout=stdout,stderr=stderr)
done.set();thread.join(timeout=8)
r={'schema':'n83.observed-stage-launch/v1','stage':stage,'columns':k,'command':args,'exit_code':p.returncode,'wall_seconds':time.monotonic()-start,'observer_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),'live_inspection_captured':(out/'container-inspect.json').exists(),'observer_errors':errors[-5:]}
(root/(stage+'-k'+str(k)+'.launch.json')).write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r));sys.exit(p.returncode)
