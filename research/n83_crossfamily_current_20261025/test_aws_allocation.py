"""Failure gates for the EC2 launch, termination and archive path."""

import io
import tarfile
import tempfile
import unittest
from pathlib import Path

from research.n83_crossfamily_current_20261025 import allocate_aws
from research.n83_crossfamily_current_20261025 import aws_deadline_guard
from research.n83_crossfamily_current_20261025 import collect_aws


class FakeEc2:
    def __init__(self, project='N83HighKCurrent'):
        self.project = project

    def describe_instances(self, **kwargs):
        return {'Reservations': [{'Instances': [{
            'InstanceId': 'i-example',
            'State': {'Name': 'running'},
            'Tags': [{'Key': 'Project', 'Value': self.project},
                     {'Key': 'RunId', 'Value': 'run1'},
                     {'Key': 'SourceHead', 'Value': 'a' * 40}],
        }]}]}


class AwsAllocationTest(unittest.TestCase):
    def test_user_data_has_both_guards_and_fits_ec2_limit(self):
        plan = {'account_id': '590183823895', 'region': 'us-east-1',
                'instance_type': 'g5g.4xlarge', 'image_id': 'ami-example',
                'source_head_sha': 'a' * 40,
                'source_archive_sha256': 'b' * 64,
                'source_manifest_sha256': 'c' * 64,
                'cap_seconds': 21600}
        zone = {'stratum': 0, 'zone': 'us-east-1d'}
        script = allocate_aws.user_data(
            plan, zone, {'source.tar.gz': 'https://example.com/source',
                         'manifest.json': 'https://example.com/manifest'},
            'https://example.com/results', 'https://example.com/status', 'run1')
        self.assertLess(len(script.encode()), 16384)
        self.assertIn('systemd-run --unit=n83-hard-stop', script)
        self.assertIn('aws_runner', script)
        self.assertIn('run1', script)

    def test_termination_rejects_unrelated_ec2_tag(self):
        receipt = {'instance_ids': ['i-example'], 'run_id': 'run1',
                   'source_head_sha': 'a' * 40}
        self.assertEqual(aws_deadline_guard.verified_live_ids(
            FakeEc2(), receipt), ['i-example'])
        with self.assertRaises(ValueError):
            aws_deadline_guard.verified_live_ids(
                FakeEc2(project='Other'), receipt)

    def test_archive_extraction_rejects_parent_path(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            archive = root / 'unsafe.tar.gz'
            with tarfile.open(archive, 'w:gz') as stream:
                payload = b'bad'
                member = tarfile.TarInfo('results/../../escape')
                member.size = len(payload)
                stream.addfile(member, io.BytesIO(payload))
            with self.assertRaises(ValueError):
                collect_aws.safe_extract(archive, root / 'extracted')
            self.assertFalse((root / 'escape').exists())


if __name__ == '__main__':
    unittest.main()
