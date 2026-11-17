"""AWS allocation and host identity gates for the frozen N83 high-K panel."""

import json
import tempfile
import unittest
from pathlib import Path

from research.n83_crossfamily_current_20261025 import aws_runner


class AwsRunnerTest(unittest.TestCase):
    def setUp(self):
        self.allocation = {
            'schema': aws_runner.SCHEMA, 'stratum': 0,
            'account_id': '590183823895', 'region': 'us-west-2',
            'instance_type': 'g5g.4xlarge', 'market': 'spot',
            'state': 'running', 'instance_id': 'i-0123456789abcdef0',
            'run_id': '20261026T190000Zdeadbeef',
            'availability_zone': 'us-west-2c', 'image_id': 'ami-0123456789abcdef0',
            'source_head_sha': 'a' * 40,
            'source_archive_sha256': 'b' * 64,
            'source_manifest_sha256': 'c' * 64,
        }
        self.identity = {
            'instance_life_cycle': 'spot', 'pkcs7': 'signed-data',
            'document': {'accountId': self.allocation['account_id'],
                         'instanceId': self.allocation['instance_id'],
                         'instanceType': self.allocation['instance_type'],
                         'availabilityZone': self.allocation['availability_zone'],
                         'region': self.allocation['region'],
                         'imageId': self.allocation['image_id']},
        }

    def test_exact_allocation_and_instance(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'allocation.json'
            path.write_text(json.dumps(self.allocation))
            self.assertEqual(aws_runner.read_allocation(path, 0), self.allocation)
            aws_runner.check_instance(self.allocation, self.identity)
            self.allocation['state'] = 'terminated'
            path.write_text(json.dumps(self.allocation))
            with self.assertRaises(ValueError):
                aws_runner.read_allocation(path, 0)

    def test_rejects_cross_instance_or_zone(self):
        for key, other in [('instanceId', 'i-other'),
                           ('availabilityZone', 'us-west-2a'),
                           ('imageId', 'ami-other')]:
            with self.subTest(key=key):
                identity = {**self.identity,
                            'document': {**self.identity['document'], key: other}}
                with self.assertRaises(ValueError):
                    aws_runner.check_instance(self.allocation, identity)


if __name__ == '__main__':
    unittest.main()
