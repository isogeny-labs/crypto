"""Bounded runtime-only assembler. V2 binding is unadmitted; internal checks are not whole-process containment."""
import argparse, contextlib, hashlib, io, json, os, re, runpy, signal, stat, subprocess, time
from pathlib import Path
from datetime import datetime, timezone
HERE=Path(__file__).resolve().parent
BASE='sha256:347667d6ecc9294ab7bfcc83411fb47f7548c8d1ca5f7d315cca23fc4b115476'
FORMAT='{ "Id":{{json .Id}}, "OnBuild":{{json .Config.OnBuild}}, "User":{{json .Config.User}}, "Entrypoint":{{json .Config.Entrypoint}}, "Labels":{{json .Config.Labels}}, "Layers":{{json .RootFS.Layers}}, "OS":{{json .Os}}, "Architecture":{{json .Architecture}} }'
def require(ok,msg):
 if not ok:raise ValueError(msg)
def sha(raw):return hashlib.sha256(raw).hexdigest()
def strict(raw):
 def unique(items):
  out={}
  for k,v in items:
   require(k not in out,'duplicate JSON key');out[k]=v
  return out
 return json.loads(raw,object_pairs_hook=unique)
def read_pin(path,digest):
 p=Path(path);require(p.is_absolute(),'absolute pinned path')
 for q in [p,*p.parents]:require(not q.is_symlink(),'pinned symlink')
 require(p.is_file(),'pinned regular file');raw=p.read_bytes();require(sha(raw)==digest,'pinned digest '+str(p));return raw
def core_hash(b):return sha(json.dumps(b['inputs'],sort_keys=True,separators=(',',':')).encode())
def admission(b):
 require(b.get('admitted') is True,'not admitted')
 x=b['inputs'];require(x['assembler_sha256']==sha(Path(__file__).read_bytes()),'assembler source binding')
 require(re.fullmatch(r'[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}',x['owner']) is not None,'owner nonce')
 require(x['tag']=='pencil-runtime-replacement:'+x['owner'],'fresh tag binding')
 require(x['base_image']==BASE,'exact base')
 require(datetime.now(timezone.utc)<datetime.fromisoformat(x['not_after_utc'].replace('Z','+00:00')),'software admission deadline')
 require(x['limits']=={'total_seconds':600,'reconciliation_reserve_seconds':60,'build_client_seconds':240,'attempts':1},'limits')
 review=strict(read_pin(b['review_path'],b['review_sha256']))
 require(review.get('decision')=='approve' and review.get('scope')=='one-local-runtime-copy-build','review scope')
 require(review.get('inputs_sha256')==core_hash(b) and review.get('assembler_sha256')==x['assembler_sha256'],'review joins')
 require(review.get('independent') is True and review.get('reviewer') and review['reviewer']!=x['author'],'independent review identity')
 require(review.get('execution_allowance_approved') is True,'execution allowance')
 return x
def inspect_args(ref):return ['image','inspect','--format',FORMAT,ref]
def checked_image(obj,expected):
 require(obj['Id']==expected and obj['OS']=='linux' and obj['Architecture']=='amd64' and not obj['OnBuild'],'image identity/platform/OnBuild')
def completion_guard(start,not_after_utc,pending_signal):
 require(pending_signal is None,'interrupted before success')
 require(time.monotonic()<start+535 and datetime.now(timezone.utc)<datetime.fromisoformat(not_after_utc.replace('Z','+00:00')),'completion deadline')
def workflow(x,out,call,verify_context,final_check):
 """One build at most. All external calls pass through the supplied bounded driver."""
 def inspect(ref,label):
  rc,raw,err=call(label,inspect_args(ref),15);require(rc==0,'image inspect failed');return strict(raw)
 rc,raw,err=call('daemon',['info','--format','{{.ID}}'],15);require(rc==0 and raw.decode().strip()==x['daemon_id'],'daemon mismatch')
 parent=inspect(BASE,'base');checked_image(parent,BASE)
 require(parent['Layers']==x['base_layers'],'base layer binding')
 rc,raw,err=call('tag-absence',inspect_args(x['tag']),15)
 require(rc!=0 and b'No such image' in err and x['tag'].encode() in err,'tag not authoritatively absent')
 verify_context()
 argv=['build','--pull=false','--network=none','--platform=linux/amd64','--iidfile',str(out/'runtime.iid'),'--build-arg','BASE_IMAGE='+BASE,'--label','isolab.local-assembly.owner='+x['owner'],'--label','isolab.local-assembly.phase=runtime','--tag',x['tag'],'--file',str(Path(x['context'])/'Dockerfile'),x['context']]
 rc,raw,err=call('build-runtime',argv,240);require(rc==0,'build failed or uncertain')
 iid=out/'runtime.iid';require(not iid.is_symlink(),'IID symlink');st=iid.lstat()
 require(stat.S_ISREG(st.st_mode) and st.st_nlink==1 and st.st_uid==0 and stat.S_IMODE(st.st_mode)==0o600 and st.st_size in (71,72),'IID custody')
 rc,raw,err=call('read-iid',['__read_iid__',str(iid)],15);require(rc==0 and re.fullmatch(rb'sha256:[0-9a-f]{64}\n?',raw),'IID read')
 after=iid.lstat();require((st.st_dev,st.st_ino,st.st_size,st.st_mtime_ns)==(after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'IID changed')
 image=raw.decode().strip();built=inspect(x['tag'],'built-tag');checked_image(built,image)
 require(built['User']=='0:0' and not built['Entrypoint'],'runtime configuration')
 labels=built['Labels'];require(labels.get('isolab.local-assembly.owner')==x['owner'] and labels.get('isolab.local-assembly.phase')=='runtime' and labels.get('org.opencontainers.image.revision')==x['source_revision'],'runtime labels')
 require(built['Layers'][:len(parent['Layers'])]==parent['Layers'] and len(built['Layers'])>len(parent['Layers']),'runtime layer prefix')
 verify_context();final_check();return {'status':'built_pending_qualification','image_id':image,'tag':x['tag'],'layers':built['Layers']}
class Driver:
 def __init__(self,out,start,not_after_utc):
  absolute_remaining=(datetime.fromisoformat(not_after_utc.replace('Z','+00:00'))-datetime.now(timezone.utc)).total_seconds()
  self.out=out;self.start=start;self.deadline=min(start+600,time.monotonic()+absolute_remaining);self.work_deadline=self.deadline-60;self.seq=0;self.calls=[];self.pending_signal=None
  config=out/'empty-docker-config';config.mkdir(mode=0o700)
  self.env={'PATH':'/usr/bin:/bin','HOME':str(config),'LANG':'C'}
  self.prefix=['/usr/bin/sudo','-n','/usr/bin/env','-i','PATH=/usr/bin:/bin','HOME='+str(config),'DOCKER_CONFIG='+str(config),'DOCKER_BUILDKIT=0','/usr/bin/docker','--host','unix:///var/run/docker.sock']
 def call(self,label,args,cap,readback=False):
  require(readback or self.pending_signal is None,'interrupted')
  remaining=(self.deadline if readback else self.work_deadline)-time.monotonic();require(remaining>5,'deadline reserve')
  cap=min(cap,remaining-3);self.seq+=1;folder=self.out/f'{self.seq:03d}-{label}';folder.mkdir()
  if args[0]=='__read_iid__':
   require(args==['__read_iid__',str(self.out/'runtime.iid')],'IID read scope');argv=['/usr/bin/sudo','-n','/usr/bin/cat',args[1]]
  else:argv=self.prefix+args
  receipt={'argv':argv,'cap_seconds':cap,'client_terminal_exit':None,'client_unknown':True};self.calls.append(receipt);proc=None;start=time.monotonic()
  try:
   with (folder/'stdout').open('w+b') as so,(folder/'stderr').open('w+b') as se:
    proc=subprocess.Popen(argv,env=self.env,stdout=so,stderr=se,start_new_session=True);receipt['pid']=proc.pid;receipt['process_group']=proc.pid
    while proc.poll() is None:
     require(readback or self.pending_signal is None,'interrupted');require(time.monotonic()-start<cap,'client timeout')
     require(os.fstat(so.fileno()).st_size<=16*1024**2 and os.fstat(se.fileno()).st_size<=16*1024**2,'observed output cap');time.sleep(.04)
    so.seek(0);se.seek(0);raw=so.read(16*1024**2+1);err=se.read(16*1024**2+1);require(max(len(raw),len(err))<=16*1024**2,'output cap');return proc.returncode,raw,err
  finally:
   if proc is not None and proc.poll() is None:
    try:
     os.killpg(proc.pid,signal.SIGTERM)
     try:proc.wait(timeout=1)
     except subprocess.TimeoutExpired:os.killpg(proc.pid,signal.SIGKILL);proc.wait(timeout=1)
    except (ProcessLookupError,PermissionError,subprocess.TimeoutExpired) as e:receipt['reap_error']=type(e).__name__
   if proc is not None:receipt['client_terminal_exit']=proc.poll();receipt['client_unknown']=proc.poll() is None
   receipt['elapsed_seconds']=time.monotonic()-start;(folder/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
def claim_once(marker,record):
 with marker.open('x') as f:f.write(json.dumps(record)+'\n');f.flush();os.fsync(f.fileno())
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--binding',required=True);ap.add_argument('--receipts',required=True);args=ap.parse_args();start=time.monotonic()
 b=strict(Path(args.binding).read_bytes());x=admission(b) # No effects before admission succeeds.
 out=Path(args.receipts);require(out.is_absolute(),'absolute receipts')
 for c in [out,*out.parents]:require(not c.is_symlink(),'receipt symlink')
 out.mkdir(mode=0o700) # Existing receipts are never reused.
 result={'status':'preparation_failed','scientific_execution':False,'automatic_retry':False,'owner':x['owner'],'tag':x['tag']};driver=None
 try:
  (out/'binding.json').write_text(json.dumps(b,indent=2)+'\n')
  checker=read_pin(x['checker_path'],x['checker_sha256']);require(x['context']==strict(read_pin(x['context_receipt_path'],x['context_receipt_sha256']))['context_path'],'context receipt join')
  read_pin('/usr/bin/docker',x['docker_sha256'])
  def final_check():completion_guard(start,x['not_after_utc'],driver.pending_signal if driver else None)
  def verify_context():
   require(time.monotonic()<start+535 and datetime.now(timezone.utc)<datetime.fromisoformat(x['not_after_utc'].replace('Z','+00:00')),'preflight deadline')
   require(sha(Path(x['checker_path']).read_bytes())==sha(checker),'checker changed')
   with contextlib.redirect_stdout(io.StringIO()) as stream:runpy.run_path(x['checker_path'],run_name='__replacement_input_check__')
   require(sha(stream.getvalue().encode())==x['input_check_sha256'],'complete input readback binding')
   final_check() # Detect overrun/interruption after synchronous checks return.
   checked=strict(stream.getvalue());require(checked['context_path']==x['context'] and checked['source_revision']==x['source_revision'] and checked['binary_sha256']==x['binary_sha256'],'input joins')
  verify_context()
  marker=Path('/var/tmp')/('pencil-runtime-replacement-'+x['owner']+'.attempt.json')
  claim_once(marker,{'inputs_sha256':core_hash(b),'receipts':str(out),'owner':x['owner']})
  result['attempt_marker']=str(marker)
  driver=Driver(out,start,x['not_after_utc'])
  def interrupted(signum,frame):driver.pending_signal=signum
  signal.signal(signal.SIGINT,interrupted);signal.signal(signal.SIGTERM,interrupted)
  completed=workflow(x,out,driver.call,verify_context,final_check)
  final_check()
  result.update(completed)
 except BaseException as e:
  result.update(status='assembly_unresolved' if driver else 'preparation_failed',error=type(e).__name__+': '+str(e))
  if driver and not any(c['client_unknown'] for c in driver.calls):
   try:
    rc,raw,err=driver.call('failure-tag-readback',inspect_args(x['tag']),15,readback=True)
    result['readback']={'exit':rc,'image':strict(raw) if rc==0 else None,'proves_build_termination':False}
   except BaseException as err:result['readback_error']=type(err).__name__+': '+str(err)
 finally:
  result['elapsed_seconds']=time.monotonic()-start;result['calls']=driver.calls if driver else [];result['pending_signal']=driver.pending_signal if driver else None
  result['qualification_required']=True;(out/'receipt.json').write_text(json.dumps(result,indent=2)+'\n')
 return 0 if result['status']=='built_pending_qualification' else 1
if __name__=='__main__':raise SystemExit(main())
