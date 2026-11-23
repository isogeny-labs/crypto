"""Full-byte checked owned storage relocation; no mount/device/policy change."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto-chained-balanced-main');s=r/'research/chained_richelot_map_20261117';dst=Path('/dev/net/chained-transfers-owned-storage-20261117');assert dst.is_dir() and dst.stat().st_uid==os.getuid()
objects=(r/'.git/objects').resolve();assert str(objects)=='/run/lock/chained-transfers-ct669/owned-canonical-gitdir/objects'
packs=Path('/dev/shm/ct703-canonical-owned-packs');assert packs.is_dir() and not packs.is_symlink()
env=dict(os.environ,GIT_INDEX_FILE='/run/lock/chained-transfers-ct669/canonical-current-index',GIT_OPTIONAL_LOCKS='0',GIT_NO_LAZY_FETCH='1')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def inventory(root):
 rows=[]
 for base,dirs,files in os.walk(root,followlinks=False):
  for n in sorted(dirs+files):
   p=Path(base)/n;row={'path':str(p.relative_to(root)),'mode':p.lstat().st_mode&0o7777}
   if p.is_symlink():row.update(kind='symlink',target=os.readlink(p))
   elif p.is_file():row.update(kind='file',bytes=p.stat().st_size,sha256=sha(p))
   elif p.is_dir():row.update(kind='dir')
   else:raise RuntimeError('nonregular owned cache '+str(p))
   rows.append(row)
 return sorted(rows,key=lambda x:x['path'])
records=[];backups=[]
for old,name in [(objects,'canonical-owned-objects'),(packs,'canonical-owned-packs')]:
 new=dst/name;assert not new.exists();before=inventory(old);shutil.copytree(old,new,symlinks=True);assert inventory(new)==before
 backup=old.with_name(old.name+'.ct752-before-checked-relocation');assert not backup.exists();old.rename(backup);old.symlink_to(new,target_is_directory=True)
 backups.append((old,backup));records.append({'old_path':str(old),'new_owned_storage':str(new),'all_full_bytes_modes_symlink_targets_equal':True,'inventory':before})
try:
 source='f35867831245560527428e3562ab55d74eefb5a0';e=s/'compiler-revision-F';lock=e/'SOURCE_LOCK_14F.json';checks=[]
 for row in json.loads(lock.read_text())['files']+[{'path':lock.name}]:
  p=e/row['path'];q=subprocess.run(['git','cat-file','blob',source+':'+str(p.relative_to(r))],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert q.returncode==0 and q.stdout==p.read_bytes();checks.append(str(p.relative_to(r)))
 q=subprocess.run(['git','cat-file','blob',source+':pcc/ledger.jsonl'],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert q.returncode==0 and (r/'pcc/ledger.jsonl').read_bytes().startswith(q.stdout);checks.append('ledger full prefix')
 q=subprocess.run(['git','cat-file','blob',source+':pcc/keyring.json'],cwd=r,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE);assert q.returncode==0 and (r/'pcc/keyring.json').read_bytes()==q.stdout;checks.append('keyring full equality')
except BaseException:
 for old,backup in reversed(backups):old.unlink();backup.rename(old)
 raise
for old,backup in backups:shutil.rmtree(backup)
old=Path('/dev/shm/ct751-retained-primary-archives/complete-first-strict-Clippy-D2-source-full-rlibs-compiled-ELF-and-logs.tar.xz');new=dst/old.name
assert old.is_file() and not new.exists();shutil.copy2(old,new);assert old.read_bytes()==new.read_bytes() and old.stat().st_mode==new.stat().st_mode
primary={'old_path':str(old),'new_retained_primary':str(new),'bytes':new.stat().st_size,'sha256':sha(new),'all_bytes_and_mode_equal':True};old.unlink()
receipt={'time':datetime.now(timezone.utc).isoformat(),'scope':'ordinary private regular files on unused existing local /dev tmpfs; no device node, mount, network, provider, privilege or experiment-limit change','owned_cache_relocations':records,'all_frozen_source_and_PCC_reads_after':checks,'full_D2_primary_relocation':primary,'protected_original_shared_objects_and_all_indices_untouched':True,'old_logical_cache_paths_remain_via_symlinks':True,'free_after':{str(p):shutil.disk_usage(p).free for p in [r,Path('/dev/shm'),Path('/run/lock'),dst]}}
out=s/'evidence/first-arithmetic-F-09/ACTUAL_OWNED_STORAGE_RELOCATION.json';out.write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({'complete_regular_file_cache_entries':sum(len(x['inventory']) for x in records),'source_PCC_controls':len(checks),'free_after':receipt['free_after']}))
