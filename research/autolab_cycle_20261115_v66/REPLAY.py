from pathlib import Path
import json,hashlib,gzip,base64
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
archive=json.loads(gzip.decompress(base64.b64decode((R/'research/vieta_guard_cold_20261115_v66/cold_runs.json.gz.b64').read_bytes())))
assert len(archive)==16
for row in archive:assert hashlib.sha256(row['content'].encode()).hexdigest()==row['sha256'];assert json.loads(row['content'])['parsed']['exact_root_count']==6
for d in ['vieta_guard_20261115_v66','vieta_guard_trials_20261115_v66','vieta_guard_cold_20261115_v66']:assert json.loads((R/'research'/d/'raw.json').read_text())['error']is None
full=json.loads((R/'research/vieta_guard_trials_20261115_v66/raw.json').read_text())['out']['records'];assert len(full)==3
for r in full:assert r['tests']==261457 and r['rejected']==r['verified_rejected']and r['exact_root_count']==6
print('Hashes,lossless16-run archive,all guard rejections and complete original roots verified. No qualified50xclaim.')
