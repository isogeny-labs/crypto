#!/usr/bin/env python3
"""Check the selected-finalist bridge choice, convergence and censoring."""

import copy
import json
from pathlib import Path
from tempfile import TemporaryDirectory

from research.n83_final_extended_20261018 import select as extended_select
from research.n83_final_extended_20261018 import test_select as extended_test
from research.n83_root4_frontier_current_20261018 import select as root4_select
from research.n83_root4_frontier_current_20261018 import test_select as root4_test
from research.n83_global_bridge_20261018 import arms, run


def files(root, first, second):
    paths = [root / name for name in ('extended_validated.json',
                                      'extended_selection.json',
                                      'root4_validated.json',
                                      'root4_selection.json')]
    first['run_id'], first['head_sha'] = 1, 'a' * 40
    second['run_id'], second['head_sha'] = 2, 'b' * 40
    for path, data in zip(paths, (first, extended_select.select(first),
                                  second, root4_select.select(second))):
        path.write_text(json.dumps(data))
    return paths


def main():
    with TemporaryDirectory() as temporary:
        root = Path(temporary)
        first, second = extended_test.fixture(), root4_test.fixture()
        chosen, choice = arms.resolve(*files(root, first, second))
        assert len(chosen) == 2
        assert choice['status'] == 'TWO_DISTINCT_ADMITTED'
        assert choice['ordered_candidate_ids'] == [arm[3] for arm in chosen]
        assert len(choice['candidate_manifest_sha256']) == 2
        for stratum in range(2):
            events = run.schedule(stratum, chosen)
            assert len(events) == 128
            assert sum(item['kind'] == 'ic' for item in events) == 16
            assert sum(item['kind'] == 'rho' for item in events) == 16
            assert sum(item['kind'] == 'calibration' for item in events) == 96

        unresolved = copy.deepcopy(second)
        unresolved['points'][0]['arms'][2]['status'] = 'TIMEOUT'
        chosen, choice = arms.resolve(*files(root, first, unresolved))
        assert not chosen and choice['status'] == 'UPSTREAM_UNRESOLVED'

        converged = copy.deepcopy(second)
        for point in converged['points']:
            point['arms'][1]['one_target_online_ms'] = 20
            point['arms'][1]['cold_process_ms'] = 80_000
            point['arms'][2]['one_target_online_ms'] = 50
            point['arms'][2]['cold_process_ms'] = 120_000
        chosen, choice = arms.resolve(*files(root, first, converged))
        assert not chosen and choice['status'] == 'CONVERGED_IDENTICAL'

        tampered = copy.deepcopy(second)
        tampered['source_sha256'] = '0' * 64
        try:
            arms.resolve(*files(root, first, tampered))
        except ValueError as error:
            assert 'upstream' in str(error)
        else:
            raise AssertionError('tampered upstream source was admitted')
    print(json.dumps({'status': 'PASS_GLOBAL_BRIDGE_ARMS', 'cases': 4},
                     sort_keys=True))


if __name__ == '__main__':
    main()
