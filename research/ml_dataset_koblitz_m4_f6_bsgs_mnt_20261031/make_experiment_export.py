#!/usr/bin/env python3
"""Deterministically project the four retained native m4 cells to experiment.export/v1.

Reads local native evidence and the pinned official checker. Never generates a cell
or contacts S3. Existing outputs are immutable unless their bytes are identical.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

HERE = Path(__file__).resolve().parent
EXPECTED_ROOT = Path('/mnt/spot-codex-state/tmp/ml-dataset-koblitz-m4-f6-bsgs-20261031')

def sha(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for part in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(part)
    return h.hexdigest()

def read(path):
    return json.loads(path.read_text())

def encoded(obj):
    return (json.dumps(obj, indent=2, sort_keys=True, ensure_ascii=False, allow_nan=False) + '\n').encode()

def write_equal(path, data):
    if path.exists():
        if path.read_bytes() != data:
            raise RuntimeError(f'existing output differs: {path}')
    else:
        path.write_bytes(data)

def file_entry(path, kind):
    try:
        location = str(path.relative_to(HERE))
    except ValueError:
        location = str(path)
    return {'path': location, 'sha256': sha(path), 'bytes': path.stat().st_size, 'kind': kind}

def iso_ns(value):
    return datetime.fromtimestamp(int(value) / 1_000_000_000, timezone.utc).isoformat()

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--native-root', type=Path, default=EXPECTED_ROOT)
    parser.add_argument('--tooling', type=Path, required=True, help='Pinned f755 official experiment_protocol.py')
    args = parser.parse_args()
    if args.native_root.resolve() != EXPECTED_ROOT or not args.tooling.is_file():
        raise RuntimeError('unexpected native root or missing official tooling')
    protocol, plan = read(HERE / 'protocol.json'), read(HERE / 'plan.json')
    if protocol['freeze']['status'] != 'frozen' or protocol['version'] != 2 or len(protocol['arms']) != 4:
        raise RuntimeError('unexpected frozen protocol')
    if plan['protocol_sha256'] != sha(HERE / 'protocol.json') or plan['field_sizes'] != [23] or plan['arities'] != [4] or plan['solvers'] != ['f6', 'bsgs'] or plan['twists'] is not True:
        raise RuntimeError('native plan does not bind frozen scope')
    spec = importlib.util.spec_from_file_location('pinned_experiment_protocol', args.tooling)
    official = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(official)
    arms = {arm['id']: arm for arm in protocol['arms']}
    rows, sources = [], []
    for index in range(4):
        shard = args.native_root / f'shard-{index:02d}'
        status_path = shard / 'local-status.json'
        marker_path = shard / 'attempt-start.json'
        receipt_path = shard / 'runner/receipt.json'
        manifest_path = shard / f'runner/attempt/cells/{index:012d}/manifest.json'
        checkpoint_path = HERE / 'checkpoints' / f'shard-{index:02d}.json'
        pointer_path = args.native_root / 'custody' / f'shard-{index:02d}.json'
        archive_path = args.native_root / 'archives' / f'shard-{index:02d}.tar.gz'
        marker, status, receipt, manifest, checkpoint, pointer = [read(p) for p in (marker_path, status_path, receipt_path, manifest_path, checkpoint_path, pointer_path)]
        cell = status['cell_spec']
        solver, twist = cell['solver'], cell['twist']
        if solver != ['f6', 'bsgs'][index % 2] or twist is not bool(index // 2):
            raise RuntimeError(f'mixed-radix cell mismatch at {index}')
        arm_id = f"koblitz23-m4-{solver}-{'twisted' if twist else 'untwisted'}"
        arm = arms[arm_id]
        if marker['index'] != status['index'] or status['index'] != manifest['cell_index'] or checkpoint['index'] != index or checkpoint['custody']['index'] != index:
            raise RuntimeError(f'cell identity mismatch at {index}')
        if not marker['one_attempt_only'] or marker['protocol_file_sha256'] != sha(HERE / 'protocol.json') or marker['plan_file_sha256'] != sha(HERE / 'plan.json'):
            raise RuntimeError(f'attempt marker binding mismatch at {index}')
        if checkpoint['custody']['protocol_file_sha256'] != sha(HERE / 'protocol.json') or checkpoint['custody']['plan_file_sha256'] != sha(HERE / 'plan.json'):
            raise RuntimeError(f'checkpoint binding mismatch at {index}')
        if checkpoint['custody'] != pointer or checkpoint['published']['pointer_sha256'] != sha(pointer_path):
            raise RuntimeError(f'pointer mismatch at {index}')
        if checkpoint['custody']['archive_sha256'] != sha(archive_path) or checkpoint['custody']['archive_bytes'] != archive_path.stat().st_size:
            raise RuntimeError(f'archive mismatch at {index}')
        if not (checkpoint['published']['archive_fresh_get'] and checkpoint['published']['pointer_fresh_get'] and checkpoint['custody']['fresh_get_byte_comparison']):
            raise RuntimeError(f'custody readback unproven at {index}')
        if receipt['binary_sha256_before'] != receipt['binary_sha256_after'] or receipt['binary_sha256_before'] != arm['instrument']['binary_sha256'] or receipt['source_commit'] != arm['instrument']['commit']:
            raise RuntimeError(f'native instrument mismatch at {index}')
        if receipt['exit_code'] != 0 or receipt['timed_out'] or status['verify'] != 0 or status['report'] != 0 or status['export'] != 0 or status['sealed_cell_verified'] is not True or status['artifact_hashes_verified'] is not True:
            raise RuntimeError(f'native stage not verified at {index}')
        outcome = manifest['outcome']
        if outcome['status'] != 'verified' or outcome['solver'] != solver:
            raise RuntimeError(f'native outcome mismatch at {index}')
        if solver == 'f6':
            pair = [a for a in manifest['artifacts'] if a['path'] == 'f6-pair-table.json' and a['role'] == 'independently-reconstructed-native-pair-domain']
            pair_path = manifest_path.parent / 'f6-pair-table.json'
            if len(pair) != 1 or pair[0]['sha256'] != sha(pair_path) or pair[0]['bytes'] != pair_path.stat().st_size:
                raise RuntimeError(f'F6 pair domain mismatch at {index}')
            if not (status['f6_pair_domain_retained'] and status['f6_pair_domain_promoted'] and status['f6_solver_completed'] and outcome['solver_completed']):
                raise RuntimeError(f'F6 completion mismatch at {index}')
            if status['ecdlp_solve_promoted'] or status['independently_solved_ecdlp_cells'] != 0 or outcome['scalar_recovered_without_witness'] or outcome['scalar_replay_verified'] or status['f6_pdp_witness_verified']:
                raise RuntimeError(f'unsupported F6 scalar or PDP label at {index}')
            pair_count, solve_count = 1, 0
        else:
            if not (status['ecdlp_solve_promoted'] and status['independently_solved_ecdlp_cells'] == 1 and outcome['scalar_recovered_without_witness'] and outcome['scalar_replay_verified']):
                raise RuntimeError(f'BSGS scalar replay mismatch at {index}')
            pair_count, solve_count = 0, 1
        rows.append({
            'run_id': f'shard-{index:02d}', 'arm': arm_id, 'instance_id': '0',
            'native_instance_id': f"g{cell['generator_index']}k{cell['key_index']}",
            'seed': plan['seed'], 'replicate': 0, 'warmup': False,
            'status': 'succeeded', 'verification': 'verified',
            'sealed-cell-verified': 1, 'f6-pair-domain-retained': pair_count,
            'independent-ecdlp-solve': solve_count,
            'wall_seconds': int(receipt['observed_wall_nanoseconds']) / 1_000_000_000,
            'native_status': outcome['status'], 'native_manifest_sha256': sha(manifest_path),
            'native_receipt_sha256': sha(receipt_path), 'local_status_sha256': sha(status_path),
            'checkpoint_sha256': sha(checkpoint_path), 'pointer_sha256': sha(pointer_path),
            'restricted_archive_sha256': sha(archive_path),
            'restricted_archive_s3_uri': checkpoint['custody']['s3_uri'],
        })
        sources.append((receipt, outcome, marker_path, status_path, receipt_path, manifest_path, checkpoint_path, pointer_path, archive_path))
    rows_path = HERE / 'experiment-rows.jsonl'
    rows_bytes = b''.join((json.dumps(row, sort_keys=True, ensure_ascii=False, allow_nan=False) + '\n').encode() for row in rows)
    write_equal(rows_path, rows_bytes)
    export = official.from_rows(rows_path, protocol, 'protocol.json', 'koblitz23-m4-f6-bsgs-mnt-4-20261031', {})
    export['created'] = iso_ns(max(int(src[0]['finished_unix_nanoseconds']) for src in sources))
    export['producer'] = {'runtime': 'codex_cli', 'model': None, 'session': 'T-1074', 'role': 'executor'}
    export['source'] = {'repo': protocol['repository'], 'commit': protocol['arms'][0]['instrument']['commit'], 'dirty': False}
    export['backend'] = 'local'
    export['arms'] = [{'id': a['id'], 'role': a['role'], 'binary_sha256': a['instrument']['binary_sha256'], 'commit': a['instrument']['commit']} for a in protocol['arms']]
    export['files'].extend(file_entry(HERE / name, kind) for name, kind in (
        ('protocol.json', 'protocol'), ('plan.json', 'raw'), ('preflight.json', 'receipt'), ('make_experiment_export.py', 'other')))
    for run, src in zip(export['runs'], sources):
        receipt, outcome, marker_path, status_path, receipt_path, manifest_path, checkpoint_path, pointer_path, archive_path = src
        run['verification']['verifier'] = 'crypto dataset verify; F6 pair-domain artifact or BSGS scalar replay by native verifier'
        run['artifacts'] = [file_entry(path, kind) for path, kind in (
            (marker_path, 'receipt'), (status_path, 'receipt'), (receipt_path, 'receipt'),
            (manifest_path, 'checkpoint_manifest'), (checkpoint_path, 'checkpoint_manifest'),
            (pointer_path, 'checkpoint_manifest'), (archive_path, 'raw'))]
        run['backend_ref'] = f"native-cell-{int(run['run_id'].split('-')[1]):012d}"
        run['started_at'] = iso_ns(receipt['started_unix_nanoseconds'])
        run['finished_at'] = iso_ns(receipt['finished_unix_nanoseconds'])
        run['measurements']['counters'] = {'observed-wall-ns': receipt['observed_wall_nanoseconds']}
        for key, value in outcome.get('counters', {}).items():
            if isinstance(value, (int, float, str)):
                run['measurements']['counters']['native_' + key] = value
        run['measurements']['wall_valid'] = False
        run['isolation']['tier'] = 'none'
    export['flow'] = official.compute_flow(export['runs'])
    export['notes'] = [
        {'kind': 'observation', 'text': 'All four fixed native cells passed verify, report and export. F6 cells 0 and 2 completed a bounded geometric four-sum over retained pair domains, with no PDP witness or scalar label; native negative exhaustiveness is not independently certified.', 'by': 'deterministic receipt projection'},
        {'kind': 'observation', 'text': 'BSGS cells 1 and 3 independently recovered and replayed an ECDLP scalar. Wrapper durations are diagnostic and wall_valid is false for all rows; retired hardware instructions and first/last fall degrees are unavailable.', 'by': 'deterministic receipt projection'},
        {'kind': 'observation', 'text': 'Each restricted local archive and custody pointer matches the Git-ready checkpoint and has producer fresh S3 GET receipts. This export does not perform a new remote readback.', 'by': 'deterministic receipt projection'},
    ]
    export_path = HERE / 'experiment-export.json'
    write_equal(export_path, encoded(export))
    command = [sys.executable, str(args.tooling), 'check-export', str(export_path), '--protocol', str(HERE / 'protocol.json'), '--verify-files']
    checked = subprocess.run(command, capture_output=True, text=True, check=False)
    if checked.returncode != 0:
        raise RuntimeError(f'official checker failed ({checked.returncode}): {checked.stdout}{checked.stderr}')
    report = {'schema': 'ml-dataset.koblitz-m4-f6-bsgs-experiment-export-receipt/v1',
              'export_sha256': sha(export_path), 'rows_sha256': sha(rows_path), 'tooling_sha256': sha(args.tooling),
              'protocol_sha256': protocol['freeze']['protocol_sha256'], 'flow': export['flow'],
              'f6_pair_domain_rows': 2, 'independent_bsgs_scalar_rows': 2, 's3_archives_fresh_get_receipted': 4,
              'official_checker': {'argv': command, 'exit_code': checked.returncode, 'stdout': checked.stdout, 'stderr': checked.stderr}}
    write_equal(HERE / 'experiment-export-producer-receipt.json', encoded(report))
    print(json.dumps({'export': str(export_path), 'sha256': sha(export_path), 'flow': export['flow']}, sort_keys=True))

if __name__ == '__main__':
    main()
