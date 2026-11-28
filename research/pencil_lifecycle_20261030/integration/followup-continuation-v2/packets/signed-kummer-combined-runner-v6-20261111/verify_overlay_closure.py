#!/usr/bin/python3
import hashlib,json,stat
from pathlib import Path

ROOT=Path(__file__).resolve().parent
rows=json.loads((ROOT/'overlays.json').read_text())
assert isinstance(rows,list) and len(rows)==8
seen_paths=set();seen_overlay_paths=set()
for row in rows:
    required={'path','pre_sha256','in_donor_source','post_sha256','post_bytes','overlay_path','operation'}
    assert required<=set(row)<=required|{'pre_origin'}
    assert row['path'] not in seen_paths and row['overlay_path'] not in seen_overlay_paths
    seen_paths.add(row['path']);seen_overlay_paths.add(row['overlay_path'])
    rel=Path(row['overlay_path'])
    assert not rel.is_absolute() and '..' not in rel.parts and rel.parts[0]=='overlay'
    path=ROOT/rel;mode=path.lstat().st_mode
    assert stat.S_ISREG(mode) and not path.is_symlink()
    data=path.read_bytes()
    assert len(data)==row['post_bytes']
    assert hashlib.sha256(data).hexdigest()==row['post_sha256']
print(json.dumps({'schema':'pencil.overlay-closure-check/v1','rows':len(rows),'all_regular':True,'all_post_hashes_match':True},sort_keys=True))
