#!/usr/bin/env python3
"""Bind twelve complete degree-five cells and independent square witnesses."""
import hashlib
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
MASKS=(9735,17371,21624,44227,64725,90937,92153,
       100118,124586,144548,156795,159256)
source_hash=hashlib.sha256((HERE/'screen.sage').read_bytes()).hexdigest()
witness_hash=hashlib.sha256((HERE/'square_witness.sage').read_bytes()).hexdigest()
assert sorted(int(p.stem.split('_')[1]) for p in HERE.glob('cell_*.json'))==list(MASKS)
expected={'constant':243,'quadratic_branch':405,'double_linear':58806,
          'two_quadratic_branches':270,
          'quadratic_branch_double_linear':98010,
          'square_quadratic_nonbranch':58801*243}
rows=[]
for mask in MASKS:
    d=json.loads((HERE/('cell_%d.json'%mask)).read_text())
    assert d['schema']=='ghs.trace-minus14.isogenous-degree5/v1'
    assert d['source_sha256']==source_hash
    assert (d['fmask'],d['bmask'],d['trace'])==(mask,202,-14)
    assert d['logical_assignment_count']==14446377
    assert d['denominator_count']==60269
    assert sum(d['square_quadratic_rank_hist'].values())==243
    assert d['consistent_rank_deficient']==11
    assert d['fallback_H_checks']==646811
    assert set(d['counts'])==set(expected)
    for kind,n in expected.items():
        assert d['counts'][kind]['logical_assignments']==n
        assert d['counts'][kind]['full_equation_hits']==0
    assert d['hits']==[] and len(d['survivors'])==1
    assert d['survivors'][0]['shape']=='two_quadratic_branches'
    rows.append({'fmask':mask,'cpu_seconds':d['cpu_seconds'],
                 'survivors':len(d['survivors']),'hits':len(d['hits'])})
w=json.loads((HERE/'SQUARE_WITNESS.json').read_text())
assert w['schema']=='ghs.trace-minus14.degree5-square-witness/v1'
assert w['source_sha256']==witness_hash
assert w['cells']==12 and w['exact_square_hits']==0
assert [r['fmask'] for r in w['rows']]==list(MASKS)
kinds={k:sum(r['obstruction']['kind']==k for r in w['rows']) for k in
       ('nonsquare-leading-coefficient','lower-coefficient-contradiction')}
assert kinds=={'nonsquare-leading-coefficient':6,'lower-coefficient-contradiction':6}
out={'schema':'ghs.trace-minus14.isogenous-degree5-audit/v1',
     'source_sha256':source_hash,'square_witness_sha256':witness_hash,
     'cells':12,'total_logical_assignments':12*14446377,
     'total_fallback_H_checks':12*646811,
     'total_survivors':12,'total_hits':0,
     'witness_kinds':kinds,'cpu_seconds_sum':sum(r['cpu_seconds'] for r in rows),
     'rows':rows}
(HERE/'AUDIT.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps(out,sort_keys=True))
