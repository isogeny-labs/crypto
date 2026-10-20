import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import criu_local


class LocalCriuTests(unittest.TestCase):
    def generation(self, root, job):
        (root/'images').mkdir(parents=True)
        payloads = {'state':b'captured','images/inventory.img':b'public inventory',
                    'images/pages-1.img':b'public pages','backing-directories.json':b'{}'}
        for name, data in payloads.items():
            (root/name).write_bytes(data)
        manifest = {'schema':'spot.criu-local/v1','committed':True,'job':job,
                    'files':[{'path':name,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()}
                             for name,data in payloads.items()]}
        criu_local.atomic_json(root/'manifest.json', manifest)
        return manifest

    def test_discovery_falls_back_and_binds_the_registered_identity(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            job = {'pid':123,'start_ticks':'17','command_sha256':'public'}
            (root/'registry.json').write_text(json.dumps({'jobs':[job]}))
            old = root/'123/gen-1'
            newest = root/'123/gen-2'
            self.generation(old, job)
            self.generation(newest, job)
            (newest/'state').write_bytes(b'tampered')
            report = criu_local.discover(root, 123)
            self.assertEqual(report['selected'], str(old))
            self.assertEqual(len(report['rejected']), 1)
            wrong = dict(job, start_ticks='18')
            (root/'registry.json').write_text(json.dumps({'jobs':[wrong]}))
            self.assertIsNone(criu_local.discover(root, 123)['selected'])

    def test_captured_working_directory_keeps_its_restore_permissions(self):
        with tempfile.TemporaryDirectory() as tmp:
            source = Path(tmp)/'working-directory'
            source.mkdir(mode=0o755)
            (source/'output').write_bytes(b'public fixture')
            generation = Path(tmp)/'generation'
            generation.mkdir()
            (generation/'backing-paths.json').write_text(json.dumps([str(source/'output')]))
            with patch.dict(criu_local.os.environ, {'CRTOOLS_SCRIPT_ACTION':'post-dump'}), patch.object(criu_local.os, 'chown'):
                criu_local.capture(generation)
            copied = generation/'filesystem'/source.relative_to('/')
            self.assertEqual(copied.stat().st_mode & 0o777, 0o755)
            self.assertEqual((copied/'output').read_bytes(), b'public fixture')
            metadata = json.loads((generation/'backing-directories.json').read_text())
            self.assertEqual(metadata[str(source)]['mode'], 0o755)

    def test_integrity_and_path_checks_precede_restore_or_export(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            manifest = self.generation(root, {'pid':123})
            self.assertEqual(criu_local.validate_generation(root)['job']['pid'], 123)
            (root/'state').write_bytes(b'tampered')
            with self.assertRaises(ValueError):
                criu_local.validate_generation(root)
            manifest['files'][0]['path'] = '../outside'
            criu_local.atomic_json(root/'manifest.json', manifest)
            with self.assertRaises(ValueError):
                criu_local.validate_generation(root)

    def test_symlink_components_cannot_supply_checkpoint_bytes(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)/'generation'
            root.mkdir()
            outside = Path(tmp)/'outside'
            outside.mkdir()
            (outside/'state').write_bytes(b'captured')
            (root/'images').symlink_to(outside, target_is_directory=True)
            criu_local.atomic_json(root/'manifest.json', {'schema':'spot.criu-local/v1','committed':True,
                'files':[{'path':'images/inventory.img','bytes':8,'sha256':'unused'},
                         {'path':'images/pages-1.img','bytes':8,'sha256':'unused'},
                         {'path':'backing-directories.json','bytes':2,'sha256':'unused'},
                         {'path':'images/state','bytes':8,'sha256':hashlib.sha256(b'captured').hexdigest()}]})
            with self.assertRaises(ValueError):
                criu_local.validate_generation(root)

    def test_reused_or_changed_process_is_never_dumped(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root/'registry.json').write_text(json.dumps({'jobs':[{'pid':123}]}))
            with patch.object(criu_local, 'same_process', return_value=False), patch.object(criu_local.subprocess, 'run') as run:
                criu_local.snapshot(root, Path('/not-executed'))
                run.assert_not_called()

    def test_atomic_metadata_is_private_and_complete(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp)/'registry.json'
            criu_local.atomic_json(path, {'one':1})
            self.assertEqual(path.stat().st_mode & 0o777, 0o600)
            self.assertEqual(json.loads(path.read_text()), {'one':1})
            self.assertFalse(path.with_name(path.name+'.tmp').exists())


if __name__ == '__main__':
    unittest.main()
