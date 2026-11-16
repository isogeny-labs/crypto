#!/usr/bin/env python3
"""Seal a disjoint 4096-point public N83 diagnostic workload."""
import hashlib
import importlib.util
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = Path('/Volumes/SSD990/crypto/worktrees/n83-root4-frontier-20261018')
GROUP = ROOT / 'research/n83_indexed5_20261013/analyze.py'
DOMAIN = b'n83-highk-wide3-fresh4096-20261019-v1'
COUNT = 4096
PRIOR = [
    ROOT / 'research/n83_root4_frontier_current_20261018/fresh_targets.jsonl',
    ROOT / 'research/n83_root4_frontier_current_20261018/finalists_historical_targets.jsonl',
    Path('/private/tmp/n83_current_id30_frontier_20261019/one_pod/targets_stratum0.jsonl'),
    Path('/private/tmp/n83_current_id30_frontier_20261019/one_pod/targets_stratum1.jsonl'),
]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write_once(path, data):
    if path.exists() and path.read_bytes() != data:
        raise ValueError(f'frozen file differs: {path}')
    if not path.exists():
        path.write_bytes(data)


def points(path):
    raw = path.read_bytes()
    result = set()
    for line in raw.splitlines():
        row = json.loads(line)
        if not isinstance(row, list) or len(row) != 2:
            raise ValueError(f'invalid prior point row: {path}')
        result.add((int(row[0]), int(row[1])))
    return result, sha(raw)


def main():
    spec = importlib.util.spec_from_file_location('n83_wide3_group', GROUP)
    group = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(group)
    targets = []
    answers = []
    generated = set()
    for i in range(COUNT):
        seed = hashlib.sha256(DOMAIN + i.to_bytes(4, 'little')).digest()
        scalar = 1 + int.from_bytes(seed, 'little') % (group.R - 1)
        point = group.multiply(group.G, scalar)
        if (point is None or point in generated or not group.on_curve(point)
                or group.multiply(point, group.R) is not None):
            raise ValueError(f'public fixture point invalid or repeated: {i}')
        generated.add(point)
        targets.append(json.dumps([str(v) for v in point], separators=(',', ':')) + '\n')
        answers.append(json.dumps({'fixture_index': i, 'scalar': str(scalar),
                                   'seed_sha256': seed.hex()},
                                  separators=(',', ':'), sort_keys=True) + '\n')
    exclusions = []
    for path in PRIOR:
        prior, digest = points(path)
        count = len(generated & prior)
        exclusions.append({'path': str(path), 'sha256': digest,
                           'unique_points': len(prior), 'overlap_count': count})
        if count:
            raise ValueError(f'prior workload overlap: {path}: {count}')
    t = ''.join(targets).encode()
    a = ''.join(answers).encode()
    write_once(HERE / 'targets.jsonl', t)
    write_once(HERE / 'answers.jsonl', a)
    manifest = {'schema': 'n83.wide3-fresh4096-fixture/v1',
                'status': 'SEALED_UNMEASURED',
                'curve_id': 'EC1N83Ckb1a1haacb69ee9f37',
                'domain': DOMAIN.decode(), 'point_count': COUNT,
                'scalar_rule': '1+little_endian(SHA256(domain||u32_le(index))) mod (r-1)',
                'subgroup_order': group.R,
                'targets_sha256': sha(t), 'answers_sha256': sha(a),
                'generator_sha256': sha(Path(__file__).read_bytes()),
                'group_source_sha256': sha(GROUP.read_bytes()),
                'prior_exclusions': exclusions}
    write_once(HERE / 'fixture_manifest.json',
               (json.dumps(manifest, sort_keys=True, indent=2) + '\n').encode())
    print(json.dumps({'status': manifest['status'],
                      'point_count': COUNT,
                      'targets_sha256': manifest['targets_sha256'],
                      'prior_unique_points': sum(row['unique_points'] for row in exclusions)},
                     sort_keys=True))


if __name__ == '__main__':
    main()
