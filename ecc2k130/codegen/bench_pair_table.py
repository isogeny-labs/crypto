#!/usr/bin/env python3
"""Benchmark the ECC2K-130 fixed-workflow pair-table builder.

The reported rate is wall-clock candidates per second, including the factor
base and process start-up. Run it against a checkout whose engine you want to
measure; a regression should not move the rate by more than the observed
run-to-run spread.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidates', type=int, default=200000)
    parser.add_argument('--params', type=Path,
                        default=ROOT / 'docs/ic/params/ecc2k130-fixed.json')
    parser.add_argument('--dir', type=Path,
                        help='campaign directory; a temporary one is used by default')
    parser.add_argument('--keep', action='store_true', help='keep the temporary campaign')
    parser.add_argument('--json', action='store_true')
    args = parser.parse_args()

    directory = args.dir
    temporary = directory is None
    if temporary:
        directory = Path(tempfile.mkdtemp(prefix='ecc2k130-pair-bench-',
                                           dir=os.environ.get('TMPDIR') or None))
    else:
        if directory.exists():
            shutil.rmtree(directory)
        directory.mkdir(parents=True)
    try:
        started = time.perf_counter()
        result = subprocess.run(
            [sys.executable, str(HERE / 'indexcalc_fixed.py'),
             '--params', str(args.params), '--dir', str(directory),
             '--stage', 'pairs', '--pair-budget', str(args.candidates)],
            cwd=HERE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        elapsed = time.perf_counter() - started
        try:
            report = json.loads(result.stdout)
        except json.JSONDecodeError as error:
            raise SystemExit('engine did not return JSON: %s\n%s' % (error, result.stderr[-2000:]))
        if report.get('status') not in ('pair_budget', 'complete'):
            raise SystemExit('engine stopped: %s %s' % (report.get('status'), report.get('message')))
        database = directory / 'campaign.sqlite3'
        record = {
            'degree': report.get('degree'),
            'candidates': report.get('reuse', {}).get('pair_candidates_built', 0),
            'pairs': report.get('pair_table', {}).get('serial', 0),
            'seconds': round(elapsed, 3),
            'candidates_per_second': round(
                report.get('reuse', {}).get('pair_candidates_built', 0) / elapsed, 1),
            'database_bytes': database.stat().st_size if database.exists() else 0,
            'status': report.get('status'),
        }
        if args.json:
            print(json.dumps(record, sort_keys=True))
        else:
            print('degree %s: %d candidates in %.3f s = %.1f candidates/s; %d pairs; %.1f MiB'
                  % (record['degree'], record['candidates'], record['seconds'],
                     record['candidates_per_second'], record['pairs'],
                     record['database_bytes'] / (1024 * 1024)))
    finally:
        if temporary and not args.keep:
            shutil.rmtree(directory, ignore_errors=True)


if __name__ == '__main__':
    main()
