"""File/Git object custody for the complete original source snapshot, no math."""
from pathlib import Path
from datetime import datetime,timezone
import hashlib,json,os,shutil,subprocess
r=Path('/workspace/crypto');source=Path('/dev/shm/ct351-whole-source-archive-for-original-store/entire-original-guarded-source.tar.xz');assert source.is_file() and not source.is_symlink();b=source.read_bytes();sha=hashlib.sha256(b).hexdigest();assert len(b)==14170288 and sha=='45155cf75dcc9a46d4beeac3327c38810955f10e3c9bb5eade1d1ee17e990271'
side=Path('/workspace/scratch/ct374-entire-source-payload-byte-sidecar');side.mkdir();whole=side/source.name;whole.write_bytes(b);assert whole.read_bytes()==b
payload=r/'research/chained_jacobian_transfers_20261110/evidence/whole-guarded-source-original-object-12/entire-original-guarded-source.tar.xz';payload.parent.mkdir(parents=True);os.link(whole,payload);assert not payload.is_symlink() and payload.read_bytes()==b
# Keep the original RAM-hold pathname and all of its full bytes readable while
# moving this duplicate's allocation to the retained regular sidecar inode.
source.unlink();source.symlink_to(whole);assert source.read_bytes()==b
objects=Path('/dev/shm/ct374-entire-source-Git-object-storage');objects.mkdir();env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0';env['GIT_OBJECT_DIRECTORY']=str(objects);env['GIT_ALTERNATE_OBJECT_DIRECTORIES']=str(r/'.git/objects')
q=subprocess.run(['git','hash-object','--no-filters','--write',str(payload)],cwd=r,env=env,capture_output=True,timeout=120);(side/'actual-whole-Git-object.stdout').write_bytes(q.stdout);(side/'actual-whole-Git-object.stderr').write_bytes(q.stderr);assert q.returncode==0;oid=q.stdout.decode().strip();assert len(oid)==40
alternates=r/'.git/objects/info/alternates';before=alternates.read_bytes() if alternates.exists() else b'';(side/'entire-before-Git-alternates.txt').write_bytes(before);updated=before+(b'' if not before or before.endswith(b'\n') else b'\n')+str(objects).encode()+b'\n';alternates.write_bytes(updated)
env=os.environ.copy();env['GIT_OPTIONAL_LOCKS']='0'
assert subprocess.check_output(['git','cat-file','-s',oid],cwd=r,env=env,text=True).strip()==str(len(b))
p=subprocess.Popen(['git','cat-file','blob',oid],cwd=r,env=env,stdout=subprocess.PIPE);actual=hashlib.sha256();count=0
while block:=p.stdout.read(1<<20):actual.update(block);count+=len(block)
assert p.wait()==0 and count==len(b) and actual.hexdigest()==sha
receipt=dict(at=datetime.now(timezone.utc).isoformat(),whole_compressed_bytes=len(b),whole_compressed_sha256=sha,original_TAR_bytes=39331840,original_TAR_sha256='b441b4375616df79da9b879dd3d1fc04b8f31ae81b0d2c68c98833c3077fb7b8',Root_regular_payload=str(payload.relative_to(r)),whole_workspace_sidecar=str(whole),original_RAM_hold_path_still_resolves_to_entire_bytes=str(source),complete_Git_blob_oid=oid,complete_Git_object_directory=str(objects),actual_Git_blob_full_byte_replay=0,source_and_raw_bytes_deleted=False,scope='whole local regular payload/preseeded ordinary Git object storage; original store/remote migration still required, no policy bypass or scientific execution')
g=payload.parent;(g/'actual-whole-source-and-Git-object-custody.json').write_text(json.dumps(receipt,indent=2)+'\n');shutil.copyfile(__file__,g/'entire-actual-byte-storage-command.py')
for n in ['actual-whole-Git-object.stdout','actual-whole-Git-object.stderr']:shutil.copyfile(side/n,g/n)
cmd=['python3','tools/pcc/pcc.py','claim','new','--kind','statement','--claim-type','data','--statement','Whole original guarded-source39331840B TAR/b441b4 is retained exactly inside lossless14170288B XZ/45155cf7. Complete regular Root payload and whole workspace sidecar share an inode, original RAM-hold pathname remains full-byte readable through explicit storage relocation, no source/raw content or hold path lost. Entire ordinary Git blob preseeded in retained RAM object storage and registered via standard alternate; original Git cat-file full-byte replay0 of14170288B matches SHA. This avoids duplicate large overlay allocation while preserving the complete object and normal source commit path. Original S3 tool/policy must select/upload/replay the whole payload before genuine bot/merge; originalPCC exact hydration must precede it. Not uploaded or signed/scientificNEW28/image/runtime/EKS/cost qualification. Native2c/Root44 snapshot provenance and all1663 whole files remain original; no sharding/truncation/grants/newidentity/paid execution.' ]
for f in sorted(g.iterdir()):cmd+=['--proof','attestation:'+f.relative_to(r).as_posix()]
claim=subprocess.check_output(cmd,cwd=r,env=env,text=True).strip();print(claim,json.dumps(receipt))
