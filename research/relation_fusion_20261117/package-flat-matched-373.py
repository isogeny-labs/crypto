from pathlib import Path
import json,hashlib,shutil,tarfile,os,re
repo=Path('/workspace/relation-supervisor-live');s=repo/'research/relation_fusion_20261117';pkg=Path('/workspace/flat-matched-package-373');pkg.mkdir()
def copy(src,dst):
 p=pkg/dst;p.parent.mkdir(parents=True,exist_ok=True);shutil.copy2(src,p)
for src,dst in [('flat-supervised-neutral-367','neutral-367'),('flat-supervised-hypothetical20-368','hypothetical20-368')]:shutil.copytree(Path('/workspace')/src,pkg/dst,copy_function=os.link)
ctx=json.loads((s/'FLAT_MATCHED_SYNTHETIC_SOURCE_366.json').read_text())
for p,h in ctx['source_sha256'].items():assert hashlib.sha256((repo/p).read_bytes()).hexdigest()==h;copy(repo/p,Path('source')/p)
old=json.loads((s/'SUPERVISED_SYNTHETIC_SOURCE_253.json').read_text())
for p,h in old['source_sha256'].items():assert hashlib.sha256((repo/p).read_bytes()).hexdigest()==h;copy(repo/p,Path('source')/p)
for p,h in ctx['tool_sha256'].items():assert hashlib.sha256(Path(p).read_bytes()).hexdigest()==h;copy(p,Path('bin')/Path(p).name)
shutil.copytree('/workspace/flat-supervised-before-365',pkg/'before-365')
for n in ['prepare-flat-supervised-364.py','freeze-flat-draft-366.py','check-flat-canaries-369.py','render-flat-matched-372.py']:copy(Path('/workspace')/n,Path('scripts')/n)
for p in s.iterdir():
 m=re.search(r'_(\d+)\.',p.name)
 if p.is_file() and m and 363<=int(m[1])<=374:copy(p,Path('study')/p.name)
for n in ['SUPERVISED_SYNTHETIC_SOURCE_253.json','flat-matched-report-372.typ','flat-matched-report-372.pdf']:copy(s/n,Path('study')/n)
for p in (s/'visual-372').iterdir():copy(p,Path('study/visual-372')/p.name)
for p in (s/'development-01').iterdir():
 m=re.search(r'-(\d+)\.',p.name)
 if p.is_file() and m and 364<=int(m[1])<=370:copy(p,Path('logs')/p.name)
source=Path('/workspace/flat-supervised-neutral-367')
for num,mode in [(369,'clock'),(370,'duplicate')]:
 src=Path(f'/workspace/flat-supervised-{mode}-canary-{num}');delta=[]
 for p in sorted(src.rglob('*')):
  if p.is_file():
   rel=p.relative_to(src)
   if not (source/rel).is_file() or p.read_bytes()!=(source/rel).read_bytes():copy(p,Path('canary-deltas')/mode/rel);delta.append(str(rel))
 dst=pkg/'canary-deltas'/mode;(dst/'delta-roster.json').write_text(json.dumps({'base':'neutral-367','mode':mode,'changed_files':delta,'replace_atomically':True},indent=2)+'\n')
copy(__file__,Path('package.py'))
files=[{'path':str(p.relative_to(pkg)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(pkg.rglob('*')) if p.is_file()]
(pkg/'index.json').write_text(json.dumps({'schema':'flat-matched-synthetic-archive/v1','synthetic_only':True,'performance_observations_collected':0,'signed_execution_verdict':False,'files':files},indent=2)+'\n')
a=s/'flat-matched-qualification-373.tar.xz'
with tarfile.open(a,'x:xz',format=tarfile.GNU_FORMAT,preset=6) as t:
 for p in sorted(pkg.rglob('*')):
  if p.is_file():
   info=t.gettarinfo(str(p),str(p.relative_to(pkg)));info.mtime=1795392000;info.uid=info.gid=0;info.uname=info.gname='';info.mode=0o755 if p.stat().st_mode&0o111 else 0o644
   if info.islnk():t.addfile(info)
   else:
    with p.open('rb') as f:t.addfile(info,f)
h=hashlib.sha256(a.read_bytes()).hexdigest();(s/(a.name+'.sha256')).write_text(f'{h}  {a.name}\n');r={'schema':'flat-matched-synthetic-archive-receipt/v1','archive':a.name,'sha256':h,'bytes':a.stat().st_size,'indexed_files':len(files),'uncompressed_path_bytes':sum(x['bytes'] for x in files),'synthetic_cohorts':2,'cells_per_cohort':3600,'complete_analytic_proof_replays':7200,'stored_canary_deltas':2,'performance_observations_collected':0,'signed_execution_verdict':False,'15x_admitted':False,'policy':'LocalCPU0/one thread/zero cloud/checkpointS3waiver; deterministic GNU tar preserving internalhardlinks,xz single-threadpreset6'};(s/'FLAT_MATCHED_ARCHIVE_373.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
