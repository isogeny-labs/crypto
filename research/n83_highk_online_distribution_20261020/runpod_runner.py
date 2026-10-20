#!/usr/bin/env python3
"""Run one sealed N83 distribution phase on its identified RunPod Pod."""

import argparse
import json
import os
import platform
import shutil
import subprocess
import sys
from datetime import datetime
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_highk_online_distribution_20261020 import run as runner
from research.n83_highk_online_distribution_20261020 import check_dispatch
from research.n83_highk_online_distribution_20261020 import runpod_provider


CPU_V1 = Path('/sys/fs/cgroup/cpu')
MEMORY_V1 = Path('/sys/fs/cgroup/memory')


def v1_snapshot(cpu=CPU_V1, memory=MEMORY_V1):
    """Retain raw v1 counters and expose conservative v2-style gate fields."""
    def read(directory, name):
        value = (directory / name).read_text().strip()
        if not value:
            raise ValueError(f'empty cgroup v1 counter: {directory / name}')
        return value

    cpu_stat = read(cpu, 'cpu.stat')
    cpu_fields = dict(line.split(maxsplit=1) for line in cpu_stat.splitlines())
    if not {'nr_periods', 'nr_throttled', 'throttled_time'} <= cpu_fields.keys():
        raise ValueError('cgroup v1 cpu.stat is incomplete')
    quota = int(read(cpu, 'cpu.cfs_quota_us'))
    period = int(read(cpu, 'cpu.cfs_period_us'))
    if quota < -1 or period <= 0:
        raise ValueError('invalid cgroup v1 CPU quota')
    current = int(read(memory, 'memory.usage_in_bytes'))
    peak = int(read(memory, 'memory.max_usage_in_bytes'))
    limit = int(read(memory, 'memory.limit_in_bytes'))
    failcnt = int(read(memory, 'memory.failcnt'))
    oom_control = read(memory, 'memory.oom_control')
    oom_fields = dict(line.split(maxsplit=1) for line in oom_control.splitlines())
    if not {'under_oom', 'oom_kill'} <= oom_fields.keys():
        raise ValueError('cgroup v1 OOM counters are incomplete')
    oom_kills = int(oom_fields['oom_kill'])
    under_oom = int(oom_fields['under_oom'])
    if min(current, peak, limit, failcnt, oom_kills, under_oom,
           *(int(value) for value in cpu_fields.values())) < 0:
        raise ValueError('negative cgroup v1 counter')
    # v1 failcnt counts memory-limit denials; using it as the v2 oom gate is
    # conservative. A zero delta proves no limit denial or OOM kill occurred.
    return {
        'cgroup_interface': 'v1',
        'cgroup_v1_paths': {'cpu': str(cpu), 'memory': str(memory)},
        'cgroup_v1_raw': {
            'cpu.stat': cpu_stat, 'cpu.cfs_quota_us': str(quota),
            'cpu.cfs_period_us': str(period),
            'memory.usage_in_bytes': str(current),
            'memory.max_usage_in_bytes': str(peak),
            'memory.limit_in_bytes': str(limit),
            'memory.failcnt': str(failcnt),
            'memory.oom_control': oom_control,
        },
        'memory.current': str(current), 'memory.peak': str(peak),
        'memory.max': str(limit),
        'memory.events': f'oom {max(failcnt, under_oom)}\noom_kill {oom_kills}\n',
        'cpu.stat': cpu_stat + '\n',
        'cpu.max': f'{"max" if quota == -1 else quota} {period}\n',
        'cpu.pressure': None, 'memory.pressure': None,
    }


def read_provider(path, expected_stratum):
    receipt = json.loads(path.read_text())
    if (receipt.get('schema') != 'n83.online-distribution-runpod-allocation/v1' or
            receipt.get('stratum') != expected_stratum or
            not isinstance(receipt.get('pod_id'), str) or
            not receipt['pod_id'].isalnum() or
            not isinstance(receipt.get('machine_id'), str) or
            not receipt['machine_id'] or
            receipt.get('desired_status') != 'RUNNING' or
            not isinstance(receipt.get('cost_per_hr'), (int, float)) or
            not 0 < receipt['cost_per_hr'] <= 0.20 or
            receipt.get('cloud_type') != 'COMMUNITY' or
            receipt.get('compute_type') != 'GPU' or
            receipt.get('gpu_type_id') not in runpod_provider.GPU_TYPES or
            receipt.get('gpu_count') != 1 or
            not isinstance(receipt.get('cpu_affinity_ids'), list) or
            len(receipt['cpu_affinity_ids']) != 4 or
            receipt['cpu_affinity_ids'] != sorted(set(receipt['cpu_affinity_ids'])) or
            any(not isinstance(cpu, int) or cpu < 0 for cpu in
                receipt['cpu_affinity_ids']) or
            receipt.get('vcpu_count', 0) < 4 or
            receipt.get('memory_in_gb', 0) < 32 or
            not isinstance(receipt.get('snapshot_sha256'), str) or
            len(receipt['snapshot_sha256']) != 64):
        raise ValueError('RunPod allocation receipt is incomplete')
    created = datetime.fromisoformat(receipt['created_utc'].replace('Z', '+00:00'))
    stopped = datetime.fromisoformat(receipt['stop_after_utc'].replace('Z', '+00:00'))
    if (created.tzinfo is None or stopped.tzinfo is None or
            not 0 < (stopped - created).total_seconds() <= 25200):
        raise ValueError('RunPod allocation stop cap differs')
    return receipt


def bind_provider(receipt, head_sha):
    original_host_receipt = runner.parent.host_receipt
    original_write_json = runner.parent.write_json
    adapter_sha256 = runner.parent.digest(Path(__file__))
    preflight = v1_snapshot()  # Fail before a timed child if counters are absent.
    if sorted(os.sched_getaffinity(0)) != receipt['cpu_affinity_ids']:
        raise ValueError('runner affinity differs from frozen Pod allocation')
    quota, period = preflight['cpu.max'].split()
    if (quota == 'max' or int(quota) < 4 * int(period) or
            int(preflight['memory.max']) < 24 * 1024**3 + 256 * 1024**2):
        raise ValueError('RunPod cgroup quota or memory limit below frozen envelope')
    runner.parent.cgroup_snapshot = lambda _group: v1_snapshot()

    def host_receipt(group):
        host = original_host_receipt(group)
        host.update({
            'provider': 'runpod',
            'instance_id': f"rp-{receipt['pod_id']}-{receipt['machine_id']}",
            'run_url': f"https://www.runpod.io/console/pods/{receipt['pod_id']}",
            'git_sha': head_sha,
            'runner_name': receipt['pod_id'],
            'runner_os': platform.system(),
            'runner_arch': platform.machine(),
            'image_version': receipt.get('image_name'),
            'runpod_pod_id': receipt['pod_id'],
            'runpod_machine_id': receipt['machine_id'],
            'runpod_frozen_cpu_affinity': receipt['cpu_affinity_ids'],
            'runpod_allocation_sha256': runner.parent.digest(receipt['_path']),
            'runpod_adapter_sha256': adapter_sha256,
            'cgroup_interface': 'v1',
        })
        return host

    def write_json(path, value):
        if Path(path).name == 'source.json':
            value = {**value, 'runpod_adapter_sha256': adapter_sha256,
                     'runpod_allocation_sha256':
                     runner.parent.digest(receipt['_path'])}
        return original_write_json(path, value)

    runner.parent.host_receipt = host_receipt
    runner.parent.write_json = write_json


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--stratum', type=int, choices=(0,), required=True)
    parser.add_argument('--phase', choices=('screen', 'holdout'), required=True)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--rho-binary', type=Path)
    parser.add_argument('--candidate-id')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--provider-receipt', type=Path, required=True)
    args = parser.parse_args()
    receipt_path = args.provider_receipt.resolve()
    receipt = read_provider(receipt_path, args.stratum)
    receipt['_path'] = receipt_path
    if os.environ.get('RUNPOD_POD_ID') not in (None, receipt['pod_id']):
        raise ValueError('Pod environment differs from allocation receipt')
    head_sha = subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    if head_sha != receipt.get('source_head_sha'):
        raise ValueError('Pod checkout differs from frozen allocation source')
    check_dispatch.verify()
    runner.fixture()
    runner.arms.choose()
    bind_provider(receipt, head_sha)
    output = args.output.resolve()
    try:
        if args.phase == 'screen':
            if not args.binary or not args.rho_binary:
                parser.error('screen needs --binary and --rho-binary')
            return runner.run_screen(output, args.binary.resolve(),
                                     args.rho_binary.resolve())
        if not args.candidate_id:
            parser.error('holdout needs --candidate-id')
        if runner.parent.digest(output / 'runpod_allocation.json') != runner.parent.digest(receipt_path):
            raise ValueError('holdout allocation differs from screen')
        return runner.run_holdout(output, args.candidate_id)
    finally:
        if args.phase == 'screen' and output.is_dir():
            shutil.copy2(receipt_path, output / 'runpod_allocation.json')


if __name__ == '__main__':
    raise SystemExit(main())
