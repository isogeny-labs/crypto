"""Conditional immutable S3 publication and fresh byte-custody verification."""
from pathlib import Path
import base64,hashlib,json,subprocess
h=Path(__file__).resolve().parent;repo=h.parents[3]
meta=json.loads((h/'raw-meta.json').read_text());raw=h/'runs.tar.xz';digest=hashlib.sha256(raw.read_bytes()).hexdigest();assert digest==meta['raw_sha256']
bucket='setec-astronomy-artifacts';key='store/sha256/'+digest[:2]+'/'+digest
pub=h/'publication';pub.mkdir(exist_ok=True)
fresh=Path('/var/tmp/koblitz-r51-final-fresh.tar.xz');restore=Path('/var/tmp/koblitz-r51-final-restore')
assert not fresh.exists() and not restore.exists(),'reconcile existing custody outputs before retrying'
checksum=base64.b64encode(bytes.fromhex(digest)).decode()
put=subprocess.run(['aws','s3api','put-object','--bucket',bucket,'--key',key,'--body',str(raw),'--if-none-match','*','--checksum-algorithm','SHA256','--checksum-sha256',checksum,'--metadata','sha256='+digest],capture_output=True,text=True)
(pub/'raw-put.json').write_text(json.dumps({'exit':put.returncode,'stdout':put.stdout,'stderr':put.stderr,'bucket':bucket,'key':key},indent=2)+'\n')
if put.returncode and 'PreconditionFailed' not in put.stderr:raise RuntimeError('conditional artifact upload failed;see raw-put.json')
get=subprocess.run(['aws','s3api','get-object','--bucket',bucket,'--key',key,'--checksum-mode','ENABLED',str(fresh)],capture_output=True,text=True)
(pub/'raw-get.json').write_text(json.dumps({'exit':get.returncode,'stdout':get.stdout,'stderr':get.stderr,'bucket':bucket,'key':key},indent=2)+'\n')
assert get.returncode==0,'fresh direct GetObject failed'
subprocess.run(['python3',str(h.parent/'27/VERIFY_ARCHIVE.py'),'51',str(fresh),str(restore)],check=True,cwd=repo)
meta.update(fresh_download_verified=True,state='conditional immutable object published;fresh direct GET,all member hashes and deterministic repack verified')
(h/'raw-meta.json').write_text(json.dumps(meta,indent=2)+'\n')
(h/'.artifacts.json').write_text(json.dumps({'schema':'artifacts.manifest/v1','files':{'runs.tar.xz':{'sha256':digest,'bytes':raw.stat().st_size}}},indent=2)+'\n')
(h/'ARTIFACTS.md').write_text(f'''# R51 immutable raw custody

SHA256 `{digest}`;{meta['bytes']}bytes,{meta['regular_files']}regular files.
Scientific source `{meta['source_revision']}`;{meta['source_pins']}scientific source pins.
All20attempts,including4rejections,and both compiled Lean artifacts are retained.
The conditional PUT targets setec-astronomy-artifacts/store. A fresh direct
GetObject,all member hashes and byte-identical deterministic repack pass.
Historical Git source bytes and terminal receipts are verified separately.

```bash
ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/51
cd research/koblitz_modular_ic_20261018/rounds/51
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

Read README.md,CM.md,PROOF_MAP.md and NEXT.md. Do not regenerate accepted
producers for publication. These are byte-custody checks,not a substitute for
native mathematical replay or independent semantic review.
''')
print(json.dumps({'bucket':bucket,'key':key,'raw_sha256':digest,'fresh_download_verified':True}))
