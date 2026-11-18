"""One immutable publication with fresh direct download/member/repack checks."""
from pathlib import Path
import base64,hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
meta=json.loads((h/'raw-meta.json').read_text());raw=h/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest();assert digest==meta['raw_sha256']
bucket='setec-astronomy-artifacts';key='store/sha256/'+digest[:2]+'/'+digest
pub=h/'publication';pub.mkdir(exist_ok=True)
fresh=Path('/var/tmp/koblitz-r61-final-fresh.tar.xz');restore=Path('/var/tmp/koblitz-r61-final-restore')
assert not fresh.exists() and not restore.exists(),'reconcile publication state before any retry'
checksum=base64.b64encode(bytes.fromhex(digest)).decode()
put=subprocess.run(['aws','s3api','put-object','--bucket',bucket,'--key',key,'--body',str(raw),'--if-none-match','*','--checksum-algorithm','SHA256','--checksum-sha256',checksum,'--metadata','sha256='+digest],capture_output=True,text=True)
(pub/'raw-put.json').write_text(json.dumps({'exit':put.returncode,'stdout':put.stdout,'stderr':put.stderr,'bucket':bucket,'key':key},indent=2)+'\n')
if put.returncode and 'PreconditionFailed' not in put.stderr:raise RuntimeError('conditional upload failed;see raw-put.json')
get=subprocess.run(['aws','s3api','get-object','--bucket',bucket,'--key',key,'--checksum-mode','ENABLED',str(fresh)],capture_output=True,text=True)
(pub/'raw-get.json').write_text(json.dumps({'exit':get.returncode,'stdout':get.stdout,'stderr':get.stderr,'bucket':bucket,'key':key},indent=2)+'\n')
assert get.returncode==0,'fresh GetObject failed'
subprocess.run(['python3',str(h.parent/'27/VERIFY_ARCHIVE.py'),'61',str(fresh),str(restore)],check=True,cwd=repo)
meta.update(fresh_download_verified=True,state='immutable object published;fresh direct GET,all member hashes and deterministic repack verified')
(h/'raw-meta.json').write_text(json.dumps(meta,indent=2)+'\n')
(h/'.artifacts.json').write_text(json.dumps({'schema':'artifacts.manifest/v1','files':{'runs.tar.xz':{'sha256':digest,'bytes':raw.stat().st_size}}},indent=2)+'\n')
(h/'ARTIFACTS.md').write_text(f'''# R61 immutable raw custody

SHA256 `{digest}`; {meta['bytes']} bytes, {meta['regular_files']} regular files.
Scientific source `{meta['source_revision']}`; {meta['source_pins']} source pins.
All terminal workflows,including the rejected native attempt and interrupted pre-native launch,
are retained. All executed source/protocol/driver pins match historical Git.
The compiled Lean object and full audit are included; six authored claims give six audited declarations,zero failing/trusted.

Conditional PUT targets setec-astronomy-artifacts/store. Fresh direct GET,
all member hashes and deterministic repacking pass. These are byte-custody
checks,not mathematical producer reruns or an independent scientific review.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/61
cd research/koblitz_modular_ic_20261018/rounds/61
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read CM.md,PROOF_MAP.md,SESSION.md and NEXT.md. The donor manifest also
requires retained R27/R43/R50/R52/R57/R58/R60 archives for a native replay. Do not repeat an
accepted producer merely to publish or verify custody.
''')
print(json.dumps({'raw_sha256':digest,'bucket':bucket,'key':key,'fresh_download_verified':True}))
