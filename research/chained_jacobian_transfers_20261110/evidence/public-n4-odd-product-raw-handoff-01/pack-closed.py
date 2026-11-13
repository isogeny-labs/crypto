"""Resumable whole-tree tar/XZ custody; no scientific computation."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,io,json,lzma,shutil,subprocess,tarfile
c=Path('/workspace/cryptanalysis');cs=c/'experiments/chained-transfer-public-target';r=Path('/workspace/crypto')
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()=='4cce14d072ae6c1c5f8588b3e955269d6256dbc5'
root=r/'research/chained_jacobian_transfers_20261110/evidence/public-n4-odd-product-raw-handoff-01';root.mkdir(exist_ok=True)
attempts=[('chain-kernel-full', 'ct61-chain-kernel-full-01'), ('chain-kernel-CI', 'ct61-chain-kernel-own-CI-raw-01'), ('odd-source-failed', 'ct62-odd-source-native-01'), ('odd-source', 'ct62-odd-source-native-02'), ('odd-source-full', 'ct62-odd-source-full-01'), ('odd-source-CI', 'ct62-odd-source-own-CI-raw-01'), ('odd-product-models', 'ct63-odd-product-models-native-01'), ('odd-product-maps-01', 'ct63-odd-product-maps-native-01'), ('odd-product-maps-02', 'ct63-odd-product-maps-native-02')]
shutil.copyfile(__file__,root/'pack-closed.py');rows=[]
def digest(raw):return hashlib.sha256(raw).hexdigest()
for label,name in attempts:
 source=Path('/workspace/scratch')/name;assert source.is_dir()
 if(source/'native.exit').exists():assert(source/'finish.txt').exists()and(source/'guard.exit').read_text().strip()=='0'
 d=cs/'evidence'/('closed-ct64-'+label)
 if(d/'custody.json').exists():
  row=json.loads((d/'custody.json').read_text());z=(root/(label+'.tar.xz')).read_bytes();iz=(root/(label+'-members.json.xz')).read_bytes();assert digest(z)==row['archive_sha256']and digest(iz)==row['complete_index_xz_sha256'];members=json.loads(lzma.decompress(iz));assert {p.relative_to(source).as_posix()for p in source.rglob('*')if p.is_file()}==set(members)
  assert all(digest((source/n).read_bytes())==m['sha256']for n,m in members.items())
 else:
  members={};b=io.BytesIO()
  with tarfile.open(fileobj=b,mode='w',format=tarfile.PAX_FORMAT)as tf:
   for p in sorted(source.rglob('*')):
    if p.is_file():
     raw=p.read_bytes();key=p.relative_to(source).as_posix();members[key]=dict(bytes=len(raw),sha256=digest(raw));tf.add(p,arcname=key,recursive=False)
  raw=b.getvalue();z=lzma.compress(raw,preset=6);assert lzma.decompress(z)==raw
  with tarfile.open(fileobj=io.BytesIO(raw))as tf:
   assert set(tf.getnames())==set(members)
   for item in tf.getmembers():
    data=tf.extractfile(item).read();assert len(data)==members[item.name]['bytes']and digest(data)==members[item.name]['sha256']
  index=json.dumps(members,indent=2).encode()+b'\n';iz=lzma.compress(index,preset=6);assert lzma.decompress(iz)==index
  d.mkdir();(d/'whole-raw.tar.xz').write_bytes(z);(d/'members.json.xz').write_bytes(iz);(root/(label+'.tar.xz')).write_bytes(z);(root/(label+'-members.json.xz')).write_bytes(iz)
  row=dict(label=label,at=datetime.now(timezone.utc).isoformat(),original_closed_tree=str(source),files=len(members),original_native_exit=int((source/'native.exit').read_text())if(source/'native.exit').exists()else None,raw_bytes=len(raw),raw_sha256=digest(raw),archive_bytes=len(z),archive_sha256=digest(z),complete_index_raw_bytes=len(index),complete_index_raw_sha256=digest(index),complete_index_xz_bytes=len(iz),complete_index_xz_sha256=digest(iz),whole_standard_tarXZ_roundtrip=True,every_member_hash_roundtrip=True,build_caches_outside_raw=True,remote_availability='pending actual unchanged trusted publisher;no packaging/threshold waiver')
  (d/'custody.json').write_text(json.dumps(row,indent=2)+'\n')
 claimpath=d/'claim-path.txt'
 if not claimpath.exists():
  rel=str((d/'custody.json').relative_to(c));matches=[]
  for p in(c/'pcc/claims').glob('*.json'):
   claim=json.loads(p.read_text())
   if any(v.get('kind')=='attestation'and v.get('subject',{}).get('path')==rel and v['subject']['sha256']==digest((d/'custody.json').read_bytes())for v in claim.get('proofs',[])):matches.append(str(p.relative_to(c)))
  if matches:assert len(matches)==1;cl=matches[0]
  else:cl=subprocess.check_output(['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','WHOLEclosed '+label+' bytecustody '+str(row['files'])+'files/ONEstandardtarXZ/'+str(row['archive_bytes'])+'bytes plusCOMPLETE memberindex;everymember/raw/archive/hashroundtrip. Alloriginalnativefailuresandsuccesses/sourceclosures preserved. Actualremoteavailabilityrequiresunchangedtrustedpublisher;not execution/math/performanceproof.','--proof','attestation:'+rel],cwd=c,text=True).strip()
  claimpath.write_text(cl+'\n')
 else:cl=claimpath.read_text().strip()
 rows.append(row);(root/'handoff-progress.json').write_text(json.dumps(dict(total=9,completed=len(rows),attempts=rows),indent=2)+'\n');print(label,cl,row['files'],row['archive_bytes'],row['complete_index_xz_bytes'],flush=True)
assert len(rows)==9
(root/'handoff.json').write_text(json.dumps(dict(scope='NINE whole closedtrees plus NINE complete indices;actualtrustedpublication pending;all failures/rawbytes preserved;olddata/inputs unchanged',attempts=rows),indent=2)+'\n')
print('ALL9 wholeclosedtrees/18 complete objects ready',flush=True)
