import contextlib
import fcntl
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

import runner


class RunRecordTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.job = {'id': 'handoff', 'cwd': '.',
                    'argv': ['/bin/sh', '-c', 'printf %s "$DECLARED_SETTING"'],
                    'env': {'DECLARED_SETTING': 'explicit'},
                    'checkpoint_contract': 'atomic-files-and-sigterm-v1'}

    def saved_run(self, checkpoint=True):
        state = self.root / 'state/handoff'
        (state / 'checkpoint').mkdir(parents=True)
        runner.atomic_json(state / 'spec.json', {'sha256': runner.fingerprint(self.job), 'spec': self.job})
        if checkpoint:
            (state / 'checkpoint/progress').write_text('public fixture state')
        runner.publish_record(self.root, self.job, 'interrupted', required=True)
        return state / 'run.json'

    def cli(self, record, *extra):
        out = io.StringIO()
        with patch.object(sys, 'argv', ['runner.py', '--root', str(self.root), '--resume', str(record), *extra]), \
                contextlib.redirect_stdout(out):
            result = runner.main()
        return result, json.loads(out.getvalue())

    def test_record_is_private_and_contains_declared_settings_without_session_or_secrets(self):
        (self.root / 'jobs.json').write_text(json.dumps([self.job]))
        with patch.dict(os.environ, {'CLAUDE_SESSION_ID': 'old-session-not-required',
                                    'SYNTHETIC_INHERITED_SECRET': 'do-not-serialize'}):
            runner.run_job(self.root, self.job, threading.Event())
        path = self.root / 'state/handoff/run.json'
        encoded = path.read_text();record = json.loads(encoded)
        self.assertEqual(record['schema'], 'experiment.run/v1')
        self.assertEqual(record['status']['phase'], 'complete')
        self.assertEqual(record['resume']['argv'][0], 'experiment-runner')
        self.assertEqual(record['job']['env'], {'DECLARED_SETTING': 'explicit'})
        self.assertEqual(path.stat().st_mode & 0o777, 0o600)
        self.assertEqual((self.root / 'state/handoff/stdout.log').read_text(), 'explicit')
        self.assertNotIn('old-session-not-required', encoded)
        self.assertNotIn('do-not-serialize', encoded)
        self.assertTrue(all(not Path(p).is_absolute() for p in record['paths'].values()))

    def test_resume_reconstructs_admission_and_preserves_other_jobs_without_launching_from_client(self):
        record = self.saved_run()
        other = dict(self.job, id='other')
        (self.root / 'jobs.json').write_text(json.dumps([other]))
        with patch.object(runner.subprocess, 'Popen') as spawn:
            code, response = self.cli(record)
        self.assertEqual((code, response['state']), (0, 'submitted'))
        self.assertEqual(runner.read_jobs(self.root), [other, self.job])
        request = json.loads((self.root / 'state/handoff/resume-request.json').read_text())
        self.assertEqual(request['spec_sha256'], runner.fingerprint(self.job))
        spawn.assert_not_called()

    def test_export_discovers_saved_runs_even_without_original_manifest_or_record(self):
        path = self.saved_run()
        path.unlink()
        with patch.object(runner.subprocess, 'Popen') as spawn:
            index = runner.list_runs(self.root, export=True)
        self.assertEqual(index['schema'], 'experiment.run-index/v1')
        self.assertEqual(index['runs'][0]['job'], self.job)
        self.assertEqual(json.loads(path.read_text())['schema'], 'experiment.run/v1')
        self.assertFalse((self.root / 'jobs.json').exists())
        self.assertFalse((self.root / 'state/handoff/attempt.json').exists())
        spawn.assert_not_called()

    def test_tampered_specification_or_paths_are_refused_before_admission(self):
        record = self.saved_run()
        original = json.loads(record.read_text())
        for field in ['job', 'paths']:
            changed = json.loads(json.dumps(original))
            if field == 'job':
                changed['job']['argv'] = ['/bin/false']
                changed['spec_sha256'] = runner.fingerprint(changed['job'])
            else:
                changed['paths']['checkpoint'] = '../different-state'
            runner.atomic_json(record, changed)
            code, response = self.cli(record)
            self.assertEqual((code, response['state']), (1, 'refused'))
            self.assertFalse((self.root / 'jobs.json').exists())

    def test_empty_resume_remains_blocked_when_background_supervisor_admits_it(self):
        record = self.saved_run(checkpoint=False)
        code, response = self.cli(record)
        self.assertEqual((code, response['state']), (1, 'refused'))
        with patch.object(runner.subprocess, 'Popen') as spawn, self.assertRaises(RuntimeError):
            runner.run_job(self.root, self.job, threading.Event())
        spawn.assert_not_called()
        self.assertFalse((self.root / 'state/handoff/complete.json').exists())
        self.assertEqual(json.loads(record.read_text())['status']['phase'], 'blocked')

    def test_completed_record_does_not_rerun_or_recreate_missing_manifest(self):
        record = self.saved_run()
        runner.atomic_json(self.root / 'state/handoff/complete.json',
                           {'sha256': runner.fingerprint(self.job), 'completed': 1})
        with patch.object(runner.subprocess, 'Popen') as spawn:
            code, response = self.cli(record, '--once', '--no-imds')
        self.assertEqual((code, response['state']), (0, 'complete'))
        self.assertFalse((self.root / 'jobs.json').exists())
        spawn.assert_not_called()

    def test_foreground_resume_submits_when_another_supervisor_owns_workspace(self):
        record = self.saved_run()
        with (self.root / '.supervisor.lock').open('a+') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            result = subprocess.run([sys.executable, str(Path(runner.__file__)), '--root', str(self.root),
                                     '--resume', str(record), '--once', '--no-imds'],
                                    capture_output=True, text=True, timeout=8)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(json.loads(result.stdout)['state'], 'submitted')
        self.assertFalse((self.root / 'state/handoff/stdout.log').exists())

    def test_ssh_caller_recovers_worker_settings_and_uses_admission_before_queueing(self):
        record = self.saved_run()
        runner.atomic_json(self.root / 'runner-config.json', {'schema': 'experiment.worker-config/v1',
                           'activity_table': 'synthetic-table', 'region': 'us-east-2'})
        def admitted(root):
            self.assertEqual(root, self.root)
            self.assertEqual(os.environ['SPOT_ACTIVITY_TABLE'], 'synthetic-table')
            self.assertEqual(os.environ['AWS_REGION'], 'us-east-2')
            self.assertFalse((root / 'jobs.json').exists())
        with patch.dict(os.environ, {}, clear=True), patch('worker_state.admit', side_effect=admitted) as admit:
            code, response = self.cli(record)
        self.assertEqual((code, response['state']), (0, 'submitted'))
        admit.assert_called_once()

    def test_record_submission_wakes_managed_worker_without_inheriting_original_session(self):
        record = self.saved_run()
        runner.atomic_json(self.root / 'runner-config.json', {'schema':'experiment.worker-config/v1',
            'activity_table':'synthetic-table', 'region':'us-east-2', 'lifecycle_function':'arn:synthetic:function'})
        def wake(config, action, **payload):
            self.assertEqual(config, {'function':'arn:synthetic:function','region':'us-east-2'})
            self.assertEqual((action,payload), ('wake',{'managed_work':True}))
            self.assertEqual(runner.read_jobs(self.root), [self.job])
            self.assertTrue((self.root/'state/handoff/resume-request.json').exists())
            return {'state':'starting'}
        with patch.dict(os.environ, {}, clear=True), patch('wake.invoke',side_effect=wake) as invoke, \
                patch('worker_state.admit') as admit:
            code,response=self.cli(record)
        self.assertEqual((code,response['state']),(0,'submitted'))
        invoke.assert_called_once();admit.assert_not_called()


if __name__ == '__main__':
    unittest.main()
