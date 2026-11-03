#!/usr/bin/env python3
"""Compare exact N83 root-index bytes and stage cost under a frozen ABBA order."""
import argparse
import hashlib
import json
import os
import platform
import subprocess
import time
from pathlib import Path

FLAGS = {
    'KIC_INDEX_BUILD_ONLY': '1',
    'KIC_PACKED_ROOT_TABLE': '1',
    'KIC_ROOT_ONLY11': '1',
    'KIC_INDEXED_ROOT5': '1',
    'KIC_INDEXED5_LOAD75': '1',
    'KIC_INDEXED5_ID30': '1',
    'KIC_ROOT_BLOOM': '1',
    'KIC_ROOT_BLOOM_PROFILE': 'wide3',
    'KIC_SYMMETRIC_S3': '1',
    'KIC_INDEX_ROOT_BATCH': '1',
    'KIC_SPARSE_CANONICAL': '1',
}
EXACT = ('base_hash', 'orbit_columns', 'factor_base_points',
         'regular_states', 'root_table_entries', 'state_digest_blake3',
         'table_digest_blake3')

def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1 << 20), b''):
            value.update(block)
    return value.hexdigest()

def run_one(binary, k, threads, output, number):
    environment = {key: value for key, value in os.environ.items()
                   if not key.startswith('KIC_')}
    environment.update(FLAGS)
    environment['KIC_INDEX_ROOT_THREADS'] = str(threads)
    command = [str(binary), f'construct:83:1:{k}', '/dev/null', '29',
               str(output / f'ignored-target-{number}.jsonl')]
    started = time.monotonic()
    result = subprocess.run(command, env=environment, cwd=output,
                            capture_output=True, text=True, timeout=2400)
    if result.returncode or result.stderr:
        raise RuntimeError(f'index stage exited {result.returncode}: {result.stderr[-500:]}')
    rows = [json.loads(line) for line in result.stdout.splitlines() if line.startswith('{')]
    if len(rows) != 1 or rows[0].get('status') != 'STAGE_DIAGNOSTIC_ONLY':
        raise ValueError('expected one index-stage JSON receipt')
    return {'k': k, 'threads': threads, 'order': number,
            'wall_seconds': time.monotonic() - started,
            'command': command, 'flags': environment['KIC_INDEX_ROOT_THREADS'],
            'receipt': rows[0]}

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--k', nargs='+', type=int, default=[64, 256, 512])
    args = parser.parse_args()
    binary = args.binary.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    records = []
    for k in args.k:
        if not 1 <= k <= 4405:
            raise ValueError('K outside tested N83 bound')
        group = [run_one(binary, k, threads, output, number)
                 for number, threads in enumerate((1, 4, 4, 1))]
        baseline = group[0]['receipt']
        for row in group[1:]:
            if any(row['receipt'].get(key) != baseline.get(key) for key in EXACT):
                raise ValueError(f'K={k}: parallel root index changed exact contents')
        records.extend(group)
        print(json.dumps({'k': k, 'status': 'PASS_EXACT_INDEX',
                          'index_build_ms': [row['receipt']['index_build_ms']
                                             for row in group]}), flush=True)
    report = {'schema': 'n83.parallel-index-stage/v1',
              'status': 'PASS_EXACT_INDEX', 'binary_sha256': digest(binary),
              'host': platform.platform(), 'records': records}
    (output / 'report.json').write_text(json.dumps(report, sort_keys=True,
                                                   indent=2) + '\n')

if __name__ == '__main__':
    main()
