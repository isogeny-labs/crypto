"""New immutable payload blobs only in RAM; existing Git objects are untouched."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,re,subprocess,zlib
def stage_payload(root,objects,receipt_path):
 r=Path(root);cache=Path('/dev/shm/ct105-new-payload-blob-cache-27/objects');cache.mkdir(parents=True,exist_ok=True)
 names=sorted(row['path']for row in objects if row['original_policy_requires_store']);assert names
 for row in objects:
  if row['path']in names:
   p=r/row['path'];assert p.is_file()and not p.is_symlink()and p.stat().st_size==row['bytes']
   with p.open('rb')as f:assert hashlib.file_digest(f,'sha256').hexdigest()==row['sha256']
 env=dict(os.environ,GIT_OBJECT_DIRECTORY=str(cache),GIT_ALTERNATE_OBJECT_DIRECTORIES=str(r/'.git/objects'),GIT_NO_LAZY_FETCH='1')
 env={k:v for k,v in env.items()if not k.startswith('GIT_TRACE')}
 paths=Path('/workspace/scratch/ct105-payload27-exact-large-data.paths0');paths.write_bytes(b'\0'.join(p.encode()for p in names)+b'\0')
 subprocess.run(['git','add','--sparse','--pathspec-from-file='+str(paths),'--pathspec-file-nul'],cwd=r,env=env,check=True)
 records=[]
 for p in sorted(cache.glob('[0-9a-f][0-9a-f]/*')):
  oid=p.parent.name+p.name;assert re.fullmatch('[0-9a-f]{40}',oid);body=p.read_bytes();canonical=zlib.decompress(body);assert hashlib.sha1(canonical).hexdigest()==oid
  kind=canonical.split(b' ',1)[0].decode();assert kind=='blob','only actual immutable data blobs may be cached here'
  target=r/'.git/objects'/p.parent.name/p.name;target.parent.mkdir(exist_ok=True)
  if target.exists()or target.is_symlink():assert target.is_file()and zlib.decompress(target.read_bytes())==canonical
  else:target.symlink_to(p)
  assert target.read_bytes()==body
  records.append(dict(oid=oid,kind=kind,compressed_bytes=len(body),compressed_sha256=hashlib.sha256(body).hexdigest(),actual_RAM_path=str(p),exposed_Git_path=str(target),new_existing_object_not_replaced=True))
 index=subprocess.check_output(['git','ls-files','--stage','-z','--',*names],cwd=r)
 indexed={}
 for item in index.split(b'\0'):
  if not item:continue
  meta,path=item.split(b'\t',1);mode,oid,stage=meta.decode().split();assert mode=='100644'and stage=='0';indexed[path.decode()]=oid
 assert set(indexed)==set(names)
 assert {row['oid']for row in records}<=set(indexed.values()),'every RAM cache blob must be a final immutable declared payload input'
 snapshot=dict(payload_paths=names,indexed_blob_oids=indexed,objects=records,all_payload_worktree_inputs_regular=True,all_blob_bytes_backed_by_closed_held_regular_input_files=True,no_existing_local_or_nonpromisor_object_moved_or_deleted=True,recovery='If RAM is lost before publication, recreate exactly these new blob OIDs from unchanged held regular Root payload inputs with git hash-object in a new RAM store. After normalpush0, immutable payload commit and originalstore replays provide remote recovery. Never delete unrelated local/nonpromisor objects.',scope='new derived immutable data-blob cache only; small metadata/tree/commit objects use normal persistent Root storage')
 receipt=Path(receipt_path)
 if receipt.exists():assert json.loads(receipt.read_text())==snapshot
 else:receipt.write_text(json.dumps(snapshot,indent=2)+'\n')
 return names
