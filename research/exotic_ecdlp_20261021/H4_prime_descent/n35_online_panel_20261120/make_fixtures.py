#!/usr/bin/env python3
"""Freeze ten fresh n35 public points for the secondary paired panel."""
from sage.all import *
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = ROOT / 'research/exotic_ecdlp_20261021/H4_prime_descent/n35_full_pipeline_20261120/run.py'
spec = importlib.util.spec_from_file_location('n35_full_runner', SOURCE)
run = importlib.util.module_from_spec(spec)
spec.loader.exec_module(run)
E, elt = run.build_context()
base, reps, encoding, G = run.build_base(E, elt)
assert len(base) == 4096 and len(reps) == 2048
curve_id, candidate_id, _ = run.manifest(base, reps, G)
assert candidate_id == json.loads((SOURCE.parent/'candidate.json').read_text())['candidate_id']
R = run.R
rows = []
for i in range(10):
    label = f'h4-n35-panel-target-{i}'
    scalar = int.from_bytes(hashlib.sha256(label.encode()).digest(), 'big') % (R-1) + 1
    Q = scalar * G
    record = {'curve_id': curve_id, 'subgroup_order': R, 'G': run.point_record(G),
              'target': run.point_record(Q), 'target_count': 1,
              'target_law': 'sha256_nonzero_scalar_times_G',
              'target_label': label, 'precomputation_state': 'ready'}
    rows.append({'index': i, 'label': label, 'fixture_scalar': scalar,
                 'target': run.point_record(Q), 'workload_id': 'W'+run.digest(record)[:12],
                 'target_point_sha256': run.digest(run.point_record(Q))})
assert len({tuple(row['target']) for row in rows}) == 10
assert all(row['fixture_scalar'] != json.loads((SOURCE.parent/'result.json').read_text())['target_fixture_scalar'] for row in rows)
result = {'schema': 'h4-n35-online-panel-fixtures-v1', 'curve_id': curve_id,
          'candidate_id': candidate_id, 'generator': run.point_record(G),
          'subgroup_order': R, 'targets': rows}
(HERE/'fixtures.json').write_text(json.dumps(result, indent=2, sort_keys=True)+'\n')
print(json.dumps({'targets': len(rows), 'curve_id': curve_id, 'candidate_id': candidate_id}))
