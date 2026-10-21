#!/usr/bin/env python3
"""Run a frozen two-point IC/rho comparison after K4549 capacity admission."""

import argparse
import datetime as dt
import hashlib
import importlib.util
import json
import os
import signal
import subprocess
import time
from pathlib import Path

DIAG_RUNNER = Path('/workspace/load80_diag_input/runpod_diag.py')
SCREEN = Path('/workspace/n83_tiebreak')
SOURCE = Path('/workspace/load80_crypto')
RHO_CAP = 120
IC_CAP = 1200
GLOBAL_CAP = 65 * 60
ARCHIVE_RESERVE = 15 * 60

spec = importlib.util.spec_from_file_location('n83_load80_diag', DIAG_RUNNER)
diag = importlib.util.module_from_spec(spec)
spec.loader.exec_module(diag)


def digest(path):
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def now():
    return dt.datetime.now(dt.timezone.utc)


def run_rho(event, root, binary, config, protocol):
    folder = root / 'runs' / event['name']
    folder.mkdir(parents=True, exist_ok=False)
    target = (root / 'calibration.jsonl' if event['kind'] == 'calibration'
              else root / 'targets' / f"p{event['point_index']:02d}.jsonl")
    command = [str(binary), str(target), str(event['seed']),
               str(config), str(folder / 'receipt.json')]
    write(folder / 'input.json', {
        'schema': 'n83.load80-online-pair-rho-input/v1',
        'event': event, 'command': command, 'binary_sha256': digest(binary),
        'config_sha256': digest(config), 'target_sha256': digest(target),
        'cap_seconds': RHO_CAP,
    })
    started = time.monotonic()
    before = diag.snapshot()
    peak_sample = 0
    stop_reason = None
    with (folder / 'stdout.jsonl').open('wb') as stdout, \
         (folder / 'stderr.log').open('wb') as stderr:
        child = subprocess.Popen(command, cwd=SOURCE, stdout=stdout,
                                 stderr=stderr, start_new_session=True)
        while True:
            pid, status_word, usage = os.wait4(child.pid, os.WNOHANG)
            if pid == child.pid:
                code = os.waitstatus_to_exitcode(status_word)
                peak = max(peak_sample, usage.ru_maxrss * 1024)
                break
            status_path = Path('/proc') / str(child.pid) / 'status'
            if status_path.exists():
                for line in status_path.read_text().splitlines():
                    if line.startswith('VmRSS:'):
                        peak_sample = max(peak_sample, int(line.split()[1]) * 1024)
                        break
            if time.monotonic() - started >= RHO_CAP:
                stop_reason = 'TIMEOUT_120S'
                try:
                    os.killpg(child.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
            if stop_reason:
                time.sleep(2)
                pid, status_word, usage = os.wait4(child.pid, os.WNOHANG)
                if pid == 0:
                    try:
                        os.killpg(child.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    pid, status_word, usage = os.wait4(child.pid, 0)
                code = os.waitstatus_to_exitcode(status_word)
                peak = max(peak_sample, usage.ru_maxrss * 1024)
                break
            time.sleep(0.5)
    child.returncode = code
    status = stop_reason or ('PASS_PROCESS' if code == 0 else 'FAILED_PROCESS')
    record = {
        'schema': 'n83.load80-online-pair-rho-usage/v1',
        'name': event['name'], 'status': status, 'exit_code': code,
        'wall_seconds': time.monotonic() - started,
        'peak_rss_bytes': peak,
        'cgroup_before': before, 'cgroup_after': diag.snapshot(),
        'finished_utc': now().isoformat(),
    }
    diag.resource_gate(record)
    write(folder / 'usage.json', record)
    return record


def verify_event(event, root, record, protocol):
    if record['resource_status'] != 'PASS':
        return 'RESOURCE_OR_PROCESS_FAILURE'
    folder = root / 'runs' / event['name']
    if event['kind'] == 'ic':
        summary = json.loads((folder / 'stdout.jsonl').read_text())
        rows = [json.loads(line) for line in
                (folder / 'targets_result.jsonl').read_text().splitlines()]
        if (len(rows) != 1 or summary.get('orbit_columns') != event['k'] or
                summary.get('rank') != event['k'] or
                summary.get('factor_base_points') != 166 * event['k'] or
                summary.get('root_table_allocated_bytes') != 10 * 1024**3 or
                summary.get('indexed5_load80_enabled') is not True or
                not rows[0].get('group_verified') or
                int(rows[0]['recovered_scalar']) !=
                int(protocol['points'][event['point_index']]['scalar'])):
            return 'IC_OUTPUT_MISMATCH'
        return 'PASS_REPORTED_OUTPUT'
    receipt = json.loads((folder / 'receipt.json').read_text())
    if (receipt.get('status') != 'PASS_VERIFIED_PUBLIC_TARGET' or
            receipt.get('generic_scalar_replay_verified') is not True or
            receipt.get('group_verified') is not True or
            not isinstance(receipt.get('online_ms'), (int, float)) or
            receipt['online_ms'] <= 0):
        return 'RHO_OUTPUT_MISMATCH'
    if event['kind'] == 'rho' and int(receipt['recovered_scalar']) != int(
            protocol['points'][event['point_index']]['scalar']):
        return 'RHO_SCALAR_MISMATCH'
    return 'PASS_REPORTED_OUTPUT'


def schedule():
    events = []
    for point in range(2):
        for repetition in range(3):
            events.append({'name': f'p{point:02d}_cal_before_{repetition}',
                           'kind': 'calibration', 'point_index': point,
                           'seed': 2026101601})
        kinds = (('ic', 4405), ('ic', 4500), ('ic', 4549),
                 ('rho', None)) if point == 0 else (
            ('rho', None), ('ic', 4549), ('ic', 4500))
        for kind, k in kinds:
            name = f'p{point:02d}_{"k" + str(k) if k else "rho"}'
            events.append({'name': name, 'kind': kind, 'k': k,
                           'point_index': point, 'seed': 2026120500 + point})
        for repetition in range(3):
            events.append({'name': f'p{point:02d}_cal_after_{repetition}',
                           'kind': 'calibration', 'point_index': point,
                           'seed': 2026101601})
    return events


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--protocol', type=Path, required=True)
    parser.add_argument('--ic-binary', type=Path, required=True)
    parser.add_argument('--rho-binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_text())
    root = args.output.resolve()
    ic = args.ic_binary.resolve()
    rho = args.rho_binary.resolve()
    config = Path(protocol['rho_config_path'])
    calibration = Path(protocol['calibration_target_path'])
    capacity = SCREEN / 'load80_diag'
    if (protocol.get('schema') != 'n83.load80-online-pair/v1' or
            protocol.get('events') != schedule() or
            protocol.get('pod_id') != os.environ.get('RUNPOD_POD_ID') or
            sorted(os.sched_getaffinity(0)) != [0, 1, 2, 3] or
            digest(Path(__file__)) != protocol['runner_sha256'] or
            digest(DIAG_RUNNER) != protocol['diag_runner_sha256'] or
            subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=SOURCE,
                                    text=True).strip() !=
            protocol['rust_source_commit'] or
            digest(SOURCE / 'examples/koblitz_orbit_dlp_fast.rs') !=
            protocol['ic_source_sha256'] or
            digest(SOURCE / 'examples/n83_rho_optimization_eval.rs') !=
            protocol['rho_source_sha256'] or
            digest(config) != protocol['rho_config_sha256'] or
            digest(calibration) != protocol['calibration_target_sha256'] or
            not ic.is_file() or not rho.is_file() or root.exists()):
        raise ValueError('frozen source, host, schedule or output differs')
    summary_path = capacity / 'summary.json'
    if not summary_path.exists():
        raise ValueError('capacity panel has not completed')
    capacity_summary = json.loads(summary_path.read_text())
    if capacity_summary.get('status') != 'COMPLETE':
        raise ValueError('capacity panel was capped')
    for name in ('p00_k4405_load80', 'p00_k4500_load80',
                 'p00_k4549_load80'):
        folder = capacity / 'runs' / name
        usage = json.loads((folder / 'usage.json').read_text())
        row = json.loads((folder / 'targets_result.jsonl').read_text())
        source = json.loads((folder / 'stdout.jsonl').read_text())
        if (usage.get('resource_status') != 'PASS' or
                row.get('group_verified') is not True or
                source.get('root_table_allocated_bytes') != 10 * 1024**3):
            raise ValueError(f'capacity admission failed: {name}')
    deadline = dt.datetime.fromisoformat(
        protocol['provider_stop_utc'].replace('Z', '+00:00'))
    if now() + dt.timedelta(seconds=GLOBAL_CAP + ARCHIVE_RESERVE) > deadline:
        raise ValueError('remaining Pod time below frozen producer cap')
    host = diag.snapshot()
    quota, period = host['cpu.max'].split()
    if (quota == 'max' or int(quota) < 4 * int(period) or
            int(host['memory.max']) < diag.RSS_CAP + 256 * 1024**2):
        raise ValueError('Pod cgroup envelope below frozen requirement')
    root.mkdir(parents=True)
    (root / 'runs').mkdir()
    (root / 'targets').mkdir()
    for index, point in enumerate(protocol['points']):
        source = Path(point['source_path'])
        target = root / 'targets' / f'p{index:02d}.jsonl'
        target.write_bytes(source.read_bytes())
        if digest(target) != point['sha256']:
            raise ValueError('fresh point differs')
    (root / 'calibration.jsonl').write_bytes(calibration.read_bytes())
    write(root / 'source.json', {
        'schema': 'n83.load80-online-pair-source/v1',
        'protocol': protocol, 'protocol_sha256': digest(args.protocol),
        'ic_binary_sha256': digest(ic), 'rho_binary_sha256': digest(rho),
        'capacity_summary_sha256': digest(summary_path),
        'host_before': host, 'started_utc': now().isoformat(),
    })
    started = time.monotonic()
    completed = []
    for event in schedule():
        cap = IC_CAP if event['kind'] == 'ic' else RHO_CAP
        if (time.monotonic() - started + cap > GLOBAL_CAP or
                now() + dt.timedelta(seconds=cap + ARCHIVE_RESERVE) > deadline):
            write(root / 'producer_stop.json', {
                'status': 'GLOBAL_OR_PROVIDER_CAP_BEFORE_EVENT',
                'next_event': event})
            break
        if event['kind'] == 'ic':
            row = diag.run_child({
                'name': event['name'], 'k': event['k'], 'load': '80',
                'point_index': event['point_index'],
            }, root, ic, protocol)
        else:
            row = run_rho(event, root, rho, config, protocol)
        verified = verify_event(event, root, row, protocol)
        write(root / 'runs' / event['name'] / 'reported_verification.json',
              {'status': verified, 'event': event})
        completed.append({'event': event, 'process': row['status'],
                          'resource': row['resource_status'],
                          'reported_verification': verified})
        print(event['name'], row['status'], row['resource_status'], verified,
              flush=True)
        if verified != 'PASS_REPORTED_OUTPUT':
            break
    write(root / 'summary.json', {
        'schema': 'n83.load80-online-pair-summary/v1',
        'status': 'COMPLETE_REPORTED_OUTPUT' if len(completed) ==
        len(schedule()) else 'CAPPED_OR_FAILED',
        'events': completed, 'expected_events': len(schedule()),
        'host_after': diag.snapshot(), 'finished_utc': now().isoformat(),
    })


if __name__ == '__main__':
    main()
