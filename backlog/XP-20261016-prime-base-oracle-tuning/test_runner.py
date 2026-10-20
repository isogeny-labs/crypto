#!/usr/bin/env python3
"""Runner control tests with a fake ecbench; no measurements are executed."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

CARD = Path(__file__).resolve().parent
MOCK = r'''#!/usr/bin/env python3
import json, os, pathlib, sys
args = sys.argv[1:]
with open(os.environ['MOCK_CALLS'], 'a') as f:
    f.write(json.dumps(args) + '\n')
def option(name):
    return args[args.index(name) + 1]
if args[0] == 'verify':
    pathlib.Path(option('--out')).write_text('{"ok":false}\n')
    sys.exit(1)
if args[0] == 'plan':
    print('mock plan only')
elif args[0] == 'run':
    d = pathlib.Path(option('--out'))
    d.mkdir(parents=True)
    (d / 'session.json').write_text('{"status":"complete"}\n')
elif args[0] == 'band':
    rows = json.loads(os.environ['MOCK_ROWS'])
    for word in args[2:args.index('--json')]:
        d = pathlib.Path(word)
        if d.name.startswith('train-ext-') and json.loads((d / 'session.json').read_text())['status'] == 'complete':
            n = int(d.name.rsplit('-n', 1)[1])
            rows.append(dict(arm=f'mitm2-n{n}', runs=8, verified=8, uncounted=[], mean_s=[2,2,2]))
    pathlib.Path(option('--json')).write_text(json.dumps({'rows': rows}) + '\n')
    print('mock band only')
else:
    raise SystemExit('unexpected mock command: ' + repr(args))
'''


def row(n, score, runs=8):
    return dict(arm=f'mitm2-n{n}', runs=runs, verified=runs,
                uncounted=[], mean_s=[score] * 3)


class RunnerControls(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.card = self.root / 'backlog' / CARD.name
        self.card.mkdir(parents=True)
        shutil.copy2(CARD / 'run.sh', self.card / 'run.sh')
        shutil.copytree(CARD / 'specs', self.card / 'specs')
        self.out = self.root / 'out'
        self.calls = self.root / 'calls.jsonl'
        bin_dir = self.root / 'bin'
        bin_dir.mkdir()
        ecbench = bin_dir / 'ecbench'
        ecbench.write_text(MOCK)
        ecbench.chmod(0o755)
        taskset = bin_dir / 'taskset'
        taskset.write_text('#!/bin/sh\nshift 2\nexec "$@"\n')
        taskset.chmod(0o755)
        self.env = dict(os.environ, ECBENCH=str(ecbench), PBO_SIZES='20',
                        MOCK_CALLS=str(self.calls), MOCK_ROWS=json.dumps([row(32, 1)]),
                        PATH=str(bin_dir) + os.pathsep + os.environ['PATH'])

    def session(self, name, status='complete'):
        d = self.out / 'sessions' / name
        d.mkdir(parents=True)
        (d / 'session.json').write_text(json.dumps({'status': status}))
        return d

    def run_stage(self, stage, ok=True):
        result = subprocess.run(['sh', str(self.card / 'run.sh'), stage, str(self.out)],
                                cwd=self.root, env=self.env, capture_output=True, text=True)
        if ok:
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        else:
            self.assertNotEqual(result.returncode, 0, result.stdout + result.stderr)
        return result

    def read_calls(self):
        return [json.loads(line) for line in self.calls.read_text().splitlines()]

    def test_failed_identity_receipt_cannot_turn_resume_into_success(self):
        self.run_stage('identity', ok=False)
        self.run_stage('identity', ok=False)
        self.assertEqual(len(self.read_calls()), 2)
        kept = list((self.out / 'audits').glob('identity-frontier-P20.json.previous.*'))
        self.assertEqual(len(kept), 1)
        self.assertEqual(json.loads(kept[0].read_text()), {'ok': False})

    def test_failed_session_audit_is_replayed_and_prior_receipt_retained(self):
        self.session('train-P20')
        self.run_stage('audit', ok=False)
        self.run_stage('audit', ok=False)
        self.assertEqual(len(self.read_calls()), 2)
        self.assertEqual(len(list((self.out / 'audits').glob('train-P20.json.previous.*'))), 1)

    def test_partial_training_cell_cannot_win_selection(self):
        self.session('train-P20')
        self.env['MOCK_ROWS'] = json.dumps([row(16, 0.1, runs=4), row(32, 1)])
        self.run_stage('select')
        selected = json.loads((self.out / 'select/selection-P20.json').read_text())
        self.assertEqual(selected['selected']['n'], 32)

    def test_selection_requires_complete_candidate_and_size_control(self):
        self.session('train-P20')
        self.env['MOCK_ROWS'] = json.dumps([row(16, 0.1, runs=4)])
        self.run_stage('select', ok=False)
        self.assertFalse((self.out / 'select/FROZEN.sha256').exists())
        self.assertFalse(any(call[0] == 'plan' for call in self.read_calls()))

    def test_frozen_selection_is_reused_and_training_band_tampering_rejected(self):
        self.session('train-P20')
        self.run_stage('select')
        files = {p: p.read_bytes() for parent in ['select', 'specs']
                 for p in (self.out / parent).iterdir()}
        calls = self.read_calls()
        self.env['MOCK_ROWS'] = json.dumps([row(64, 0.01)])
        self.run_stage('select')
        self.run_stage('train')
        self.assertEqual(self.read_calls(), calls)
        for p, data in files.items():
            self.assertEqual(p.read_bytes(), data)
        (self.out / 'select/band-train-P20.json').write_text('{"rows":[]}\n')
        self.run_stage('select', ok=False)
        self.run_stage('holdout', ok=False)
        self.assertEqual(self.read_calls(), calls)

    def test_incomplete_edge_extension_is_archived_and_retried(self):
        self.session('train-P20')
        d = self.session('train-ext-P20-mitm2-n91', status='running')
        old = (d / 'session.json').read_bytes()
        self.env['MOCK_ROWS'] = json.dumps([row(16, 4), row(64, 1)])
        self.run_stage('train')
        runs = [call for call in self.read_calls() if call[0] == 'run']
        self.assertEqual(len(runs), 1)
        self.assertEqual(Path(runs[0][runs[0].index('--out') + 1]).name, d.name)
        archived = list(d.parent.glob(d.name + '.interrupted-*'))
        self.assertEqual(len(archived), 1)
        self.assertEqual((archived[0] / 'session.json').read_bytes(), old)
        self.assertEqual(json.loads((d / 'session.json').read_text())['status'], 'complete')


if __name__ == '__main__':
    unittest.main()
