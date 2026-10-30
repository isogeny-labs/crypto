#!/usr/bin/env python3
"""Archive/JSON orchestration only; no solver, scalar computation or timing run."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

ARCHIVE_SHA256 = '550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339'
BITS = [16, 20, 24, 28, 32, 36, 40]

def prepare(archive, out):
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    if digest != ARCHIVE_SHA256:
        raise ValueError('unexpected frozen archive hash')
    raw = subprocess.run(['zstd', '-dc', str(archive)], check=True, stdout=subprocess.PIPE).stdout
    payloads, rows = {}, []
    with tarfile.open(fileobj=io.BytesIO(raw), mode='r:') as tar:
        for bits in BITS:
            prefix = f'sessions_projective_chart_v2/order-genus-{bits}'
            payloads[f'archive-spec-{bits}.json'] = tar.extractfile(prefix + '/spec.json').read()
            for line in tar.extractfile(prefix + '/records.jsonl').read().splitlines():
                record = json.loads(line)
                if record['arm'] != 'genus_projective':
                    continue
                w = record['workload']
                rows.append(dict(bits=bits, workload_id=w['workload_id'], target=w['target'],
                                 target_seed=w['target_seed'], target_index=w['target_index'],
                                 algorithm_seed=record['algorithm_seed']))
    if len(rows) != 1260 or len({r['workload_id'] for r in rows}) != 1260:
        raise ValueError('archive cohort inventory mismatch')
    payloads['expected-workloads.jsonl'] = ''.join(json.dumps(r, separators=(',', ':')) + '\n' for r in rows).encode()
    out.mkdir(exist_ok=False)
    for name, data in payloads.items():
        (out / name).write_bytes(data)
    manifest = dict(schema='pencil-archive-input-validation/v1',
                    status='expected_inputs_extracted_native_parity_pending',
                    source_archive_sha256=digest, rows=len(rows),
                    purpose='Run-free native plan comparison only. Archived specs are not the new execution protocol.',
                    projection='Archived workload_id, target, target_seed, target_index, algorithm_seed and size only; no planted answer.',
                    files={name: hashlib.sha256(data).hexdigest() for name, data in sorted(payloads.items())})
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')

def check(binary, inputs, out):
    manifest = json.loads((inputs / 'manifest.json').read_text())
    for name, digest in manifest['files'].items():
        if hashlib.sha256((inputs / name).read_bytes()).hexdigest() != digest:
            raise ValueError('input hash mismatch: ' + name)
    expected = [json.loads(line) for line in (inputs / 'expected-workloads.jsonl').read_text().splitlines()]
    observed = []
    out.mkdir(exist_ok=False)
    for bits in BITS:
        completed = subprocess.run([str(binary.resolve()), 'plan', '--spec', str(inputs / f'archive-spec-{bits}.json'), '--doc'], check=True, capture_output=True)
        (out / f'plan-{bits}.json').write_bytes(completed.stdout)
        plan = json.loads(completed.stdout)
        for execution in plan['executions']:
            if execution['warmup'] or execution['round'] != 0:
                continue
            arm = plan['arms'][execution['arm']]
            if arm['name'] != 'genus_projective':
                continue
            w = plan['workloads'][execution['workload']]
            observed.append(dict(bits=bits, workload_id=w['workload_id'], target=w['target'],
                                 target_seed=w['target_seed'], target_index=w['target_index'],
                                 algorithm_seed=str(execution['algorithm_seed'])))
    sort = lambda rows: sorted(rows, key=lambda r: r['workload_id'])
    passed = len(observed) == 1260 and sort(observed) == sort(expected)
    result = dict(schema='pencil-native-input-parity/v1', passed=passed, expected=len(expected), observed=len(observed),
                  binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), source_archive_sha256=manifest['source_archive_sha256'],
                  scope='Workload identities, target coordinates and archived first algorithm seeds only; no solves, cost measurements or source-build attestation.')
    (out / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    if not passed:
        raise ValueError('native workload parity mismatch; retained plans and result')

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('prepare'); p.add_argument('archive', type=Path); p.add_argument('out', type=Path)
    p = sub.add_parser('check'); p.add_argument('binary', type=Path); p.add_argument('inputs', type=Path); p.add_argument('out', type=Path)
    args = parser.parse_args()
    if args.command == 'prepare':
        prepare(args.archive, args.out)
    else:
        check(args.binary, args.inputs, args.out)
