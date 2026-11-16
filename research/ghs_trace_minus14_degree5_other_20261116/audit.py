#!/usr/bin/env python3
"""Audit degree-five exclusion on all recorded trace -14 equation pairs.

The F3 coordinate transformations are reimplemented here in Python. Sage
interpolation remains a separate, source-bound computational argument.
"""
import hashlib
import itertools
import json
from pathlib import Path

HERE=Path(__file__).resolve().parent
PARENT=HERE.parent
MASKS=(9735,17371,21624,44227,64725,90937,92153,
       100118,124586,144548,156795,159256)
EXPECTED={'constant':243,'quadratic_branch':405,'double_linear':58806,
          'two_quadratic_branches':270,
          'quadratic_branch_double_linear':98010,
          'square_quadratic_nonbranch':58801*243}
assert sum(EXPECTED.values())==14446377

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def pmul(u,v):
    out=[0]*(len(u)+len(v)-1)
    for i,a in enumerate(u):
        for j,b in enumerate(v):
            out[i+j]=(out[i+j]+a*b)%3
    return out

def ppow(u,n):
    out=[1]
    for _ in range(n):
        out=pmul(out,u)
    return out

def transformed_mask(mask,matrix):
    a,b,c,d=matrix
    assert (a*d-b*c)%3!=0
    coeff=[(mask//3**i)%3 for i in range(11)]+[1]
    out=[0]*13
    for i,fi in enumerate(coeff):
        if fi==0:continue
        term=pmul(ppow([b,a],i),ppow([d,c],12-i))
        for j,v in enumerate(term):
            out[j]=(out[j]+fi*v)%3
    if out[12] or out[11]!=1:
        return None
    return sum(out[i]*3**i for i in range(11))

orbit_receipt=json.loads((HERE/'CARRIER_ORBITS.json').read_text())
assert orbit_receipt['source_sha256']==digest(HERE/'carrier_orbits.sage')
assert orbit_receipt['covered_models']==12
assert len(orbit_receipt['orbits'])==2
all_carriers=set()
for orbit in orbit_receipt['orbits']:
    rep=orbit['representative']
    assert rep in (9735,17371) and orbit['pgl2_classes']==24
    classes={}
    for t in itertools.product(range(3),repeat=4):
        if (t[0]*t[3]-t[1]*t[2])%3==0:continue
        neg=tuple((-v)%3 for v in t)
        key=min(t,neg)
        classes[key]=transformed_mask(rep,key)
    assert len(classes)==24
    models={m for m in classes.values() if m is not None}
    assert models==set(orbit['models'])=={int(k) for k in orbit['maps']}
    assert len(models)==6
    for target,matrix in orbit['maps'].items():
        assert transformed_mask(rep,matrix)==int(target)
    assert not (all_carriers&models)
    all_carriers.update(models)
assert all_carriers==set(MASKS)

class_census=json.loads((PARENT/'ghs_odd5_carrier_20261115/class_count.json').read_text())
source_masks={r[1] for r in class_census['selected_rows'] if r[2]==-14}
assert len(source_masks)==20
source_orbits=json.loads((PARENT/'ghs_trace_minus14_isogenous_degree4_20261116/AUDIT.json').read_text())['source_orbits']
assert len(source_orbits)==4 and set(sum(source_orbits,[]))==source_masks

old_dir=PARENT/'ghs_trace_minus14_20261116'
iso_dir=PARENT/'ghs_trace_minus14_degree5_isogenous_20261116'
specs=[(11,9735,old_dir/'degree5_screen.json',old_dir/'degree5_screen.sage',
        'ghs.odd5.degree5-screen/v1'),
       (11,17371,HERE/'cell_11_17371.json',HERE/'screen_11.sage',
        'ghs.trace-minus14.last-orbit-degree5/v1')]
for bmask in (38,80):
    specs.extend((bmask,mask,HERE/f'cell_{bmask}_{mask}.json',HERE/'screen.sage',
                  'ghs.trace-minus14.other-orbits-degree5/v1')
                 for mask in MASKS)
specs.extend((202,mask,iso_dir/f'cell_{mask}.json',iso_dir/'screen.sage',
              'ghs.trace-minus14.isogenous-degree5/v1') for mask in MASKS)
assert len(specs)==38
survivors={}
for bmask,mask,path,source,schema in specs:
    d=json.loads(path.read_text())
    assert d['schema']==schema and d['source_sha256']==digest(source)
    if (bmask,mask)!=(11,9735):
        assert (d['bmask'],d['fmask'])==(bmask,mask)
    assert d['logical_assignment_count']==14446377
    assert sum(d['square_quadratic_rank_hist'].values())==243
    assert d['consistent_rank_deficient']==11
    assert d['fallback_H_checks']==646811
    assert set(d['counts'])==set(EXPECTED)
    for shape,n in EXPECTED.items():
        assert d['counts'][shape]['logical_assignments']==n
        assert d['counts'][shape]['full_equation_hits']==0
    assert d['hits']==[]
    survivors[bmask,mask]=len(d['survivors'])
assert sum(survivors.values())==54

def check_witness(path,source,expected_count):
    w=json.loads(path.read_text())
    assert w['source_sha256']==digest(source)
    assert w['candidate_count']==expected_count and w['exact_square_hits']==0
    assert len(w['rows'])==expected_count
    for row in w['rows']:
        kind=row['obstruction']['kind']
        assert kind in ('nonsquare-leading-coefficient',
                        'lower-coefficient-contradiction')
        if kind=='lower-coefficient-contradiction':
            assert row['obstruction']['residual_degree']<4
    return w

w38=check_witness(HERE/'SQUARE_WITNESS.json',HERE/'square_witness.sage',39)
w11=check_witness(HERE/'SQUARE_WITNESS_11.json',HERE/'square_witness_11.sage',3)
w202=json.loads((iso_dir/'SQUARE_WITNESS.json').read_text())
assert w202['source_sha256']==digest(iso_dir/'square_witness.sage')
assert w202['cells']==12 and w202['exact_square_hits']==0
assert len(w202['rows'])==12
assert sum(survivors.values())==39+3+12
assert {(r['bmask'],r['fmask'],r['survivor_index']) for r in w38['rows']}=={
    (b,m,j) for (b,m),n in survivors.items() if b in (38,80) for j in range(n)}
assert {(r['fmask'],r['survivor_index']) for r in w11['rows']}=={
    (m,j) for (b,m),n in survivors.items() if b==11 for j in range(n)}

out={'schema':'ghs.trace-minus14.degree5-full-slice-audit/v1',
     'source_representatives':[11,202,38,80],
     'carrier_representatives':[9735,17371],
     'recorded_sources':20,'recorded_carriers':12,
     'exact_equation_pairs':240,'screened_representative_cells':38,
     'screened_logical_assignments':38*14446377,
     'retained_numerators':54,'independent_square_obstructions':54,
     'exact_hits':0,'carrier_orbit_source_sha256':digest(HERE/'carrier_orbits.sage'),
     'degree5_source_sha256':digest(HERE/'screen.sage'),
     'witness_38_80_sha256':digest(HERE/'square_witness.sage'),
     'witness_11_sha256':digest(HERE/'square_witness_11.sage')}
(HERE/'AUDIT.json').write_text(json.dumps(out,indent=2,sort_keys=True)+'\n')
print(json.dumps(out,sort_keys=True))
