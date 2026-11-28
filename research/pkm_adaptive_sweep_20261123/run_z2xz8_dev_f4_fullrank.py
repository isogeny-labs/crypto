#!/usr/bin/env python3
"""Resumable full-rank F4 selected-map versus interval development panel."""

import argparse
import datetime as dt
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

import run_z2xz8_holdout_cold_v2 as cold

HERE = Path(__file__).resolve().parent
SOURCE_PANEL = HERE / 'RESULT_Z2XZ8_JOIN_BUILDERS_20261128.json'
SOURCE_PANEL_SHA = 'f6ea41b675302fc64a618c59c51fc771b54c6d43eca216d36c5b4c9faf3cbbba'
ROOT = Path('/Volumes/SSD990-2/pkm-z2xz8-dev-f4-fullrank-20261128')
OUT = HERE / 'RESULT_Z2XZ8_DEV_F4_FULLRANK_20261128.json'
TRIALS = 20_000
PROTOCOL_SHA = '34b7202c689eea8aefea2bebde2bde92d83f5189dcb8f31e430884371f1396d7'
PRIOR_RESULT_SHA = '4437722a94f0a4954175107e0eebef5027248dc19a09f9be72788fc913e6872c'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def append(helper, path, row):
    helper.append(path, row)


def run_diagnostic(command, stdout_path, stderr_path, helper):
    """Enforce resource caps while retaining host-contended diagnostic output."""
    started = time.perf_counter()
    pre = helper.quiet()
    peak_sampled = 0
    cap_hit = None
    with stdout_path.open('xb') as out, stderr_path.open('xb') as err:
        proc = subprocess.Popen(command, stdout=out, stderr=err, start_new_session=True)
        while True:
            child, status, usage = os.wait4(proc.pid, os.WNOHANG)
            if child:
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            peak_sampled = max(peak_sampled, helper.rss_kib(proc.pid))
            elapsed = time.perf_counter() - started
            if peak_sampled > helper.RSS_CAP_KIB:
                cap_hit = 'memory_cap'
            elif elapsed > helper.WALL_CAP_S:
                cap_hit = 'wall_cap'
            if cap_hit:
                try:
                    os.killpg(proc.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                _, status, usage = os.wait4(proc.pid, 0)
                proc.returncode = os.waitstatus_to_exitcode(status)
                break
            time.sleep(0.25)
    stdout, stderr = stdout_path.read_bytes(), stderr_path.read_bytes()
    os_peak = usage.ru_maxrss // 1024 if sys.platform == 'darwin' else usage.ru_maxrss
    peak = max(peak_sampled, os_peak)
    if peak > helper.RSS_CAP_KIB:
        cap_hit = 'memory_cap'
    post = helper.quiet()
    return {'exit_code': proc.returncode, 'wall_s_diagnostic': time.perf_counter() - started,
            'process_cpu_s': usage.ru_utime + usage.ru_stime, 'peak_rss_kib': peak,
            'cap_hit': cap_hit, 'precall_load': pre, 'postcall_load': post,
            'host_contended': bool(pre['competing_processes'] or post['competing_processes']
                                   or pre['loadavg'][0] > pre['logical_cpus']
                                   or post['loadavg'][0] > post['logical_cpus']),
            'stdout': {'path': str(stdout_path), 'sha256': sha(stdout), 'bytes': len(stdout)},
            'stderr': {'path': str(stderr_path), 'sha256': sha(stderr), 'bytes': len(stderr)}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--dry-run', action='store_true')
    parser.add_argument('--max-new', type=int, default=0)
    args = parser.parse_args()
    if args.max_new < 0:
        parser.error('--max-new must be nonnegative')
    if (sha((HERE / 'PROTOCOL_Z2XZ8_DEV_F4_FULLRANK_20261128.md').read_bytes()) != PROTOCOL_SHA
            or sha((HERE / 'RESULT_Z2XZ8_DEV_F4_EQUAL_SIZE_20261128.json').read_bytes()) != PRIOR_RESULT_SHA):
        raise ValueError('full-rank protocol or prior result changed')
    if sha(SOURCE_PANEL.read_bytes()) != SOURCE_PANEL_SHA:
        raise ValueError('development inputs changed')
    panel = read(SOURCE_PANEL)
    checkout = Path('/Volumes/SSD990-2/pkm-oracle-parity-full-20261128')
    binary = Path('/Volumes/SSD990-2/pkm-oracle-parity-build-20261128/release/ic')
    build = Path('/Volumes/SSD990-2/pkm-oracle-parity-build-20261128/BUILD.json')
    if (subprocess.check_output(['git', '-C', str(checkout), 'rev-parse', 'HEAD'], text=True).strip() != cold.SOURCE
            or subprocess.check_output(['git', '-C', str(checkout), 'status', '--porcelain=v1', '--untracked-files=no'], text=True).strip()
            or subprocess.check_output(['git', '-C', str(checkout), 'config', '--get', 'core.sparseCheckout'], text=True).strip() != 'false'
            or sha(binary.read_bytes()) != cold.BINARY_SHA
            or sha(build.read_bytes()) != cold.BUILD_SHA):
        raise ValueError('source or binary identity changed')
    helper, helper_sha = cold.load_helper(checkout)
    seeds = {}
    inputs = {}
    for row in panel['rows']:
        curve = row['curve']
        command = row['command']
        seed = int(command[command.index('--seed') + 1])
        path = Path(command[command.index('--prime-workload') + 1])
        if sha(path.read_bytes()) != row['workload_sha256']:
            raise ValueError(f'development workload changed: {curve}')
        if curve in seeds and (seed != seeds[curve] or path != inputs[curve]):
            raise ValueError(f'development seed or input differs: {curve}')
        seeds[curve], inputs[curve] = seed, path
    if set(seeds) != {'t5_c0', 't5_c1', 't5_c2'}:
        raise ValueError('expected three development curves')
    cases = [(curve, arm) for curve in sorted(seeds) for arm in ('candidate_a', 'interval_a')]
    if args.dry_run:
        print(json.dumps({'schema': 'pkm-z2xz8-dev-f4-fullrank-plan/v1',
                          'source': cold.SOURCE, 'binary_sha256': cold.BINARY_SHA,
                          'source_panel_sha256': SOURCE_PANEL_SHA,
                          'protocol_sha256': PROTOCOL_SHA,
                          'interval_factor_base': 'prime-interval:t=5,lo=0',
                          'cases': cases, 'max_trials': TRIALS}, sort_keys=True))
        return
    ROOT.mkdir(parents=True, exist_ok=True)
    store = ROOT / 'receipts.jsonl'
    prior = helper.records(store)
    header = {'id': 'header', 'schema': 'pkm-z2xz8-dev-f4-fullrank/v1',
              'source_commit': cold.SOURCE, 'binary_sha256': cold.BINARY_SHA,
              'build_receipt_sha256': cold.BUILD_SHA,
              'source_panel_sha256': SOURCE_PANEL_SHA,
              'protocol_sha256': PROTOCOL_SHA,
              'prior_64_trial_result_sha256': PRIOR_RESULT_SHA,
              'interval_factor_base': 'prime-interval:t=5,lo=0',
              'runner_sha256': sha(Path(__file__).read_bytes()),
              'helper_sha256': helper_sha, 'max_trials': TRIALS,
              'wall_cap_s': helper.WALL_CAP_S, 'rss_cap_kib': helper.RSS_CAP_KIB,
              'cases': len(cases), 'qualification': 'development-only, host-contended; full native prices unavailable',
              'created_utc': dt.datetime.now(dt.timezone.utc).isoformat()}
    if not prior:
        append(helper, store, header)
        prior['header'] = header
    elif {k: v for k, v in prior['header'].items() if k != 'created_utc'} != {k: v for k, v in header.items() if k != 'created_utc'}:
        raise ValueError('receipt header differs')
    new = 0
    for curve, arm in cases:
        case = f'{curve}:{arm}'
        if 'result:' + case in prior:
            continue
        if 'launch:' + case in prior:
            raise ValueError(f'unresolved launch: {case}')
        if args.max_new and new >= args.max_new:
            return
        command = cold.arm_command(binary, inputs[curve], arm, seeds[curve])
        if arm == 'interval_a':
            command[command.index('--factor-base') + 1] = 'prime-interval:t=5,lo=0'
        command[command.index('--max-trials') + 1] = str(TRIALS)
        raw = ROOT / 'raw'
        raw.mkdir(exist_ok=True)
        stdout = raw / f'{case.replace(":", "_")}.stdout.json'
        stderr = raw / f'{case.replace(":", "_")}.stderr.txt'
        input_sha = sha(inputs[curve].read_bytes())
        scalar = read(inputs[curve])['targets'][0]['scalar']
        launch = {'id': 'launch:' + case, 'command': command,
                  'input_sha256': input_sha, 'target_scalar': scalar,
                  'stdout_path': str(stdout), 'stderr_path': str(stderr)}
        append(helper, store, launch)
        prior[launch['id']] = launch
        result = run_diagnostic(command, stdout, stderr, helper)
        result['id'] = 'result:' + case
        result['summary'] = cold.summary(stdout, input_sha, scalar,
                                         int(read(inputs[curve])['curve']['subgroup_order'], 16),
                                         arm, helper)
        append(helper, store, result)
        prior[result['id']] = result
        new += 1
        print(json.dumps({'case': case, 'cap': result['cap_hit'],
                          'exit_code': result['exit_code'],
                          'status': result['summary'].get('status'),
                          'relations': result['summary'].get('relations_found')}, sort_keys=True), flush=True)
    rows = []
    for curve, arm in cases:
        case = f'{curve}:{arm}'
        launch = prior['launch:' + case]
        result = prior['result:' + case]
        stdout = Path(launch['stdout_path'])
        if sha(stdout.read_bytes()) != result['stdout']['sha256']:
            raise ValueError(f'raw hash differs: {case}')
        if result['cap_hit'] or not stdout.stat().st_size:
            rows.append({'curve': curve, 'arm': arm, 'cap_hit': result['cap_hit'],
                         'exit_code': result['exit_code'], 'parsed': False,
                         'stdout_sha256': result['stdout']['sha256']})
            continue
        report = read(stdout)
        if (report.get('prime_workload_sha256') != launch['input_sha256']
                or report.get('frozen_target_scalars') != [launch['target_scalar']]
                or report.get('configurations_run') != 1
                or len(report.get('rows') or []) != 1):
            raise ValueError(f'output identity differs: {case}')
        row = report['rows'][0]
        rows.append({'curve': curve, 'arm': arm, 'status': report['status'],
                     'parsed': True, 'host_contended': result['host_contended'],
                     'cap_hit': result['cap_hit'], 'exit_code': result['exit_code'],
                     'planted': launch['target_scalar'],
                     'recovered': row['recovered'], 'verified': row['verified'],
                     'full_rank': row['linear_algebra']['rank'] == row['factor_base']['columns'] + 1,
                     'rho_verified': (report.get('rho_reference') or {}).get('all_verified'),
                     'rho_recovered': ((report.get('rho_reference') or {}).get('per_run') or [{}])[0].get('recovered'),
                     'base_abscissae': row['factor_base']['abscissae'],
                     'trials': row['decomposition']['cost']['native']['trials'],
                     'relations': row['decomposition']['relations_found'],
                     'solver_calls': row['decomposition']['solver']['calls'],
                     'solver_ops': row['decomposition']['solver']['ops'],
                     'rank': row['linear_algebra']['rank'],
                     'reported_gae_lower_bound': row['total_gae'],
                     'unpriced_native': cold.positive_unpriced(report),
                     'stdout_sha256': result['stdout']['sha256']})
    for curve in sorted(seeds):
        matching = [r for r in rows if r['curve'] == curve and r.get('parsed')]
        if len(matching) == 2 and matching[0]['base_abscissae'] != matching[1]['base_abscissae']:
            raise ValueError(f'equal-size control differs on {curve}')
    output = {'schema': 'pkm-z2xz8-dev-f4-fullrank-result/v1',
              'source_commit': cold.SOURCE, 'binary_sha256': cold.BINARY_SHA,
              'receipts_sha256': sha(store.read_bytes()),
              'protocol_sha256': PROTOCOL_SHA,
              'interval_factor_base': 'prime-interval:t=5,lo=0',
              'qualification': 'Development-only; host-contended; 120s, 2GiB and 20,000-trial censors; positive unpriced native work; no complete cost claim.',
              'rows': rows}
    data = (json.dumps(output, sort_keys=True, indent=2) + '\n').encode()
    if OUT.exists() and OUT.read_bytes() != data:
        raise ValueError('result file differs')
    OUT.write_bytes(data)
    print(json.dumps({'rows': len(rows), 'result_sha256': sha(data)}, sort_keys=True))


if __name__ == '__main__':
    main()
