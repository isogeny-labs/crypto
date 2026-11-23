from pathlib import Path
import hashlib,json
B=Path('/home/ec2-user/ml-dataset-resume-20261029');P=B/'owned-search-activation-audit-20261107';A=B/'owned-search-continuation-delivery-20261107/assembled'
head={}
for row in Path('/tmp/owned-activation-head-tree.bin').read_bytes().split(b'\0'):
 if not row:continue
 meta,path=row.split(b'\t',1);mode,kind,oid=meta.decode().split();head[path.decode()]={'mode':mode,'oid':oid}
manifest=json.loads((A.parent/'ASSEMBLED_HASHES.json').read_text())
rows=[];same=0
for name,sha in manifest.items():
 if not (name.startswith('src/') or name in ('Cargo.toml','Cargo.lock')):continue
 f=A/name;data=f.read_bytes();actual=hashlib.sha256(data).hexdigest();assert actual==sha,name
 oid=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
 old=head.get(name)
 if old and old['oid']==oid:same+=1;continue
 rows.append({'path':name,'status':'modified' if old else 'added','head_blob':old['oid'] if old else None,'assembled_blob':oid,'assembled_sha256':sha,'bytes':len(data),'scope':'dataset' if name.startswith('src/ml_dataset/') else 'native' if name.startswith('src/cryptanalysis/') else 'other'})
missing=sorted(set(head)-set(manifest))
(P/'DIFFERENCES.json').write_text(json.dumps({'head':'11ad50cd86e60055d481af88869c407d6ef814d1','same':same,'differences':rows,'head_paths_absent_from_assembly':missing},indent=2)+'\n')
print('same',same,'different',len(rows),'head paths absent assembly',len(missing))
for r in rows:print(r['status'],r['path'])
