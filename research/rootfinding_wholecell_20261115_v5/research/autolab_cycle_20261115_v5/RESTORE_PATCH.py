"""Usage: python RESTORE_PATCH.py BASE3 BASE4 PATCH5 PATCH5_SHA256 NEW_DIRECTORY"""
from pathlib import Path
import hashlib,json,sys,zipfile
def sha(b):return hashlib.sha256(b).hexdigest()
b3,b4,p5,expected,dest=sys.argv[1:];dest=Path(dest)
assert not dest.exists(),'destination must be NEW'
assert sha(Path(p5).read_bytes())==expected,'patch archive hash'
with zipfile.ZipFile(p5)as z:
    p=json.loads(z.read('research/autolab_cycle_20261115_v5/JOURNAL_PATCH.json'))
for path,spec in zip([b3,b4],p['base_archives']):assert sha(Path(path).read_bytes())==spec['sha256'],'base hash'
dest.mkdir(parents=True)
for archive in [b3,b4,p5]:
    with zipfile.ZipFile(archive)as z:
        for name in z.namelist():assert not name.startswith('/')and'..'not in Path(name).parts,'unsafe archive path'
        z.extractall(dest)
j=dest/p['journal_path'];old=j.read_bytes();extra=(dest/p['append_path']).read_bytes()
assert len(old)==p['old_bytes']and sha(old)==p['old_sha256'],'journal prefix'
assert sha(extra)==p['append_sha256'],'journal append'
with j.open('ab')as f:f.write(extra)
assert j.stat().st_size==p['new_bytes']and sha(j.read_bytes())==p['new_sha256'],'new journal'
idx=json.loads((dest/'research/autolab_cycle_20261115_v5/SOURCE_HASHES.json').read_text())
for name,h in idx.items():assert sha((dest/name).read_bytes())==h,name
print(json.dumps(dict(restored=str(dest),verified_hashes=len(idx))))
