#!/usr/bin/env python3
"""Replay one frozen full-lane serial/batch pair on every public Q fixture."""

import argparse
from hashlib import sha256
import json
from pathlib import Path
import subprocess


EXPECTED_Q_SHA256 = 'f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8'
EXACT_FIELDS = (
    'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
    'walk_steps', 'distinguished_points', 'verified_distinguished_points',
    'dead_trails', 'units_completed', 'checkins',
)


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--q-manifest', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    parser.add_argument('--repair-commit', required=True)
    parser.add_argument('--dp-bits', type=int, default=8)
    parser.add_argument('--step-cap', type=int, default=2048)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    q_manifest = args.q_manifest.resolve(strict=True)
    if digest(q_manifest) != EXPECTED_Q_SHA256:
        parser.error('frozen Q manifest hash mismatch')
    fixtures = json.loads(q_manifest.read_text())['fixtures']
    if len(fixtures) != 32:
        parser.error('expected 32 Q fixtures')
    args.output_dir.mkdir(parents=True, exist_ok=False)
    rows = []
    failures = []
    with (args.output_dir / 'arms.jsonl').open('w') as stream:
        for q_index, fixture in enumerate(fixtures):
            results = {}
            for mode, width in (('serial', 1), ('batch', 32)):
                command = [str(binary), str(q_manifest), str(q_index), mode,
                           '32', str(args.dp_bits), str(args.step_cap), str(width)]
                try:
                    completed = subprocess.run(command, capture_output=True,
                                               text=True, timeout=180, check=False)
                    stdout, stderr, returncode = (completed.stdout,
                                                   completed.stderr, completed.returncode)
                    timed_out = False
                except subprocess.TimeoutExpired as error:
                    stdout = error.stdout or ''
                    stderr = error.stderr or ''
                    if isinstance(stdout, bytes):
                        stdout = stdout.decode(errors='replace')
                    if isinstance(stderr, bytes):
                        stderr = stderr.decode(errors='replace')
                    returncode = None
                    timed_out = True
                try:
                    output = json.loads(stdout)
                except (json.JSONDecodeError, TypeError):
                    output = None
                issues = []
                if timed_out or returncode != 0 or stderr:
                    issues.append('process failure, timeout, or stderr')
                if output is None or output.get('status') != 'complete_frozen_work':
                    issues.append('complete worker output missing')
                else:
                    if output.get('q_index') != q_index or output.get('q_hex') != fixture['q_hex']:
                        issues.append('wrong frozen Q')
                    if output.get('mode') != mode or output.get('batch_width') != width:
                        issues.append('wrong mode or width')
                    if output.get('verified_distinguished_points') != output.get(
                            'distinguished_points'):
                        issues.append('DP replay failed')
                row = {'q_index': q_index, 'mode': mode, 'command': command,
                       'returncode': returncode, 'timed_out': timed_out,
                       'stdout': stdout, 'stderr': stderr, 'output': output,
                       'issues': issues}
                rows.append(row)
                stream.write(json.dumps(row, sort_keys=True) + '\n')
                stream.flush()
                results[mode] = row
                if issues:
                    failures.append({'q_index': q_index, 'mode': mode, 'issues': issues})
            serial, batch = results['serial']['output'], results['batch']['output']
            if serial is not None and batch is not None:
                unequal = [key for key in EXACT_FIELDS if serial.get(key) != batch.get(key)]
                if unequal:
                    failures.append({'q_index': q_index, 'mode': 'pair',
                                     'issues': ['exact fields differ: ' + ','.join(unequal)]})
            else:
                failures.append({'q_index': q_index, 'mode': 'pair',
                                 'issues': ['arm output missing']})
    summary = {
        'schema': 'rho-worker-batch-fixture-replay-v1',
        'status': 'pass' if not failures and len(rows) == 64 else 'fail',
        'scope': '32 public secp256k1 Q points, 32 workers per Q, serial and batch width 32',
        'dp_bits': args.dp_bits,
        'step_cap': args.step_cap,
        'measurement_tier': 'local_correctness_only',
        'repair_commit': args.repair_commit,
        'q_manifest_sha256': EXPECTED_Q_SHA256,
        'binary_sha256': digest(binary),
        'arm_count': len(rows),
        'pair_count': len(fixtures),
        'failures': failures,
        'arms_jsonl_sha256': digest(args.output_dir / 'arms.jsonl'),
        'performance_result': None,
    }
    (args.output_dir / 'summary.json').write_text(
        json.dumps(summary, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'status': summary['status'], 'arms': len(rows),
                      'failures': len(failures), 'output': str(args.output_dir)}))
    if summary['status'] != 'pass':
        raise SystemExit(1)


if __name__ == '__main__':
    main()
