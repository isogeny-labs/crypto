import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import criu_export


class ExportTests(unittest.TestCase):
    def test_encryption_size_metadata_and_download_must_all_match(self):
        with tempfile.TemporaryDirectory() as tmp:
            payload = Path(tmp)/'public-fixture'
            payload.write_bytes(b'public fixture, no process memory')
            sha = criu_export.digest(payload)
            valid = {'ContentLength':payload.stat().st_size,
                     'ServerSideEncryption':'AES256', 'Metadata':{'sha256':sha}}
            cases = [valid, dict(valid, ServerSideEncryption=''),
                     dict(valid, ContentLength=0), dict(valid, Metadata={})]
            for head in cases:
                with patch.object(criu_export, 'aws', side_effect=['',json.dumps(head)]), \
                        patch.object(criu_export, 'downloaded_digest', return_value=sha):
                    if head == valid:
                        self.assertTrue(criu_export.upload_verified(payload, 'fixture')['download_sha256_verified'])
                    else:
                        with self.assertRaises(ValueError):
                            criu_export.upload_verified(payload, 'fixture')
            with patch.object(criu_export, 'aws', side_effect=['',json.dumps(valid)]), \
                    patch.object(criu_export, 'downloaded_digest', return_value='corrupted'):
                with self.assertRaises(ValueError):
                    criu_export.upload_verified(payload, 'fixture')

    def test_unapproved_registry_cannot_export(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'root'
            staging = Path(tmp)/'staging'
            root.mkdir()
            staging.mkdir()
            (root/'registry.json').write_text(json.dumps({'jobs':[{'pid':123,'start_ticks':'changed'}]}))
            with patch.object(criu_export, 'aws') as aws:
                with self.assertRaisesRegex(ValueError, 'authorization'):
                    criu_export.export(root, staging, [{'pid':123,'start_ticks':'original'}])
                aws.assert_not_called()
                self.assertFalse((root/'last-export.json').exists())

    def test_missing_checkpoint_never_publishes_committed_index(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'root'
            staging = Path(tmp)/'staging'
            root.mkdir()
            staging.mkdir()
            jobs = [{'pid':123}]
            (root/'registry.json').write_text(json.dumps({'jobs':jobs}))
            with patch.object(criu_export, 'discover', return_value={'selected':None}), \
                    patch.object(criu_export, 'aws') as aws:
                with self.assertRaisesRegex(ValueError, 'No valid checkpoint'):
                    criu_export.export(root, staging, jobs)
                aws.assert_not_called()
                self.assertFalse((root/'last-export.json').exists())

    def test_one_time_grant_cannot_export_a_new_generation(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'root'
            staging = Path(tmp)/'staging'
            root.mkdir()
            staging.mkdir()
            jobs = [{'pid':123}]
            (root/'registry.json').write_text(json.dumps({'jobs':jobs}))
            with patch.object(criu_export, 'discover', return_value={'selected':str(root/'123/gen-new')}), \
                    patch.object(criu_export, 'aws') as aws:
                with self.assertRaisesRegex(ValueError, 'recurring memory-export authorization'):
                    criu_export.export(root, staging, jobs, {'123':'gen-authorized'})
                aws.assert_not_called()


if __name__ == '__main__':
    unittest.main()
