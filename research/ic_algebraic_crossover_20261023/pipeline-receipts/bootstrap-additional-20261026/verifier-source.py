"""Actual remote-byte validation for bounded, approved study publications."""
from pathlib import Path
import hashlib
import json
import re
import sys
import tempfile

ROOT=Path('/workspace/crypto-integration-2550')
sys.path.insert(0,str(ROOT/'tools/artifacts'))
import artifacts
d=json.loads(Path(sys.argv[1]).read_text())
label=d.get('receipt_id','bootstrap-additional-20261026')
assert re.fullmatch('[a-z0-9-]{1,70}',label)
cfg=artifacts.Config.load(ROOT);assert cfg.remote=='s3://setec-astronomy-artifacts/store'
remote=artifacts.Remote(cfg);rows=[]
for row in d['files']:
    path=Path(row['path'])
    assert not path.is_absolute() and '..' not in path.parts
    assert any(str(path).startswith(scope+'/') for scope in ('research/ic_algebraic_crossover_20261023','benchmarks/20261025-ic-algebraic-stage'))
    binding=json.loads((ROOT/path.parent/'.artifacts.json').read_text())['files'][path.name]
    assert binding=={'bytes':row['bytes'],'sha256':row['sha256']}
    with tempfile.TemporaryDirectory(prefix='ic-extra-s3-readback-') as folder:
        p=Path(folder)/'object';remote.get(row['sha256'],p)
        assert p.stat().st_size==row['bytes']
        assert hashlib.sha256(p.read_bytes()).hexdigest()==row['sha256']
    rows.append({**row,'actual_s3_get_sha256':'pass','remote_key':remote.key(row['sha256'])})
target=ROOT/'research/ic_algebraic_crossover_20261023/pipeline-receipts'/label
target.mkdir(parents=True,exist_ok=True)
result={'status':'actual_s3_bytes_match_every_bound_file','files':rows,'file_count':len(rows),'file_bytes':sum(r['bytes'] for r in rows),'remote':cfg.remote,'credential_values_recorded':False}
output=target/'remote-byte-replay.json';assert not output.exists(), 'readback receipt is write-once'
output.write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({k:v for k,v in result.items() if k!='files'}))
