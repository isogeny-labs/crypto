#!/usr/bin/env python3
"""Freeze the fresh-target physical-host worker panel after allocation."""

import argparse
from hashlib import sha256
import json
from pathlib import Path

Q_SHA256 = '546fe0b842a2f559395c9ac5d74ced12b96ce366e41322387eb0baec5dcb3adc'
PAIR_FIELDS = [
    'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
    'walk_steps', 'distinguished_points', 'dead_trails',
    'units_completed', 'checkins',
]


def case(binary, q_manifest, q_index, q_hex, kind, reference, candidate):
    def arm(mode):
        width = 1 if mode == 'serial' else 32
        return [str(binary), str(q_manifest), str(q_index), mode,
                '128', '12', '8192', str(width), '64', '--strict-output']

    return {
        'id': f'q{q_index:02d}-{kind}', 'kind': kind,
        'q_index': q_index, 'q_hex': q_hex,
        'reference_mode': reference, 'candidate_mode': candidate,
        'reference_width': 1 if reference == 'serial' else 32,
        'candidate_width': 1 if candidate == 'serial' else 32,
        'reference': arm(reference), 'candidate': arm(candidate),
    }


def main():
    parser = argparse.ArgumentParser()
    for name in ('binary', 'q-manifest', 'source-bundle', 'allocation-receipt',
                 'allocation-id', 'execution-cpu', 'out'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    binary = Path(args.binary).resolve(strict=True)
    q_manifest = Path(args.q_manifest).resolve(strict=True)
    bundle = Path(args.source_bundle).resolve(strict=True)
    allocation = Path(args.allocation_receipt).resolve(strict=True)
    execution_cpu = int(args.execution_cpu)
    if execution_cpu < 0 or any(not x.is_file() for x in (binary, bundle, allocation)):
        parser.error('invalid benchmark artifact or execution CPU')
    if sha256(q_manifest.read_bytes()).hexdigest() != Q_SHA256:
        parser.error('fresh-target Q manifest hash mismatch')
    fixtures = json.loads(q_manifest.read_text())['fixtures']
    if len(fixtures) != 32 or len({x['q_hex'] for x in fixtures}) != 32:
        parser.error('require 32 distinct frozen Q points')
    cases = []
    for index, fixture in enumerate(fixtures):
        q = fixture['q_hex']
        cases.extend((
            case(binary, q_manifest, index, q, 'comparison', 'serial', 'batch'),
            case(binary, q_manifest, index, q, 'aa_serial', 'serial', 'serial'),
            case(binary, q_manifest, index, q, 'aa_batch', 'batch', 'batch'),
        ))
    plan = {
        'schema': 1, 'measurement_tier': 'tier0_physical_host',
        'objective': 'fresh-Q full collaborative rho lane charged setup plus walking',
        'workdir': str(binary.parent),
        'allocation': {'id': args.allocation_id, 'receipt': str(allocation)},
        'isolation': {'execution_cpu': execution_cpu},
        'artifacts': [str(binary), str(q_manifest), str(bundle), str(allocation)],
        'fixture_manifest_sha256': Q_SHA256,
        'repetitions': 4, 'outer_repeats': 1, 'timeout_s': 180,
        'workload': {'walkers_requested': 128, 'dp_bits': 12,
                     'step_cap': 8192, 'checkin_every': 64,
                     'batch_width': 32, 'direct_strict_output': True},
        'pair_fields': PAIR_FIELDS, 'cases': cases,
        'claim_gate': {
            'comparison_lower_95_gt': 1.05,
            'aa_ratio_min': 0.90, 'aa_ratio_max': 1.11,
            'aa_extreme_min': 0.80, 'aa_extreme_max': 1.25,
            'max_aa_outliers': 2,
            'aa_mode_median_min': 0.98, 'aa_mode_median_max': 1.02,
            'aa_q_mode_median_min': 0.95, 'aa_q_mode_median_max': 1.05,
            'max_relative_peak_rss_ratio': 1.10,
            'max_absolute_peak_rss_delta_kib': 65536,
            'host_gate': 'every arm passes physical-host isolation and local-noise bounds',
            'failure_policy': 'retain all invalid arms; leave aggregate speedup unknown',
        },
    }
    output = Path(args.out)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(plan, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'plan': str(output), 'cases': len(cases),
                      'arms': len(cases) * 8,
                      'sha256': sha256(output.read_bytes()).hexdigest()}))


if __name__ == '__main__':
    main()
