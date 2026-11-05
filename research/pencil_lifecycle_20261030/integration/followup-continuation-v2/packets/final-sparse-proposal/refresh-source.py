#!/usr/bin/env python3
"""Prospective root-owned isolated source refresh. Never a build or cleanup command."""
from pathlib import Path
import argparse,json,hashlib,subprocess,time,os,fcntl,stat,signal
ROOT=Path('/home/ec2-user/pencil-musl-source-b985-20261031')
TARGET=Path('/home/ec2-user/pencil-musl-target-b985-20261031')
OLD='b985ddb30b46abbd9a050e5753c7e301f40b07f6'
H=lambda b:hashlib.sha256(b).hexdigest()
def need(x,m):
 if not x:raise RuntimeError(m)
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--binding',required=True);ap.add_argument('--receipts',required=True);ap.add_argument('--seconds',type=int,required=True);a=ap.parse_args();need(0<a.seconds<=600,'cap must be 1..600 seconds')
 d=Path(a.receipts);need(d.is_absolute() and str(d).startswith('/var/tmp/') and not d.exists(),'fresh private receipts');d.mkdir(mode=0o700)
 pending_signal=[]
 def interrupted(signum,frame):pending_signal.append(signum)
 signal.signal(signal.SIGINT,interrupted);signal.signal(signal.SIGTERM,interrupted)
 start=time.monotonic();end=start+a.seconds;b=json.loads(Path(a.binding).read_text());result={'status':'not_started','mutations_started':False,'scientific_execution':False};seq=0;lock=None
 def run(label,args,input=None):
  nonlocal seq
  need(not pending_signal,'deferred signal '+repr(pending_signal));need(time.monotonic()<end,'shared deadline exhausted');seq+=1;c=['git','-c','core.hooksPath=/dev/null','-c','core.fsmonitor=false','-C',str(ROOT),*args]
  t=time.monotonic();(d/f'{seq:02d}-{label}-started.json').write_text(json.dumps({'command':c,'started_monotonic':t},indent=2))
  try:r=subprocess.run(c,input=input,capture_output=True,timeout=max(.1,min(120,end-t)))
  except subprocess.TimeoutExpired as e:
   (d/f'{seq:02d}-{label}.stdout').write_bytes(e.stdout or b'');(d/f'{seq:02d}-{label}.stderr').write_bytes(e.stderr or b'');(d/f'{seq:02d}-{label}.json').write_text(json.dumps({'command':c,'timeout':True,'seconds':time.monotonic()-t,'reconcile_required':True},indent=2));raise
  (d/f'{seq:02d}-{label}.stdout').write_bytes(r.stdout);(d/f'{seq:02d}-{label}.stderr').write_bytes(r.stderr);(d/f'{seq:02d}-{label}.json').write_text(json.dumps({'command':c,'exit':r.returncode,'seconds':time.monotonic()-t},indent=2));need(r.returncode==0,label+' failed');need(not pending_signal,'deferred signal '+repr(pending_signal));return r.stdout
 def verify(selected,label):
  rows=json.loads(selected.read_text())['files'];expected={r['path']:r for r in rows};out=[];errors=[]
  for path,row in expected.items():
   need(not pending_signal,'deferred signal '+repr(pending_signal));need(time.monotonic()<end,'verification deadline exhausted');p=ROOT/path
   try:
    s=p.lstat();need(stat.S_ISREG(s.st_mode),'nonregular');raw=p.read_bytes();obj=hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest();need(obj==row['object'],'blob mismatch');need(bool(s.st_mode&0o111)==(row['mode']=='100755'),'mode mismatch');out.append({'path':path,'sha256':H(raw),'git_blob':obj,'bytes':len(raw)})
   except Exception as e:errors.append({'path':path,'error':repr(e)})
  actual=[]
  for base,dirs,files in os.walk(ROOT,followlinks=False):
   need(not pending_signal and time.monotonic()<end,'inventory interrupted/deadline')
   if Path(base)==ROOT:dirs[:]=[n for n in dirs if n!='.git'];files=[n for n in files if n!='.git']
   for n in list(dirs):
    p=Path(base)/n
    if p.is_symlink():actual.append(str(p.relative_to(ROOT)));dirs.remove(n)
   actual.extend(str((Path(base)/n).relative_to(ROOT)) for n in files)
  extra=sorted(set(actual)-set(expected));missing=sorted(set(expected)-set(actual));(d/(label+'.json')).write_text(json.dumps({'selected_sha256':H(selected.read_bytes()),'files':out,'errors':errors,'unexpected':extra,'missing':missing},indent=2));need(not errors and not extra and not missing,'materialized source mismatch')
 try:
  need(b.get('admitted') is True,'not admitted');need(b['reviewed_refresh_sha256']==H(Path(__file__).read_bytes()),'reviewed wrapper mismatch')
  final=b['commit'];need(final=='3c592af96f3916fef3403e4cf173f22b6f7ef536','final commit mismatch');need(b['tree']=='703733197d635aea5999c17ec7b99d10ab216c41','final tree mismatch')
  new=Path(b['selected']);old=Path(b['old_selected']);patterns=Path(b['patterns'])
  for p,key in [(new,'selected_sha256'),(old,'old_selected_sha256'),(patterns,'patterns_sha256')]:need(H(p.read_bytes())==b[key],key)
  need(json.loads(new.read_text())['commit']==final and json.loads(old.read_text())['commit']==OLD,'selection commit')
  need(patterns.read_text()==''.join('/'+r['path']+'\n' for r in json.loads(new.read_text())['files']),'patterns/selection mismatch')
  config=Path(b['external_config']);need(H(config.read_bytes())==b['external_config_sha256'],'external config ledger hash')
  ledger=json.loads(config.read_text());need(os.environ.get('CARGO_HOME')==ledger['CARGO_HOME'],'CARGO_HOME changed')
  for row in ledger['rows']:
   cp=Path(row['path']);need(cp.exists()==row['exists'] and cp.is_symlink()==row['symlink'],'external config existence changed')
   if row['sha256'] is not None:need(H(cp.read_bytes())==row['sha256'],'external config bytes changed')
  lock=(TARGET/'.pencil-musl-owner.lock').open('a+');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
  conflicts=[]
  for p in Path('/proc').iterdir():
   if not p.name.isdigit() or int(p.name)==os.getpid():continue
   try:
    cmd=(p/'cmdline').read_bytes().replace(b'\0',b' ').decode(errors='replace');comm=(p/'comm').read_text().strip()
    target_env=next((v.split(b'=',1)[1].decode() for v in (p/'environ').read_bytes().split(b'\0') if v.startswith(b'CARGO_TARGET_DIR=')),None) if comm in ['cargo','rustc','rustdoc'] else None
    if comm in ['cargo','rustc','rustdoc','git','cc','gcc','ld','ld.lld'] and (str(ROOT) in cmd or str(TARGET) in cmd or target_env==str(TARGET) or os.path.realpath(p/'cwd')==str(ROOT) or os.path.realpath(p/'cwd').startswith(str(ROOT)+'/')):conflicts.append({'pid':p.name,'comm':comm})
   except (OSError,ProcessLookupError):pass
  need(not conflicts,'active source/cache owner '+repr(conflicts));(d/'binding.json').write_text(json.dumps(b,indent=2))
  need(run('toplevel',['rev-parse','--show-toplevel']).decode().strip()==str(ROOT),'wrong checkout')
  need(run('detached-before',['rev-parse','--abbrev-ref','HEAD']).strip()==b'HEAD','not detached')
  need(run('head-before',['rev-parse','HEAD']).decode().strip()==OLD,'old HEAD mismatch; reconcile rather than retry')
  run('tree-before',['rev-parse','HEAD^{tree}']);run('clean-before',['diff-index','--quiet','HEAD','--']);need(not run('all-untracked-before',['ls-files','--others']).strip(),'unexpected untracked including ignored')
  verify(old,'old-materialized')
  result['mutations_started']=True
  run('patterns',['sparse-checkout','set','--no-cone','--stdin'],patterns.read_bytes())
  run('checkout',['checkout','--detach',final]);run('reapply',['sparse-checkout','reapply'])
  need(run('head-after',['rev-parse','HEAD']).decode().strip()==final,'new HEAD mismatch');need(run('tree-after',['rev-parse','HEAD^{tree}']).decode().strip()==b['tree'],'new tree mismatch');need(run('detached-after',['rev-parse','--abbrev-ref','HEAD']).strip()==b'HEAD','not detached')
  run('clean-after',['diff-index','--quiet','HEAD','--']);need(not run('all-untracked-after',['ls-files','--others']).strip(),'unexpected untracked including ignored');verify(new,'new-materialized');result['status']='refreshed_verified_no_build'
 except BaseException as e:result.update({'status':'failed_reconcile_same_destination','error':type(e).__name__+': '+str(e)})
 finally:
  result.update({'elapsed_seconds':time.monotonic()-start,'cap_seconds':a.seconds,'source_root':str(ROOT),'target_lock':str(TARGET/'.pencil-musl-owner.lock'),'lock_released_at_return':True});(d/'receipt.json').write_text(json.dumps(result,indent=2));
  if lock is not None:lock.close()
 return 0 if result['status']=='refreshed_verified_no_build' else 1
if __name__=='__main__':raise SystemExit(main())
