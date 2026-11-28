"""Supplemental cells without modifying or overwriting the frozen primary run.

Usage: python benchmark_extra.py --suite large|paper
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import statistics
import subprocess
import sys
import time

from benchmark import HERE, make_case


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--suite', choices=('large', 'paper'), default='large')
    args = parser.parse_args()
    from fast_trace.kernel import initialize, flint
    initialize()
    out = HERE / ('results_' + args.suite)
    out.mkdir(exist_ok=True)
    fixtures = out / 'fixtures'
    fixtures.mkdir(exist_ok=True)
    parameters = ((1000003, 2, 3, 256), (1000033, 2, 3, 256)) if args.suite == 'large' else ((83, 31, 21, 8), (83, 31, 21, 16))
    cases = []
    for prime, a, b, degree in parameters:
        for kind in ('generic', 'collision', 'exception_plus', 'exception_minus'):
            case = make_case(prime, a, b, degree, kind, 20261116 + degree + prime)
            path = fixtures / (case['id'] + '.json')
            path.write_text(json.dumps(case) + '\n')
            cases.append((case, path))
    rows, failures = [], []
    for repeat in range(3):
        for number, (case, path) in enumerate(cases):
            methods = ('trace', 'dense') if (number + repeat) % 2 else ('dense', 'trace')
            for method in methods:
                before = resource.getrusage(resource.RUSAGE_CHILDREN)
                start = time.perf_counter()
                environment = dict(os.environ, OMP_NUM_THREADS='1', OPENBLAS_NUM_THREADS='1', MKL_NUM_THREADS='1', PYTHONPATH=str(HERE.parent))
                command = [sys.executable, str(HERE / 'benchmark.py'), '--worker', str(path), '--method', method]
                try:
                    process = subprocess.run(command, text=True, capture_output=True, env=environment, timeout=90)
                    after = resource.getrusage(resource.RUSAGE_CHILDREN)
                    if process.returncode:
                        raise RuntimeError(process.stderr)
                    row = json.loads(process.stdout)
                    row.update(repeat=repeat, fresh_child_user_system_cpu_seconds=after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime, fresh_child_wall_seconds=time.perf_counter() - start)
                    rows.append(row)
                except Exception as error:
                    failures.append(dict(case=case['id'], method=method, repeat=repeat, error=repr(error)))
                (out / 'raw.json').write_text(json.dumps(dict(rows=rows, failures=failures), indent=2) + '\n')
            print(json.dumps(dict(repeat=repeat, case=case['id'], completed_rows=len(rows), failures=len(failures))), flush=True)
    summary = []
    for case, _ in cases:
        item = {name: case[name] for name in ('p', 'degree', 'kind')}
        item['case'] = case['id']
        for method in ('trace', 'dense'):
            values = [row for row in rows if row['case'] == case['id'] and row['method'] == method]
            item[method] = dict(samples=len(values), median_cpu_seconds=statistics.median(row['complete_call_cpu_seconds'] for row in values) if values else None, max_fresh_process_peak_rss_kib=max((row['fresh_process_peak_rss_kib'] for row in values), default=None))
        if item['trace']['median_cpu_seconds'] and item['dense']['median_cpu_seconds']:
            item['dense_over_trace'] = item['dense']['median_cpu_seconds'] / item['trace']['median_cpu_seconds']
        summary.append(item)
    metadata = dict(platform=sys.platform, python=sys.version, python_flint=flint.__version__, affinity=[0], flint_threads=1, repeats=3, source_sha256={path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in (HERE / 'kernel.py', HERE / 'recurrence.cpp', HERE / 'benchmark.py', Path(__file__))}, scope='Same full-call and fresh-child accounting as primary benchmark.py. Input construction and helper compilation excluded equally. Shared pinned container; quadratic BM remains.')
    (out / 'summary.json').write_text(json.dumps(dict(metadata=metadata, summary=summary, failures=failures), indent=2) + '\n')
    print(json.dumps(dict(rows=len(rows), failures=len(failures), summary=str(out / 'summary.json'))))


if __name__ == '__main__':
    main()
