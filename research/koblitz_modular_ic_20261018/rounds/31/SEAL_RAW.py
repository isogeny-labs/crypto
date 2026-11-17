#!/usr/bin/env python3
"""One-time deterministic artifact packaging and hashing, not math replay."""
from pathlib import Path
import hashlib, json, subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
assert not (h/'runs.tar.xz').exists()
assert (h/'results.json').exists()
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
tracked=subprocess.check_output(['git','ls-files','--',str(h.relative_to(repo))],cwd=repo,text=True).splitlines()
exclude={'.artifacts.json','.gitignore','report.pdf','artifact-verification.json','export.json',
 'source.sha256','source.files','science.files','source-revision.txt','raw-meta.json',
 'runs.sha256','runs.tar.xz.sha256','ARTIFACTS.md','publication-validation.json','VISUAL_REVIEW.md','visual-review.json'}
files=[str((repo/p).relative_to(h)) for p in tracked
       if Path(p).name not in exclude and not str((repo/p).relative_to(h)).startswith(('runs/','source-data/','publication-attempts/'))]
files += ['../27/LAMBDA14.sing','../../native/src/carrier16.rs','../30/COLOR30.sing',
          '../15/CM.md','../../native/src/cmray15.rs','../../../../tools/lean/AuditAxioms.lean',
          '../27/VERIFY_ARCHIVE.py','source-data/proof-cache-source-roots.txt']
files=sorted(set(files))
(h/'science.files').write_text('\n'.join(files)+'\n');(h/'source.files').write_text('\n'.join(files)+'\n')
(h/'source.sha256').write_text(''.join(hashlib.sha256((h/p).read_bytes()).hexdigest()+'  '+p+'\n' for p in files))
(h/'source-revision.txt').write_text(revision+'\n')
contents=['runs','source-data','publication-attempts'];members=[]
for name in contents:
 for p in sorted((h/name).rglob('*')):
  assert not p.is_symlink()
  if p.is_file():members.append(p)
(h/'runs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(h))+'\n' for p in sorted(members)))
flags=['--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner']
subprocess.run(['tar',*flags,'-cJf',str(h/'runs.tar.xz'),'-C',str(h),*contents],check=True)
raw=h/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest()
(h/'runs.tar.xz.sha256').write_text(digest+'  runs.tar.xz\n')
meta={'schema':'koblitz.raw-meta/v1','round':31,'source_revision':revision,'source_pins':len(files),
 'archive_contents':contents,'tar_flags':flags,'raw_sha256':digest,'bytes':raw.stat().st_size,
 'regular_files':len(members),'fresh_download_verified':False,'state':'sealed locally; remote verification follows'}
(h/'raw-meta.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta))
