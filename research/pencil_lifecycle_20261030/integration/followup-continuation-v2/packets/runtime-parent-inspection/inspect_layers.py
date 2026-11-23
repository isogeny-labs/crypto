"""Read selected layer headers after verifying image/config/diff IDs; no extraction."""
import hashlib,json,tarfile
from pathlib import Path,PurePosixPath
def file_digest(f):
 h=hashlib.sha256()
 for chunk in iter(lambda:f.read(1024*1024),b''): h.update(chunk)
 return h.hexdigest()

p=Path(__file__).parent
r=json.loads((p/'save-receipt.json').read_text()); assert r['exit'] in (0,None)
with open(r['archive'],'rb') as f: assert file_digest(f)==r['sha256']
selected=['.','opt','opt/isolab','opt/isolab/recovery','opt/isolab/recovery/lib','fixtures','fixtures/.mountpoint','qualification','qualification/exec-gate']
events=[]; layers=[]; desc={x:False for x in selected}; concerns=[]
with tarfile.open(r['archive'],'r:') as outer:
 manifest=json.load(outer.extractfile('manifest.json')); assert len(manifest)==1
 m=manifest[0];raw=outer.extractfile(m['Config']).read();config=json.loads(raw)
 assert 'sha256:'+hashlib.sha256(raw).hexdigest()==r['image']
 diffs=config['rootfs']['diff_ids']; assert len(m['Layers'])==len(diffs)
 for i,(name,expected) in enumerate(zip(m['Layers'],diffs)):
  with outer.extractfile(name) as f: actual='sha256:'+file_digest(f)
  assert actual==expected
  count=0; seen=set()
  with tarfile.open(fileobj=outer.extractfile(name),mode='r|') as layer:
   for t in layer:
    n=str(PurePosixPath(t.name)); assert not n.startswith('/') and '..' not in PurePosixPath(n).parts
    if n in seen: concerns.append(dict(layer=i,path=n,reason='duplicate layer entry'))
    seen.add(n);count+=1
    for s in selected:
     if s!='.' and n.startswith(s+'/'): desc[s]=True
    if n in selected:
     e=dict(layer=i,path=n,type=t.type.decode('ascii'),mode=oct(t.mode),uid=t.uid,gid=t.gid,linkname=t.linkname,pax=t.pax_headers)
     events.append(e)
     if not t.isdir(): concerns.append(dict(layer=i,path=n,reason='selected path not directory',type=e['type']))
    if PurePosixPath(n).name.startswith('.wh.'):
     parent=str(PurePosixPath(n).parent)
     if parent=='.' or any(s==parent or s.startswith(parent+'/') for s in selected):
      concerns.append(dict(layer=i,path=n,reason='relevant whiteout; manual interpretation required'))
  layers.append(dict(index=i,archive_member=name,diff_id=actual,header_count=count))
 result=[]
 for s in selected:
  hits=[e for e in events if e['path']==s]
  result.append(dict(path=s,status='explicit_header' if hits else 'implicit_or_unspecified' if s=='.' or desc[s] else 'absent_in_all_layers',last_header=hits[-1] if hits else None))
 out=dict(schema='runtime.base-layer-metadata/v1',image=r['image'],archive_sha256=r['sha256'],config_digest_verified=True,layer_digests_verified=True,layers=layers,events=events,paths=result,concerns=concerns,root_metadata_inferred=False,container_actions=0,scientific_execution=False)
 (p/'layer-observations.json').write_text(json.dumps(out,indent=2)+'\n')
 print(json.dumps({'layers':len(layers),'headers':sum(x['header_count'] for x in layers),'paths':result,'concerns':concerns}))
