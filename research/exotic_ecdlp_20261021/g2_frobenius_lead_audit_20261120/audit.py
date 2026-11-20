#!/usr/bin/env python3
"""Replay frozen archive claims without modifying the source receipts."""
import hashlib
import json
from pathlib import Path

ROOT = Path('/Volumes/SSD990/cryptanalysis/experiments')
SOURCES = {
    'g2-n31-frobenius-linear-action-20261011/covariance_receipt.json': 'ea81ad7dd9df327f8ea96723a54ea3d5f259c5597bbf9cd45d3635d3e0e925c6',
    'g2-n31-frobenius-linear-action-20261011/rows_receipt.json': 'ed5220705100e19ab10c7127ff22091df43413c5533d4444627fa86370bd69e3',
    'g2-n31-frobenius-linear-action-20261011/rows_validation_receipt.json': '5ecf2aadfa7fd34d5f2bd7b9da5047d87f38407e63027331f8387a440368281c',
    'g2-n31-heldout-pivot-panel-20261011/union_test_receipt.json': 'aafd9506c877f4cf103e79bfc1365b4f94552a54e81a1eaff7dd29fbbdce09ed',
    'g2-n31-heldout-pivot-panel-20261011/union_failure_receipt.json': '75dbdcb4f9c85ba0eb287dd3cb6aa5e07d52c19bca1070adf3116185831d7931',
}

def read(name, digest):
    data = (ROOT / name).read_bytes()
    assert hashlib.sha256(data).hexdigest() == digest, name
    value = json.loads(data)
    assert value['status'] == 'verified', name
    return value

def main():
    docs = {name.rsplit('/', 1)[-1]: read(name, digest) for name, digest in SOURCES.items()}
    covariance = docs['covariance_receipt.json']['cases']
    assert len(covariance) == 2
    for case in covariance:
        assert case['equations_checked'] == 31
        assert case['frobenius_orbit_length'] == 31
        assert case['symbolic_equality'] is True
        assert case['q2_after_leaf_square_sha256'] == case['squared_equations_sha256']
        assert case['q2_same_leaf_sha256'] != case['squared_equations_sha256']
    rows = docs['rows_receipt.json']['cases']
    assert {case['basis'] for case in rows} == {'code', 'component'}
    assert {case['rank'] for case in rows} == {15841}
    assert {case['rows']['nnz'] for case in rows} == {2107769, 2110510}
    assert len(docs['rows_validation_receipt.json']['cases']) == 2
    panel = docs['union_test_receipt.json']
    assert len(panel['information_set_columns']) == 15856
    assert panel['rank_preserved_count'] == 14
    fresh = [case for case in panel['cases'] if case['label'].startswith('fresh_')]
    assert len(fresh) == 16
    assert sum(case['projected_rank'] == case['expected_rank'] for case in fresh) == 14
    failures = [case for case in fresh if case['projected_rank'] < case['expected_rank']]
    assert {case['label'] for case in failures} == {'fresh_1', 'fresh_15'}
    witnesses = docs['union_failure_receipt.json']['cases']
    assert {case['label'] for case in witnesses} == {'fresh_1', 'fresh_15'}
    for case in witnesses:
        assert case['full_rank'] == 15841 and case['projected_rank'] == 15835
        assert case['outside_union_count'] == len(case['outside_union_pivots']) == 11
    result = {
        'status': 'PASS', 'source_sha256': SOURCES,
        'covariant_targets': [case['q_bits'] for case in covariance],
        'fixed_target_self_symmetry': False,
        'full_matrix_rank': 15841,
        'basis_row_nnz': {case['basis']: case['rows']['nnz'] for case in rows},
        'heldout_projection': {'preserved': 14, 'total': 16, 'failed_labels': sorted(case['label'] for case in failures)},
    }
    print(json.dumps(result, indent=2, sort_keys=True))

if __name__ == '__main__':
    main()
