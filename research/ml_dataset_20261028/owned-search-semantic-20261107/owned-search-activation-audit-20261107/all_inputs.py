from pathlib import Path
import hashlib,json,subprocess
B=Path('/home/ec2-user/ml-dataset-resume-20261029');P=B/'owned-search-activation-audit-20261107';A=B/'owned-search-continuation-delivery-20261107/assembled';repo='/home/ec2-user/crypto-ml-ordinary-ownership-20261101'
manifest=json.loads((A.parent/'ASSEMBLED_HASHES.json').read_text());names=list(manifest)
p=subprocess.run(['git','-C',repo,'cat-file','--batch-check'],input=''.join('HEAD:'+x+'\n' for x in names),text=True,capture_output=True,check=True)
lines=p.stdout.splitlines();assert len(lines)==len(names);rows=[];same=0
bases=[('pre_owning_integration',B/'owning-search-composed-source-20261107/BASE_HASHES.json'),('owning_integration',B/'owning-search-composed-source-20261107/ASSEMBLED_HASHES.json'),('capture_integration',B/'owned-search-capture-composed-20261107/ASSEMBLED_HASHES.json')];bases=[(n,json.loads(f.read_text())) for n,f in bases]
for name,line in zip(names,lines):
 data=(A/name).read_bytes();sha=hashlib.sha256(data).hexdigest();assert sha==manifest[name];oid=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest();head=None if line.endswith(' missing') else line.split()[0]
 if head==oid:same+=1;continue
 versions={n:hashes.get(name) for n,hashes in bases};last='continuation_delivery'
 for n,h in reversed(bases):
  if h.get(name)==sha:last=n
  else:break
 rows.append({'path':name,'status':'modified' if head else 'added','head_blob':head,'assembled_blob':oid,'assembled_sha256':sha,'bytes':len(data),'versions':versions,'last_content_layer':last})
(P/'ALL_INPUT_DIFFERENCES.json').write_text(json.dumps({'bound_inputs':len(names),'same':same,'differences':rows},indent=2)+'\n');print('inputs',len(names),'same',same,'diff',len(rows))
for r in rows:print(r['last_content_layer'],r['path'])
