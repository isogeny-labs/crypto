#!/usr/bin/env python3
"""File/hash reconstruction only. Does not execute or verify research arithmetic."""
from pathlib import Path
import hashlib,json,shutil,sys,tarfile
v=Path(__file__).resolve().parent
out=Path(sys.argv[1]).resolve()
if out.exists():raise SystemExit('fresh output directory required')
out.mkdir(parents=True)
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
raw=out/'raw';raw.mkdir()
with tarfile.open(v/'runs.tar.xz','r:xz') as tf:
 for m in tf.getmembers():
  assert m.isfile() and not Path(m.name).is_absolute() and '..' not in Path(m.name).parts
  p=raw/m.name;p.parent.mkdir(parents=True,exist_ok=True)
  p.write_bytes(tf.extractfile(m).read());p.chmod(m.mode)
manifest=[]
for line in (v/'RUNS.sha256').read_text().splitlines():
 h,n=line.split('  ',1);assert digest(raw/n)==h;manifest.append(n)
assert set(manifest)=={p.relative_to(raw).as_posix()for p in raw.rglob('*')if p.is_file()}
repack=out/'repacked.tar.xz'
with tarfile.open(repack,'w:xz',preset=9)as tf:
 for n in manifest:
  p=raw/n;m=tf.gettarinfo(str(p),arcname=n);m.uid=m.gid=0;m.uname=m.gname='';m.mtime=0
  with p.open('rb')as f:tf.addfile(m,f)
assert repack.read_bytes()==(v/'runs.tar.xz').read_bytes()
upper=v.parents[1]/'native/cm8-37-b3-high-20261030/cases'
lower=v.parents[1]/'native/cm8-37-b3-20261030/cases'
def check_index(tree):
 idx=json.loads((tree/'index.json').read_text());names=[i['name']for i in idx['files']]
 assert len(names)==len(set(names))
 assert {p.name for p in tree.iterdir()}==set(names)|{'index.json'}
 for item in idx['files']:assert digest(tree/item['name'])==item['sha256']
 return len(names)
assert check_index(upper)==1344
assert check_index(lower)==3024
summary=[]
for c in json.loads((v/'control-layout.json').read_text()):
 dest=out/('control-'+c['kind']);shutil.copytree(lower if c['base']=='lower' else upper,dest)
 for n in c['removed']:(dest/n).unlink()
 for n in c['overlays']:shutil.copyfile(v/'controls'/c['kind']/n,dest/n)
 assert digest(dest/'index.json')==c['index_sha256']
 count=check_index(dest)
 summary.append({'kind':c['kind'],'case_files':count,'index_sha256':c['index_sha256']})
shutil.copyfile(v/'false-transfer.json',out/'false-transfer.json')
print(json.dumps({'raw_files':len(manifest),'archive_repack_equal':True,'new_case_files':1344,'lower_reference_case_files':3024,'controls':summary,'false_transfer_preserved':True}))
