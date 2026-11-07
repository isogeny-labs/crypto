#!/usr/bin/env python3
"""Seal exact inputs/outputs; archive and hash operations only, no math replay."""
from pathlib import Path
import hashlib,json,subprocess
base=Path(__file__).resolve().parent.parent;repo=base.parents[2]
revision=subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip()
for rid in [27,28,29]:
 here=base/str(rid);archive=here/'runs.tar.xz';assert not archive.exists(),str(archive)+' already sealed'
 tracked=subprocess.check_output(['git','ls-files',str(here.relative_to(repo))],cwd=repo,text=True).splitlines()
 scientific=[]
 for name in tracked:
  p=repo/name
  if p.suffix in ['.sing','.rs','.lean','.sh','.py','.typ','.svg','.md','.json','.sha256'] and p.name not in ['source.sha256','runs.sha256','runs.tar.xz.sha256']:
   scientific.append(p)
 scientific.append(repo/'tools/lean/AuditAxioms.lean')
 if rid==27:scientific.extend(base/str(n)/'CM.md'for n in [22,23,24,25,26])
 if rid==28:scientific.extend([base/'15/REPORT.md',base.parent/'native/src/norms15.rs',base.parent/'native/src/ordinary13.rs',base.parent/'native/src/round04.rs'])
 if rid==29:scientific.extend([base/'27/CM.md',base/'28/DUAL28.rs'])
 import os
 scientific=sorted(set(scientific),key=lambda p:os.path.relpath(p,here))
 (here/'source.files').write_text(''.join(os.path.relpath(p,here)+'\n'for p in scientific))
 (here/'source.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+os.path.relpath(p,here)+'\n'for p in scientific))
 (here/'source-revision.txt').write_text(revision+'\n')
 contents=['runs','source-data']+(['native']if rid in [28,29]else[])
 members=sorted([p for part in contents for p in (here/part).rglob('*')if p.is_file()],key=lambda p:str(p.relative_to(here)))
 assert all(not p.is_symlink()for p in members)
 (here/'science.files').write_text(''.join(str(p.relative_to(here))+'\n'for p in members))
 (here/'runs.sha256').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest()+'  '+str(p.relative_to(here))+'\n'for p in members))
 subprocess.run(['tar','--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner','-cJf',str(archive),'-C',str(here),*contents],check=True)
 digest=hashlib.sha256(archive.read_bytes()).hexdigest()
 (here/'runs.tar.xz.sha256').write_text(digest+'  runs.tar.xz\n')
 info={'schema':'koblitz.sealed-raw/v1','round':rid,'source_revision':revision,'raw_sha256':digest,'raw_bytes':archive.stat().st_size,'raw_regular_files':len(members),'source_pin_count':len(scientific),'archive_contents':contents,'tar_flags':['--format=gnu','--sort=name','--mtime=@0','--owner=0','--group=0','--numeric-owner'],'fresh_download_verified':False}
 (here/'raw-meta.json').write_text(json.dumps(info,indent=2)+'\n');print(json.dumps(info))
