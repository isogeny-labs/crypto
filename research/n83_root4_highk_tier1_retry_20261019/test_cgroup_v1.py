"""Cgroup v1 normalization and raw-to-gate receipt checks."""

import json
import tempfile
import unittest
from pathlib import Path

from research.n83_root4_highk_tier1_retry_20261019.runpod_runner import read_provider, v1_snapshot
from research.n83_root4_highk_tier1_retry_20261019.verify import v1_snapshot_consistent


class V1TelemetryTests(unittest.TestCase):
    def test_zero_denials_and_throttling_have_auditable_raw_receipts(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            cpu, memory = root / 'cpu', root / 'memory'
            cpu.mkdir()
            memory.mkdir()
            (cpu / 'cpu.stat').write_text(
                'nr_periods 14\nnr_throttled 0\nthrottled_time 0\n')
            (cpu / 'cpu.cfs_quota_us').write_text('400000\n')
            (cpu / 'cpu.cfs_period_us').write_text('100000\n')
            for name, value in (
                ('memory.usage_in_bytes', '1000'),
                ('memory.max_usage_in_bytes', '2000'),
                ('memory.limit_in_bytes', '64000000000'),
                ('memory.failcnt', '0'),
                ('memory.oom_control',
                 'oom_kill_disable 0\nunder_oom 0\noom_kill 0'),
            ):
                (memory / name).write_text(value + '\n')
            snapshot = v1_snapshot(cpu, memory)
            self.assertTrue(v1_snapshot_consistent(snapshot))
            self.assertEqual(snapshot['cpu.max'], '400000 100000\n')
            reduced = dict(snapshot, **{'cpu.max': '300000 100000\n'})
            reduced['cgroup_v1_raw'] = dict(snapshot['cgroup_v1_raw'],
                                            **{'cpu.cfs_quota_us': '300000'})
            self.assertFalse(v1_snapshot_consistent(reduced))
            self.assertEqual(snapshot['memory.events'], 'oom 0\noom_kill 0\n')
            snapshot['memory.events'] = 'oom 0\noom_kill 1\n'
            self.assertFalse(v1_snapshot_consistent(snapshot))
            (memory / 'memory.oom_control').write_text('under_oom 0\n')
            with self.assertRaises(ValueError):
                v1_snapshot(cpu, memory)


    def test_provider_receipt_requires_frozen_four_cpu_mask_and_time_cap(self):
        receipt = {
            'schema': 'n83.highk-retry-runpod-allocation/v1',
            'stratum': 0, 'pod_id': 'abc123', 'machine_id': 'machine1',
            'desired_status': 'RUNNING', 'cost_per_hr': 0.19,
            'cloud_type': 'COMMUNITY', 'compute_type': 'GPU',
            'gpu_type_id': 'Tesla V100-PCIE-16GB', 'gpu_count': 1,
            'vcpu_count': 6, 'memory_in_gb': 47,
            'cpu_affinity_ids': [0, 1, 2, 3],
            'snapshot_sha256': '0' * 64,
            'created_utc': '2026-10-19T17:00:00Z',
            'stop_after_utc': '2026-10-20T02:00:00Z',
        }
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'allocation.json'
            path.write_text(json.dumps(receipt))
            self.assertEqual(read_provider(path, 0)['cpu_affinity_ids'],
                             [0, 1, 2, 3])
            receipt['cpu_affinity_ids'] = [0, 0, 1, 2]
            path.write_text(json.dumps(receipt))
            with self.assertRaises(ValueError):
                read_provider(path, 0)
            receipt['cpu_affinity_ids'] = [0, 1, 2, 3]
            receipt['stop_after_utc'] = '2026-10-20T02:00:01Z'
            path.write_text(json.dumps(receipt))
            with self.assertRaises(ValueError):
                read_provider(path, 0)


if __name__ == '__main__':
    unittest.main()
