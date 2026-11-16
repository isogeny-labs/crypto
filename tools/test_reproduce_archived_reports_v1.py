"""Controls for the frozen-input and output-isolation boundary."""
import importlib.util
from pathlib import Path
import subprocess
import tempfile
import unittest

spec = importlib.util.spec_from_file_location('reproducer', Path(__file__).with_name('reproduce_archived_reports_v1.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class IsolationTests(unittest.TestCase):
    def test_fresh_external_path_allowed_but_existing_path_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            self.assertEqual(module.new_destination(root / 'new', root / 'repo'), root / 'new')
            with self.assertRaises(FileExistsError):
                module.new_destination(root, root / 'repo')

    def test_archive_destination_rejected_even_when_absent(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            with self.assertRaises(ValueError):
                module.new_destination(root / 'rounds' / 'new', root)

    def test_symlink_destination_and_ancestor_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            (root / 'alias').symlink_to(root, target_is_directory=True)
            for path in (root / 'alias', root / 'alias' / 'new'):
                with self.subTest(path=path), self.assertRaises(ValueError):
                    module.new_destination(path, root / 'repo')

    def test_committed_input_identity_and_wrong_pin(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp).resolve()
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            (root / 'input').write_bytes(b'frozen input\n')
            subprocess.run(['git', '-C', str(root), 'add', 'input'], check=True)
            subprocess.run(['git', '-C', str(root), '-c', 'user.name=Test', '-c',
                            'user.email=test@example.invalid', 'commit', '-qm', 'fixture'], check=True)
            (root / 'input').write_bytes(b'mutable checkout\n')
            self.assertEqual(module.source(root, 'HEAD', 'input', module.digest(b'frozen input\n')), b'frozen input\n')
            with self.assertRaises(ValueError):
                module.source(root, 'HEAD', 'input', '0' * 64)


if __name__ == '__main__':
    unittest.main()
