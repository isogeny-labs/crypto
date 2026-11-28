"""Write-once deterministic byte custody after all mathematical jobs terminate."""
from pathlib import Path
import hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
assert not (h/'runs.tar.xz').exists()
r=json.loads((h/'results.json').read_text())
assert len(r['runs'])==len([p for p in (h/'runs').iterdir() if p.is_dir()])
assert all(x['status'] in ('accepted','rejected') for x in r['runs'])
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
tracked=subprocess.check_output(['git','ls-files','--',str(h.relative_to(repo))],cwd=repo,text=True).splitlines()
exclude={'.artifacts.json','report.pdf','source.sha256','source.files','science.files','source-revision.txt','raw-meta.json','runs.sha256','runs.tar.xz.sha256','ARTIFACTS.md','artifact-verification.json','.gitignore'}
files=[str((repo/name).relative_to(h)) for name in tracked if Path(name).name not in exclude]
files+=['../../../../tools/lean/AuditAxioms.lean','../../../../scripts/checkpoint.sh','../27/VERIFY_ARCHIVE.py']
files+=[row['path'] for row in json.loads((h/'donors.json').read_text())]
files=sorted(set(files))
for name in ['source.files','science.files']:(h/name).write_text('\n'.join(files)+'\n')
(h/'source.sha256').write_text(''.join(hashlib.sha256((h/p).read_bytes()).hexdigest()+'  '+p+'\n' for p in files))
(h/'source-revision.txt').write_text(revision+'\n')
contents=['runs','publication-checks'];members=[]
for folder in contents:
 for p in sorted((h/folder).rglob('*')):
  assert not p.is_symlink()
  if p.is_file():members.append(p)
(h/'runs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(h))+'\n' for p in sorted(members)))
flags=['--hard-dereference','--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner']
subprocess.run(['tar',*flags,'-cJf',str(h/'runs.tar.xz'),'-C',str(h),*contents],check=True)
raw=h/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest()
(h/'runs.tar.xz.sha256').write_text(digest+'  runs.tar.xz\n')
meta={'schema':'koblitz.raw-meta/v1','round':57,'source_revision':revision,'source_pins':len(files),'archive_contents':contents,'tar_flags':flags,'raw_sha256':digest,'bytes':raw.stat().st_size,'regular_files':len(members),'fresh_download_verified':False,'state':'sealed locally; remote verification follows'}
(h/'raw-meta.json').write_text(json.dumps(meta,indent=2)+'\n');print(json.dumps(meta))
