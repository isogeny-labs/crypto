#!/usr/bin/env python3
"""Independently audit screen coverage, provenance and complete cell receipts."""
import hashlib
import json
import math
import re
from pathlib import Path

HERE=Path(__file__).resolve().parent
OLD=HERE.parent/'ghs_odd5_carrier_20261115'
source=json.loads((OLD/'screen.json').read_text())
classes=json.loads((OLD/'class_count.json').read_text())
manifest=json.loads((HERE/'manifest.json').read_text())
fmasks=sorted(row[0] for row in source['rows'] if row[-1]==22)
assert len(fmasks)==12 and len(set(fmasks))==12
bmasks=sorted(row[1] for row in classes['selected_rows'] if row[2]==22)
orbit1=(68,120,74,99,111)
orbit2=(84,144,211,96,105)
assert bmasks==sorted(orbit1+orbit2)
assert manifest['source_sha256']==hashlib.sha256((HERE/'screen_variant.sage').read_bytes()).hexdigest()
assert manifest['runner_sha256']==hashlib.sha256((HERE/'run_variants.py').read_bytes()).hexdigest()
assert manifest['preparsed_sha256']==hashlib.sha256((HERE/'screen_variant.sage.py').read_bytes()).hexdigest()
assert manifest['total_hits']==0
expected={(fm,bm) for fm in fmasks for bm in (68,84)}
assert {(row['fmask'],row['bmask']) for row in manifest['jobs']}==expected
assert len(manifest['jobs'])==len(expected)==24

rows=[]
for fm,bm in sorted(expected):
    stem=f'f{fm}_e{bm}'
    output=HERE/f'{stem}.json'
    stderr=HERE/f'{stem}.stderr'
    d=json.loads(output.read_text())
    assert d['schema']=='ghs.odd5.trace22-carrier-variant-screen/v1'
    assert d['source_sha256']==manifest['source_sha256']
    assert (d['fmask'],d['bmask'],d['target_order'])==(fm,bm,222)
    assert d['allowed_H']==58559 and d['hits']==[]
    assert d['counts']['Hquadratic']['systems']==243
    assert d['counts']['QiHquadratic']['systems']==405
    assert d['counts']['Hquadratic']['logical_assignments']==14229837
    assert d['counts']['QiHquadratic']['logical_assignments']==23716395
    assert sum(c['full_equation_hits'] for c in d['counts'].values())==len(d['hits'])
    record=stderr.read_text()
    m=re.search(r'^\s*([0-9.]+) real\s+([0-9.]+) user\s+([0-9.]+) sys',record,re.M)
    assert m and 'Traceback' not in record and 'error:' not in record
    real,user,system=map(float,m.groups())
    rows.append({'fmask':fm,'bmask':bm,'real_s':real,'user_cpu_s':user,
                 'system_cpu_s':system,'receipt_sha256':hashlib.sha256(output.read_bytes()).hexdigest(),
                 'stderr_sha256':hashlib.sha256(stderr.read_bytes()).hexdigest()})

total_user=sum(row['user_cpu_s'] for row in rows)
total_system=sum(row['system_cpu_s'] for row in rows)
result={'schema':'ghs.odd5.trace22-carrier-variant-audit/v1',
        'carrier_masks':fmasks,'source_masks':bmasks,'source_orbits':[orbit1,orbit2],
        'screened_representative_pairs':len(rows),
        'pairs_covered_by_frobenius':len(fmasks)*len(bmasks),
        'total_hits':0,'total_logical_assignments':len(rows)*(14229837+23716395),
        'aggregate_user_cpu_s':round(total_user,2),
        'aggregate_system_cpu_s':round(total_system,2),
        'cells':rows,
        'audit_scope':'coverage, hashes, counts, timing and census membership; not an independent algebraic elimination'}
(HERE/'audit_manifest.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
print(json.dumps({k:v for k,v in result.items() if k!='cells'},sort_keys=True))
