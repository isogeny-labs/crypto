#!/usr/bin/env python3
"""Derive the strict isolated-bench worker panel from a frozen Tier 1 case plan."""

import argparse
import json
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    for key in ('plan', 'wrapper', 'strict-runner', 'allocation', 'host-probe',
                'cgroup', 'cpus', 'mem-node', 'execution-cpu', 'out'):
        p.add_argument('--' + key, required=True)
    args = p.parse_args()
    plan_path = Path(args.plan).resolve(strict=True)
    plan = json.loads(plan_path.read_text())
    direct = plan['workload'].get('direct_strict_output') is True
    if (plan['measurement_tier'] != ('tier0_physical_host' if direct else 'tier1_instance') or
            plan['workload']['checkin_every'] != 64 or
            plan['workload']['walkers_requested'] != 128 or
            len(plan['cases']) != 96 or plan['repetitions'] != 4):
        raise ValueError('unexpected worker panel')
    wrapper = str(Path(args.wrapper).resolve(strict=True))
    strict_runner = str(Path(args.strict_runner).resolve(strict=True))
    artifacts = list(dict.fromkeys([
        *plan['artifacts'], str(plan_path), strict_runner,
        str(Path(args.allocation).resolve(strict=True)),
        str(Path(args.host_probe).resolve(strict=True)),
        str(Path(__file__).resolve(strict=True)),
        *([str((Path(strict_runner).parent / 'isolated_bench.py').resolve(strict=True))] if direct else []),
        *([] if direct else [wrapper]),
    ]))
    cases = []
    for case in plan['cases']:
        cases.append({'id': case['id'],
                      'reference': case['reference'] if direct else [wrapper, *case['reference']],
                      'candidate': case['candidate'] if direct else [wrapper, *case['candidate']]})
    manifest = {
        'schema': 1,
        'measurement_tier': 'tier0_physical_host',
        'workdir': plan['workdir'],
        'isolation': {'cpus': args.cpus, 'mem_nodes': args.mem_node,
                      'execution_cpu': int(args.execution_cpu),
                      'cgroup': args.cgroup},
        'repetitions': plan['repetitions'],
        'timeout_s': plan['timeout_s'],
        'measurement_boundary': 'Rust total_charged_ns/1e6: Q-dependent job construction plus full rho lane; fixture loading and independent DP replay outside, both retained in raw output',
        'pair_fields': [*plan['pair_fields'], 'q_index', 'q_hex',
                        'dp_bits', 'step_cap', 'checkin_every'],
        'cases': cases,
        'artifacts': artifacts,
        'worker_plan': str(plan_path),
        'claim_gate': plan['claim_gate'],
    }
    if direct:
        manifest['noise_gate'] = {
            'local_interrupt_classification': 'nonnumeric_proc_interrupt_label',
            'interrupt_source_authority': 'proc_interrupts',
            'max_local_interrupts_per_second': 5,
            'max_local_interrupts_fixed': 10,
            'max_cpu_pressure_usec': 1000,
            'max_involuntary_switches': 2,
            'max_major_faults': 0,
            'max_external_interrupts': 0,
        }
    output = Path(args.out)
    output.write_text(json.dumps(manifest, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'manifest': str(output), 'cases': len(cases),
                      'pairs': len(cases) * manifest['repetitions']}))


if __name__ == '__main__':
    main()
