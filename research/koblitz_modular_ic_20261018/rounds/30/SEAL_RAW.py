#!/usr/bin/env python3
"""One-time deterministic metadata/raw seal, not mathematical computation."""
from pathlib import Path
import hashlib,json,subprocess
here=Path(__file__).resolve().parent;repo=here.parents[3]
assert not(here/'runs.tar.xz').exists()
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
tracked=subprocess.check_output(['git','ls-files','--',str(here.relative_to(repo))],cwd=repo,text=True).splitlines()
exclude={'report.pdf','visual-review.json','VISUAL_REVIEW.md','.artifacts.json','.gitignore','source.sha256','source.files','science.files','source-revision.txt','raw-meta.json','runs.sha256','runs.tar.xz.sha256','artifact-verification.json','export.json','publication-validation.json','ARTIFACTS.md'}
files=[str((repo/p).relative_to(here)) for p in tracked if Path(p).name not in exclude]
files+=['../22/RECOVERY.md','../27/CM.md','../29/ACCEPTANCE.md','../29/NEXT.md','../../native/src/ordinary13_lift.rs','../22/runs/proof-cache-source-roots.txt','../../../../tools/lean/AuditAxioms.lean','../../../../tools/progress_metrics.py','../27/VERIFY_ARCHIVE.py']
files=sorted(set(files));(here/'science.files').write_text('\n'.join(files)+'\n');(here/'source.files').write_text('\n'.join(files)+'\n');(here/'source.sha256').write_text(''.join(hashlib.sha256((here/p).read_bytes()).hexdigest()+'  '+p+'\n' for p in files));(here/'source-revision.txt').write_text(revision+'\n')
contents=['runs','source-data','publication-attempts'];members=[]
for name in contents:
 for p in sorted((here/name).rglob('*')):
  assert not p.is_symlink()
  if p.is_file():members.append(p)
(here/'runs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(here))+'\n' for p in sorted(members)))
flags=['--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner']
subprocess.run(['tar',*flags,'-cJf',str(here/'runs.tar.xz'),'-C',str(here),*contents],check=True)
raw=here/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest();(here/'runs.tar.xz.sha256').write_text(digest+'  runs.tar.xz\n')
meta={'schema':'koblitz.raw-meta/v1','round':30,'source_revision':revision,'source_pins':len(files),'archive_contents':contents,'tar_flags':flags,'raw_sha256':digest,'bytes':raw.stat().st_size,'regular_files':len(members),'fresh_download_verified':False,'state':'sealed locally; fresh object verification is a later publication step'}
(here/'raw-meta.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta))
