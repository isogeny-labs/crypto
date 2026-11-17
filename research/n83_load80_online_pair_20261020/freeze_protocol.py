#!/usr/bin/env python3
"""Bind the sealed two-point fixture to the current Pod and fixed schedule."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
DIAG = ROOT / 'research/n83_root4_load80_20261020/load80_diag_protocol.json'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def schedule():
    events = []
    for point in range(2):
        for repetition in range(3):
            events.append({'name': f'p{point:02d}_cal_before_{repetition}',
                           'kind': 'calibration', 'point_index': point,
                           'seed': 2026101601})
        kinds = (('ic', 4405), ('ic', 4500), ('ic', 4549),
                 ('rho', None)) if point == 0 else (
            ('rho', None), ('ic', 4549), ('ic', 4500))
        for kind, k in kinds:
            events.append({'name': f'p{point:02d}_{"k" + str(k) if k else "rho"}',
                           'kind': kind, 'k': k, 'point_index': point,
                           'seed': 2026120500 + point})
        for repetition in range(3):
            events.append({'name': f'p{point:02d}_cal_after_{repetition}',
                           'kind': 'calibration', 'point_index': point,
                           'seed': 2026101601})
    return events


def freeze():
    base = json.loads(DIAG.read_text())
    fixture = json.loads((HERE / 'fixture.json').read_text())
    answers = [json.loads(line) for line in
               (HERE / 'answers.jsonl').read_text().splitlines()]
    protocol = {
        'schema': 'n83.load80-online-pair/v1',
        'status': 'FROZEN_CONDITIONAL_AFTER_K4549_CAPACITY',
        'curve_id': fixture['curve_id'],
        'pod_id': base['pod_id'], 'machine_id': base['machine_id'],
        'provider_stop_utc': base['provider_stop_utc'],
        'cpu_affinity_ids': [0, 1, 2, 3],
        'rust_source_commit': base['rust_source_commit'],
        'ic_source_sha256': base['rust_source_sha256'],
        'rho_source_sha256': digest(ROOT / 'examples/n83_rho_optimization_eval.rs'),
        'runner_sha256': digest(HERE / 'runpod_pair.py'),
        'diag_runner_sha256': base['runner_sha256'],
        'fixture_sha256': digest(HERE / 'fixture.json'),
        'fresh_exclusion_audit_sha256':
            digest(HERE / 'fresh_exclusion_audit.json'),
        'targets_sha256': fixture['targets_sha256'],
        'answers_sha256': fixture['answers_sha256'],
        'rho_config_path': '/workspace/load80_pair_input/rho_config.json',
        'rho_config_sha256': digest(HERE / 'rho_config.json'),
        'calibration_target_path': '/workspace/load80_pair_input/calibration.jsonl',
        'calibration_target_sha256': digest(HERE / 'calibration.jsonl'),
        'algorithm_flags': base['algorithm_flags'],
        'points': [{
            'point_index': index,
            'source_path': f'/workspace/load80_pair_input/p{index:02d}.jsonl',
            'sha256': digest(HERE / f'p{index:02d}.jsonl'),
            'scalar': answers[index]['scalar'],
            'answer_seed_sha256': answers[index]['seed_sha256'],
        } for index in range(2)],
        'events': schedule(),
        'ic_cap_seconds': 1200, 'rho_cap_seconds': 120,
        'global_cap_seconds': 3900, 'archive_reserve_seconds': 900,
        'rss_cap_bytes': base['rss_cap_bytes'],
        'screen_status': 'BOUNDED_UNKNOWN',
        'candidate_id': None,
    }
    path = HERE / 'protocol.json'
    blob = (json.dumps(protocol, indent=2, sort_keys=True) + '\n').encode()
    if path.exists() and path.read_bytes() != blob:
        raise ValueError('frozen protocol differs')
    path.write_bytes(blob)
    print(json.dumps({'protocol_sha256': digest(path),
                      'event_count': len(protocol['events']),
                      'points': len(protocol['points'])}, sort_keys=True))


if __name__ == '__main__':
    freeze()
