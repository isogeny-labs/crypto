"""Exercise Pod identity, allocation binding, and overlap gates."""

import json
import tarfile
import tempfile
import unittest
from pathlib import Path

from research.n83_crossfamily_current_20261025 import runpod_provider
from research.n83_crossfamily_current_20261025 import runpod_runner
from research.n83_crossfamily_current_20261025 import aggregate_runpod
from research.n83_crossfamily_current_20261025 import verify as replay


HEAD = 'a' * 40
ARCHIVE = 'b' * 64
MANIFEST = 'c' * 64


class RunPodProviderTests(unittest.TestCase):
    def test_raw_archive_byte_binding_rejects_tamper(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            results = root / 'results'
            results.mkdir()
            (results / 'receipt.json').write_text('{"status":"PASS"}\n')
            archive = root / 'raw.tar.gz'
            with tarfile.open(archive, 'w:gz') as bundle:
                bundle.add(results, arcname='results')
            self.assertEqual(aggregate_runpod.bind_archive(
                archive, results)['file_count'], 1)
            (results / 'receipt.json').write_text('{"status":"ALTERED"}\n')
            with self.assertRaisesRegex(ValueError, 'differ'):
                aggregate_runpod.bind_archive(archive, results)

    def test_adapter_writes_honest_host_and_source_identity(self):
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / 'allocation.json'
            path.write_text('{}')
            receipt = dict(pod_id='pod0', machine_id='machine0',
                           image_name='ubuntu:24.04', _path=path,
                           source_archive_sha256=ARCHIVE,
                           source_manifest_sha256=MANIFEST)
            parent = runpod_runner.runner.parent
            original_host = parent.host_receipt
            original_write = parent.write_json
            written = {}
            try:
                parent.host_receipt = lambda _group: {'visible_affinity': [0, 1, 2, 3]}
                parent.write_json = lambda target, value: written.update(
                    {Path(target).name: value})
                runpod_runner.bind_provider(receipt, HEAD)
                host = parent.host_receipt(Path(temp))
                parent.write_json(Path(temp) / 'source.json', {'schema': 'source'})
                self.assertEqual(host['provider'], 'runpod')
                self.assertEqual(host['runpod_machine_id'], 'machine0')
                self.assertEqual(written['source.json']['runpod_adapter_sha256'],
                                 host['runpod_adapter_sha256'])
            finally:
                parent.host_receipt = original_host
                parent.write_json = original_write

    def test_provider_binding_and_distinct_machine_gate(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            snapshot_path = root / 'snapshot.json'
            pods = [dict(stratum=i, pod_id=f'pod{i}', machine_id=f'machine{i}',
                         desired_status='RUNNING', cost_per_hr=0.27,
                         image_name='ubuntu:24.04') for i in range(2)]
            snapshot = dict(schema='n83.crossfamily-current-runpod-snapshot/v1',
                            source_head_sha=HEAD,
                            source_archive_sha256=ARCHIVE,
                            source_manifest_sha256=MANIFEST,
                            recorded_utc='2026-10-18T22:00:00Z', pods=pods)
            snapshot_path.write_text(json.dumps(snapshot))
            adapter_hash = runpod_provider.digest(
                runpod_provider.HERE / 'runpod_runner.py')
            strata = []
            for i, pod in enumerate(pods):
                folder = root / f'stratum{i}'
                folder.mkdir()
                allocation = dict(schema='n83.crossfamily-current-runpod-allocation/v1',
                                  source_head_sha=HEAD,
                                  source_archive_sha256=ARCHIVE,
                                  source_manifest_sha256=MANIFEST,
                                  snapshot_sha256=runpod_provider.digest(snapshot_path),
                                  **pod)
                path = folder / 'runpod_allocation.json'
                path.write_text(json.dumps(allocation))
                host = dict(provider='runpod', git_sha=HEAD,
                            source_archive_sha256=ARCHIVE,
                            source_manifest_sha256=MANIFEST,
                            runpod_pod_id=pod['pod_id'],
                            runpod_machine_id=pod['machine_id'],
                            runpod_allocation_sha256=runpod_provider.digest(path),
                            runpod_adapter_sha256=adapter_hash,
                            parent_instance_id=
                            f"rp-{pod['pod_id']}-{pod['machine_id']}",
                            instance_id=f"rp-{pod['pod_id']}-{pod['machine_id']}"
                                        f'-crossfamily-current-stratum{i}',
                            recorded_utc=f'2026-10-18T22:0{i}:00Z')
                (folder / 'host.json').write_text(json.dumps(host))
                source = dict(
                    source_archive_sha256=ARCHIVE,
                    source_manifest_sha256=MANIFEST,
                    runpod_allocation_sha256=runpod_provider.digest(path),
                    runpod_adapter_sha256=adapter_hash)
                (folder / 'source.json').write_text(json.dumps(source))
                (folder / 'host_telemetry_after.json').write_text(json.dumps(
                    dict(recorded_utc='2026-10-18T22:04:00Z')))
                strata.append(folder)
                self.assertEqual(runpod_runner.read_provider(path, i)['pod_id'],
                                 pod['pod_id'])
                self.assertEqual(replay.provider_receipt(
                    folder, host, source, i)['status'],
                    'PASS_RUNPOD_ALLOCATION')
            proof = runpod_provider.verify(snapshot_path, strata, HEAD)
            self.assertEqual(proof['status'],
                             'PASS_TWO_DISTINCT_CONCURRENT_RUNPOD_MACHINES')
            snapshot['pods'][1]['machine_id'] = 'machine0'
            snapshot_path.write_text(json.dumps(snapshot))
            with self.assertRaisesRegex(ValueError, 'distinct'):
                runpod_provider.verify(snapshot_path, strata, HEAD)


if __name__ == '__main__':
    unittest.main()
