#!/usr/bin/env python3
"""Replay stored N83 high-K bases and public target certificates on any checkout."""
import argparse
import gzip
import hashlib
import importlib.util
import json
import math
from pathlib import Path

ARMS = {
    'k4096_standard': (4096, 'standard'),
    'k4300_standard': (4300, 'standard'),
    'k4300_wide3': (4300, 'wide3'),
    'k4405_standard': (4405, 'standard'),
    'k4405_wide3': (4405, 'wide3'),
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def lines(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def one(path):
    value = json.loads(path.read_text())
    if not isinstance(value, dict):
        raise ValueError(f'expected one JSON object: {path}')
    return value


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--repo', type=Path, required=True)
    parser.add_argument('--evidence', type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument('--base-dir', type=Path, required=True)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    repo = args.repo.resolve()
    evidence = args.evidence.resolve()
    base_dir = args.base_dir.resolve()
    source_path = repo / 'examples/koblitz_orbit_dlp_fast.rs'
    targets_path = repo / 'research/n83_root4_frontier_current_20261018/fresh_targets.jsonl'
    answers_path = repo / 'research/n83_root4_frontier_current_20261018/fresh_answers.jsonl'
    group_path = repo / 'research/n83_indexed5_20261013/analyze.py'
    spec = importlib.util.spec_from_file_location('n83_highk_replay_group', group_path)
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    public = lines(targets_path)
    answers = lines(answers_path)
    if len(public) != 16 or len(answers) != 16:
        raise ValueError('fixture count differs')
    points = []
    scalars = []
    for i, (row, answer) in enumerate(zip(public, answers)):
        point = group.point_from_json(row)
        scalar = int(answer['scalar'])
        if (answer['fixture_index'] != i or not group.on_curve(point)
                or group.multiply(point, group.R) is not None
                or group.multiply(group.G, scalar) != point):
            raise ValueError(f'fixture replay differs at {i}')
        points.append(point)
        scalars.append(scalar)
    source_sha = sha(source_path.read_bytes())
    fixture_sha = sha(targets_path.read_bytes())
    stored = {row['k']: row for row in json.loads((evidence / 'store_blobs_manifest.json').read_text())}
    bases = {}
    base_receipts = {}
    for k in sorted({k for k, _ in ARMS.values()}):
        record = stored[k]
        path = base_dir / f'k{k}_base_dump.jsonl.gz'
        compressed = path.read_bytes()
        if sha(compressed) != record['gzip_sha256'] or len(compressed) != record['gzip_bytes']:
            raise ValueError(f'compressed base digest differs: K{k}')
        raw = gzip.decompress(compressed)
        if sha(raw) != record['raw_sha256'] or len(raw) != record['raw_bytes']:
            raise ValueError(f'raw base digest differs: K{k}')
        dump = json.loads(raw)
        bases[k] = dump
        base_receipts[str(k)] = {'raw_sha256': sha(raw), 'gzip_sha256': sha(compressed)}
    arms = {}
    for label, (k, profile) in ARMS.items():
        folder = evidence / 'runs' / label
        source = one(folder / 'source.json')
        summary = one(folder / 'stdout.jsonl')
        analysis = one(folder / 'analysis.json')
        usage = one(folder / 'usage.json')
        if (source['source_sha256'] != source_sha
                or source['target_sha256'] != fixture_sha
                or source['algorithm_flags']['KIC_ROOT_BLOOM_PROFILE'] != profile
                or analysis['status'] != 'PASS_INDEPENDENT_REPLAY'
                or analysis['fixture_sha256'] != fixture_sha
                or analysis['base_dump_sha256'] != stored[k]['raw_sha256']
                or usage['status'] != 'PASS_PRODUCER'
                or usage['exit_code'] != 0
                or summary['root_bloom_profile'] != profile
                or summary['root_bloom_bytes'] != (1 if profile == 'standard' else 2) * (1 << 30)
                or summary['rank'] != k
                or summary['factor_base_points'] != 166 * k
                or summary['targets_solved'] != 16
                or summary['targets_failed'] != 0):
            raise ValueError(f'executed source, profile or admitted run differs: {label}')
        base = group.replay_base(bases[k], summary)
        if len(base) != 166 * k:
            raise ValueError(f'base size differs: {label}')
        rows = lines(folder / 'targets_result.jsonl')
        by_index = {row['fixture_index']: row for row in rows}
        if len(rows) != 16 or set(by_index) != set(range(16)):
            raise ValueError(f'target count or indices differ: {label}')
        online = []
        for i in range(16):
            row = by_index[i]
            scalar = int(row['recovered_scalar'])
            indices = row['point_indices']
            if (row['group_verified'] is not True
                    or group.point_from_json(row['target']) != points[i]
                    or scalar != scalars[i]
                    or group.multiply(group.G, scalar) != points[i]
                    or len(indices) != 4
                    or any(not 0 <= index < len(base) for index in indices)):
                raise ValueError(f'scalar or witness differs: {label}, {i}')
            relation = None
            for index in indices:
                relation = group.add(relation, base[index])
            if relation != points[i]:
                raise ValueError(f'four-point relation differs: {label}, {i}')
            phases = [float(row[key]) for key in ('target_query_ms', 'target_pdp_ms',
                      'target_relation_check_ms', 'target_descent_ms', 'target_recovery_check_ms')]
            duration = float(row['target_ms'])
            if (not math.isfinite(duration) or duration <= 0
                    or any(not math.isfinite(x) or x < 0 for x in phases)
                    or abs(duration - sum(phases)) > 1):
                raise ValueError(f'charged target phases differ: {label}, {i}')
            online.append(duration)
        mean = sum(online) / 16
        if abs(mean - analysis['mean_online_ms_diagnostic']) > 1e-9:
            raise ValueError(f'analysis mean differs: {label}')
        arms[label] = {'status': 'PASS_INDEPENDENT_REPLAY', 'k': k,
                       'profile': profile, 'base_points': len(base),
                       'verified_points': 16, 'mean_online_ms_diagnostic': mean,
                       'analysis_sha256': sha((folder / 'analysis.json').read_bytes())}
    result = {'schema': 'n83.highk-stored-replay/v1',
              'status': 'PASS_INDEPENDENT_REPLAY',
              'source_sha256': source_sha,
              'fixture_sha256': fixture_sha,
              'base_receipts': base_receipts,
              'arms': arms,
              'selected_online_candidate_id': None}
    if args.output:
        args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + '\n')
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
