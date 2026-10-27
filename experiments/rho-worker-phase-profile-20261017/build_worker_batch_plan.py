#!/usr/bin/env python3
"""Freeze the 32-Q Tier 1 serial/batch worker panel before dispatch."""

import argparse
from hashlib import sha256
import json
from pathlib import Path


EXPECTED_Q_SHA256 = 'f46ad43788b7f9241927904541cbeb03ea70a96ca95aa84d527a9b22ba8d12b8'
PAIR_FIELDS = [
    'job_id', 'checkin_blake3_time_normalized', 'walkers_done',
    'walk_steps', 'distinguished_points', 'dead_trails',
    'units_completed', 'checkins',
]


def frozen_case(binary, q_manifest, q_index, q_hex, kind, reference, candidate,
                walkers, checkin_every, direct_strict_output=False):
    def command(mode):
        width = 1 if mode == 'serial' else 32
        argv = [str(binary), str(q_manifest), str(q_index), mode,
                str(walkers), '12', '8192', str(width)]
        if checkin_every != 8:
            argv.append(str(checkin_every))
        if direct_strict_output:
            argv.append('--strict-output')
        return argv

    return {
        'id': f'q{q_index:02d}-{kind}',
        'kind': kind,
        'q_index': q_index,
        'q_hex': q_hex,
        'reference_mode': reference,
        'candidate_mode': candidate,
        'reference_width': 1 if reference == 'serial' else 32,
        'candidate_width': 1 if candidate == 'serial' else 32,
        'reference': command(reference),
        'candidate': command(candidate),
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--q-manifest', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    parser.add_argument('--allocation-receipt', type=Path, required=True)
    parser.add_argument('--allocation-id', required=True)
    parser.add_argument('--execution-cpu', type=int, required=True)
    parser.add_argument('--walkers', type=int, default=32)
    parser.add_argument('--checkin-every', type=int, default=8)
    parser.add_argument('--direct-strict-output', action='store_true')
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    q_manifest = args.q_manifest.resolve(strict=True)
    bundle = args.source_bundle.resolve(strict=True)
    allocation_receipt = args.allocation_receipt.resolve(strict=True)
    if not binary.is_file() or not bundle.is_file() or not allocation_receipt.is_file():
        parser.error('binary, source bundle, and allocation receipt must be files')
    if args.execution_cpu < 0:
        parser.error('execution CPU must be nonnegative')
    if not 1 <= args.checkin_every <= args.walkers <= 256 or args.walkers < 32:
        parser.error('require 32 <= walkers <= 256 and 1 <= checkin-every <= walkers')
    if args.direct_strict_output and args.checkin_every == 8:
        parser.error('direct strict output requires an explicit check-in interval')
    raw_q = q_manifest.read_bytes()
    if sha256(raw_q).hexdigest() != EXPECTED_Q_SHA256:
        parser.error('frozen Q manifest hash mismatch')
    fixtures = json.loads(raw_q)['fixtures']
    if len(fixtures) != 32:
        parser.error('expected exactly 32 frozen Q points')
    cases = []
    for q_index, fixture in enumerate(fixtures):
        q_hex = fixture['q_hex']
        cases.extend([
            frozen_case(binary, q_manifest, q_index, q_hex,
                        'comparison', 'serial', 'batch', args.walkers,
                        args.checkin_every, args.direct_strict_output),
            frozen_case(binary, q_manifest, q_index, q_hex,
                        'aa_serial', 'serial', 'serial', args.walkers,
                        args.checkin_every, args.direct_strict_output),
            frozen_case(binary, q_manifest, q_index, q_hex,
                        'aa_batch', 'batch', 'batch', args.walkers,
                        args.checkin_every, args.direct_strict_output),
        ])
    plan = {
        'schema': 1,
        'measurement_tier': 'tier1_instance',
        'objective': 'same-Q full collaborative rho lane charged setup plus walking',
        'workdir': str(binary.parent),
        'allocation': {'id': args.allocation_id, 'receipt': str(allocation_receipt)},
        'isolation': {'execution_cpu': args.execution_cpu},
        'artifacts': [str(binary), str(q_manifest), str(bundle),
                      str(allocation_receipt)],
        'fixture_manifest_sha256': EXPECTED_Q_SHA256,
        'repetitions': 4,
        'outer_repeats': 1,
        'timeout_s': 180,
        'workload': {
            'walkers_requested': args.walkers,
            'dp_bits': 12,
            'step_cap': 8192,
            'checkin_every': args.checkin_every,
            'batch_width': 32,
        },
        'pair_fields': PAIR_FIELDS,
        'cases': cases,
        'claim_gate': {
            'comparison_lower_95_gt': 1.05,
            'aa_ratio_min': 0.90,
            'aa_ratio_max': 1.11,
            'max_relative_peak_rss_ratio': 1.10,
            'max_absolute_peak_rss_delta_kib': 65536,
            'host_gate': 'all arms pass the frozen Tier 1 counter and topology checks',
            'failure_policy': 'retain invalid arms and leave aggregate speedup unknown',
        },
    }
    if args.direct_strict_output:
        plan['workload']['direct_strict_output'] = True
        plan['measurement_tier'] = 'tier0_physical_host'
        plan['claim_gate']['host_gate'] = 'every arm passes frozen physical-host isolation and local-noise bounds'
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(json.dumps(plan, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'plan': str(args.out), 'cases': len(cases),
                      'arms': len(cases) * 4 * 2,
                      'sha256': sha256(args.out.read_bytes()).hexdigest()}))


if __name__ == '__main__':
    main()
