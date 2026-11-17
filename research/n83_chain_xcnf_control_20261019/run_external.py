#!/usr/bin/env python3
"""Bound a local CryptoMiniSat control while retaining first-attempt receipts."""
import argparse
import hashlib
import json
import platform
import resource
import shutil
import subprocess
import time
from pathlib import Path

GAUSS_PROFILES = {
    'default': (),
    'wide': ('--maxmatrixrows=10000', '--maxmatrixcols=50000'),
    'forced': ('--maxmatrixrows=10000', '--maxmatrixcols=50000',
               '--autodisablegauss=0', '--gaussusefulcutoff=0'),
}


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('xcnf', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--seconds', type=int, default=300)
    parser.add_argument('--conflicts', type=int, default=5000)
    parser.add_argument('--gauss-profile', choices=tuple(GAUSS_PROFILES),
                        default='default')
    args = parser.parse_args()
    if not 1 <= args.seconds <= 1200 or not 1 <= args.conflicts <= 10000000:
        raise ValueError('cap out of bounded range')
    solver = shutil.which('cryptominisat5')
    if solver is None:
        raise ValueError('cryptominisat5 unavailable')
    solver = Path(solver).resolve()
    args.output.mkdir(parents=True, exist_ok=False)
    version = subprocess.run([str(solver), '--version'], capture_output=True,
                             text=True, check=True).stdout
    command = [str(solver), '--verb=0', '--threads=1',
               f'--maxtime={args.seconds}', f'--maxconfl={args.conflicts}',
               *GAUSS_PROFILES[args.gauss_profile], str(args.xcnf.resolve())]
    start = time.monotonic()
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    try:
        proc = subprocess.run(command, capture_output=True, timeout=args.seconds + 15)
        stdout, stderr = proc.stdout, proc.stderr
        code = proc.returncode
        timed_out = False
    except subprocess.TimeoutExpired as error:
        stdout, stderr = error.stdout or b'', error.stderr or b''
        code = None
        timed_out = True
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    (args.output / 'solver.stdout').write_bytes(stdout)
    (args.output / 'solver.stderr').write_bytes(stderr)
    status_lines = [line for line in stdout.decode(errors='replace').splitlines()
                    if line.startswith('s ')]
    status = status_lines[0][2:] if len(status_lines) == 1 else None
    if timed_out:
        result = 'UNKNOWN_wall_cap'
    elif status == 'SATISFIABLE' and code in (0, 10):
        result = 'SAT_unverified'
    elif status == 'UNSATISFIABLE' and code in (0, 20):
        result = 'UNSAT_unverified'
    elif status in ('INDETERMINATE', 'UNKNOWN'):
        result = 'UNKNOWN_solver_cap'
    else:
        result = 'PRODUCER_FAILURE_solver_exit'
    receipt = {
        'schema': 'n83.external-xcnf-run/v1',
        'status': result,
        'candidate_id': None,
        'selected_best_total_runtime': None,
        'xcnf_sha256': digest(args.xcnf),
        'xcnf_bytes': args.xcnf.stat().st_size,
        'solver_path': str(solver),
        'solver_sha256': digest(solver),
        'solver_version_first_line': version.splitlines()[0] if version else '',
        'command': command,
        'cap_seconds': args.seconds,
        'cap_conflicts': args.conflicts,
        'gauss_profile': args.gauss_profile,
        'elapsed_wall_ms': (time.monotonic() - start) * 1000,
        'child_user_cpu_seconds': after.ru_utime - before.ru_utime,
        'child_system_cpu_seconds': after.ru_stime - before.ru_stime,
        'child_maxrss_raw': after.ru_maxrss,
        'exit_code': code,
        'timed_out': timed_out,
        'solver_status': status,
        'stdout_sha256': hashlib.sha256(stdout).hexdigest(),
        'stderr_sha256': hashlib.sha256(stderr).hexdigest(),
        'os': platform.platform(),
        'machine': platform.machine(),
    }
    (args.output / 'run.json').write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
    print(json.dumps({key: receipt[key] for key in
                      ('status', 'solver_status', 'elapsed_wall_ms', 'exit_code')},
                     sort_keys=True))


if __name__ == '__main__':
    main()
