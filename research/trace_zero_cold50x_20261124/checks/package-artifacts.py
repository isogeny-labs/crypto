#!/usr/bin/env python3
"""Complete byte custody; no numerical analysis."""
from pathlib import Path
import tarfile,hashlib,json,tempfile,os
p=Path('research/trace_zero_cold50x_20261124');scratch=Path('/workspace/scratch')
raw=[]
for d in ['cold50x-dense-calibration','cold50x-dense-pilot','cold50x-v2-calibration','cold50x-v2-pilot','cold50x-v2-primary','cold50x-inline-calibration','cold50x-inline-pilot','cold50x-inline-primary','cold50x-dense-equivalence','cold50x-isolated-equivalence','cold50x-inline-equivalence']:
 root=scratch/d;assert root.is_dir(),d
 raw.extend((f,'runs/'+d+'/'+str(f.relative_to(root))) for f in root.rglob('*') if f.is_file())
for name in ['baseline-cold','baseline-aa-cold']:raw.append((p/'profiles'/name/'callgrind.out','profiles/'+name+'/callgrind.out'))
for f in Path('research/trace_zero_projected_20261115/inputs').glob('*.json'):raw.append((f,'inputs/'+f.name))
raw.append((scratch/'linearized-workload-from-git.json','workload.json'))
bins=[]
for name in ['bin','bin-v2','inline/bin','invalid-preflight-v1']:
 for f in (p/name).iterdir():
  if f.is_file() and f.read_bytes()[:4]==b'\x7fELF':bins.append((f,name+'/'+f.name))
for f in [scratch/'cold50x-s4-envelope',scratch/'cold50x-s4-actual-window',Path('research/trace_zero_cold50_20261123/candidate3/bin/elimination')]:bins.append((f,'additional/'+f.name+('-frozen-profile' if f.name=='elimination' else '')))
manifest={'schema':'complete-archive-custody/v1','archives':[]}
for name,pairs in [('RAW-ALL.tar.xz',raw),('FROZEN-EXECUTABLES.tar.xz',bins)]:
 out=p/name;assert not out.exists(),out;seen=set();entries=[]
 with tarfile.open(out,'w:xz',preset=6) as t:
  for f,member in sorted(pairs,key=lambda v:v[1]):
   assert member not in seen;seen.add(member);b=f.read_bytes();entry={'path':member,'original':str(f),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)};entries.append(entry);info=t.gettarinfo(str(f),arcname=member);info.uid=info.gid=0;info.uname=info.gname='';info.mtime=0
   with f.open('rb') as src:t.addfile(info,src)
 with tempfile.TemporaryDirectory(prefix='cold50x-custody-') as temp:
  with tarfile.open(out,'r:xz') as t:t.extractall(temp,filter='data')
  for e in entries:
   b=(Path(temp)/e['path']).read_bytes();assert len(b)==e['bytes'] and hashlib.sha256(b).hexdigest()==e['sha256'],e['path']
 manifest['archives'].append({'path':name,'sha256':hashlib.sha256(out.read_bytes()).hexdigest(),'bytes':out.stat().st_size,'members':entries,'all_extracted_members_match_original':True})
(p/'archive-members.json').write_text(json.dumps(manifest,indent=2)+'\n')
(p/'checks/archive-custody.json').write_text(json.dumps({'schema':'cold50x-local-custody/v1','archives':[{'path':a['path'],'sha256':a['sha256'],'bytes':a['bytes'],'members':len(a['members']),'extracted_match':True} for a in manifest['archives']],'standing':'Unsigned local byte custody only; not remote-store or independent scientific admission'},indent=2)+'\n')
print((p/'checks/archive-custody.json').read_text())
