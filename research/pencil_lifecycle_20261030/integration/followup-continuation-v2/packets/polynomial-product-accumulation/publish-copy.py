from pathlib import Path
import hashlib,json,base64
src=Path(__file__).parent
dst=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/polynomial-product-accumulation')
dst.mkdir(parents=True,exist_ok=True);files=[]
for p in sorted(src.rglob('*')):
 if not p.is_file() or p.is_symlink():continue
 rel=p.relative_to(src)
 if any(x in ('target','.lake','deps','dependency-union','__pycache__') for x in rel.parts):continue
 data=p.read_bytes();omit=p.suffix in ('.olean','.ilean','.o','.png','.aux','.out','.pyc') or p.name in ('render.log','generic-models.jsonl')
 encoded=p.name=='Add11.lean' and 'reviews' in rel.parts
 name=str(rel)+('.txt' if p.suffix=='.lean' else '')+('.b64' if encoded else '')
 stored=base64.b64encode(data)+b'\n' if encoded else data
 files.append({'original_path':str(rel),'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'encoding':'base64' if encoded else 'identity','stored_sha256':hashlib.sha256(stored).hexdigest(),'included_path':None if omit else name,'omission_reason':'compiled/render intermediate' if omit else None})
 if not omit:
  q=dst/name;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(stored)
(dst/'custody-map.json').write_text(json.dumps({'source_directory':str(src),'files':files},indent=2)+'\n')
n=0
for x in files:
 if x['included_path']:
  stored=(dst/x['included_path']).read_bytes();assert hashlib.sha256(stored).hexdigest()==x['stored_sha256']
  raw=base64.b64decode(stored) if x['encoding']=='base64' else stored
  assert hashlib.sha256(raw).hexdigest()==x['sha256'];n+=1
print('Verified included custody hashes',n)
