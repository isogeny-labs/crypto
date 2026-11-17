from pathlib import Path
import hashlib,json,base64,gzip
src=Path(__file__).parent
dst=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/collector-composition')
assert not dst.exists()
dst.mkdir(parents=True);files=[]
for p in sorted(src.rglob('*')):
 if not p.is_file() or p.is_symlink():continue
 rel=p.relative_to(src)
 if any(x in ('target','.lake','deps','dependency-union','__pycache__') for x in rel.parts):continue
 data=p.read_bytes();omit=p.suffix in ('.olean','.ilean','.o','.png','.aux','.out','.pyc') or p.name=='render.log'
 name=str(rel)+('.txt' if p.suffix=='.lean' else '')
 encoding='identity';stored=data
 if not omit and p.suffix!='.pdf':
  if len(data)>65536:
   encoding='gzip';stored=gzip.compress(data,mtime=0);name+='.gz'
  else:
   try:
    s=data.decode()
    if any(x.endswith((' ','\t')) for x in s.splitlines()) or s.endswith('\n\n'):
     encoding='base64';stored=base64.b64encode(data)+b'\n';name+='.b64'
   except UnicodeDecodeError:pass
 files.append({'original_path':str(rel),'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'encoding':encoding,'stored_sha256':hashlib.sha256(stored).hexdigest(),'included_path':None if omit else name,'omission_reason':'compiled/render intermediate' if omit else None})
 if not omit:
  q=dst/name;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(stored)
(dst/'custody-map.json').write_text(json.dumps({'source_directory':str(src),'files':files},indent=2)+'\n')
n=0
for x in files:
 if x['included_path']:
  stored=(dst/x['included_path']).read_bytes();assert hashlib.sha256(stored).hexdigest()==x['stored_sha256']
  raw=base64.b64decode(stored) if x['encoding']=='base64' else gzip.decompress(stored) if x['encoding']=='gzip' else stored
  assert hashlib.sha256(raw).hexdigest()==x['sha256'];n+=1
print('Verified included custody hashes',n)
