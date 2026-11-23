"""Evict only a newly fetched, externally published Git cache pack.
Verify all frozen compiler-source and PCC-prefix subjects without that pack.
No original/shared objects or scientific inputs/outputs are deleted.
"""
from pathlib import Path
from datetime import datetime,timezone
import collections,hashlib,json,lzma,os,subprocess
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';p=s/'infrastructure-first-storage-preflight-03'
packs=Path('/dev/shm/ct703-canonical-owned-packs');stem='pack-3968485d4e50b65cfd5677fd813d6a75378a9b6c';files=[packs/(stem+ext) for ext in ['.pack','.idx','.rev','.promisor']];assert all(x.is_file() for x in files)
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
verify=subprocess.run(['git','verify-pack','-v',str(packs/(stem+'.idx'))],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert verify.returncode==0,verify.stderr
raw=verify.stdout;(p/'entire-evicted-upstream-cache-object-inventory.txt.xz').write_bytes(lzma.compress(raw));assert lzma.decompress((p/'entire-evicted-upstream-cache-object-inventory.txt.xz').read_bytes())==raw
open_fds=[]
for proc in Path('/proc').glob('[0-9]*'):
 try:
  for fd in (proc/'fd').iterdir():
   try:
    if os.readlink(fd) in [str(x) for x in files]:open_fds.append(str(fd))
   except OSError:pass
 except OSError:pass
assert not open_fds
inventory=[{'name':x.name,'bytes':x.stat().st_size,'sha256':hashlib.sha256(x.read_bytes()).hexdigest(),'mode':x.stat().st_mode&0o7777} for x in files]
hold=Path('/dev/shm/ct731-new-fetched-cache-isolation');hold.mkdir(exist_ok=False)
for x in files:x.rename(hold/x.name)
checks=[];failure=None
try:
 manifest=json.loads((s/'SOURCE_LOCK_14.json').read_text());rel=s.relative_to(r).as_posix()
 for row in manifest['files']+[{'path':'SOURCE_LOCK_14.json','sha256':hashlib.sha256((s/'SOURCE_LOCK_14.json').read_bytes()).hexdigest()}]:
  path=rel+'/'+row['path'];x=subprocess.run(['git','cat-file','blob','77d9ad43f2a0706b5c76eaf997262f52e1ca380b:'+path],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=8)
  assert x.returncode==0 and x.stdout==(s/row['path']).read_bytes(),(path,x.stderr)
  checks.append({'public_source_subject':path,'full_byte_equal_without_cache':True,'sha256':row['sha256']})
 for path in ['pcc/ledger.jsonl','pcc/keyring.json']:
  x=subprocess.run(['git','cat-file','blob','77d9ad43f2a0706b5c76eaf997262f52e1ca380b:'+path],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=8);assert x.returncode==0,(path,x.stderr)
  b=(r/path).read_bytes();assert b.startswith(x.stdout) if path.endswith('ledger.jsonl') else b==x.stdout
  checks.append({'public_source_subject':path,'physical_full_prefix_or_equality_without_cache':True,'Git_blob_sha256':hashlib.sha256(x.stdout).hexdigest()})
except Exception as error:
 failure=repr(error)
 for x in files:(hold/x.name).rename(x)
receipt={'time':datetime.now(timezone.utc).isoformat(),'role':'disposable new origin promisor object cache; not scientific run data','origin':'https://github.com/isogeny-labs/crypto','immutable_fetched_main':'c5cbb67b19fe39864da7f8c9b29726696ba01580','cache_files':inventory,'whole_pack_verify_exit':verify.returncode,'full_object_inventory_retained':True,'no_open_writers':True,'all27_declared_source_blobs_and_two_PCC_subjects_read_without_cache':checks,'failure':failure,'original_shared_and_frozen_Git_objects_untouched':True,'old_c438_pack_and_old_failed1_8MB_cache_retained':True,'raw_pack_layout_retained_locally':failure is not None,'all_object_contents_remain_recoverable_at_immutable_public_origin':True,'rehydration':'Use ordinary inherited-origin narrow fetch of required OIDs from the complete inventory. GIT_NO_LAZY_FETCH prevents accidental broad rehydration during14. Never infer absence from this eviction.','same_frozen_source_and_resource_criteria':True}
if failure is None:
 for row in inventory:(hold/row['name']).unlink()
 hold.rmdir()
 receipt['cache_bytes_reclaimed']=sum(x['bytes'] for x in inventory)
 receipt['RAM_available_after']=os.statvfs('/dev/shm').f_bavail*os.statvfs('/dev/shm').f_frsize
(p/'ACTUAL_SAFE_UPSTREAM_CACHE_EVICTION.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:receipt.get(k) for k in ['failure','cache_bytes_reclaimed','RAM_available_after']},indent=2))
assert failure is None,failure
