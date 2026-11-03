#!/usr/bin/env python3
"""Check the exact two-arm bridge selector and its admission gates."""

import copy
import json

from research.n83_global_bridge_20261018 import select as subject
from research.n83_root4_frontier_current_20261018 import test_select as root4_test


def fixture():
    data = root4_test.fixture()
    ids = data['candidate_ids'][1:]
    data['schema'] = 'n83.global-bridge-validated/v1'
    data['targets_sha256'] = subject.TARGETS_SHA256
    data['candidate_ids'] = ids
    data['choice'] = {
        'schema': 'n83.global-bridge-upstream/v1',
        'status': 'TWO_DISTINCT_ADMITTED',
        'source_sha256': subject.SOURCE_SHA256,
        'ordered_candidate_ids': ids,
        'upstream_online_ids': ids,
        'candidate_manifest_sha256': {candidate: 'a' * 64 for candidate in ids},
    }
    for point in data['points']:
        point['arms'] = point['arms'][1:]
        point['rho_by_candidate'] = {key: point['rho_by_candidate'][key]
                                     for key in ids}
        point['calibration_by_candidate'] = {
            key: point['calibration_by_candidate'][key] for key in ids}
    return data


def main():
    data = fixture()
    result = subject.select(data)
    assert result['admission_status'] == 'PASS'
    assert result['online']['selected_candidate_id'] == data['candidate_ids'][1]
    assert result['cold']['selected_candidate_id'] == data['candidate_ids'][1]
    assert result['online']['bonferroni_alpha_per_pair'] == 0.05
    assert result['online']['comparisons'][0]['two_sided_sign_flip_p'] == 2 / 65536
    tied = copy.deepcopy(data)
    for point in tied['points']:
        point['arms'][0]['one_target_online_ms'] = 30
        point['arms'][1]['one_target_online_ms'] = 30
    assert subject.select(tied)['online']['selected_candidate_id'] is None
    censored = copy.deepcopy(data)
    censored['points'][0]['arms'][1]['status'] = 'TIMEOUT'
    assert subject.select(censored)['online']['selected_candidate_id'] is None
    noisy = copy.deepcopy(data)
    noisy['host_strata'][0]['aa_status'] = 'NOISY_OR_INCOMPLETE'
    assert subject.select(noisy)['online']['selected_candidate_id'] is None
    print(json.dumps({'status': 'PASS_GLOBAL_BRIDGE_SELECTOR', 'cases': 4},
                     sort_keys=True))


if __name__ == '__main__':
    main()
