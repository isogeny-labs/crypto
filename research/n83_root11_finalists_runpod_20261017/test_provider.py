#!/usr/bin/env python3
"""Fail-closed RunPod allocation binding checks."""

import copy
import json
import tempfile
import unittest
from pathlib import Path

import provider

HEAD = 'a' * 40
START = '2026-10-17T21:00:00+00:00'
END = '2026-10-17T21:10:00+00:00'


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, sort_keys=True) + '\n')


class ProviderBindingTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.start_path, self.end_path = root / 'start.json', root / 'end.json'
        ids = [f'pod{number:011d}' for number in range(4)]
        pods = [{'id': pod_id, 'name': f'n83-{number}',
                 'desiredStatus': 'RUNNING', 'costPerHr': 0.22,
                 'gpuCount': 0, 'memoryInGb': 32, 'vcpuCount': 4,
                 'imageName': 'runpod/base:1.0.2-ubuntu2404',
                 'machineId': f'machine-{number}'}
                for number, pod_id in enumerate(ids)]
        self.start = {'schema': provider.SCHEMA, 'observed_utc': START,
                      'pod_ids': ids, 'pods': pods}
        self.end = copy.deepcopy(self.start)
        self.end['observed_utc'] = END
        self.strata = []
        for number, pod_id in enumerate(ids):
            results = root / f'stratum{number}' / 'results'
            write(results / 'host.json', {'recorded_utc': '2026-10-17T21:01:00+00:00'})
            write(results / 'points/p00/runs/cal_before/launch.json',
                  {'started_utc': f'2026-10-17T21:0{number + 2}:00+00:00'})
            write(results / 'points/p00/runs/cal_before/usage.json',
                  {'finished_utc': '2026-10-17T21:08:00+00:00'})
            self.strata.append({
                'stratum': number, 'results': str(results),
                'host': {'provider': 'runpod', 'recorded_utc':
                         '2026-10-17T21:01:00+00:00',
                         'runpod_pod_id': pod_id,
                         'parent_instance_id': f'rp-{pod_id}',
                         'instance_id': f'rp-{pod_id}-finalists-stratum{number}',
                         'runner_name': pod_id,
                         'run_url': f'https://console.runpod.io/pods/{pod_id}',
                         'git_sha': HEAD}})
        self.save()

    def save(self):
        write(self.start_path, self.start)
        write(self.end_path, self.end)

    def check(self):
        return provider.verify(self.start_path, self.end_path, self.strata, HEAD)

    def test_distinct_concurrent_pods_pass(self):
        self.assertEqual(self.check()['status'],
                         'PASS_FOUR_DISTINCT_CONCURRENT_PODS')

    def test_changed_machine_fails(self):
        self.end['pods'][2]['machineId'] = 'replacement'
        self.save()
        with self.assertRaisesRegex(ValueError, 'identity or size'):
            self.check()

    def test_price_above_cap_fails(self):
        self.start['pods'][1]['costPerHr'] = 0.23
        self.end['pods'][1]['costPerHr'] = 0.23
        self.save()
        with self.assertRaisesRegex(ValueError, 'identity or size'):
            self.check()

    def test_nonoverlapping_producers_fail(self):
        path = Path(self.strata[0]['results']) / 'points/p00/runs/cal_before/usage.json'
        write(path, {'finished_utc': '2026-10-17T21:03:00+00:00'})
        with self.assertRaisesRegex(ValueError, 'did not overlap'):
            self.check()


if __name__ == '__main__':
    unittest.main()
