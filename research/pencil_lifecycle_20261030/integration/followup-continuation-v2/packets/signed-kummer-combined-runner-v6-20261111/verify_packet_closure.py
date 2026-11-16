#!/usr/bin/python3
import hashlib,json,stat
from pathlib import Path

ROOT=Path(__file__).resolve().parent
required_packet_files={
    'acceptance_negative_controls.py','admission-draft.json','commands.json',
    'driver-fixture.json','driver-inputs.json','durable-launcher.py',
    'durable-worker.py','durable_io.py','expected-source-manifest.json',
    'materialize.py','overlay-closure.json','overlays.json','panel_acceptance.py',
    'prior-v3-terminal.json','prior-v5-terminal.json','stage_driver.py',
    'verify_overlay_closure.py'
}
for name in required_packet_files:
    path=ROOT/name;mode=path.lstat().st_mode
    assert stat.S_ISREG(mode) and not path.is_symlink(),name

rows=json.loads((ROOT/'overlays.json').read_text())
assert len(rows)==8
for row in rows:
    path=ROOT/row['overlay_path'];mode=path.lstat().st_mode;data=path.read_bytes()
    assert stat.S_ISREG(mode) and not path.is_symlink()
    assert len(data)==row['post_bytes'] and hashlib.sha256(data).hexdigest()==row['post_sha256']

driver=json.loads((ROOT/'driver-inputs.json').read_text())
assert driver['schema']=='bounded-offline-lock-inputs/v1' and len(driver['files'])==50
for row in driver['files']:
    assert set(row)=={'source','destination','sha256','bytes'}
    path=Path(row['source']);mode=path.lstat().st_mode;data=path.read_bytes()
    assert stat.S_ISREG(mode) and not path.is_symlink()
    assert len(data)==row['bytes'] and hashlib.sha256(data).hexdigest()==row['sha256']

print(json.dumps({'schema':'pencil.packet-closure-check/v1','packet_files':len(required_packet_files),'overlay_files':len(rows),'driver_source_files':len(driver['files']),'all_regular':True,'all_hashes_match':True},sort_keys=True))
