#!/usr/bin/env python3
"""Replay the one-input public-dispatch and direct-subroutine probe."""
import argparse
import hashlib
import os
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
SOURCE = ROOT / 'src/bin/projective_law_gate.rs'
TESTS = HERE / 'probe_tests.rs'
GENERATED = ROOT / 'src/bin/projective_f17_semantic_probe.rs'
RUST_SHA = '5976e7f30b3e9c5c49a5fc0f4095404b7322832e4b02f467bc3cb7e0d9dcc8c0'

def cargo(test_bytes: bytes, log_name: str) -> subprocess.CompletedProcess:
    if GENERATED.exists():
        raise RuntimeError(f'refusing to overwrite {GENERATED}')
    GENERATED.write_bytes(SOURCE.read_bytes() + b'\n' + test_bytes)
    try:
        cmd = ['cargo', 'test', '--release', '--offline', '--locked',
               '--manifest-path', str(ROOT / 'Cargo.toml'), '--bin',
               'projective_f17_semantic_probe',
               'author_dbl2b0_zero_weight_counterexample', '--', '--nocapture']
        env = os.environ.copy()
        env['TMPDIR'] = '/dev/shm'
        result = subprocess.run(cmd, cwd=ROOT.parent.parent, env=env,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        (HERE / log_name).write_bytes(result.stdout.rstrip(b'\n') + b'\n')
        return result
    finally:
        GENERATED.unlink()

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--author-source', required=True, type=Path)
    args = parser.parse_args()
    assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == RUST_SHA
    source = TESTS.read_bytes()
    old = b'assert_eq!(dispatch_label, "same_u_inverse");'
    assert source.count(old) == 1
    failed = cargo(source.replace(old, b'assert_eq!(dispatch_label, "b_zero_identity");'),
                   'initial-failed-assertion.log')
    assert failed.returncode != 0
    assert b'left: "same_u_inverse"' in failed.stdout
    assert b'right: "b_zero_identity"' in failed.stdout
    good = cargo(source, 'corrected-cargo.log')
    assert good.returncode == 0, good.stdout.decode(errors='replace')
    assert b'F17_TORSION_JSON' in good.stdout
    assert b'test result: ok. 1 passed; 0 failed' in good.stdout
    subprocess.run(['python3', str(HERE / 'check.py'), '--author-source',
                    str(args.author_source), '--check'], check=True)
    print('source-bound F17 dispatch and direct-subroutine probe passed')

if __name__ == '__main__':
    main()
