"""Verify actual S3 bytes against the approved inventory; metadata only."""
from pathlib import Path
import concurrent.futures
import datetime
import hashlib
import json
import sys
import tempfile

ROOT = Path('/workspace/crypto-integration-2550')
sys.path.insert(0, str(ROOT / 'tools/artifacts'))
import artifacts

inventory_path = ROOT / 'research/ic_algebraic_crossover_20261023/PUBLICATION_INVENTORY_20261026.json'
inventory = json.loads(inventory_path.read_text())
cfg = artifacts.Config.load(ROOT)
assert cfg.remote == 's3://setec-astronomy-artifacts/store'
remote = artifacts.Remote(cfg)
previous = json.loads(Path('/workspace/ic-aws-access-20261026/before-manifest-bindings.json').read_text())
for parent, values in previous.items():
    current = json.loads((ROOT / parent / '.artifacts.json').read_text())['files']
    assert all(current.get(k) == v for k, v in values.items()), parent

def verify(row):
    p = ROOT / row['path']
    binding = json.loads((p.parent / '.artifacts.json').read_text())['files'][p.name]
    assert binding == {'sha256': row['sha256'], 'bytes': row['bytes']}, row['path']
    with tempfile.TemporaryDirectory(prefix='ic-s3-byte-readback-') as temp:
        downloaded = Path(temp) / 'object'
        remote.get(row['sha256'], downloaded)
        assert downloaded.stat().st_size == row['bytes'], row['path']
        actual = hashlib.sha256(downloaded.read_bytes()).hexdigest()
        assert actual == row['sha256'], row['path']
    return {**row, 'remote_key': remote.key(row['sha256']), 'actual_download_sha256': actual, 'status': 'pass'}

with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    rows = list(pool.map(verify, inventory['files']))
output = ROOT / 'research/ic_algebraic_crossover_20261023/pipeline-receipts/artifact-publication-20261026'
output.mkdir(parents=True, exist_ok=True)
receipt = {'schema': 'ic-artifact-byte-readback/v1', 'recorded_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
           'remote': cfg.remote, 'files': rows, 'file_count': len(rows), 'file_bytes': sum(row['bytes'] for row in rows),
           'unique_objects': len({row['sha256'] for row in rows}), 'previous_manifest_bindings_preserved': True,
           'credential_values_recorded': False, 'status': 'all_actual_s3_byte_readbacks_pass'}
(output / 'remote-byte-replay.json').write_text(json.dumps(receipt, indent=2) + '\n')
print(json.dumps({k: v for k, v in receipt.items() if k != 'files'}))
