#!/usr/bin/env python3
"""One explicit pinned public-parent acquisition. Proposal only; no implicit retry."""
import argparse,hashlib,json,os,pathlib,re,signal,subprocess,tempfile,time
P=pathlib.Path
REF='public.ecr.aws/aws-cli/aws-cli@sha256:b94fc7612d131f1a3cb76145ba3385c813ad7561ef2c2b60c89fbd5dfee93d39'
H=lambda x:hashlib.sha256(x).hexdigest()
def require(v,m):
 if not v:raise RuntimeError(m)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--binding',required=True);ap.add_argument('--receipts',required=True);a=ap.parse_args();d=P(a.receipts);require(d.is_absolute() and not d.exists(),'fresh absolute receipt directory');d.mkdir(mode=0o700)
 start=time.monotonic();deadline=start+300;seq=0;pending=None;pull_attempted=False;result={'reference':REF,'status':'not_started','image':None,'pull_attempts':0,'scientific_execution':False};unknown_client=False
 def handler(s,f):
  nonlocal pending
  pending=s
 signal.signal(signal.SIGINT,handler);signal.signal(signal.SIGTERM,handler)
 try:
  b=json.loads(P(a.binding).read_text());require(b.get('admitted') is True,'not admitted');docker=P(b['docker_client']);require(docker.is_absolute() and H(docker.read_bytes())==b['docker_client_sha256'],'client binding');require(b.get('reviewed_source_sha256')==H(P(__file__).read_bytes()),'source review binding');require(re.fullmatch('[0-9a-f]{64}',b.get('prospective_review_sha256','')),'review binding');require(b.get('reference')==REF,'reference binding')
  config=d/'empty-config';config.mkdir(mode=0o700);prefix=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','HOME='+str(config),'DOCKER_CONFIG='+str(config),str(docker),'--host','unix:///var/run/docker.sock'];env={'PATH':'/usr/bin:/bin','HOME':str(config),'LANG':'C'}
  def run(name,args,cap,inspection=False):
   nonlocal seq,unknown_client
   require(pending is None,'interrupted');remaining=deadline-time.monotonic();require(remaining>3,'deadline exhausted');cap=min(cap,remaining-2);seq+=1;r=d/f'{seq:02d}-{name}';r.mkdir();cmd=prefix+args;p=None;t=time.monotonic();receipt={'argv':cmd,'cap_seconds':cap};out=None;err=None
   try:
    out=tempfile.TemporaryFile('w+b') if inspection else (r/'stdout').open('w+b');err=(r/'stderr').open('wb');unknown_client=True
    p=subprocess.Popen(cmd,env=env,stdout=out,stderr=err,start_new_session=True)
    while p.poll() is None:
     require(pending is None,'interrupted');require(time.monotonic()-t<cap,'client timeout');require(os.fstat(out.fileno()).st_size<=8*1024**2 and os.fstat(err.fileno()).st_size<=8*1024**2,'output cap');time.sleep(.04)
    out.seek(0);raw=out.read(8*1024**2+1);require(len(raw)<=8*1024**2,'output cap');obj=json.loads(raw) if inspection and p.returncode==0 else None
    if obj is not None:
     safe=json.loads(json.dumps(obj))
     for x in safe:
      if x.get('Config',{}).get('Env') is not None:x['Config']['Env']=[{'name':v.split('=',1)[0],'sha256':H(v.encode())} for v in x['Config']['Env']]
     (r/'inspection.json').write_text(json.dumps(safe,indent=2)+'\n')
    return p.returncode,raw,(r/'stderr').read_bytes(),obj
   finally:
    try:
     if p is not None and p.poll() is None:
      try:os.killpg(p.pid,signal.SIGTERM)
      except ProcessLookupError:pass
      try:p.wait(timeout=1)
      except subprocess.TimeoutExpired:
       try:os.killpg(p.pid,signal.SIGKILL)
       except ProcessLookupError:pass
       p.wait(timeout=1)
     if p is not None:unknown_client=p.poll() is None;receipt['client_terminal_exit']=p.poll()
    finally:
     if out is not None:out.close()
     if err is not None:err.close()
     receipt.update({'started_monotonic':t,'ended_monotonic':time.monotonic(),'unknown_client':unknown_client});(r/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
  def inspect(name):return run(name,['image','inspect',REF],15,True)
  def validate(obj):
   require(isinstance(obj,list) and len(obj)==1,'image inspection shape');x=obj[0];require(x['Os']=='linux' and x['Architecture']=='amd64' and not x['Config'].get('OnBuild'),'platform/OnBuild');require(REF in x.get('RepoDigests',[]),'requested digest not bound');require(re.fullmatch('sha256:[0-9a-f]{64}',x['Id']),'image ID');return {'image_id':x['Id'],'repo_digests':x['RepoDigests'],'layers':x['RootFS']['Layers'],'platform':'linux/amd64','onbuild_empty':True}
  rc,raw,err,_=run('daemon-id',['info','--format','{{.ID}}'],15);require(rc==0 and raw.strip(),'daemon unavailable');result['daemon_id']=raw.decode().strip()
  rc,raw,err,obj=inspect('before')
  if rc==0:result.update({'status':'already_present','image':validate(obj)})
  else:
   require(b'No such image' in err and REF.encode() in err,'absence not established; stop')
   require(deadline-time.monotonic()>45,'insufficient readback reserve');pull_attempted=True;result['pull_attempts']=1
   rc,raw,err,_=run('pull',['pull','--platform','linux/amd64',REF],min(240,deadline-time.monotonic()-30));require(rc==0,'pull failed')
   rc,raw,err,obj=inspect('after');require(rc==0,'post-pull inspection failed');result.update({'status':'acquired_verified','image':validate(obj)})
 except BaseException as e:
  result.update({'status':'acquisition_uncertain' if pull_attempted or unknown_client else 'preparation_failed','error':type(e).__name__+': '+str(e),'no_automatic_retry':True,'unknown_client':unknown_client})
 finally:
  result.update({'elapsed_seconds':time.monotonic()-start,'cap_seconds':300,'pull_client_termination_is_not_daemon_cancellation':True});(d/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
 return 0 if result['status'] in ['already_present','acquired_verified'] else 1
if __name__=='__main__':raise SystemExit(main())
