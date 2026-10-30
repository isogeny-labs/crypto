"""Independent two-instance and API-tag admission controls."""

import json
import tempfile
import unittest
from pathlib import Path

from research.n83_crossfamily_current_20261025 import aws_provider


class AwsProviderTest(unittest.TestCase):
    def test_distinct_overlapping_hosts_and_tag_mismatch(self):
        head = 'a' * 40
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            controller = {'schema': 'n83.crossfamily-aws-controller/v1',
                          'account_id': '590183823895', 'region': 'us-east-1',
                          'source_head_sha': head, 'run_id': 'run1',
                          'instance_ids': ['i-first', 'i-second'],
                          'strata': []}
            strata, instances = [], []
            for index, zone in enumerate(('us-east-1d', 'us-east-1d')):
                instance_id = controller['instance_ids'][index]
                results = root / str(index)
                usage = results / 'points' / f'p{index:02d}' / 'runs' / 'arm'
                usage.mkdir(parents=True)
                (usage / 'usage.json').write_text(json.dumps({
                    'finished_utc': '2026-10-26T19:10:00+00:00'}))
                (results / 'aws_allocation.json').write_text(json.dumps({
                    'stratum': index, 'instance_id': instance_id,
                    'run_id': 'run1', 'availability_zone': zone,
                    'image_id': 'ami-example'}))
                controller['strata'].append({
                    'stratum': index, 'instance_id': instance_id,
                    'zone': zone, 'subnet_id': f'subnet-{index}'})
                strata.append({'host': {
                    'parent_instance_id': instance_id,
                    'provider': 'aws_ec2_spot', 'git_sha': head,
                    'recorded_utc': '2026-10-26T19:01:00+00:00'},
                    'results': results, 'analysis': {
                        'provider_receipt': {'status': 'PASS_AWS_ALLOCATION'}}})
                instances.append({
                    'account_id': controller['account_id'],
                    'instance_id': instance_id,
                    'instance_type': 'g5g.4xlarge',
                    'instance_lifecycle': 'spot', 'image_id': 'ami-example',
                    'availability_zone': zone, 'subnet_id': f'subnet-{index}',
                    'launch_time': '2026-10-26T19:00:00+00:00',
                    'tags': {'Project': 'N83HighKCurrent',
                             'RunId': 'run1', 'Stratum': str(index),
                             'SourceHead': head}})
            snapshot = {'schema': 'n83.crossfamily-aws-provider-snapshot/v1',
                        'snapshot_source': 'ec2_describe_instances_at_launch',
                        'controller': controller, 'instances': instances,
                        'captured_utc': '2026-10-26T19:11:00+00:00'}
            self.assertEqual(aws_provider.verify(snapshot, strata, head)[
                'status'], 'PASS_TWO_DISTINCT_CONCURRENT_AWS_SPOT_INSTANCES')
            instances[1]['instance_id'] = 'i-first'
            with self.assertRaises(ValueError):
                aws_provider.verify(snapshot, strata, head)
            instances[1]['instance_id'] = 'i-second'
            instances[1]['tags']['RunId'] = 'other'
            with self.assertRaises(ValueError):
                aws_provider.verify(snapshot, strata, head)


if __name__ == '__main__':
    unittest.main()
