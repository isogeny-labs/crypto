#!/usr/bin/env python3
"""One-time deterministic byte seal; not mathematical replay."""
from pathlib import Path
import hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3];rid=int(h.name)
assert not(h/'runs.tar.xz').exists();r=json.loads((h/'results.json').read_text());assert {x['attempt'] for x in r['runs']}=={p.name for p in(h/'runs').iterdir() if p.is_dir()}
assert all(x['status']=='accepted' for x in r['runs'] if x['kind']=='native')
assert all(x['status'] in ('accepted','rejected') for x in r['runs'])
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip();tracked=subprocess.check_output(['git','ls-files','--',str(h.relative_to(repo))],cwd=repo,text=True).splitlines()
exclude={'.artifacts.json','.gitignore','report.pdf','artifact-verification.json','source.sha256','source.files','science.files','source-revision.txt','raw-meta.json','runs.sha256','runs.tar.xz.sha256','ARTIFACTS.md','publication-validation.json','VISUAL_REVIEW.md','visual-review.json','export.json','CONTINUATION.md'}
files=[str((repo/p).relative_to(h)) for p in tracked if Path(p).name not in exclude and not str((repo/p).relative_to(h)).startswith(('runs/','source-data/'))]
files+=['../35/ALG35.sing','../35/raw-meta.json','../35/runs.sha256','../37/results.json','../37/runs.sha256','../39/results.json','../39/runs.sha256','../39/FACTOR_CONSTRUCTION.md','../40/results.json','../40/runs.sha256','../15/CM.md','../18/CM.md','../35/JET35.sing','../18/CM.md','../34/LOCAL_GEOMETRY.md','../34/PREFIX34-CHECK.sing','../34/runs.sha256','../34/source-data/proof-cache-source-roots.txt','../../../../scripts/checkpoint.sh','../../../../tools/lean/AuditAxioms.lean','../27/VERIFY_ARCHIVE.py'];files=sorted(set(files))
for name in ['science.files','source.files']:(h/name).write_text('\n'.join(files)+'\n')
(h/'source.sha256').write_text(''.join(hashlib.sha256((h/p).read_bytes()).hexdigest()+'  '+p+'\n' for p in files));(h/'source-revision.txt').write_text(revision+'\n')
contents=['runs','source-data'];members=[]
for name in contents:
 for p in sorted((h/name).rglob('*')):
  assert not p.is_symlink()
  if p.is_file():members.append(p)
(h/'runs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(h))+'\n' for p in sorted(members)))
flags=['--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner'];subprocess.run(['tar',*flags,'-cJf',str(h/'runs.tar.xz'),'-C',str(h),*contents],check=True)
raw=h/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest();(h/'runs.tar.xz.sha256').write_text(digest+'  runs.tar.xz\n')
m={'schema':'koblitz.raw-meta/v1','round':rid,'source_revision':revision,'source_pins':len(files),'archive_contents':contents,'tar_flags':flags,'raw_sha256':digest,'bytes':raw.stat().st_size,'regular_files':len(members),'fresh_download_verified':False,'state':'sealed locally; remote verification follows'};(h/'raw-meta.json').write_text(json.dumps(m,indent=2)+'\n');print(json.dumps(m))
