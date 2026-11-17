#!/usr/bin/env python3
"""Package completed native rows as metadata; no numerical analysis here.

Run the frozen native analyzer first. Refuses incomplete/failed panels and
never overwrites an export. Timing inference remains outside this adapter.
"""
import copy
import csv
import datetime
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE / 'results'
PROTO = json.loads((HERE / 'protocol.json').read_text())
TEMPLATE = json.loads((HERE.parent / 'panel2/results/export.json').read_text())

def ref(path, relative=None):
    data = path.read_bytes()
    return {'path': relative or path.relative_to(OUT).as_posix(),
            'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}

assert (OUT / 'exit-code.txt').read_text().strip() == '0'
assert (OUT / 'analysis.log').read_text().startswith('PASS 150 rows, 135 measured, 15 complete cells, byte-identical full certificates')
rows = list(csv.DictReader((OUT / 'rows.csv').open()))
assert len(rows) == 150
assert all(row['verification'] == 'verified' for row in rows)
assert {(r['arm'], r['instance_id'], r['replicate']) for r in rows} == {
    (a, str(i), str(n)) for a in 'uvwxy' for i in range(3) for n in range(10)}
export = copy.deepcopy(TEMPLATE)
export.update(export_id=PROTO['protocol_id'],created=datetime.datetime.now(datetime.timezone.utc).isoformat())
export['protocol'] = {'path': '../protocol.json', 'sha256': PROTO['freeze']['protocol_sha256'],
                      'protocol_id': PROTO['protocol_id'], 'version': PROTO['version']}
export['source']['commit'] = PROTO['arms'][1]['instrument']['commit']
export['arms'] = [{'id': a['id'], 'role': a['role'],
                  'commit': a['instrument']['commit'],
                  'binary_sha256': a['instrument']['binary_sha256']} for a in PROTO['arms']]
job = 'hashed-ecdl-cpu-round13-20261030'
export['host'].update(ref(OUT / 'cpuinfo.txt'))
export['host'].pop('bytes', None)
export['host']['class_id'] = job
summary = export['host']['summary']
summary['cpu_model'] = next(line.split(':',1)[1].strip() for line in (OUT/'cpuinfo.txt').read_text().splitlines() if line.startswith('model name'))
summary['kernel'] = (OUT / 'kernel.txt').read_text().strip()
export['runs'] = []
for i, row in enumerate(rows):
    run = copy.deepcopy(TEMPLATE['runs'][0])
    run.update(run_id=f'row-{i}', arm=row['arm'], instance_id=row['instance_id'],
               seed=row['seed'], replicate=int(row['replicate']), warmup=row['warmup']=='true', backend_ref=job)
    m = run['measurements']; m['wall_seconds'] = float(row['wall_seconds'])
    m['counted'] = {k: int(row[k]) for k in m['counted']}
    m['counters'] = {k: float(row[k]) if k.endswith('_ms') else int(row[k]) for k in m['counters']}
    run['artifacts'] = [ref(OUT / f"cert-{row['arm']}-{row['instance_id']}-{row['replicate']}.txt")]
    export['runs'].append(run)
export['files'] = [{**ref(f), 'kind':'raw'} for f in sorted(OUT.iterdir()) if f.is_file() and f.name not in ['export.json','export-check.log']]
export['flow'] = {'planned':150, 'attempted':len(rows), 'completed':len(rows), 'not_completed':0, 'interrupted':0, 'resumed':0, 'excluded':[]}
export['analysis']['script'] = ref(HERE/'analyze.rs', '../analyze.rs')
export['analysis']['outputs'] = [ref(OUT/n) for n in ['summary.csv','latency.svg','analysis.log']]
export['notes'] = [
    {'kind':'observation','by':'Codex','text':'150 complete executions,135 measured,15 warmups. All three pinned revisions use cached MITM; all full certificate bytes match across arms. Candidate changes constructor/inversion/fixed-base/residual/cached-root/batch-setup/sparse-pivot/reciprocal/projective arithmetic suffix indexing and reciprocal residual batches and density-aware query scheduling and checked projective witness sums as a bundle.'},
    {'kind':'observation','by':'Codex','text':'L0 shared-host diagnostic only; wall_valid=false. No strict performance, external or rho claim. This export does not establish the further10x goal. Neutral-label native analyzer is not human blinding.'}]
with (OUT/'export.json').open('x') as f:
    json.dump(export,f,indent=2); f.write('\n')
print('Wrote complete native-panel metadata export')
