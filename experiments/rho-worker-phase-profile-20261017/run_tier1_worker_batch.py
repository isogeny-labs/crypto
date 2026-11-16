#!/usr/bin/env python3
"""Serial Tier 1 instance runner for the frozen full rho worker panel.

The child executable owns total_charged_ns. This runner records the allocation,
affinity, cgroup-v1/v2 counters, per-CPU counters, full stdout/stderr, and
every failed pair. A flock in the shared queue root excludes concurrent
panels launched against the same persistent Pod workspace.
"""

import argparse
import fcntl
from hashlib import sha256
import json
import os
from pathlib import Path
import platform
import resource
import signal
import subprocess
import time
import uuid

LOCK_NAME = '.rho-worker-panel.lock'


def digest(path):
    h = sha256()
    with open(path, 'rb') as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def read(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def write(path, value):
    Path(path).write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def numeric_file(path):
    result = {}
    for line in (read(path) or '').splitlines():
        words = line.split()
        if len(words) == 2:
            try:
                result[words[0]] = int(words[1])
            except ValueError:
                pass
    return result


def cgroup_v2_file(name):
    """Read the current process scope, not the cgroup mount root."""
    for line in (read('/proc/self/cgroup') or '').splitlines():
        if line.startswith('0::'):
            relative = line[3:].lstrip('/')
            parts = Path(relative).parts
            if '..' in parts:
                return None
            return read(Path('/sys/fs/cgroup').joinpath(*parts, name))
    return None


def cgroup_v2_numeric(name):
    result = {}
    for line in (cgroup_v2_file(name) or '').splitlines():
        words = line.split()
        if len(words) == 2:
            try:
                result[words[0]] = int(words[1])
            except ValueError:
                pass
    return result


def parse_cpu_set(value):
    result = set()
    for part in (value or '').split(','):
        if not part:
            continue
        ends = [int(word) for word in part.split('-')]
        if len(ends) == 1:
            result.add(ends[0])
        elif len(ends) == 2 and ends[0] <= ends[1]:
            result.update(range(ends[0], ends[1] + 1))
        else:
            raise ValueError('invalid cgroup CPU list')
    return result


def counters(cpu):
    cpu_line = next((line.split() for line in (read('/proc/stat') or '').splitlines()
                     if line.startswith(f'cpu{cpu} ')), [])
    cpu_times = [int(value) for value in cpu_line[1:]] if cpu_line else []
    irq_line = next((line.split() for line in (read('/proc/interrupts') or '').splitlines()
                     if line.lstrip().startswith('LOC:')), [])
    irq_counts = [int(word) for word in irq_line[1:] if word.isdigit()]
    return {
        'timestamp_ns': time.time_ns(),
        'cpu_times': cpu_times,
        'local_irq_count': irq_counts[cpu] if len(irq_counts) > cpu else None,
        'cgroup_v1_cpu_stat': numeric_file('/sys/fs/cgroup/cpu/cpu.stat'),
        'cgroup_v1_memory_failcnt': read('/sys/fs/cgroup/memory/memory.failcnt'),
        'cgroup_v2_cpu_stat': cgroup_v2_numeric('cpu.stat'),
        'cgroup_v2_memory_events': cgroup_v2_numeric('memory.events'),
        'frequency_khz': read(f'/sys/devices/system/cpu/cpu{cpu}/cpufreq/scaling_cur_freq'),
        'thermal_throttle': read(
            f'/sys/devices/system/cpu/cpu{cpu}/thermal_throttle/core_throttle_count'),
    }


def deltas(before, after):
    result = {}
    for section in ('cgroup_v1_cpu_stat', 'cgroup_v2_cpu_stat',
                    'cgroup_v2_memory_events'):
        result[section] = {key: after[section][key] - value
                           for key, value in before[section].items()
                           if key in after[section]}
    if before['cpu_times'] and len(before['cpu_times']) == len(after['cpu_times']):
        result['cpu_times'] = [new - old for old, new in
                               zip(before['cpu_times'], after['cpu_times'])]
    for key in ('local_irq_count', 'cgroup_v1_memory_failcnt', 'thermal_throttle'):
        try:
            result[key] = int(after[key]) - int(before[key])
        except (TypeError, ValueError):
            result[key] = None
    result['frequency_khz_before_after'] = [before['frequency_khz'],
                                            after['frequency_khz']]
    return result


def probe(manifest):
    cpu = manifest['isolation']['execution_cpu']
    allocation = manifest['allocation']
    allowed = sorted(os.sched_getaffinity(0)) if hasattr(os, 'sched_getaffinity') else []
    cpuset = (cgroup_v2_file('cpuset.cpus.effective') or
              read('/sys/fs/cgroup/cpuset.cpus.effective') or
              read('/sys/fs/cgroup/cpuset/cpuset.cpus') or
              read('/sys/fs/cgroup/cpuset.cpus'))
    allocated = parse_cpu_set(cpuset) if cpuset else set()
    quota = read('/sys/fs/cgroup/cpu/cpu.cfs_quota_us')
    period = read('/sys/fs/cgroup/cpu/cpu.cfs_period_us')
    memory = read('/sys/fs/cgroup/memory/memory.limit_in_bytes')
    evidence = {
        'measurement_tier': 'tier1_instance', 'allocation': allocation,
        'timestamp_ns': time.time_ns(), 'host': platform.node(),
        'system': platform.system(), 'kernel': platform.release(),
        'architecture': platform.machine(),
        'cpu_model': next((line.split(':', 1)[1].strip()
                           for line in (read('/proc/cpuinfo') or '').splitlines()
                           if line.startswith('model name')), None),
        'affinity': allowed, 'allocated_cpus': sorted(allocated),
        'selected_cpu': cpu,
        'cpuset_cpus': cpuset, 'cpu_quota_us': quota,
        'cpu_period_us': period, 'memory_limit_bytes': memory,
        'memory_nodes': (read('/sys/fs/cgroup/cpuset.mems.effective') or
                         read('/sys/fs/cgroup/cpuset/cpuset.mems')),
        'self_cgroup': read('/proc/self/cgroup'),
        'cgroup_v2_cpu_max': cgroup_v2_file('cpu.max'),
        'cgroup_v2_memory_max': cgroup_v2_file('memory.max'),
        'counter_probe': counters(cpu),
        'host_exclusivity': 'unproven_container_allocation',
    }
    problems = []
    if platform.system() != 'Linux' or platform.machine() != 'x86_64':
        problems.append('unexpected operating system or architecture')
    if cpu not in allocated or len(allocated) < 2 or cpu in allowed:
        problems.append('selected CPU absent or service not separated')
    if not cpuset:
        problems.append('effective cgroup CPU set unavailable')
    if os.geteuid() != 0:
        problems.append('runner must be root for host receipts')
    if quota is None and evidence['cgroup_v2_cpu_max'] is None:
        problems.append('CPU quota receipt unavailable')
    if memory is None and evidence['cgroup_v2_memory_max'] is None:
        problems.append('memory limit receipt unavailable')
    if cgroup_v2_file('cpu.stat') is None and not evidence['counter_probe']['cgroup_v1_cpu_stat']:
        problems.append('CPU throttle counter unavailable')
    if not Path(allocation['receipt']).is_file():
        problems.append('allocation receipt missing')
    if len(evidence['counter_probe']['cpu_times']) <= 7:
        problems.append('selected CPU or steal-time counters missing')
    return {'ok': not problems, 'problems': problems, 'evidence': evidence}


def env():
    return {'PATH': '/usr/bin:/bin', 'LANG': 'C', 'LC_ALL': 'C',
            'TZ': 'UTC', 'OMP_NUM_THREADS': '1', 'OPENBLAS_NUM_THREADS': '1'}


def arm(manifest, case, repeat, variant, serial, folder):
    cpu = manifest['isolation']['execution_cpu']
    argv = case[variant]
    before = counters(cpu)
    usage_before = resource.getrusage(resource.RUSAGE_CHILDREN)
    begin = time.monotonic_ns()
    timed_out = False
    def pin():
        os.sched_setaffinity(0, {cpu})
    proc = subprocess.Popen(argv, cwd=manifest['workdir'], env=env(),
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                            preexec_fn=pin, start_new_session=True)
    try:
        stdout, stderr = proc.communicate(timeout=manifest['timeout_s'])
    except subprocess.TimeoutExpired:
        timed_out = True
        os.killpg(proc.pid, signal.SIGKILL)
        stdout, stderr = proc.communicate()
    outer_ms = (time.monotonic_ns() - begin) / 1e6
    after = counters(cpu)
    usage_after = resource.getrusage(resource.RUSAGE_CHILDREN)
    stdout = stdout.decode('utf-8', 'replace')
    stderr = stderr.decode('utf-8', 'replace')
    stem = f'{serial:05d}'
    (folder / f'{stem}.stdout.txt').write_text(stdout)
    (folder / f'{stem}.stderr.txt').write_text(stderr)
    try:
        fields = json.loads(stdout)
        if not isinstance(fields, dict):
            raise ValueError('arm output must be a JSON object')
    except (json.JSONDecodeError, ValueError):
        fields = {}
    delta = deltas(before, after)
    issues = []
    if proc.returncode != 0 or timed_out or stderr:
        issues.append('process failed, timed out, or wrote stderr')
    if fields.get('schema') != 'rho-worker-batch-benchmark-v1':
        issues.append('worker output schema missing')
    if fields.get('status') != 'complete_frozen_work':
        issues.append('worker did not complete frozen work')
    if fields.get('fixture_manifest_sha256') != manifest['fixture_manifest_sha256']:
        issues.append('fixture manifest digest mismatch')
    if fields.get('q_index') != case['q_index'] or fields.get('q_hex') != case['q_hex']:
        issues.append('Q fixture mismatch')
    if fields.get('mode') != case[variant + '_mode']:
        issues.append('mode mismatch')
    if fields.get('batch_width') != case[variant + '_width']:
        issues.append('batch width mismatch')
    for key in ('walkers_requested', 'dp_bits', 'step_cap', 'checkin_every'):
        if fields.get(key) != manifest['workload'][key]:
            issues.append('workload mismatch: ' + key)
    if fields.get('verified_distinguished_points') != fields.get('distinguished_points'):
        issues.append('DP verification failed')
    if not isinstance(fields.get('peak_rss_kib'), int) or fields['peak_rss_kib'] <= 0:
        issues.append('process peak RSS missing')
    try:
        total_ns = int(fields['total_charged_ns'])
        setup_ns = int(fields['setup_ns'])
        walk_ns = int(fields['walk_ns'])
        if total_ns <= 0 or setup_ns <= 0 or walk_ns <= 0 or total_ns != setup_ns + walk_ns:
            raise ValueError()
    except (KeyError, ValueError):
        total_ns = None
        issues.append('charged phase metric missing or inconsistent')
    if delta['cgroup_v1_cpu_stat'].get('nr_throttled', 0) or delta[
            'cgroup_v2_cpu_stat'].get('nr_throttled', 0):
        issues.append('CPU throttling')
    if delta['cgroup_v2_memory_events'].get('oom', 0) or delta[
            'cgroup_v2_memory_events'].get('oom_kill', 0):
        issues.append('memory OOM')
    if (delta['cgroup_v1_memory_failcnt'] or 0) > 0:
        issues.append('cgroup v1 memory failure')
    if len(delta.get('cpu_times', [])) <= 7:
        issues.append('CPU steal counter missing')
    elif delta['cpu_times'][7]:
        issues.append('CPU steal time')
    if (delta['thermal_throttle'] or 0) > 0:
        issues.append('thermal throttling')
    if (usage_after.ru_majflt - usage_before.ru_majflt) > 0:
        issues.append('major fault')
    postflight = probe(manifest)
    if not postflight['ok']:
        issues.append('postflight failed')
    return {
        'serial': serial, 'case': case['id'], 'repeat': repeat, 'variant': variant,
        'status': 'valid' if not issues else 'invalid', 'issues': issues,
        'measurement_tier': 'tier1_instance', 'command': argv,
        'binary_sha256': digest(argv[0]), 'returncode': proc.returncode,
        'timed_out': timed_out, 'environment': env(), 'total_charged_ns': total_ns,
        'outer_ms': outer_ms, 'fields': fields,
        'before': before, 'after': after,
        'tier1_observations': {'counter_deltas': delta},
        'child_involuntary_switches': usage_after.ru_nivcsw - usage_before.ru_nivcsw,
        'child_major_faults': usage_after.ru_majflt - usage_before.ru_majflt,
        'postflight': postflight, 'stdout_file': f'{stem}.stdout.txt',
        'stderr_file': f'{stem}.stderr.txt',
    }


def run(manifest, folder, lock_path):
    if manifest.get('schema') != 1 or manifest.get('measurement_tier') != 'tier1_instance':
        raise ValueError('expected frozen Tier 1 manifest')
    if manifest.get('outer_repeats') != 1:
        raise ValueError('unexpected outer repeat count')
    if not manifest.get('cases') or not manifest.get('artifacts'):
        raise ValueError('empty case or artifact list')
    required_pair_fields = {
        'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
        'walk_steps', 'distinguished_points', 'dead_trails',
        'units_completed', 'checkins',
    }
    if set(manifest.get('pair_fields', [])) != required_pair_fields:
        raise ValueError('incomplete exact-output pair fields')
    if len(manifest['cases']) != 96:
        raise ValueError('expected comparison and two A/A controls for each of 32 Q points')
    for q_index in range(32):
        kinds = sorted(case['kind'] for case in manifest['cases']
                       if case['q_index'] == q_index)
        if kinds != ['aa_batch', 'aa_serial', 'comparison']:
            raise ValueError(f'incomplete frozen case set for Q {q_index}')
    if len(set(manifest['artifacts'])) != len(manifest['artifacts']):
        raise ValueError('duplicate artifact paths')
    folder.mkdir(parents=True, exist_ok=False)
    write(folder / 'manifest.json', manifest)
    artifacts_before = {path: digest(path) for path in manifest['artifacts']}
    health = probe(manifest)
    write(folder / 'preflight.json', health)
    expected = len(manifest['cases']) * manifest['repetitions']
    rows = 0
    pairs = 0
    valid_pairs = 0
    error = None
    try:
        if health['ok']:
            with (folder / 'runs.jsonl').open('w') as ledger, (
                    folder / 'pairs.jsonl').open('w') as pair_ledger:
                for repeat in range(manifest['repetitions']):
                    for index, case in enumerate(manifest['cases']):
                        order = ('reference', 'candidate') if (repeat + index) % 2 == 0 else (
                            'candidate', 'reference')
                        outcomes = {}
                        for variant in order:
                            row = arm(manifest, case, repeat, variant, rows + 1, folder)
                            rows += 1
                            outcomes[variant] = row
                            ledger.write(json.dumps(row, sort_keys=True) + '\n')
                            ledger.flush()
                        issues = []
                        if any(row['status'] != 'valid' for row in outcomes.values()):
                            issues.append('invalid arm')
                        for field in manifest['pair_fields']:
                            if outcomes['reference']['fields'].get(field) != outcomes[
                                    'candidate']['fields'].get(field):
                                issues.append('pair field mismatch: ' + field)
                        ratio = None
                        if not issues:
                            ratio = outcomes['reference']['total_charged_ns'] / outcomes[
                                'candidate']['total_charged_ns']
                            valid_pairs += 1
                        pair = {'case': case['id'], 'kind': case['kind'], 'repeat': repeat,
                                'measurement_tier': 'tier1_instance',
                                'run_serials': {key: value['serial']
                                                for key, value in outcomes.items()},
                                'status': 'valid' if not issues else 'invalid',
                                'issues': issues, 'speedup': ratio}
                        pairs += 1
                        pair_ledger.write(json.dumps(pair, sort_keys=True) + '\n')
                        pair_ledger.flush()
    except BaseException as failure:
        error = f'{type(failure).__name__}: {failure}'
        raise
    finally:
        try:
            artifacts_after = {path: digest(path) for path in manifest['artifacts']}
        except OSError:
            artifacts_after = {}
        stable = artifacts_before == artifacts_after
        complete = health['ok'] and rows == 2 * expected and pairs == expected and error is None
        summary = {'status': 'completed' if complete else 'rejected',
                   'measurement_tier': 'tier1_instance', 'expected_pairs': expected,
                   'queue_root': str(lock_path.parent),
                   'lock_path': str(lock_path),
                   'lock_scope': 'shared_queue_root_mount',
                   'completed_arms': rows, 'completed_pairs': pairs,
                   'error': error, 'valid_pairs': valid_pairs,
                   'artifact_sha256': artifacts_before,
                   'artifacts_stable': stable, 'runner_sha256': digest(__file__),
                   'manifest_sha256': digest(folder / 'manifest.json'),
                   'paired_speedup': None}
        write(folder / 'summary.json', summary)
    return summary


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--manifest', type=Path, required=True)
    parser.add_argument('--queue-root', type=Path,
                        default=Path('/workspace/isolated-bench'))
    args = parser.parse_args()
    manifest = json.loads(args.manifest.read_text())
    args.queue_root.mkdir(parents=True, exist_ok=True)
    lock_path = args.queue_root.resolve() / LOCK_NAME
    with lock_path.open('a+') as stream:
        fcntl.flock(stream, fcntl.LOCK_EX | fcntl.LOCK_NB)
        job = uuid.uuid4().hex
        folder = args.queue_root / 'results' / job
        result = run(manifest, folder, lock_path)
        print(json.dumps({'job_id': job, 'output': str(folder),
                          'status': result['status'], 'valid_pairs': result['valid_pairs']}))
        if result['status'] != 'completed':
            raise SystemExit(3)


if __name__ == '__main__':
    main()
