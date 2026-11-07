#!/usr/bin/env python3
"""Fail-closed hash and count audit for the order-4126 cover screens."""

from hashlib import sha256
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PARENT = ROOT/'ghs_automorphism_screen_q16_20261026'
NEIGHBOR = ROOT/'ghs_degree3_neighbor_screen_q16_20261026'
CATALOG = PARENT/'catalog_models_raw.json'
CLOSURE = ROOT/'ghs_cubic_hess_closure_q16_20261027/certificate.json'


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def check_hashes(row, paths):
    for key, path in paths.items():
        assert row[key] == digest(path), (key, path)


def main():
    one_path = HERE/'certificate.json'
    fast_path = HERE/'all_sources_certificate.json'
    replay_path = HERE/'replay_378_certificate.json'
    one = json.loads(one_path.read_text())
    fast = json.loads(fast_path.read_text())
    replay = json.loads(replay_path.read_text())
    check_hashes(one, {
        'protocol_sha256': HERE/'PROTOCOL.md',
        'script_sha256': HERE/'screen.py',
        'catalog_sha256': CATALOG,
        'closure_sha256': CLOSURE,
        'parent_enumerator_sha256': PARENT/'screen_degree3_pattern.py',
        'parent_neighbor_sha256': NEIGHBOR/'screen_neighbors.py',
    })
    check_hashes(fast, {
        'protocol_sha256': HERE/'ALL_SOURCES_PROTOCOL.md',
        'script_sha256': HERE/'fast_screen.py',
        'catalog_sha256': CATALOG,
        'closure_sha256': CLOSURE,
        'one_source_certificate_sha256': one_path,
        'parent_enumerator_sha256': PARENT/'screen_degree3_pattern.py',
        'parent_neighbor_sha256': NEIGHBOR/'screen_neighbors.py',
    })
    check_hashes(replay, {
        'script_sha256': HERE/'replay_378.py',
        'catalog_sha256': CATALOG,
        'fast_certificate_sha256': fast_path,
    })
    assert one['models_checked'] == 140
    assert one['four_support_checked'] == 3360
    assert one['five_support_checked'] == 4584160
    assert one['five_support_residue_pass'] == 0 and one['hits'] == []
    assert len(one['per_model']) == 140
    assert all(row['four_support_checked'] == 24 and
               row['five_support_checked'] == 32744 and row['hits'] == 0
               for row in one['per_model'])
    assert len(fast['source_a6_codes']) == len(set(fast['source_a6_codes'])) == 108
    assert [row['a6'] for row in fast['per_source']] == fast['source_a6_codes']
    assert len(fast['per_source']) == 108 and fast['catalog_models'] == 140
    assert fast['hits'] == []
    totals = fast['totals']
    assert totals == {
        'pairs':15120,
        'four_assignments_covered':362880,
        'four_residue_pass':96,
        'five_assignments_covered':495089280,
        'five_finite_candidates':60444,
        'five_residue_pass':0,
    }
    assert all(row['pairs']==140 and
               row['four_assignments_covered']==3360 and
               row['five_assignments_covered']==4584160 and
               row['five_residue_pass']==row['hits']==0
               for row in fast['per_source'])
    assert all(sum(row[key] for row in fast['per_source']) == totals[key]
               for key in totals)
    assert fast['controls']['fast_four_hits'] >= 1
    assert fast['controls']['fast_five_hits'] >= 1
    assert replay['source_a6']==378
    assert replay['models_checked']==140
    assert replay['four_assignments_enumerated']==3360
    assert replay['five_assignments_enumerated']==4584160
    assert replay['five_residue_pass']==replay['hits']==0

    sparse = json.loads((PARENT/'sparse_certificate.json').read_text())
    nonsparse = json.loads((PARENT/'degree2_all_nonsparse_certificate.json').read_text())
    s = next(row for row in sparse['per_class'] if row['label']=='3.16.a_a_bd')
    n = nonsparse['per_class_summary']['3.16.a_a_bd']
    assert s['all_models']==140 and s['sparse_gamma_nonzero']==4
    assert n['selected']==n['checked']==136
    assert nonsparse['outside_results']==[]
    print(json.dumps({'status':'PASS','pairs':15120,
                      'degree_two_models':140,
                      'degree_three_hits':0,
                      'four_residue_pass':96,
                      'five_residue_pass':0},sort_keys=True))


if __name__=='__main__':
    main()
