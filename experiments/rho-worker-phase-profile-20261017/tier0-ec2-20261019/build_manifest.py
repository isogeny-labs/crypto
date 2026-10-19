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
    if (plan['measurement_tier'] != 'tier1_instance' or
            plan['workload']['checkin_every'] != 64 or
            plan['workload']['walkers_requested'] != 128 or
            len(plan['cases']) != 96 or plan['repetitions'] != 4):
        raise ValueError('unexpected worker panel')
    wrapper = str(Path(args.wrapper).resolve(strict=True))
    strict_runner = str(Path(args.strict_runner).resolve(strict=True))
    artifacts = list(dict.fromkeys([
        *plan['artifacts'], str(plan_path), wrapper, strict_runner,
        str(Path(args.allocation).resolve(strict=True)),
        str(Path(args.host_probe).resolve(strict=True)),
    ]))
    cases = []
    for case in plan['cases']:
        cases.append({'id': case['id'],
                      'reference': [wrapper, *case['reference']],
                      'candidate': [wrapper, *case['candidate']]})
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
    output = Path(args.out)
    output.write_text(json.dumps(manifest, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'manifest': str(output), 'cases': len(cases),
                      'pairs': len(cases) * manifest['repetitions']}))


if __name__ == '__main__':
    main()
