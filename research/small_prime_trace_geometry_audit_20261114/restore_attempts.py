"""Restore losslessly deduplicated build provenance; no research computation."""
from pathlib import Path
import hashlib,json,sys,tarfile
archive=Path(__file__).resolve().parent/'evidence/attempts.tar.gz'
dest=Path(sys.argv[1]).resolve()
dest.mkdir(parents=True,exist_ok=True)
with tarfile.open(archive,'r:gz') as t:
    manifest=json.load(t.extractfile('manifest.json'))
    for item in manifest['inventory']:
        out=dest/item['member']
        assert out.resolve().is_relative_to(dest),item['member']
        data=t.extractfile('objects/'+item['sha256']).read()
        assert len(data)==item['bytes'] and hashlib.sha256(data).hexdigest()==item['sha256']
        if out.exists():
            assert out.read_bytes()==data,('refuse overwrite',str(out))
        else:
            out.parent.mkdir(parents=True,exist_ok=True)
            out.write_bytes(data)
print(json.dumps({'files':len(manifest['inventory']),'unique_objects':manifest['unique_objects'],
    'destination':str(dest),'all_bytes_match':True}))
