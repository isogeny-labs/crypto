"""Check provider SSH endpoint selection before any paid dispatch."""

import unittest
import json
from datetime import datetime, timedelta, timezone
from pathlib import Path
import tempfile
from unittest.mock import patch

from research.n83_crossfamily_current_20261025 import dispatch_runpod


ALLOCATIONS = [
    {'stratum': i, 'pod_id': f'pod{i}', 'machine_id': f'machine{i}'}
    for i in range(2)
]


def pods():
    return [
        {'id': f'pod{i}', 'machineId': f'machine{i}',
         'desiredStatus': 'RUNNING',
         'runtime': {'ports': [
             {'ip': f'198.51.100.{i + 10}', 'isIpPublic': True,
              'privatePort': 22, 'publicPort': 21000 + i, 'type': 'tcp'},
             {'ip': f'198.51.100.{i + 10}', 'isIpPublic': True,
              'privatePort': 8888, 'publicPort': 22000 + i, 'type': 'http'},
         ]}}
        for i in range(2)
    ]


class EndpointTests(unittest.TestCase):
    def test_exact_two_public_ssh_ports(self):
        self.assertEqual(dispatch_runpod.endpoints(pods(), ALLOCATIONS),
                         {0: ('198.51.100.10', 21000),
                          1: ('198.51.100.11', 21001)})

    def test_identity_change_fails_closed(self):
        changed = pods()
        changed[1]['machineId'] = 'replacement'
        with self.assertRaisesRegex(ValueError, 'provider Pod state'):
            dispatch_runpod.endpoints(changed, ALLOCATIONS)

    def test_private_or_multiple_ssh_ports_fail_closed(self):
        changed = pods()
        changed[0]['runtime']['ports'][0]['isIpPublic'] = False
        with self.assertRaisesRegex(ValueError, 'public provider SSH'):
            dispatch_runpod.endpoints(changed, ALLOCATIONS)

    def test_watch_records_complete_custody_and_failed_stage_separately(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            plan = {'schema': 'n83.crossfamily-current-allocation-plan/v1',
                    'terminate_at_utc': (datetime.now(timezone.utc) +
                                         timedelta(minutes=5)).isoformat()}
            (root / 'allocation_plan.json').write_text(json.dumps(plan))
            for status, expected in (
                    ('RECOVERED', 'PASS_ARCHIVES_AND_POD_ABSENCE'),
                    ('FAILED_BEFORE_RAW_ARCHIVE',
                     'PODS_ABSENT_WITH_FAILED_STAGE')):
                state = {'strata': [{'stratum': i, 'status': status}
                                    for i in range(2)],
                         'terminated_and_absent': {0: True, 1: True}}
                with patch.object(dispatch_runpod, 'collect', return_value=state):
                    dispatch_runpod.watch(root, root, root, 'fake', 0)
                self.assertEqual(json.loads((root / 'watch.json').read_text())
                                 ['status'], expected)
        changed = pods()
        changed[0]['runtime']['ports'].append(
            {'ip': '198.51.100.12', 'isIpPublic': True,
             'privatePort': 22, 'publicPort': 21012, 'type': 'tcp'})
        with self.assertRaisesRegex(ValueError, 'public provider SSH'):
            dispatch_runpod.endpoints(changed, ALLOCATIONS)


if __name__ == '__main__':
    unittest.main()
