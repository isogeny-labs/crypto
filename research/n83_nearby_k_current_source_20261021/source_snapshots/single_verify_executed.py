#!/usr/bin/env python3
"""Independently replay the three public-point nearby-K diagnostics."""
import argparse
import gzip
import hashlib
import json
import math
from pathlib import Path

from research.n83_highk_beyond4405_20261021 import run_k1107_base_stage as base
from research.n83_indexed5_20261013 import analyze as group
from research.n83_online_finalists_20261018.run import ic_environment

ROOT = Path(__file__).resolve().parent
PRIOR = base.HERE / 'k1107_current_source_base/base_dump.jsonl.gz'
PHASES = ('target_query_ms', 'target_pdp_ms', 'target_relation_check_ms',
          'target_descent_ms', 'target_recovery_check_ms')
PRIOR_SHA256 = '08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def replay_new_columns(dump, prior, k):
    """Check every added orbit against the independently replayed prefix."""
    coords = [group.point_from_json(value) for value in
              dump['factor_base_point_coordinates']]
    labels = dump['factor_base_point_labels']
    reps = [group.point_from_json(value) for value in
            dump['factor_base_representatives']]
    if len(coords) != 166*k or len(labels) != len(coords) or len(reps) != k:
        return False
    seen = {group.point_from_json(value) for value in
            prior['factor_base_point_coordinates']}
    if len(seen) != len(prior['factor_base_point_coordinates']):
        return False
    for column in range(len(prior['factor_base_representatives']), k):
        rep = reps[column]
        if rep is None or not group.on_curve(rep) or group.multiply(rep, group.R) is not None:
            return False
        current, coefficient = rep, 1
        for rotation in range(group.N):
            positive = 166*column + 2*rotation
            negative = positive + 1
            opposite = (current[0], current[0] ^ current[1])
            if (coords[positive] != current or coords[negative] != opposite or
                    labels[positive] != [column, coefficient] or
                    labels[negative] != [column, (group.R-coefficient) % group.R]):
                return False
            for point in (current, opposite):
                if point in seen:
                    return False
                seen.add(point)
            current = (group.product(current[0], current[0]),
                       group.product(current[1], current[1]))
            coefficient = coefficient*group.LAMBDA % group.R
        if current != rep or coefficient != 1:
            return False
    return len(seen) == len(coords)


def verify(k):
    output = ROOT / f'k{k}'
    if not (output / 'receipt.json').exists():
        return {'schema': 'n83.nearby-k-current-source-replay/v1',
                'status': 'BOUNDED_UNKNOWN', 'k': k, 'reason': 'NO_RECEIPT'}
    launch = json.loads((output / 'launch.json').read_text())
    receipt = json.loads((output / 'receipt.json').read_text())
    if receipt['status'] != 'PROCESS_EXITED' or receipt['exit_code'] != 0:
        return {'schema': 'n83.nearby-k-current-source-replay/v1',
                'status': 'BOUNDED_UNKNOWN', 'k': k,
                'reason': receipt['status']}
    summary = json.loads((output / 'stdout.jsonl').read_text())
    row = json.loads((output / 'targets_result.jsonl').read_text())
    dump = json.loads((output / 'base_dump.jsonl').read_text())
    target = json.loads((output / 'target.jsonl').read_text())
    answer = json.loads((base.ROOT / 'research/n83_cold_factor_base_20261016/answers.jsonl').read_text().splitlines()[0])
    prior_blob = gzip.open(PRIOR, 'rb').read()
    prior = json.loads(prior_blob)
    prior_replay = json.loads((base.HERE /
        'k1107_current_source_base/independent_replay.json').read_text())
    certified_prefix = (hashlib.sha256(prior_blob).hexdigest() == PRIOR_SHA256 and
                        prior_replay['status'] == 'PASS_INDEPENDENT_BASE_REPLAY' and
                        prior_replay['base_dump_sha256'] == PRIOR_SHA256)
    expected_flags = {name: value for name, value in
                      ic_environment(output, 'root11_dynamic', 'k1107_root11').items()
                      if name.startswith('KIC_')}
    checks = {
        'source_and_binary': launch['source_sha256'] == base.SOURCE_SHA256 ==
            base.sha256(base.SOURCE) and launch['binary_sha256'] ==
            digest(Path(launch['command'][0])),
        'launch_and_flags': launch['k'] == k and launch['command'][1] ==
            f'construct:83:1:{k}' and launch['flags'] == expected_flags and
            launch['target_sha256'] == digest(output / 'target.jsonl'),
        'receipt_hashes': receipt['stdout_sha256'] == digest(output / 'stdout.jsonl') and
            receipt['stderr_sha256'] == digest(output / 'stderr.log') and
            receipt['base_dump_sha256'] == digest(output / 'base_dump.jsonl') and
            receipt['target_result_sha256'] == digest(output / 'targets_result.jsonl'),
        'base_shape': summary['n'] == 83 and summary['a'] == 1 and
            summary['orbit_columns'] == k and summary['factor_base_points'] == 166*k and
            len(dump['factor_base_point_coordinates']) == 166*k and
            dump['base_hash'] == summary['base_hash'],
        'rank_and_target_count': summary['rank'] == k and summary['targets'] == 1 and
            summary['targets_solved'] == 1 and summary['targets_failed'] == 0,
        'base_prefix': all(dump[name][:min(len(dump[name]), len(prior[name]))] ==
            prior[name][:min(len(dump[name]), len(prior[name]))]
            for name in ('factor_base_point_coordinates', 'factor_base_point_labels',
                         'factor_base_representatives')),
        'scalar': target == answer['point'] and
            int(row['recovered_scalar']) == int(answer['scalar']) and
            group.multiply(group.G, int(row['recovered_scalar'])) ==
            group.point_from_json(target) and row['group_verified'] is True,
        'phase_accounting': all(isinstance(row[name], (int, float)) and
            math.isfinite(row[name]) and row[name] >= 0 for name in PHASES) and
            abs(math.fsum(row[name] for name in PHASES) - row['target_ms']) <= 1,
    }
    relation = None
    for index in row['point_indices']:
        if not isinstance(index, int) or not 0 <= index < 166*k:
            checks['four_point_relation'] = False
            break
        relation = group.add(relation, group.point_from_json(
            dump['factor_base_point_coordinates'][index]))
    else:
        checks['four_point_relation'] = (
            len(row['point_indices']) == 4 and
            relation == group.point_from_json(target))
    if k > 1107:
        checks['new_base_suffix_replay'] = certified_prefix and \
            replay_new_columns(dump, prior, k)
    else:
        checks['prior_base_replay'] = certified_prefix
    timing = summary['timing_ms']
    parts = (timing['setup_total'], timing['rank_stage'], timing['targets_total'])
    checks['cold_accounting'] = (
        all(math.isfinite(part) and part >= 0 for part in parts) and
        math.fsum(parts) <= timing['process_total'] + 1 and
        abs(timing['targets_total'] - row['target_ms']) <= 2)
    result = {'schema': 'n83.nearby-k-current-source-replay/v1',
              'status': 'PASS_EXPLORATORY_ONE_TARGET_REPLAY' if all(checks.values())
                        else 'VERIFICATION_FAILED',
              'tier': 'shared-host correctness and stage diagnostic',
              'k': k, 'checks': checks, 'source_sha256': launch['source_sha256'],
              'binary_sha256': launch['binary_sha256'],
              'base_dump_sha256': receipt['base_dump_sha256'],
              'prior_k1107_base_dump_sha256': hashlib.sha256(prior_blob).hexdigest(),
              'usable_points': summary['factor_base_points'],
              'folded_columns': summary['orbit_columns'],
              'one_target_online_ms': row['target_ms'],
              'cold_process_ms': timing['process_total'],
              'process_wall_seconds': receipt['wall_seconds']}
    (output / 'independent_replay.json').write_text(json.dumps(result, indent=2,
                                                           sort_keys=True) + '\n')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('k', type=int)
    args = parser.parse_args()
    result = verify(args.k)
    print(json.dumps({'k': args.k, 'status': result['status'],
                      'checks': result.get('checks')}, sort_keys=True))
    raise SystemExit(0 if result['status'] == 'PASS_EXPLORATORY_ONE_TARGET_REPLAY' else 2)
