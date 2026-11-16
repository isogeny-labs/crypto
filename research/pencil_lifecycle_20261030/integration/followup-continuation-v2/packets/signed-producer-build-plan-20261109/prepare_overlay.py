"""Source preparation only: exact archived patch application, no compiler or payload."""
from pathlib import Path
import json,hashlib,re
O=Path(__file__).resolve().parent;P=O.parent;ROOT=P.parents[4]
A=P/'signed-event-adapter-20261109';Q=P/'signed-producer-qualification-20261109';DONOR=Path('/var/tmp/pencil-root-materialize-t1953-20261109/source')
def parts(text):return [s for s in re.split(r'(?=^--- a/)',text,flags=re.M) if s.startswith('--- a/')]
def apply_exact(raw,part):
 old=raw.decode().splitlines(True);lines=part.splitlines(True);out=[];cursor=0;i=2
 while i<len(lines):
  m=re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@',lines[i]);assert m,lines[i];start=int(m[1])-1;assert start>=cursor;out+=old[cursor:start];cursor=start;i+=1;removed=added=0
  while i<len(lines) and not lines[i].startswith('@@ '):
   z=lines[i];i+=1;assert z[0] in ' +-'
   if z[0] in ' -':assert old[cursor]==z[1:];cursor+=1;removed+=1
   if z[0] in ' +':out.append(z[1:]);added+=1
  assert removed==int(m[2] or 1) and added==int(m[4] or 1)
 return ''.join(out+old[cursor:]).encode()
selected=parts((A/'candidate.patch').read_text())[:2];(O/'selected-adapter.patch').write_text(''.join(selected));(O/'native-tests.patch').write_bytes((Q/'native-tests.patch').read_bytes());overlays=[]
for part in selected+parts((Q/'native-tests.patch').read_text()):
 name=part.splitlines()[0][6:];origin=DONOR/name
 if not origin.exists():
  assert name=='research/pencil_lifecycle_20261030/integration/followup-continuation-v2/target-stream/src/lib.rs';origin=A/'lib.rs.base'
 base=origin.read_bytes();post=apply_exact(base,part);target=O/'overlay'/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(post)
 overlays.append({'path':name,'pre_sha256':hashlib.sha256(base).hexdigest(),'pre_origin':str(origin),'in_donor_source':(DONOR/name).exists(),'post_sha256':hashlib.sha256(post).hexdigest(),'post_bytes':len(post),'overlay_path':'overlay/'+name,'operation':'apply_exact_bound_patch'})
for src in [Q/'native_producer_tests.rs',Q/'labels-root.rs',Q/'signed-panel.json',Q/'kummer-fixture.json',Q/'protocol.json']:
 name='research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/signed-producer-qualification-20261109/'+src.name;target=O/'overlay'/name;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(src.read_bytes());overlays.append({'path':name,'pre_sha256':None,'in_donor_source':False,'post_sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'post_bytes':src.stat().st_size,'overlay_path':'overlay/'+name,'operation':'add_bound_qualification_file'})
manifest=Path('/home/ec2-user/pencil-validation-receipts/t1953-materializer-20261109/worker/materialized-inputs.json');data=manifest.read_bytes();assert hashlib.sha256(data).hexdigest()=='c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2';j=json.loads(data);rows={str(Path(path).relative_to(DONOR)):row for path,row in j['immutable'].items() if Path(path).is_relative_to(DONOR)}
for row in overlays:
 if row['in_donor_source']:assert rows[row['path']]['sha256']==row['pre_sha256']
 else:assert row['path'] not in rows
 rows[row['path']]={'sha256':row['post_sha256'],'bytes':row['post_bytes']}
(O/'expected-source-manifest.json').write_text(json.dumps(rows,indent=2)+'\n');(O/'overlays.json').write_text(json.dumps(overlays,indent=2)+'\n')
bind={}
for src in [manifest,A/'candidate.patch',A/'base-bindings.json',A/'lib.rs.base',Q/'manifest.json',Q/'protocol.json',P/'signed-producer-qualification-review-20261109/review.json',P/'root-library-validation-v4-20261109/source-bindings.json']:
 bind[str(src.resolve())]={'sha256':hashlib.sha256(src.read_bytes()).hexdigest(),'bytes':src.stat().st_size}
(O/'bindings.json').write_text(json.dumps(bind,indent=2)+'\n');print('overlayfiles',len(overlays),'sourcefiles',len(rows),'bytes',sum(v['bytes'] for v in rows.values()))
