#!/usr/bin/env python3
"""Verify byte-identical publication; optionally restore compressed archives.

No experiments, launch scripts, subprocesses or numerical kernels are run.
"""
from pathlib import Path
import argparse,hashlib,json,zlib,tempfile,os
ROOT=Path(__file__).resolve().parent

def safe(rel):
 p=(ROOT/rel).resolve()
 if not p.is_relative_to(ROOT):raise ValueError('path outside bundle')
 return p

def digest(p):
 h=hashlib.sha256()
 with p.open('rb') as f:
  for b in iter(lambda:f.read(1048576),b''):h.update(b)
 return h.hexdigest()

def main():
 if not __debug__:raise SystemExit('Run without -O: verification assertions must remain enabled')
 ap=argparse.ArgumentParser(description=__doc__);ap.add_argument('--hydrate',action='store_true');args=ap.parse_args()
 manifest=json.loads((ROOT/'publication-manifest.json').read_text());entries={z['path']:z for z in manifest['entries']}
 for rel,z in entries.items():
  if z['storage']=='file':
   p=safe(rel);assert p.stat().st_size==z['bytes'] and digest(p)==z['sha256'],rel
 restored=0
 for z in manifest['archives']:
  target=safe(z['path']);entry=entries[z['path']]
  assert entry['bytes']==z['bytes'] and entry['sha256']==z['sha256']
  exists=target.exists()
  if exists:assert target.stat().st_size==z['bytes'] and digest(target)==z['sha256'],str(target)
  tmp=None;output=None
  if args.hydrate and not exists:
   target.parent.mkdir(parents=True,exist_ok=True)
   output=tempfile.NamedTemporaryFile(dir=target.parent,prefix=target.name+'.',suffix='.restore',delete=False);tmp=Path(output.name)
  try:
   d=zlib.decompressobj(31);h=hashlib.sha256();count=0
   for part in z['parts']:
    path=safe(part['path']);assert path.stat().st_size==part['bytes'] and digest(path)==part['sha256'],str(path)
    with path.open('rb') as f:
     for block in iter(lambda:f.read(1048576),b''):
      data=block
      while data:
       plain=d.decompress(data,1048576);data=d.unconsumed_tail
       h.update(plain);count+=len(plain);assert count<=z['bytes']
       if output:output.write(plain)
   plain=d.flush();h.update(plain);count+=len(plain)
   if output:output.write(plain)
   assert d.eof and not d.unused_data and count==z['bytes'] and h.hexdigest()==z['sha256'],z['path']
   if output:
    output.close();output=None
    # Fail closed if another process created the destination while verifying.
    os.link(tmp,target);tmp.unlink();tmp=None;restored+=1
  finally:
   if output:output.close()
   if tmp and tmp.exists():tmp.unlink()
 # Frozen core source/input locks bind original bytes, regardless of storage.
 lock_entries=0
 for name in ['direct-companion-validation','companion-blocks-validation','companion-schur-validation','companion-fused-validation']:
  for kind in ['source-lock.json','input-lock.json']:
   for rel,want in json.loads((ROOT/name/kind).read_text()).items():
    assert rel in entries and entries[rel]['sha256']==want,(name,rel)
    lock_entries+=1
 print(json.dumps(dict(status='verified',files=len(entries),compressed_archives=len(manifest['archives']),frozen_lock_entries=lock_entries,restored=restored,numerical_trials_run=0),indent=2))
if __name__=='__main__':main()
