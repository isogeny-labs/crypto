"""Preserve complete regenerable Git promisor cache bytes in RAM; prevent automatic repack duplication."""
from pathlib import Path
from datetime import datetime,timezone
import os,hashlib,json,shutil,stat,subprocess
raw=Path('/dev/shm/ct99-remote-cache-relocation-03');raw.mkdir();shutil.copyfile(__file__,raw/'original-command.py')
env=dict(os.environ,GIT_NO_LAZY_FETCH='1')
repos=[Path('/workspace/crypto'),Path('/workspace/cryptanalysis')]
def run(r,*args):return subprocess.check_output(['git',*args],cwd=r,env=env)
def digest(p):
 with p.open('rb')as f:return hashlib.file_digest(f,'sha256').hexdigest()
baselines={str(r):dict(refs=run(r,'show-ref').decode(),head=run(r,'rev-parse','HEAD').decode(),status=run(r,'status','--porcelain').decode())for r in repos}
# Existing uncommitted custody projection is allowed and must remain byte-for-byte status-identical.
(raw/'before.json').write_text(json.dumps(baselines,indent=2)+'\n')
for r in repos:
 for key,value in [('gc.auto','0'),('maintenance.auto','false')]:subprocess.run(['git','config','--local',key,value],cwd=r,env=env,check=True)
rows=[]
for r in repos:
 store=raw/r.name;store.mkdir()
 candidates=[p for p in(r/'.git/objects/pack').glob('pack-*.pack')if not p.is_symlink()and p.with_suffix('.promisor').is_file()and p.stat().st_size>=128*1024*1024]
 for p in candidates:
  st=p.stat();dest=store/p.name;shutil.copy2(p,dest)
  with p.open('rb')as x,dest.open('rb')as y:
   while True:
    a,b=x.read(1048576),y.read(1048576);assert a==b
    if not a:break
  h=digest(p);assert digest(dest)==h and stat.S_IMODE(dest.stat().st_mode)==stat.S_IMODE(st.st_mode)
  rows.append(dict(original_path=str(p),RAM_path=str(dest),bytes=st.st_size,mode=stat.S_IMODE(st.st_mode),sha256=h,remote_promisor_marker=str(p.with_suffix('.promisor')),full_byte_comparison_before_relocation=True))
  (raw/'whole-cache-manifest-in-progress.json').write_text(json.dumps(rows,indent=2)+'\n')
  p.unlink();p.symlink_to(dest);assert digest(p)==h
  print(r.name,p.name,st.st_size,flush=True)
for r in repos:
 b=baselines[str(r)]
 assert run(r,'show-ref').decode()==b['refs']and run(r,'rev-parse','HEAD').decode()==b['head']and run(r,'status','--porcelain').decode()==b['status']
assert rows
v=os.statvfs('/workspace')
out=dict(at=datetime.now(timezone.utc).isoformat(),whole_remote_cache_packs=rows,all_refs_HEAD_status_unchanged=True,all_research_sources_raw_executables_prepared_copies_unchanged=True,gc_auto='0',maintenance_auto='false',available_bytes=v.f_bavail*v.f_frsize,recovery='If RAM is lost, restore whole cache bytes from RAM when available, or remove only listed broken symlinks and their same-basename idx/rev/promisor/mtimes remote-cache metadata so the partial clone refetches immutable objects. Never remove local/nonpromisor objects, refs or research files.',scope='Cache relocation, no history rewrite or scientific change')
(raw/'outcome.json').write_text(json.dumps(out,indent=2)+'\n')
persist=Path('/workspace/scratch/ct99-remote-cache-relocation-03');persist.mkdir()
for p in raw.iterdir():
 if p.is_file():shutil.copyfile(p,persist/p.name)
print(json.dumps(out),flush=True)
