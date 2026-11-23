"""Download only from the official store and check every archive member."""
from pathlib import Path
import hashlib,json,sys,tempfile,tarfile
sys.path.insert(0,str(Path.cwd()/'tools/artifacts'))
from artifacts import Config,Remote,object_key
p=Path(__file__).resolve().parent;cfg=Config.load(Path.cwd());remote=Remote(cfg);entries=json.loads((p/'.artifacts.json').read_text())['files'];out=Path(tempfile.mkdtemp(prefix='first-information-readback-'));results=[]
for name,e in entries.items():
 target=out/name;remote.get(e['sha256'],target);raw=target.read_bytes();assert len(raw)==e['bytes'] and hashlib.sha256(raw).hexdigest()==e['sha256'];results.append({'file':name,**e,'uri':cfg.remote.rstrip('/')+'/'+object_key(e['sha256']),'fresh_download_matches':True})
members=json.loads((p/'boundary-manifest.json').read_text())['members']
with tarfile.open(out/'boundaries.tar.gz') as t:
 assert len(t.getmembers())==len(members)==64 and set(t.getnames())=={m['file'] for m in members}
 for m in members:
  raw=t.extractfile(m['file']).read();assert len(raw)==m['bytes'] and hashlib.sha256(raw).hexdigest()==m['sha256']
(p/'remote-readback.json').write_text(json.dumps({'schema':'artifact.remote-readback/v1','objects':results,'boundary_members_verified':64},indent=2)+'\n')
