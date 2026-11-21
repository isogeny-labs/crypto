#!/usr/bin/env python3
"""Replay the isolated Curve25519 pair collector candidate; no research algorithm in Python."""
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
boundary = here / 'boundary-extraction.rs'
expected_boundary = next(x['sha256'] for x in manifest['artifacts'] if x['path'] == boundary.name)
if digest(boundary) != expected_boundary:
    raise SystemExit('ledger/trait extraction hash mismatch')
harness = 'mod hash {' + module('sha256', repo / 'src/hash/sha256.rs') + '}\n'
harness += 'mod cryptanalysis { pub mod residual_walk { pub fn is_prime_u64(_:u64)->bool { panic!("unused Fp64 primality path in Fp256 workload test") } }'
for name, path in [('fp_backend', repo / 'src/cryptanalysis/fp_backend.rs'), ('ic_boundary', boundary), ('curve25519_wide', repo / 'src/cryptanalysis/curve25519_wide.rs')]:
    harness += module(name, path)
harness += 'pub mod ecbench {'
for name in ('canonical', 'exact_scalar', 'exact_sampling'):
    harness += module(name, repo / f'src/cryptanalysis/ecbench/{name}.rs')
harness += module('curve25519_pair_collector', here / 'candidate.rs') + '}}\n'
(out / 'harness.rs').write_text(harness)
deps = args.target.resolve() / 'release/deps'
argv = ['rustc', '--edition=2021', '--test', '-O', str(out / 'harness.rs'), '-L', f'dependency={deps}', '-o', str(out / 'tests')]
identities = {}
for name in ('num_bigint', 'num_traits', 'num_integer', 'serde', 'serde_json', 'hex', 'rand'):
    choices = list(deps.glob(f'lib{name}-*.rlib'))
    if not choices:
        raise SystemExit(f'missing locked build dependency {name}')
    if name == 'num_bigint':
        compatible = []
        for path in choices:
            suffix = path.stem.removeprefix('libnum_bigint-')
            fingerprint = deps.parent / '.fingerprint' / f'num-bigint-{suffix}' / 'lib-num_bigint.json'
            if fingerprint.exists() and 'rand' in json.loads(fingerprint.read_text())['features']:
                compatible.append(path)
        if len(compatible) != 1:
            raise SystemExit('need exactly one num-bigint rlib with the repository rand feature')
        selected = compatible[0]
    else:
        selected = max(choices, key=lambda path: path.stat().st_mtime)
    argv += ['--extern', f'{name}={selected}']
    identities[name] = {'path': str(selected), 'sha256': digest(selected)}
env = dict(os.environ, CARGO_MANIFEST_DIR=str(repo))
with (out / 'build.log').open('w') as log:
    build = subprocess.run(argv, env=env, stdout=log, stderr=subprocess.STDOUT)
result = None
if build.returncode == 0:
    with (out / 'tests.log').open('w') as log:
        result = subprocess.run([str(out / 'tests'), 'cryptanalysis::ecbench::curve25519_pair_collector::tests', '--test-threads=2'], stdout=log, stderr=subprocess.STDOUT).returncode
receipt = {'status': 'unsigned_producer_observation', 'scope': 'standalone native development tests', 'build_exit': build.returncode, 'tests_exit': result, 'dependencies': identities, 'rustc': subprocess.check_output(['rustc', '--version'], text=True).strip(), 'argv': argv}
receipt['files'] = {p.name: {'sha256': digest(p), 'bytes': p.stat().st_size} for p in out.iterdir() if p.is_file()}
(out / 'receipt.json').write_text(json.dumps(receipt, indent=2) + '\n')
raise SystemExit(build.returncode or result or 0)
