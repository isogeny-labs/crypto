#!/usr/bin/env python3
"""Extract the exact K600 Gray/hash signed-orbit domains for one frozen query."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
PLAN = (HERE.parent / 'koblitz_n83_factor_base_sweep_20261008/verification/'
        'k600-gray-hash-paired-protocol-20261014/plan.json')
PLAN_SHA256 = 'fbd3fb943bc0f181797d106c41ee9961b8d70768b5c65a5584002e3c1b5e488f'
ORBIT_SIZE = 166
K = 600


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_archive(path, expected_sha256):
    require(digest(path) == expected_sha256, f'{path.name}: compressed SHA-256 differs')
    points = []
    reps = []
    with gzip.open(path, 'rt', encoding='utf-8') as stream:
        header = json.loads(next(stream))
        require(header.get('schema') == 'n83.factor-base/v1' and
                header.get('point_count') == K * ORBIT_SIZE,
                f'{path.name}: archive header differs')
        for index, line in enumerate(stream):
            row = json.loads(line)
            point = tuple(map(int, row['point']))
            require(len(point) == 2 and all(0 <= c < 1 << 83 for c in point) and
                    row['column'] == index // ORBIT_SIZE and
                    row['phase'] == (index % ORBIT_SIZE) // 2 and
                    row['negative'] is (index % 2 == 1),
                    f'{path.name}: point or orbit labeling differs at {index}')
            if index % ORBIT_SIZE == 0:
                reps.append(point[0])
            points.append(point)
    require(len(points) == K * ORBIT_SIZE and len(reps) == K and
            len(set(points)) == len(points) and len(set(reps)) == K,
            f'{path.name}: archive cardinality differs')
    for offset in range(0, len(points), 2):
        x1, y1 = points[offset]
        x2, y2 = points[offset + 1]
        require(x1 == x2 and y1 ^ y2 == x1,
                f'{path.name}: signed pair differs at {offset}')
    return points, reps


def main(archives, output):
    require(digest(PLAN) == PLAN_SHA256, 'paired protocol plan bytes differ')
    plan = json.loads(PLAN.read_text())
    base_hash = plan['arms'][0]['base_object_blake3']
    base_sha = plan['arms'][0]['base_compressed_sha256']
    base_points, base_reps = read_archive(archives / f'{base_hash}.jsonl.gz', base_sha)
    base_set = set(base_points)
    base_x = {point[0] for point in base_points}
    output.mkdir(parents=True, exist_ok=False)
    for arm in plan['arms']:
        require(arm['base_object_blake3'] == base_hash and
                arm['base_compressed_sha256'] == base_sha,
                'paired arms use different small domains')
        pool_hash = arm['pool_object_blake3']
        pool_sha = arm['pool_compressed_sha256']
        pool_points, pool_reps = read_archive(archives / f'{pool_hash}.jsonl.gz', pool_sha)
        residual_reps = []
        shared = 0
        residual_x = set()
        for column, rep in enumerate(pool_reps):
            orbit = pool_points[column * ORBIT_SIZE:(column + 1) * ORBIT_SIZE]
            overlap = sum(point in base_set for point in orbit)
            require(overlap in (0, ORBIT_SIZE), 'partial signed-orbit overlap')
            if overlap:
                shared += 1
            else:
                xs = {point[0] for point in orbit}
                require(not (xs & base_x) and not (xs & residual_x),
                        'residual x-domain overlaps an earlier domain')
                residual_x.update(xs)
                residual_reps.append(rep)
        require(shared == arm['shared_signed_orbits'] and
                len(residual_reps) == arm['residual_signed_orbits'],
                f"{arm['role']}: residual cardinality differs")
        record = {
            'schema': 'n83.k600-natural-xcnf-input/v1',
            'role': arm['role'],
            'plan_sha256': PLAN_SHA256,
            'base_object_blake3': base_hash,
            'base_compressed_sha256': base_sha,
            'pool_object_blake3': pool_hash,
            'pool_compressed_sha256': pool_sha,
            'shared_signed_orbits': shared,
            'trial': 0,
            'small_reps': [str(value) for value in base_reps],
            'residual_reps': [str(value) for value in residual_reps],
        }
        path = output / f"{arm['role']}.json"
        path.write_text(json.dumps(record, sort_keys=True, separators=(',', ':')) + '\n')
        print(json.dumps({'role': arm['role'], 'status': 'PASS_EXACT_K600_DOMAINS',
                          'small_orbits': K, 'residual_orbits': len(residual_reps),
                          'shared_orbits': shared, 'input_sha256': digest(path)},
                         sort_keys=True))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('archives', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    main(args.archives, args.output)
