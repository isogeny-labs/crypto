#!/usr/bin/env python3
"""Capture the guest allocation and noise-control state before dispatch."""

import hashlib
import argparse
import json
import os
from pathlib import Path
import platform
import subprocess
import time

ROOT = Path('/home/ec2-user/rho-worker-tier1-20261018')
CPU = 15
parser = argparse.ArgumentParser()
parser.add_argument('--source-bundle', type=Path, default=ROOT / 'source.tar.gz')
parser.add_argument('--binary', type=Path,
                    default=ROOT / 'source/target/release/examples/rho_worker_batch_benchmark')
parser.add_argument('--out', type=Path, default=ROOT / 'allocation.json')
args = parser.parse_args()


def read(path):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return None


def command(*args):
    result = subprocess.run(args, text=True, capture_output=True, check=False)
    return {'argv': list(args), 'exit_code': result.returncode,
            'stdout': result.stdout.strip(), 'stderr': result.stderr.strip()}


def sha256(path):
    h = hashlib.sha256()
    with open(path, 'rb') as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b''):
            h.update(chunk)
    return h.hexdigest()


def cpus(value):
    out = set()
    for group in (value or '').split(','):
        if group:
            ends = [int(part) for part in group.split('-')]
            out.update(range(ends[0], ends[-1] + 1))
    return out


irq_overlap = []
irq_count = 0
for path in Path('/proc/irq').glob('*/effective_affinity_list'):
    value = read(path)
    if value:
        irq_count += 1
        if CPU in cpus(value):
            irq_overlap.append({'irq': path.parent.name, 'affinity': value})

base = '/sys/fs/cgroup'
receipt = {
    'schema': 'rho-worker-ec2-tier1-allocation-v1',
    'captured_unix_ns': time.time_ns(),
    'tier': 'tier1_cloud_instance',
    'host_exclusivity': 'unproven_cloud_physical_host',
    'host': platform.node(),
    'kernel': platform.release(),
    'architecture': platform.machine(),
    'selected_cpu': CPU,
    'cpu_model': next((line.split(':', 1)[1].strip() for line in
                       (read('/proc/cpuinfo') or '').splitlines()
                       if line.startswith('model name')), None),
    'online_cpus': read('/sys/devices/system/cpu/online'),
    'numa_nodes': read('/sys/devices/system/node/online'),
    'selected_cpu_siblings': read(f'/sys/devices/system/cpu/cpu{CPU}/topology/thread_siblings_list'),
    'boot_cmdline': read('/proc/cmdline'),
    'nohz_full': read('/sys/devices/system/cpu/nohz_full'),
    'isolated_cpus': read('/sys/devices/system/cpu/isolated'),
    'irqbalance': command('systemctl', 'is-active', 'irqbalance'),
    'irq_effective_affinity_checked': irq_count,
    'irq_selected_cpu_overlap': irq_overlap,
    'cgroups': {name: {
        'cpus': read(f'{base}/{name}/cpuset.cpus.effective'),
        'memory_nodes': read(f'{base}/{name}/cpuset.mems.effective'),
        'cpu_max': read(f'{base}/{name}/cpu.max'),
        'memory_max': read(f'{base}/{name}/memory.max'),
    } for name in ('system.slice', 'user.slice', 'init.scope',
                   'rho.slice/rho-runner.slice')},
    'frequency_governor': read(f'/sys/devices/system/cpu/cpu{CPU}/cpufreq/scaling_governor'),
    'hard_stop_timer': command('systemctl', 'is-active', 'rho-hard-stop.timer'),
    'hard_stop_schedule': command('systemctl', 'cat', 'rho-hard-stop.timer'),
    'build': {
        'source_bundle_sha256': sha256(args.source_bundle),
        'binary_sha256': sha256(args.binary),
        'q_manifest_sha256': sha256(ROOT / 'source/experiments/rho-worker-phase-profile-20261017/rho-closeout-q-manifest.json'),
        'rustc': command('rustc', '--version'),
        'cargo': command('cargo', '--version'),
    },
}
assert receipt['selected_cpu_siblings'] == '15'
assert receipt['irq_selected_cpu_overlap'] == []
assert receipt['hard_stop_timer']['stdout'] == 'active'
assert receipt['cgroups']['system.slice']['cpus'] == '0-14'
assert receipt['cgroups']['user.slice']['cpus'] == '0-14'
assert receipt['cgroups']['init.scope']['cpus'] == '0-14'
assert receipt['cgroups']['rho.slice/rho-runner.slice']['cpus'] == '0-15'
assert receipt['build']['q_manifest_sha256'] == 'f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8'
out = args.out
out.write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
print(json.dumps({'path': str(out), 'sha256': sha256(out),
                  'irqs_checked': irq_count, 'irq_overlap': len(irq_overlap)}))
