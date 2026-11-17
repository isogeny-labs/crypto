#!/usr/bin/env python3
"""Run frozen N83 load80 capacity controls after the sealed tie-break screen."""

import argparse
import datetime as dt
import hashlib
import json
import os
import resource
import signal
import subprocess
import time
from pathlib import Path

CGROUP = Path('/sys/fs/cgroup')
SOURCE = Path('/workspace/load80_crypto')
SCREEN = Path('/workspace/n83_tiebreak')
RSS_CAP = 24 * 1024**3
RUN_CAP = 1200
GLOBAL_CAP = 7200
ARCHIVE_RESERVE = 900


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(chunk)
    return value.hexdigest()


def write(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def now():
    return dt.datetime.now(dt.timezone.utc)


def snapshot():
    names = ('cpu.stat', 'cpu.max', 'memory.current', 'memory.peak',
             'memory.max', 'memory.events', 'cpu.pressure', 'memory.pressure')
    result = {name: (CGROUP / name).read_text() for name in names}
    result['cgroup_interface'] = 'v2'
    return result


def counters(blob):
    return dict(line.split(maxsplit=1) for line in blob.splitlines())


def snapshot_ok(snapshot):
    try:
        quota, period = (int(v) for v in snapshot['cpu.max'].split())
        cpu = counters(snapshot['cpu.stat'])
        memory = counters(snapshot['memory.events'])
        values = [int(snapshot[key]) for key in
                  ('memory.current', 'memory.peak', 'memory.max')]
        values += [int(cpu[key]) for key in
                   ('usage_usec', 'nr_periods', 'nr_throttled')]
        values += [int(memory[key]) for key in ('oom', 'oom_kill')]
        return (snapshot['cgroup_interface'] == 'v2' and
                quota >= 4 * period > 0 and
                int(snapshot['memory.max']) >= RSS_CAP + 256 * 1024**2 and
                int(snapshot['memory.current']) <=
                int(snapshot['memory.peak']) <= int(snapshot['memory.max']) and
                all(value >= 0 for value in values))
    except (KeyError, TypeError, ValueError, AttributeError):
        return False


def resource_gate(record):
    before, after = record['cgroup_before'], record['cgroup_after']
    cpu_before, cpu_after = (counters(x['cpu.stat']) for x in (before, after))
    mem_before, mem_after = (counters(x['memory.events']) for x in (before, after))
    throttle = int(cpu_after['nr_throttled']) - int(cpu_before['nr_throttled'])
    oom = int(mem_after['oom']) - int(mem_before['oom'])
    kills = int(mem_after['oom_kill']) - int(mem_before['oom_kill'])
    record.update({'throttled_periods_delta': throttle, 'oom_events_delta': oom,
                   'oom_kills_delta': kills})
    record['resource_status'] = ('PASS' if
        record['status'] == 'PASS_PROCESS' and
        record['peak_rss_bytes'] <= RSS_CAP and
        snapshot_ok(before) and snapshot_ok(after) and
        (throttle, oom, kills) == (0, 0, 0) else 'FAILED_OR_INCOMPLETE')


def run_child(spec, root, binary, protocol):
    folder = root / 'runs' / spec['name']
    folder.mkdir(parents=True, exist_ok=False)
    point = root / 'targets' / f"p{spec['point_index']:02d}.jsonl"
    flags = dict(protocol['algorithm_flags'])
    flags.update({
        'KIC_INDEXED5_LOAD75': '1' if spec['load'] == '75' else '0',
        'KIC_INDEXED5_LOAD80': '1' if spec['load'] == '80' else '0',
        'KIC_DUMP_BASE': str(folder / 'base_dump.jsonl'),
    })
    command = [str(binary), f"construct:83:1:{spec['k']}", str(point), '29',
               str(folder / 'targets_result.jsonl')]
    write(folder / 'input.json', {
        'schema': 'n83.load80-runpod-child/v1', **spec,
        'command': command, 'environment_flags': flags,
        'source_head': protocol['rust_source_commit'],
        'source_sha256': protocol['rust_source_sha256'],
        'binary_sha256': digest(binary),
        'target_sha256': digest(point),
        'expected_scalar': protocol['points'][spec['point_index']]['scalar'],
        'cap_seconds': RUN_CAP, 'rss_cap_bytes': RSS_CAP,
    })
    env = {**os.environ, **flags, 'TMPDIR': '/workspace/load80_tmp'}
    started = time.monotonic()
    before = snapshot()
    peak_sample = 0
    stop_reason = None
    with (folder / 'stdout.jsonl').open('wb') as stdout, \
         (folder / 'stderr.log').open('wb') as stderr:
        child = subprocess.Popen(command, cwd=SOURCE, env=env,
                                 stdout=stdout, stderr=stderr,
                                 start_new_session=True)
        while True:
            pid, status, usage = os.wait4(child.pid, os.WNOHANG)
            if pid == child.pid:
                code = os.waitstatus_to_exitcode(status)
                peak = max(peak_sample, usage.ru_maxrss * 1024)
                break
            status_path = Path('/proc') / str(child.pid) / 'status'
            if status_path.exists():
                for line in status_path.read_text().splitlines():
                    if line.startswith('VmRSS:'):
                        peak_sample = max(peak_sample, int(line.split()[1]) * 1024)
                        break
            if time.monotonic() - started >= RUN_CAP:
                stop_reason = 'TIMEOUT_1200S'
                try:
                    os.killpg(child.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
            elif peak_sample > RSS_CAP:
                stop_reason = 'RSS_CAP_24GIB'
                try:
                    os.killpg(child.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
            if stop_reason:
                time.sleep(2)
                pid, status, usage = os.wait4(child.pid, os.WNOHANG)
                if pid == 0:
                    try:
                        os.killpg(child.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    pid, status, usage = os.wait4(child.pid, 0)
                code = os.waitstatus_to_exitcode(status)
                peak = max(peak_sample, usage.ru_maxrss * 1024)
                break
            time.sleep(0.5)
    child.returncode = code
    record = {
        'schema': 'n83.load80-runpod-usage/v1',
        'name': spec['name'], 'status': stop_reason or
        ('PASS_PROCESS' if code == 0 else 'FAILED_PROCESS'),
        'exit_code': code, 'wall_seconds': time.monotonic() - started,
        'peak_rss_bytes': peak, 'sampled_peak_rss_bytes': peak_sample,
        'cgroup_before': before, 'cgroup_after': snapshot(),
        'finished_utc': now().isoformat(),
    }
    resource_gate(record)
    write(folder / 'usage.json', record)
    print(spec['name'], record['status'], round(record['wall_seconds'], 1),
          record['resource_status'], flush=True)
    return record


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--protocol', type=Path, required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    protocol = json.loads(args.protocol.read_text())
    root = args.output.resolve()
    binary = args.binary.resolve()
    if (protocol.get('schema') != 'n83.load80-runpod-diagnostic/v1' or
            protocol.get('pod_id') != os.environ.get('RUNPOD_POD_ID') or
            protocol.get('machine_id') != '0h2bmdx95lus' or
            sorted(os.sched_getaffinity(0)) != [0, 1, 2, 3] or
            subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=SOURCE,
                                    text=True).strip() !=
            protocol['rust_source_commit'] or
            digest(SOURCE / 'examples/koblitz_orbit_dlp_fast.rs') !=
            protocol['rust_source_sha256'] or
            digest(Path(__file__)) != protocol['runner_sha256'] or
            not binary.is_file() or
            not (SCREEN / 'run.status').read_text().startswith(
                'DONE screen=0 screen_verify=0 ') or
            digest(SCREEN / 'raw_results.tar.gz') !=
            (SCREEN / 'raw_results.sha256').read_text().split()[0] or
            root.exists()):
        raise ValueError('source, provider, screen archive or output differs')
    host = snapshot()
    quota, period = host['cpu.max'].split()
    if (quota == 'max' or int(quota) < 4 * int(period) or
            int(host['memory.max']) < RSS_CAP + 256 * 1024**2):
        raise ValueError('Pod cgroup envelope below frozen diagnostic')
    deadline = dt.datetime.fromisoformat(
        protocol['provider_stop_utc'].replace('Z', '+00:00'))
    if not now() < deadline:
        raise ValueError('provider stop deadline passed')
    root.mkdir(parents=True)
    (root / 'targets').mkdir()
    for index, point in enumerate(protocol['points']):
        source = Path(point['source_path'])
        target = root / 'targets' / f'p{index:02d}.jsonl'
        target.write_bytes(source.read_bytes())
        if digest(target) != point['sha256']:
            raise ValueError('frozen target differs')
    write(root / 'source.json', {
        'schema': 'n83.load80-runpod-source/v1',
        'protocol': protocol, 'protocol_sha256': digest(args.protocol),
        'runner_sha256': digest(Path(__file__)),
        'binary_sha256': digest(binary), 'host_before': host,
        'provider_pod_id': protocol['pod_id'],
        'started_utc': now().isoformat(),
    })
    started = time.monotonic()
    rows = []
    for spec in protocol['runs']:
        if (time.monotonic() - started + RUN_CAP + ARCHIVE_RESERVE > GLOBAL_CAP or
                now() + dt.timedelta(seconds=RUN_CAP + ARCHIVE_RESERVE) > deadline):
            write(root / 'producer_stop.json', {
                'status': 'GLOBAL_OR_PROVIDER_CAP_BEFORE_EVENT',
                'next_event': spec})
            break
        rows.append(run_child(spec, root, binary, protocol))
    write(root / 'summary.json', {
        'schema': 'n83.load80-runpod-summary/v1',
        'status': 'COMPLETE' if len(rows) == len(protocol['runs']) else 'CAPPED',
        'run_count': len(rows), 'expected_count': len(protocol['runs']),
        'host_after': snapshot(), 'finished_utc': now().isoformat(),
    })


if __name__ == '__main__':
    main()
