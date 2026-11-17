"""Read-only S3/Git provenance check for inherited duplicate tracked data."""
from pathlib import Path
import concurrent.futures
import hashlib
import json
import sys
import tempfile

ROOT=Path('/workspace/crypto-integration-2550')
sys.path.insert(0,str(ROOT/'tools/artifacts'))
import artifacts
inventory=json.loads(Path('/workspace/ic-inherited-artifact-reconciliation-20261026.json').read_text())
assert inventory['files'] and all(r['local_equals_main_binding'] for r in inventory['files']), 'Divergent evidence: no automatic reconciliation'
cfg=artifacts.Config.load(ROOT)
assert cfg.remote=='s3://setec-astronomy-artifacts/store'
remote=artifacts.Remote(cfg)
def verify(row):
    assert row['main_binding']=={'bytes':row['bytes'],'sha256':row['sha256']}
    with tempfile.TemporaryDirectory(prefix='ic-inherited-s3-readback-') as folder:
        p=Path(folder)/'object';remote.get(row['sha256'],p)
        assert p.stat().st_size==row['bytes'],row['path']
        h=hashlib.sha256()
        with p.open('rb') as stream:
            while part:=stream.read(1024*1024):h.update(part)
        assert h.hexdigest()==row['sha256'],row['path']
    return {**row,'actual_s3_readback':'pass','remote_key':remote.key(row['sha256'])}
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:rows=list(pool.map(verify,inventory['files']))
out=ROOT/'research/ic_algebraic_crossover_20261023/pipeline-receipts/inherited-artifact-reconciliation-20261026'
out.mkdir(parents=True,exist_ok=True)
receipt={'schema':'ic-inherited-artifact-byte-readback/v1','main':inventory['main'],'files':rows,
    'file_count':len(rows),'file_bytes':sum(r['bytes'] for r in rows),'s3_writes':0,'tracking_changed':False,
    'status':'all_inherited_git_bytes_equal_real_main_binding_and_actual_s3_bytes'}
(out/'remote-byte-replay.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps({k:v for k,v in receipt.items() if k!='files'}))
