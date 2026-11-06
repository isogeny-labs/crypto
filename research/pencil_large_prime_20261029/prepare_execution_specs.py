#!/usr/bin/env python3
"""Prepare fixed ecbench specs from archived JSON; this never executes a solver."""
import argparse
import copy
import hashlib
import json
from pathlib import Path

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('inputs',type=Path)
p.add_argument('out',type=Path)
a=p.parse_args()
m=json.loads((a.inputs/'manifest.json').read_text())
for name,digest in m['files'].items():
    if hashlib.sha256((a.inputs/name).read_bytes()).hexdigest()!=digest:
        raise ValueError('archive projection hash mismatch: '+name)
specs={}
for bits in [16,20,24,28,32,36,40]:
    spec=json.loads((a.inputs/f'archive-spec-{bits}.json').read_text())
    rho=next(arm for arm in spec['arms'] if arm['name']=='rho_neg')
    spec['arms']=[dict(name='pencil',role='candidate',method=dict(id='ic.three_leads_genus2_pencil_symbolic_v2',params=dict(small_denominator='8',matrix_coefficient_cap='25000000'))),dict(name='rho',role='reference',method=rho['method']),dict(name='rho_AA',role='control',method=copy.deepcopy(rho['method']))]
    spec['label']=f'pencil-symbolic-paired-{bits}'
    spec['description']='Prepared specification; execution requires a frozen experiment protocol and admission. Public synthetic archived targets; fixed denominator8 selected before symbolic cohort observations. Cold/online clocks, failures and memory retained; no full-price GAE claim.'
    spec['measurement'].update(warmup=1,rounds=5,algorithm_seed_round='measured_index',isolation_required='L2',timeout_seconds=3600,order='shuffle')
    specs[f'cohort-{bits}.json']=spec
small=copy.deepcopy(specs['cohort-16.json'])
small['workloads']['curves']=[c for c in small['workloads']['curves'] if c['p']==739]
assert len(small['workloads']['curves'])==1
small['label']='pencil-symbolic-gate-p739'
specs['gate-p739.json']=small
upper=copy.deepcopy(specs['cohort-32.json'])
upper['workloads']['curves']=[c for c in upper['workloads']['curves'] if c['p']==186379]
assert len(upper['workloads']['curves'])==1
upper['workloads']['targets_per_curve']=1
upper['label']='pencil-symbolic-gate-p186379-target0'
specs['gate-p186379-target0.json']=upper
a.out.mkdir(exist_ok=False)
for name,spec in specs.items():
    (a.out/name).write_text(json.dumps(spec,indent=2)+'\n')
manifest=dict(schema='pencil-prepared-specs/v1',status='not_frozen_not_launch_authorization',source_archive_sha256=m['source_archive_sha256'],small_denominator=8,matrix_coefficient_cap=25000000,memory_cap_gib=8,per_execution_timeout_seconds=3600,gate_order=['gate-p739.json','gate-p186379-target0.json'],extension_condition='upper target verifies under the original resource gate and independent replay; retain failure and do not automatically extend after censoring',measured_seed_semantics='index0 preserves archived seed; indices1..4 are additional predetermined paired algorithm streams, not repetitions of index0',qualified_ratios_available=False,files={name:hashlib.sha256((a.out/name).read_bytes()).hexdigest() for name in sorted(specs)})
(a.out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
