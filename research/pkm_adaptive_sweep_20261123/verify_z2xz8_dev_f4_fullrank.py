#!/usr/bin/env python3
"""Independent point, identity, and raw-receipt replay of F4 full-rank runs."""

import hashlib
import json
from pathlib import Path

from freeze_z2xz8_holdout import multiply

HERE = Path(__file__).resolve().parent
RESULT = HERE / 'RESULT_Z2XZ8_DEV_F4_FULLRANK_20261128.json'
RESULT_SHA = 'c0b9118e4b3d1560afc0ef034b1b8191fb9e06c7802353f7e20015fae72107d5'
RUNNER = HERE / 'run_z2xz8_dev_f4_fullrank.py'
PROTOCOL = HERE / 'PROTOCOL_Z2XZ8_DEV_F4_FULLRANK_20261128.md'
RAW = Path('/Volumes/SSD990-2/pkm-z2xz8-dev-f4-fullrank-20261128')
OUT = HERE / 'VERIFY_Z2XZ8_DEV_F4_FULLRANK_20261128.json'
BINARY = Path('/Volumes/SSD990-2/pkm-oracle-parity-build-20261128/release/ic')
BINARY_SHA = 'efe776236bea16b4f0bb31cbdaa2c8e83216b5914a337b4054a8f317106b9bf8'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def point(value):
    return int(value['x'], 16), int(value['y'], 16)


def carriers(p, a, b):
    return sum((lambda y: y == 0 or pow(y, (p - 1) // 2, p) == 1)((x*x*x+a*x+b) % p)
               for x in range(32))


def main():
    if sha(RESULT.read_bytes()) != RESULT_SHA or sha(BINARY.read_bytes()) != BINARY_SHA:
        raise ValueError('pinned result or binary differs')
    aggregate = json.loads(RESULT.read_bytes())
    receipts_path = RAW / 'receipts.jsonl'
    if sha(receipts_path.read_bytes()) != aggregate['receipts_sha256']:
        raise ValueError('receipt store differs')
    receipts = {}
    for line in receipts_path.read_text().splitlines():
        receipt = json.loads(line)
        if receipt['id'] in receipts:
            raise ValueError('duplicate receipt id')
        receipts[receipt['id']] = receipt
    header = receipts['header']
    if (len(receipts) != 13 or header['schema'] != 'pkm-z2xz8-dev-f4-fullrank/v1'
            or header['runner_sha256'] != sha(RUNNER.read_bytes())
            or header['protocol_sha256'] != sha(PROTOCOL.read_bytes())
            or header['binary_sha256'] != BINARY_SHA
            or header['source_commit'] != aggregate['source_commit']
            or header['cases'] != 6 or header['max_trials'] != 20_000):
        raise ValueError('header identity differs')
    replay = []
    summary = {(r['curve'], r['arm']): r for r in aggregate['rows']}
    if len(summary) != 6:
        raise ValueError('aggregate coverage differs')
    for curve in ('t5_c0', 't5_c1', 't5_c2'):
        for arm in ('candidate_a', 'interval_a'):
            case = f'{curve}:{arm}'
            launch, saved = receipts['launch:' + case], receipts['result:' + case]
            stdout = Path(launch['stdout_path']).read_bytes()
            stderr = Path(launch['stderr_path']).read_bytes()
            if (sha(stdout) != saved['stdout']['sha256']
                    or sha(stderr) != saved['stderr']['sha256']
                    or saved['exit_code'] != 0 or saved['cap_hit'] is not None):
                raise ValueError(f'raw or cap differs: {case}')
            command = launch['command']
            expected_base = ('prime-interval:t=5,lo=0' if arm == 'interval_a' else
                             'prime-isogeny:t=5,seed=0,aux=ordinary,family=z2xz8,selection=equal-size')
            workload_path = Path(command[command.index('--prime-workload') + 1])
            workload_raw = workload_path.read_bytes()
            workload = json.loads(workload_raw)
            if (command[0] != str(BINARY) or command[command.index('--factor-base') + 1] != expected_base
                    or command[command.index('--max-trials') + 1] != '20000'
                    or command[command.index('--rho-runs') + 1] != '1'
                    or command[command.index('--linalg') + 1] != 'incremental-gauss-full-rank-checked'
                    or sha(workload_raw) != launch['input_sha256']
                    or len(workload['targets']) != 1):
                raise ValueError(f'command or input differs: {case}')
            report = json.loads(stdout)
            target = workload['targets'][0]
            scalar = target['scalar']
            c = workload['curve']
            p, a, b = int(c['field']['modulus'], 16), int(c['a'], 16), int(c['b'], 16)
            if (report['operation'] != 'bench' or report['status'] != 'complete'
                    or report['prime_workload_sha256'] != launch['input_sha256']
                    or report['frozen_target_scalars'] != [scalar]
                    or len(report['rows']) != 1 or scalar != launch['target_scalar']):
                raise ValueError(f'workload identity differs: {case}')
            row = report['rows'][0]
            cols = row['factor_base']['columns']
            rank = row['linear_algebra']['rank']
            expected_cols = carriers(p, a, b)
            if (cols != expected_cols or row['factor_base']['abscissae'] != expected_cols
                    or rank != cols + 1 or row['recovered'] != scalar or not row['verified']
                    or multiply(p, a, scalar, point(c['generator'])) != point(target['point'])):
                raise ValueError(f'point, base size, or rank differs: {case}')
            rho = report['rho_reference']
            rho_runs = rho['per_run']
            if (rho['runs'] != 1 or len(rho_runs) != 1 or not rho['all_verified']
                    or rho_runs[0]['planted'] != scalar or not rho_runs[0]['verified']):
                raise ValueError(f'matched rho differs: {case}')
            trials = row['decomposition']['targets_tried']
            relations = row['decomposition']['relations_found']
            solver = row['decomposition']['solver']
            saved_summary = summary[(curve, arm)]
            if (trials > 20_000 or saved_summary['stdout_sha256'] != sha(stdout)
                    or saved_summary['trials'] != trials
                    or saved_summary['relations'] != relations
                    or saved_summary['solver_calls'] != solver['calls']
                    or saved_summary['solver_ops'] != solver['ops']
                    or saved_summary['rank'] != rank
                    or saved_summary['recovered'] != scalar):
                raise ValueError(f'aggregate differs: {case}')
            replay.append({'curve': curve, 'arm': arm, 'scalar': scalar,
                           'columns': cols, 'rank': rank, 'trials': trials,
                           'relations': relations, 'solver_calls': solver['calls'],
                           'solver_ops': solver['ops'], 'rho_steps': rho_runs[0]['steps'],
                           'reported_gae_lower_bound': row['total_gae'],
                           'stdout_sha256': sha(stdout)})
    pairs = []
    for curve in ('t5_c0', 't5_c1', 't5_c2'):
        candidate, interval = (next(r for r in replay if r['curve'] == curve and r['arm'] == arm)
                               for arm in ('candidate_a', 'interval_a'))
        if (candidate['scalar'] != interval['scalar']
                or candidate['columns'] != interval['columns']):
            raise ValueError(f'pair identity differs: {curve}')
        pairs.append({'curve': curve, 'candidate_over_interval_trials': candidate['trials']/interval['trials'],
                      'candidate_over_interval_solver_ops': candidate['solver_ops']/interval['solver_ops'],
                      'candidate_over_interval_reported_gae_lower_bound': candidate['reported_gae_lower_bound']/interval['reported_gae_lower_bound']})
    output = {'schema': 'pkm-z2xz8-dev-f4-fullrank-replay/v1',
              'result_sha256': RESULT_SHA, 'receipts_sha256': sha(receipts_path.read_bytes()),
              'runner_sha256': sha(RUNNER.read_bytes()),
              'all_six_point_rank_and_matched_rho_replays_verified': True,
              'complete_cold_cost': False,
              'reason': 'development curves and contended host; positive unpriced native work',
              'rows': replay, 'pairs': pairs}
    data = (json.dumps(output, sort_keys=True, indent=2) + '\n').encode()
    if OUT.exists() and OUT.read_bytes() != data:
        raise ValueError('existing replay differs')
    OUT.write_bytes(data)
    print(json.dumps({'rows': len(replay), 'sha256': sha(data), 'pairs': pairs}, sort_keys=True))


if __name__ == '__main__':
    main()
