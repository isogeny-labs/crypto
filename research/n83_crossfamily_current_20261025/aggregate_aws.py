#!/usr/bin/env python3
"""Replay two raw EC2 Spot archives and apply the frozen N83 selector."""

import argparse
import json
from pathlib import Path

from research.n83_crossfamily_current_20261025 import aggregate_runpod
from research.n83_crossfamily_current_20261025 import aws_provider
from research.n83_crossfamily_current_20261025 import run as runner
from research.n83_crossfamily_current_20261025 import select as selector


def aggregate(strata, snapshot, head):
    first = strata[0]
    source_keys = (
        'hashes', 'source_sha256', 'binary_sha256', 'rho_binary_sha256',
        'runner_sha256', 'targets_sha256', 'answers_sha256',
        'fixture_manifest_sha256', 'arms', 'choice', 'rho_seed_rule',
        'calibration_seed', 'calibration_repetitions_per_side',
        'calibration_comparison', 'calibration_target_sha256',
        'calibration_answer_sha256', 'global_producer_cap_seconds',
        'ic_cap_seconds', 'layout_receipt_sha256',
        'source_archive_sha256', 'source_manifest_sha256',
        'aws_adapter_sha256')
    same_source = all(item['source'].get(key) == first['source'].get(key)
                      for item in strata for key in source_keys)
    proof = aws_provider.verify(snapshot, strata, head)
    points = sorted(
        [point for item in strata for point in item['analysis']['points']],
        key=lambda item: item['point_index'])
    candidate_ids = [arm[3] for arm in first['source']['arms']]
    base_hashes = {
        candidate: {arm['raw_replay'].get('base_hash') for point in points
                    for arm in point['arms']
                    if arm['candidate_id'] == candidate and
                    arm['status'] == 'PASS_INDEPENDENT_REPLAY'}
        for candidate in candidate_ids}
    same_base = all(len(hashes) == 1 and None not in hashes
                    for hashes in base_hashes.values())
    coverage = (len(points) == 16 and
                {point['point_index'] for point in points} == set(range(16)))
    evidence = ('PASS_INDEPENDENT_REPLAY' if
                same_source and same_base and coverage and
                all(item['analysis']['evidence_status'] ==
                    'PASS_INDEPENDENT_REPLAY' for item in strata) else
                'VERIFICATION_FAILED')
    validated = {
        'schema': 'n83.crossfamily-current-validated/v1',
        'provider': 'aws_ec2_spot',
        'curve_id': runner.generator.CURVE_ID,
        'subgroup_order': runner.generator.group.R,
        'source_sha256': runner.SOURCE_SHA256,
        'targets_sha256': runner.TARGETS_SHA256,
        'answers_sha256': runner.ANSWERS_SHA256,
        'candidate_ids': candidate_ids,
        'choice': first['source']['choice'],
        'evidence_status': evidence,
        'head_sha': head,
        'same_source_and_binary': same_source,
        'different_cloud_instances': proof['status'] ==
        'PASS_TWO_DISTINCT_CONCURRENT_AWS_SPOT_INSTANCES',
        'provider_runner_identity': proof,
        'same_candidate_bases_across_strata': same_base,
        'host_strata': [
            {'stratum': item['stratum'],
             'point_indices': item['analysis']['point_indices'],
             'tier_status': item['analysis']['tier_status'],
             'aa_status': item['analysis']['aa_status'],
             'instance_id': item['host']['instance_id'],
             'raw_archive_sha256': item['archive_sha256'],
             'raw_file_count': item['file_count']}
            for item in strata],
        'points': points,
    }
    return validated, selector.select(validated)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--stratum0', type=Path, required=True)
    parser.add_argument('--stratum1', type=Path, required=True)
    parser.add_argument('--archive0', type=Path, required=True)
    parser.add_argument('--archive1', type=Path, required=True)
    parser.add_argument('--head-sha', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    strata = [aggregate_runpod.replay(i, results, archive, args.output) for
              i, (results, archive) in enumerate(
                  ((args.stratum0, args.archive0),
                   (args.stratum1, args.archive1)))]
    validated, selection = aggregate(
        strata, json.loads(args.snapshot.read_text()), args.head_sha)
    for name, value in (('validated.json', validated),
                        ('selection.json', selection)):
        (args.output / name).write_text(json.dumps(value, sort_keys=True,
                                                  indent=2) + '\n')
    print(json.dumps({'status': selection['admission_status'],
                      'selected_online_candidate_id':
                      selection['online'].get('selected_candidate_id'),
                      'selected_cold_candidate_id':
                      selection['cold'].get('selected_candidate_id')},
                     sort_keys=True))


if __name__ == '__main__':
    main()
