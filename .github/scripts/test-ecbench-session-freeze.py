"""Exercise the actual workflow freeze command against temporary Git histories."""
from pathlib import Path
import os
import subprocess
import tempfile
import unittest

import yaml


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = yaml.safe_load((ROOT / '.github/workflows/ecbench.yml').read_text())
SCRIPTS = [step.get('run', '') for job in WORKFLOW['jobs'].values()
           for step in job.get('steps', [])]
SCRIPT, = [script for script in SCRIPTS
           if 'these committed session files were modified' in script]
SESSION = 'research/ecbench_fixture/sessions/run1'


class SessionFreeze(unittest.TestCase):
    def check_change(self, path, delete=False, new=False):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)

            def git(*args):
                return subprocess.check_output(
                    ['git', '-c', 'user.name=Fixture', '-c',
                     'user.email=fixture@example.invalid', *args], cwd=root,
                    stderr=subprocess.DEVNULL, text=True).strip()

            git('init', '-q')
            git('remote', 'add', 'origin', '.')
            for name in ['session.json', 'records.jsonl', 'spec.json',
                         'host.json', '.artifacts.json', '.gitignore']:
                target = root / SESSION / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text('original\n')
            git('add', '.')
            git('commit', '-qm', 'original capsules')
            base = git('rev-parse', 'HEAD')
            target = root / path
            if delete:
                target.unlink()
            else:
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text('new\n' if new else 'changed\n')
            git('add', '-A')
            git('commit', '-qm', 'candidate change')
            runtime = root / 'runtime'
            runtime.mkdir()
            (runtime / 'ecbench-sessions.txt').write_text(SESSION + '\n')
            result = subprocess.run(
                ['bash', '-e', '-o', 'pipefail', '-c', SCRIPT], cwd=root,
                env={**os.environ, 'BASE': base,
                     'RUNNER_TEMP': str(runtime)}, capture_output=True, text=True)
            return result

    def test_generated_ignore_metadata_can_change(self):
        result = self.check_change(SESSION + '/.gitignore')
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_existing_capsules_records_and_pointer_changes_stay_blocked(self):
        for name in ['session.json', 'records.jsonl', 'spec.json',
                     'host.json', '.artifacts.json']:
            with self.subTest(name=name):
                result = self.check_change(SESSION + '/' + name)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn(SESSION + '/' + name, result.stderr)

    def test_existing_record_deletion_stays_blocked(self):
        result = self.check_change(SESSION + '/records.jsonl', delete=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(SESSION + '/records.jsonl', result.stderr)

    def test_new_session_is_allowed(self):
        result = self.check_change('research/ecbench_fixture/sessions/run2/session.json', new=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()
