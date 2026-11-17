#!/usr/bin/env python3
"""Rebuild the native development harness; contains no research algorithm."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('--target', type=Path, required=True, help='Existing locked Cargo target directory')
parser.add_argument('--out', type=Path, required=True, help='Fresh output directory')
args = parser.parse_args()
here = Path(__file__).resolve().parent
repo = here.parents[3]
manifest = json.loads((here / 'manifest.json').read_text())
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
for name, expected in manifest['sources'].items():
    if digest(repo / name) != expected:
        raise SystemExit(f'source differs from recorded candidate dependency: {name}')
expected = next(x['sha256'] for x in manifest['artifacts'] if x['path'] == 'candidate.rs')
if digest(here / 'candidate.rs') != expected:
    raise SystemExit('candidate hash mismatch')
args.out.mkdir(parents=True, exist_ok=False)
out = args.out.resolve()
# JSON string quoting also quotes these ordinary absolute paths for Rust.
def module(name, path):
    return f'#[path={json.dumps(str(path))}] pub mod {name};'
harness = 'mod hash {' + module('sha256', repo / 'src/hash/sha256.rs') + '}\n'
harness += 'mod cryptanalysis { pub mod ecbench {'
for name in ('canonical', 'exact_scalar'):
    harness += module(name, repo / f'src/cryptanalysis/ecbench/{name}.rs')
harness += module('exact_sampling', here / 'candidate.rs') + '}}\n'
(out / 'harness.rs').write_text(harness)
deps = args.target.resolve() / 'release/deps'
argv = ['rustc', '--edition=2021', '--test', '-O', str(out / 'harness.rs'), '-L', f'dependency={deps}', '-o', str(out / 'tests')]
identities = {}
for name in ('num_bigint', 'serde', 'serde_json', 'hex'):
    choices = list(deps.glob(f'lib{name}-*.rlib'))
    if not choices:
        raise SystemExit(f'missing locked build dependency {name}')
    selected = max(choices, key=lambda path: path.stat().st_mtime)
    argv += ['--extern', f'{name}={selected}']
    identities[name] = {'path': str(selected), 'sha256': digest(selected)}
env = dict(os.environ, CARGO_MANIFEST_DIR=str(repo))
with (out / 'build.log').open('w') as log:
    build = subprocess.run(argv, env=env, stdout=log, stderr=subprocess.STDOUT)
result = None
if build.returncode == 0:
    with (out / 'tests.log').open('w') as log:
        result = subprocess.run([str(out / 'tests'), '--test-threads=2'], stdout=log, stderr=subprocess.STDOUT).returncode
receipt = {'status': 'unsigned_producer_observation', 'scope': 'standalone native development tests', 'build_exit': build.returncode, 'tests_exit': result, 'dependencies': identities, 'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(), 'argv': argv}
receipt['files'] = {p.name: {'sha256': digest(p), 'bytes': p.stat().st_size} for p in out.iterdir() if p.is_file()}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
raise SystemExit(build.returncode or result or 0)
