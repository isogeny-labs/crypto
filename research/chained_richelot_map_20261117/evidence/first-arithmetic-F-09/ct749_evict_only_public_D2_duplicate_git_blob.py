"""Owned published Git-cache representation only; primary full archive retained."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,subprocess
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';source='f35867831245560527428e3562ab55d74eefb5a0'
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def run(*args):
 p=subprocess.run(['git','-c','gc.auto=0',*args],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=30);assert p.returncode==0,(args,p.stderr);return p.stdout
assert run('rev-parse','HEAD').decode().strip()==source
assert Path('/workspace/scratch/ct748-source-F-exact-readback.txt').read_text().split()[0]==source
rel='research/chained_richelot_map_20261117/infrastructure-first-strict-Clippy-D2-07/complete-first-strict-Clippy-D2-source-full-rlibs-compiled-ELF-and-logs.tar.xz';primary=r/rel
oid=run('rev-parse',source+':'+rel).decode().strip();assert run('hash-object',str(primary)).decode().strip()==oid
assert run('cat-file','blob',oid)==primary.read_bytes()
tree=run('rev-parse',source+':'+str(Path(rel).parent)).decode().strip()
fetch=run('-c','fetch.negotiationAlgorithm=noop','fetch','--filter=tree:0','--no-write-fetch-head','--no-tags','origin',tree)
objects=(r/'.git/objects').resolve();assert objects.is_relative_to(Path('/run/lock/chained-transfers-ct669'))
cache=objects/oid[:2]/oid[2:];assert cache.is_file()
before={'path':str(cache),'bytes':cache.stat().st_size,'sha256':hashlib.sha256(cache.read_bytes()).hexdigest()}
quarantine=cache.with_name(cache.name+'.ct749-isolated');assert not quarantine.exists();cache.rename(quarantine)
checks=[]
try:
 e=s/'compiler-revision-F';lock=e/'SOURCE_LOCK_14F.json';manifest=json.loads(lock.read_text())
 for row in manifest['files']+[{'path':lock.name}]:
  p=e/row['path'];assert run('cat-file','blob',source+':'+str(p.relative_to(r)))==p.read_bytes();checks.append(str(p.relative_to(r)))
 for path in ['pcc/ledger.jsonl','pcc/keyring.json']:
  assert run('cat-file','blob',source+':'+path)==(r/path).read_bytes();checks.append(path)
 assert run('hash-object',str(primary)).decode().strip()==oid
except BaseException:
 quarantine.rename(cache);raise
quarantine.unlink()
receipt={'time':datetime.now(timezone.utc).isoformat(),'public_source':source,'public_parent_tree_normal_promisor_refetch':tree,'evicted_owned_duplicate_cache':before,'retained_complete_primary_archive':{'path':rel,'bytes':primary.stat().st_size,'sha256':hashlib.sha256(primary.read_bytes()).hexdigest(),'Git_blob':oid},'all_full_frozen_source_and_PCC_reads_without_cache':checks,'original_shared_objects_and_indices_untouched':True,'no_scientific_or_raw_primary_bytes_removed':True,'rehydration':'git hash-object -w retained exact primary archive, or ordinary origin fetch; never infer missing evidence from evicted cache','scratch_free_after':os.statvfs('/run/lock').f_bavail*os.statvfs('/run/lock').f_frsize}
(s/'infrastructure-first-strict-Clippy-D2-07/ACTUAL_PUBLIC_DUPLICATE_GIT_CACHE_EVICTION.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'reclaimed':before['bytes'],'scratch_free_after':receipt['scratch_free_after'],'frozen_full_source_PCC_controls':len(checks)}))
