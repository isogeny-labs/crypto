#!/usr/bin/env python3
"""Existing scoped private S3 custody; no permission renewal or infrastructure."""
import argparse,datetime,hashlib,json,os,pathlib,stat,subprocess,time

def require(ok,msg):
 if not ok:raise RuntimeError(msg)
def main(root,authorization):
 root=pathlib.Path(root).resolve();auth=pathlib.Path(authorization)
 require(auth.stat().st_size<=16384 and not auth.is_symlink(),'custody config cap')
 a=json.loads(auth.read_text());expiry=datetime.datetime.fromisoformat(a['deadline'].replace('Z','+00:00'))
 remaining=(expiry-datetime.datetime.now(datetime.timezone.utc)).total_seconds();require(remaining>0,'original custody deadline expired')
 deadline=time.monotonic()+min(180,remaining)
 out=root/'s3-readback';out.mkdir();logs=[];inventory=[];result={'status':'FAIL','reason':'interrupted before completion'};read_total=0
 def check():require(time.monotonic()<deadline and datetime.datetime.now(datetime.timezone.utc)<expiry,'original custody deadline exhausted')
 def meta(p):
  check();s=p.lstat();require(stat.S_ISREG(s.st_mode),'nonregular custody input');return s.st_size
 def digest(p,size,cap):
  nonlocal read_total
  require(size<=cap,'pre-read byte cap');check();h=hashlib.sha256();read=0
  with os.fdopen(os.open(p,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK),'rb') as f:
   require(stat.S_ISREG(os.fstat(f.fileno()).st_mode) and os.fstat(f.fileno()).st_size==size,'changed input size')
   while read<size:
    check();block=f.read(min(65536,size-read));require(block,'short input');read+=len(block);read_total+=len(block)
    require(read_total<=2*a['max_bytes']+4*1048576,'cumulative read cap');h.update(block)
   check();require(not f.read(1),'input grew')
  return h.hexdigest()
 try:
  require(a['existing_authorized_prefix'] is True and a['private_bucket_verified'] is True and a['new_budget_authority'] is False,'existing private custody evidence required')
  require(0<a['max_cost_usd']<=a['remaining_custody_usd'],'existing custody funds exhausted/unknown')
  require(a['bucket']=='setec-astronomy-artifacts' and a['prefix'].startswith('ml-dataset/20261105/checkpoints/') and a['prefix'].endswith('/') and '..' not in a['prefix'].split('/'),'outside existing prefix')
  require(a['aws_profile']=='ml-dataset-checkpoint' and a['aws_config_file']=='/home/ec2-user/ml-dataset-resume-20261028/aws-config' and a['aws_region']=='us-east-2','scoped AWS configuration mismatch')
  require(0<a['max_bytes']<=1073741824 and 0<a['max_files']<=10000,'custody cap')
  for pathkey,hashkey in [('evidence_path','evidence_sha256'),('archive_script','archive_script_sha256'),('aws_config_file','aws_config_sha256')]:
   p=pathlib.Path(a[pathkey]);require(digest(p,meta(p),1048576)==a[hashkey],'custody identity evidence changed')
  env={k:os.environ[k] for k in ['PATH','HOME','LANG','LC_ALL'] if k in os.environ}
  env.update(AWS_CONFIG_FILE=a['aws_config_file'],AWS_PROFILE=a['aws_profile'],AWS_REGION=a['aws_region'],AWS_DEFAULT_REGION=a['aws_region'],AWS_MAX_ATTEMPTS='1',AWS_RETRY_MODE='standard',AWS_PAGER='')
  if a.get('aws_shared_credentials_file'):
   p=pathlib.Path(a['aws_shared_credentials_file']);require(digest(p,meta(p),1048576)==a['aws_shared_credentials_sha256'],'credential provider file changed');env['AWS_SHARED_CREDENTIALS_FILE']=str(p)
  source=root/'bundle';total=0
  for p in source.rglob('*'):
   check();require(not p.is_symlink(),'bundle symlink')
   if p.is_dir():continue
   size=meta(p);total+=size;rel=p.relative_to(source).as_posix();require('..' not in pathlib.PurePosixPath(rel).parts,'unsafe relative path');inventory.append({'path':rel,'bytes':size})
   require(len(inventory)<=a['max_files'] and total<=a['max_bytes'],'bundle exceeds admitted metadata caps')
  require(inventory and 2*len(inventory)<=a['max_requests'],'request cap')
  inventory.sort(key=lambda item:item['path'])
  for item in inventory:
   rel=item['path'];size=item['bytes'];p=source/rel;item['sha256']=digest(p,size,a['max_bytes']);dest=out/'bundle'/rel;dest.parent.mkdir(parents=True,exist_ok=True);key=a['prefix']+rel
   for op,argv in [('put',['put-object','--bucket',a['bucket'],'--key',key,'--body',str(p),'--if-none-match','*']),('get',['get-object','--bucket',a['bucket'],'--key',key,str(dest)])]:
    check();record={'operation':op,'key':key,'sha256':item['sha256'],'bytes':size};logs.append(record)
    args=['aws','--profile',a['aws_profile'],'--region',a['aws_region'],'--cli-connect-timeout','5','--cli-read-timeout','15','s3api',*argv]
    proc=subprocess.run(args,env=env,capture_output=True,timeout=min(25,deadline-time.monotonic()));check()
    record.update(exit_code=proc.returncode,stdout_sha256=hashlib.sha256(proc.stdout).hexdigest(),stderr_sha256=hashlib.sha256(proc.stderr).hexdigest())
    (out/f'{len(logs):05d}.stdout').write_bytes(proc.stdout);(out/f'{len(logs):05d}.stderr').write_bytes(proc.stderr)
    require(proc.returncode==0,'custody operation failed; no retry')
   require(meta(dest)==size and digest(dest,size,a['max_bytes'])==item['sha256'],'remote readback differs')
  result={'status':'PASS','files':inventory,'bytes_read':read_total};(out/'PASS.json').write_text(json.dumps(result))
 except BaseException as exc:
  result.update(error_type=type(exc).__name__,reason=str(exc));raise
 finally:
  (out/'receipts.json').write_text(json.dumps(logs,indent=2));(out/'inventory.json').write_text(json.dumps(inventory,indent=2));(out/'result.json').write_text(json.dumps(result,indent=2))
if __name__=='__main__':
 p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--authorization',required=True);a=p.parse_args();main(a.root,a.authorization)
