#!/usr/bin/env python3
"""Strict physical-host service with measured bounds for unavoidable local IPIs.

The generic isolated service rejects every interrupt and microsecond of CPU
pressure. On this host, direct Rust execution still receives local scheduler,
call-function, TLB and timer IPIs; the first wrapper-based calibration kept
their counts. This adapter retains the generic preflight, serialization,
artifact checks and raw ledgers while enforcing the frozen small local-noise
envelope below. Device IRQs, CPU-time IRQ/softirq/steal, throttling, OOM,
thermal changes and major faults remain zero-tolerance.
"""

from math import ceil
import json
from pathlib import Path
import sys
import time

import isolated_bench as base


POLICY = {
    'local_interrupt_classification': 'nonnumeric_proc_interrupt_label',
    'max_local_interrupts_per_second': 5,
    'max_local_interrupts_fixed': 10,
    'max_cpu_pressure_usec': 1000,
    'max_involuntary_switches': 2,
    'max_major_faults': 0,
    'max_external_interrupts': 0,
}

_original_counters = base.counters
_original_noise = base.noise


def counters(cpus, cgroup):
    result = _original_counters(cpus, cgroup)
    lines = (base.read('/proc/interrupts') or '').splitlines()
    headings = [int(word[3:]) for word in lines[0].split()
                if word.startswith('CPU') and word[3:].isdigit()] if lines else []
    sources = {cpu: {} for cpu in cpus}
    for line in lines[1:]:
        columns = line.split()
        if not columns:
            continue
        name = columns[0].rstrip(':')
        for column, cpu in enumerate(headings, start=1):
            if cpu in cpus and column < len(columns):
                try:
                    sources[cpu][name] = int(columns[column])
                except ValueError:
                    pass
    result['interrupts_by_source'] = sources
    result['monotonic_ns'] = time.monotonic_ns()
    return result


def noise(before, after, usage_before, usage_after):
    ignored = {'CPU pressure accrued', 'involuntary context switches'}
    ignored.update(f'CPU {cpu} interrupt count changed' for cpu in before['per_cpu'])
    problems = [problem for problem in _original_noise(before, after, usage_before, usage_after)
                if problem not in ignored]
    elapsed = (after['monotonic_ns'] - before['monotonic_ns']) / 1_000_000_000
    if elapsed <= 0:
        problems.append('invalid counter interval')
    local_limit = POLICY['max_local_interrupts_fixed'] + ceil(
        POLICY['max_local_interrupts_per_second'] * max(elapsed, 0))
    local_count = external_count = 0
    if not before['interrupts_by_source'] or not after['interrupts_by_source']:
        problems.append('per-source interrupt counters unavailable')
    for cpu in before['per_cpu']:
        old_sources = before['interrupts_by_source'].get(cpu, {})
        new_sources = after['interrupts_by_source'].get(cpu, {})
        source_total = 0
        for source in set(old_sources) | set(new_sources):
            delta = new_sources.get(source, 0) - old_sources.get(source, 0)
            if delta < 0:
                problems.append(f'CPU {cpu} interrupt counter reset: {source}')
            source_total += delta
            if source.isdigit():
                external_count += delta
            else:
                local_count += delta
        if source_total != after['interrupts'].get(cpu, 0) - before['interrupts'].get(cpu, 0):
            problems.append(f'CPU {cpu} interrupt source sum mismatch')
    if external_count > POLICY['max_external_interrupts']:
        problems.append(f'external interrupt count {external_count}')
    if local_count > local_limit:
        problems.append(f'local interrupt count {local_count} exceeds {local_limit}')
    for kind in ('some', 'full'):
        delta = after['cpu_pressure_total'].get(kind, 0) - before['cpu_pressure_total'].get(kind, 0)
        if delta < 0 or delta > POLICY['max_cpu_pressure_usec']:
            problems.append(f'CPU pressure {kind} {delta} usec')
    involuntary = usage_after.ru_nivcsw - usage_before.ru_nivcsw
    if involuntary < 0 or involuntary > POLICY['max_involuntary_switches']:
        problems.append(f'involuntary context switches {involuntary}')
    return problems


def main():
    if len(sys.argv) < 3 or sys.argv[-2] not in ('run', 'probe'):
        raise SystemExit('use this adapter only for run or probe with a frozen manifest')
    manifest = json.loads(Path(sys.argv[-1]).read_text())
    if manifest.get('noise_gate') != POLICY or manifest.get('measurement_tier') != 'tier0_physical_host':
        raise SystemExit('frozen physical-host noise policy mismatch')
    base.counters = counters
    base.noise = noise
    base.__file__ = __file__
    base.main()


if __name__ == '__main__':
    main()
