#!/usr/bin/env python3
"""Check exact paired root4 selection and fail-closed admission."""

import copy
import json

from research.n83_root4_highk_fresh_20261019 import select as subject


def fixture():
    ids = list(subject.IDS)
    points = []
    for index in range(16):
        points.append({
            'point_index': index,
            'arms': [
                {'candidate_id': candidate, 'status': 'PASS_INDEPENDENT_REPLAY',
                 'resource_status': 'PASS', 'one_target_online_ms': 50 - 10 * arm,
                 'cold_process_ms': 120_000 - 10_000 * arm}
                for arm, candidate in enumerate(ids)],
            'rho_by_candidate': {
                candidate: {'status': 'PASS_INDEPENDENT_REPLAY',
                            'resource_status': 'PASS', 'online_ms': 100.0}
                for candidate in ids},
            'calibration_by_candidate': {
                candidate: {'status': 'PASS_10_PERCENT'} for candidate in ids},
        })
    return {
        'schema': 'n83.highk-fresh-validated/v1',
        'source_sha256': subject.SOURCE_SHA256,
        'targets_sha256': subject.TARGETS_SHA256,
        'evidence_status': 'PASS_INDEPENDENT_REPLAY',
        'different_cloud_instances': True,
        'same_candidate_bases_across_strata': True,
        'candidate_ids': ids,
        'choice': {'schema': 'n83.highk-fresh-choice/v1',
                   'source_sha256': subject.SOURCE_SHA256,
                   'ordered_candidate_ids': ids},
        'host_strata': [
            {'point_indices': list(range(i * 8, (i + 1) * 8)),
             'tier_status': 'PASS_TIER1',
             'aa_status': 'PASS_10_PERCENT_PER_PAIR'}
            for i in range(2)],
        'points': points,
    }


def main():
    minimum = 2 / (1 << 16)
    assert subject.sign_flip_p([1.0] * 16) == minimum
    assert subject.sign_flip_p([-1.0] * 16) == minimum
    assert subject.sign_flip_p([1.0, -1.0] * 8) == 1.0
    data = fixture()
    result = subject.select(data)
    assert result['admission_status'] == 'PASS'
    assert result['online']['selected_candidate_id'] == subject.IDS[2]
    assert result['cold']['selected_candidate_id'] == subject.IDS[2]
    assert len(result['online']['comparisons']) == 3
    assert result['online']['bonferroni_alpha_per_pair'] == 0.05 / 3
    noisy = copy.deepcopy(data)
    noisy['host_strata'][0]['aa_status'] = 'NOISY_OR_INCOMPLETE'
    assert subject.select(noisy)['online']['selected_candidate_id'] is None
    failed = copy.deepcopy(data)
    failed['points'][0]['arms'][2]['status'] = 'TIMEOUT'
    assert subject.select(failed)['admission_status'] == (
        'IC_REPLAY_RESOURCE_OR_TIMING_GATE')
    wrong = copy.deepcopy(data)
    wrong['different_cloud_instances'] = False
    assert subject.select(wrong)['online']['selected_candidate_id'] is None
    tied = copy.deepcopy(data)
    for point in tied['points']:
        for arm in point['arms']:
            arm['one_target_online_ms'] = 30
            arm['cold_process_ms'] = 100_000
    tied_result = subject.select(tied)
    assert tied_result['online']['status'] == 'BOUNDED_UNKNOWN'
    assert tied_result['online']['selected_candidate_id'] is None
    assert tied_result['cold']['selected_candidate_id'] is None
    original = subject.sign_flip_p
    try:
        def unstable(_differences):
            raise ValueError('enumerated and split counts differ')
        subject.sign_flip_p = unstable
        numeric = subject.select(data)
        assert numeric['admission_status'] == 'STATISTICAL_REPLAY_NUMERIC_MISMATCH'
        assert numeric['online']['selected_candidate_id'] is None
        assert numeric['cold']['selected_candidate_id'] is None
    finally:
        subject.sign_flip_p = original
    print(json.dumps({'status': 'PASS_HIGHK_FRESH_SELECTOR',
                      'cases': 9, 'exact_minimum_p': minimum}, sort_keys=True))


if __name__ == '__main__':
    main()
