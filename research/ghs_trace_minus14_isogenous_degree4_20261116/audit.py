#!/usr/bin/env python3
"""Audit all four trace -14 source orbits and degree-four cell receipts.

Finite-field orbit arithmetic below is independent Python; cover elimination
is source-bound Sage, not independently reimplemented here.
"""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PARENT = HERE.parent
CARRIER_MASKS = (9735, 17371, 21624, 44227, 64725, 90937, 92153,
                 100118, 124586, 144548, 156795, 159256)
REPS = (11, 202, 38, 80)
EXPECTED_ORBITS = ((11, 17, 71, 147, 238),
                   (202, 81, 117, 47, 187),
                   (38, 172, 223, 53, 241),
                   (80, 153, 190, 156, 217))
EXPECTED_COUNTS = {'linear_only': 243, 'triple_linear': 243,
                   'linear_plus_quadratic': 405,
                   'linear_plus_double_nonbranch': 58806}

def multiply_masks(left, right):
    lc = [(left // 3**i) % 3 for i in range(5)]
    rc = [(right // 3**i) % 3 for i in range(5)]
    c = [0] * 9
    for i in range(5):
        for j in range(5):
            c[i+j] += lc[i] * rc[j]
    # a^5 + 2a + 1 = 0, so a^5 = a + 2 in characteristic three.
    for e in range(8, 4, -1):
        c[e-4] += c[e]
        c[e-5] += 2 * c[e]
    return sum((c[i] % 3) * 3**i for i in range(5))

def cube_mask(m):
    return multiply_masks(multiply_masks(m, m), m)

orbits = []
for rep, expected in zip(REPS, EXPECTED_ORBITS):
    orbit = [rep]
    for _ in range(4):
        orbit.append(cube_mask(orbit[-1]))
    assert tuple(orbit) == expected
    assert cube_mask(orbit[-1]) == rep
    assert len(set(orbit)) == 5
    orbits.append(orbit)
assert len(set(sum(orbits, []))) == 20
census = json.loads((PARENT / 'ghs_odd5_carrier_20261115/class_count.json').read_text())
assert set(sum(orbits, [])) == {r[1] for r in census['selected_rows'] if r[2] == -14}
carriers = json.loads((PARENT / 'ghs_odd5_carrier_20261115/cubic_all.json').read_text())
assert set(CARRIER_MASKS) == {r['mask'] for r in carriers['rows'] if r['trace'] == -14}

sources = {
    11: PARENT / 'ghs_trace_minus14_carrier_degree4_20261116/screen.sage',
    202: HERE / 'screen.sage',
    38: HERE / 'screen_other.sage',
    80: HERE / 'screen_other.sage',
}
source_hashes = {rep: hashlib.sha256(path.read_bytes()).hexdigest()
                 for rep, path in sources.items()}
rows = []
for rep in REPS:
    for carrier in CARRIER_MASKS:
        if rep == 11:
            path = PARENT / f'ghs_trace_minus14_carrier_degree4_20261116/cell_{carrier}.json'
            schema = 'ghs.trace-minus14.carrier-degree4/v1'
        elif rep == 202:
            path = HERE / f'cell_{carrier}.json'
            schema = 'ghs.trace-minus14.isogenous-degree4/v1'
        else:
            path = HERE / f'other_{rep}_{carrier}.json'
            schema = 'ghs.trace-minus14.other-orbits-degree4/v1'
        r = json.loads(path.read_text())
        assert r['schema'] == schema and r['fmask'] == carrier
        if rep in (38, 80):
            assert r['bmask'] == rep
        assert r['source_sha256'] == source_hashes[rep]
        assert r['trace'] == -14 and r['source_order'] == 258
        assert r['hits'] == [] and r['survivors'] == []
        assert set(r['counts']) == set(EXPECTED_COUNTS)
        for kind, count in EXPECTED_COUNTS.items():
            c = r['counts'][kind]
            assert c['assignments'] == count
            assert all(c[k] == 0 for k in ('degree_four_crt', 'coprime_crt',
                                         'polynomial_quotient', 'square_quotient'))
        rows.append({'bmask': rep, 'fmask': carrier, 'cpu_seconds': r['cpu_seconds']})
assert len(rows) == 48

iso = json.loads((HERE / 'ISOGENY.json').read_text())
assert iso['schema'] == 'ghs.trace-minus14.two-isogeny-witness/v1'
assert iso['source_sha256'] == hashlib.sha256((HERE / 'isogeny_witness.sage').read_bytes()).hexdigest()
assert iso['degree'] == 2 and iso['source_order'] == 258
assert iso['normal_codomain_order'] == 258
assert iso['subgroup_image_order'] == 43 and iso['Q_scalar'] == 7

out = {'schema': 'ghs.trace-minus14.all-source-degree4-audit/v1',
       'carrier_masks': list(CARRIER_MASKS), 'source_orbits': orbits,
       'source_hashes': source_hashes, 'representative_cells': len(rows),
       'representative_assignments': len(rows) * 59697,
       'frobenius_extended_pairs': len(CARRIER_MASKS) * 20,
       'degree_four_numerator_candidates': 0, 'exact_hits': 0,
       'cpu_seconds_sum': sum(r['cpu_seconds'] for r in rows),
       'two_isogeny_subgroup_image_order': iso['subgroup_image_order']}
(HERE / 'AUDIT.json').write_text(json.dumps(out, indent=2, sort_keys=True) + '\n')
print(json.dumps(out, sort_keys=True))
