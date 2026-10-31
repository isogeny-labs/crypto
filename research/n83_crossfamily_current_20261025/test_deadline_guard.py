"""Exercise deadline deletion without creating provider Pods or waiting."""

from datetime import datetime, timezone
import json
from pathlib import Path
import tempfile
import unittest

from research.n83_crossfamily_current_20261025 import deadline_guard
from research.n83_crossfamily_current_20261025 import pod_deadline_guard


NOW = datetime(2026, 10, 25, 3, 0, tzinfo=timezone.utc)
HEAD = 'a' * 40
PLAN = {
    'schema': 'n83.crossfamily-current-allocation-plan/v1',
    'source_head_sha': HEAD, 'started_utc': '2026-10-25T02:00:00Z',
    'terminate_at_utc': '2026-10-25T03:00:00Z',
    'source_archive_sha256': 'b' * 64,
    'source_manifest_sha256': 'c' * 64,
    'names': [f'n83-crossfamily-{HEAD[:12]}-s{i}' for i in range(2)],
}


class DeadlineTests(unittest.TestCase):
    def test_local_guard_deletes_only_this_allocation_and_confirms_absence(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            (folder / 'allocation_plan.json').write_text(json.dumps(PLAN))
            live = [dict(id=f'pod{i}', name=PLAN['names'][i],
                         createdAt='2026-10-25T02:01:00Z') for i in range(2)]
            live.append(dict(id='unrelated', name='other',
                             createdAt='2026-10-25T02:01:00Z'))
            deleted = []

            def terminate(pod_id, key):
                deleted.append(pod_id)
                live[:] = [pod for pod in live if pod['id'] != pod_id]
                return True

            deadline_guard.guard(folder, 'fixture-key', pods=lambda _: live,
                                 terminate=terminate, clock=lambda: NOW,
                                 pause=lambda _: None)
            self.assertEqual(set(deleted), {'pod0', 'pod1'})
            self.assertEqual([pod['id'] for pod in live], ['unrelated'])
            self.assertEqual(json.loads((folder / 'deadline_guard.result.json').read_text())[
                'status'], 'MATCHING_PODS_ABSENT')

    def test_pod_side_expiry_uses_bound_pod_and_retries(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            plan = folder / 'allocation_plan.json'
            allocation = folder / 'allocation_s1.json'
            receipt = folder / 'pod_guard.json'
            plan.write_text(json.dumps(PLAN))
            allocation.write_text(json.dumps({
                'schema': 'n83.crossfamily-current-runpod-allocation/v1',
                'source_head_sha': HEAD, 'source_archive_sha256': 'b' * 64,
                'source_manifest_sha256': 'c' * 64,
                'stratum': 1, 'pod_id': 'pod1'}))
            calls = []

            def remove(pod_id):
                calls.append(pod_id)
                if len(calls) == 1:
                    raise ConnectionError('provider temporarily unavailable')

            pod_deadline_guard.guard(allocation, plan, receipt,
                                     clock=lambda: NOW, pause=lambda _: None,
                                     remove=remove)
            self.assertEqual(calls, ['pod1', 'pod1'])
            self.assertEqual(json.loads(receipt.read_text())['status'],
                             'TERMINATION_REQUESTED')


if __name__ == '__main__':
    unittest.main()
