#!/usr/bin/env python3
"""Verify frozen source and structural receipts for all degree-six orbit cells."""
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
OLD = ROOT / 'research/ghs_trace_minus14_20261116'
OTHER = ROOT / 'research/ghs_trace_minus14_degree5_other_20261116'
ISO = ROOT / 'research/ghs_trace_minus14_isogenous_degree4_20261116'
SOURCE_REPS = (11, 202, 38, 80)
CARRIER_REPS = (9735, 17371)
SHAPES = {'L': 243, 'L3': 243, 'LQi': 405, 'Ldouble': 58806,
          'L5': 243, 'L3Qi': 405, 'L3double': 58806,
          'LQiQj': 270, 'LQidouble': 98010, 'LHsquare': 14288643}
assert sum(SHAPES.values()) == 14506074

def read(path):
    return json.loads(path.read_text())

def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

source_hash = digest(HERE / 'screen.sage')
assert source_hash == '85d92e98849801cb35570e91c6a00755488b91090a4f1eaf8971082e9165250d'
source_orbits = read(ISO / 'AUDIT.json')['source_orbits']
assert len(source_orbits) == 4 and len(set(sum(source_orbits, []))) == 20
assert {orbit[0] for orbit in source_orbits} == set(SOURCE_REPS)
carrier = read(OTHER / 'CARRIER_ORBITS.json')
assert len(carrier['orbits']) == 2
assert {o['representative'] for o in carrier['orbits']} == set(CARRIER_REPS)
assert len(set(sum((o['models'] for o in carrier['orbits']), []))) == 12
assert carrier['covered_models'] == 12
cells = []
for b in SOURCE_REPS:
    for f in CARRIER_REPS:
        reused = (b, f) == (11, 9735)
        path = (OLD / 'degree6_screen.json') if reused else (HERE / f'cell_{b}_{f}.json')
        stderr = (OLD / 'degree6_screen.stderr') if reused else (HERE / f'cell_{b}_{f}.stderr')
        row = read(path)
        assert stderr.exists() and not stderr.read_text().startswith('Traceback')
        expected_hash = digest(OLD / 'degree6_screen.sage') if reused else source_hash
        assert row['source_sha256'] == expected_hash
        assert row['a6_mask'] == b and row.get('carrier_mask', 9735) == f
        assert row['field'] == 'F3[a]/(a^5+2a+1)'
        assert row['target_order'] == 258 and row['trace'] == -14
        assert row['branch_assignment_count'] == 14506074
        assert row['LQiQj_lambda_values_per_assignment'] == 242
        assert set(row['counts']) == set(SHAPES)
        for shape, n in SHAPES.items():
            assert row['counts'][shape]['branch_assignments'] == n
            assert row['counts'][shape]['full_equation_hits'] == 0
        assert row['hits'] == []
        cells.append({'source_mask': b, 'carrier_mask': f, 'reused': reused,
                      'source_sha256': expected_hash,
                      'branch_assignments': row['branch_assignment_count'],
                      'resultant_fallback_choices': row['fallback_choice_count'],
                      'high_system_solutions': row['high_system_solutions'],
                      'exact_maps': len(row['hits'])})

assert len(cells) == 8 and sum(x['reused'] for x in cells) == 1
out = {'schema': 'ghs.trace-minus14.degree6-orbit-audit/v1',
       'source_sha256': digest(Path(__file__)), 'screen_sha256': source_hash,
       'source_orbits': source_orbits, 'carrier_orbits': carrier['orbits'],
       'exact_equation_pairs': 240, 'representative_cells': 8,
       'new_cells': 7, 'branch_assignments_total': 8 * 14506074,
       'exact_maps': 0, 'cells': cells,
       'claim_boundary': 'degree-six K-morphisms under frozen denominator classification; one elimination implementation per cell'}
print(json.dumps(out, indent=2, sort_keys=True))
