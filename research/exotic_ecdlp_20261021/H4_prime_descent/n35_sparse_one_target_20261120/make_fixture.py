#!/usr/bin/env python3
"""Freeze one new n35 public point for sparse IC and same-point rho."""
from sage.all import *
import hashlib
import importlib.util
import json
from pathlib import Path
HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[3]
BASE=ROOT/'research/exotic_ecdlp_20261021/H4_prime_descent'
source=BASE/'n35_full_pipeline_20261120/run.py'
m=importlib.util.spec_from_file_location('n35_full_fixture',source)
full=importlib.util.module_from_spec(m);m.loader.exec_module(full)
E,elt=full.build_context()
base,reps,encoding,G=full.build_base(E,elt)
assert len(base)==4096 and len(reps)==2048
prior=json.loads((BASE/'n35_full_pipeline_20261120/result.json').read_text())
panel=json.loads((BASE/'n35_online_panel_20261120/fixtures.json').read_text())
label='h4-n35-sparse-primary-target-0'
scalar=int.from_bytes(hashlib.sha256(label.encode()).digest(),'big')%(full.R-1)+1
Q=scalar*G
point=full.point_record(Q)
assert point!=prior['target']
assert point not in [x['target'] for x in panel['targets']]
curve_id=prior['curve_id']
record={'curve_id':curve_id,'subgroup_order':full.R,'G':full.point_record(G),
        'target':point,'target_count':1,
        'target_law':'sha256_nonzero_scalar_times_G','target_label':label,
        'precomputation_state':'ready'}
result={'schema':'h4-n35-sparse-primary-fixture-v1','target_label':label,
        'target_fixture_scalar':scalar,'target':point,
        'target_point_sha256':full.digest(point),
        'workload_id':'W'+full.digest(record)[:12],
        'workload_record':record}
(HERE/'fixture.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps({'target':point,'workload_id':result['workload_id']}))
