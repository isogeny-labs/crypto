#!/usr/bin/env python3
"""Validate the unapplied patch in a disposable standalone CPU snapshot.

Never modifies the reserved shared source. Destination must not exist.
"""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys

here = Path(__file__).resolve().parent
repo = here.parents[2]
meta = json.loads((here / 'source.json').read_text())
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
source = repo / meta['path']
assert sha(source) == meta['base_sha256'], 'shared source changed: reconcile first'
assert sha(here / 'target-reuse.patch') == meta['patch_sha256']
snapshot = Path(sys.argv[1]).resolve()
assert not snapshot.exists(), 'refusing to overwrite a snapshot'
snapshot.mkdir(parents=True)
for name in ['hashed-ecdl-cpu', 'hashed-ecdl-tensor']:
    shutil.copytree(repo / 'crates' / name, snapshot / 'crates' / name,
                    ignore=shutil.ignore_patterns('target', '.git'))
(snapshot / meta['path']).parent.mkdir(parents=True)
shutil.copy2(source, snapshot / meta['path'])
for i in range(3):
    path = Path(f'research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-{i}-1.txt')
    (snapshot / path).parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(repo / path, snapshot / path)
subprocess.run(['git', 'apply', '--check', str(here/'target-reuse.patch')],
               cwd=snapshot, check=True)
subprocess.run(['git', 'apply', str(here/'target-reuse.patch')],
               cwd=snapshot, check=True)
assert sha(snapshot / meta['path']) == meta['candidate_sha256']
manifest = snapshot / 'crates/hashed-ecdl-cpu/Cargo.toml'
target = Path(sys.argv[2]).resolve()
common = ['--locked', '--all-targets', '--manifest-path', str(manifest), '--target-dir', str(target)]
commands = [
    ['cargo', 'test', '--release', *common],
    ['cargo', 'clippy', *common, '--', '-D', 'warnings'],
    ['cargo', 'fmt', '--check', '--manifest-path', str(manifest)],
]
records = []
for i, command in enumerate(commands):
    log = snapshot / f'validation-{i}.log'
    with log.open('w') as f:
        result = subprocess.run(command, cwd=snapshot, stdout=f, stderr=subprocess.STDOUT)
    records.append({'command': command, 'exit_code': result.returncode,
                    'log': log.name, 'sha256': sha(log)})
    if result.returncode:
        print(log.read_text())
        raise SystemExit(result.returncode)
assert sha(source) == meta['base_sha256'], 'shared source changed during validation'
record = {'source': meta, 'commands': records, 'shared_source_unchanged': True,
          'scope': 'development correctness checks only; no timing or EKS execution'}
(snapshot/'validation.json').write_text(json.dumps(record, indent=2)+'\n')
print('PASS unapplied snapshot: release tests, strict Clippy and formatting; shared source unchanged')
