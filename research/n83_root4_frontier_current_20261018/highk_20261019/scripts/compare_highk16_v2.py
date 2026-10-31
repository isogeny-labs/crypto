#!/usr/bin/env python3
"""Exploratory paired diagnostics on independently replayed exposed controls."""
import hashlib
import json
import math
from pathlib import Path
import random

ROOT = Path('/private/tmp/n83_current_id30_frontier_20261019')
ARMS = {
    'k4096_standard': ROOT / 'k4096_16_pilot',
    'k4300_standard': ROOT / 'k4300_16_pilot',
    'k4300_wide3': ROOT / 'k4300_wide3_pilot',
    'k4405_standard': ROOT / 'k4405_pilot',
    'k4405_wide3': ROOT / 'k4405_wide3_pilot',
}
PAIRS = [('k4300_standard', 'k4096_standard'),
         ('k4300_standard', 'k4405_standard'),
         ('k4405_wide3', 'k4405_standard'),
         ('k4405_wide3', 'k4096_standard'),
         ('k4405_wide3', 'k4300_standard'),
         ('k4300_wide3', 'k4300_standard'),
         ('k4300_wide3', 'k4096_standard'),
         ('k4300_wide3', 'k4405_wide3')]
SEED = 'n83-highk16-exposed-paired-diagnostic-v2'
N_BOOT = 100000


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def arm(path):
    analysis = json.loads((path / 'analysis.json').read_text())
    if analysis['status'] != 'PASS_INDEPENDENT_REPLAY':
        raise ValueError(f'{path}: replay gate failed')
    targets = [json.loads(row) for row in (path / 'targets_result.jsonl').read_text().splitlines()]
    by_index = {row['fixture_index']: row for row in targets}
    if len(targets) != 16 or sorted(by_index) != list(range(16)):
        raise ValueError(f'{path}: missing or duplicate point')
    timing = [float(by_index[i]['target_ms']) for i in range(16)]
    if any(not math.isfinite(x) or x <= 0 for x in timing):
        raise ValueError(f'{path}: invalid timing')
    return {
        'analysis_sha256': sha(path / 'analysis.json'),
        'base_dump_sha256': sha(path / 'base_dump.jsonl'),
        'fixture_sha256': analysis['fixture_sha256'],
        'mean_online_ms_diagnostic': sum(timing) / len(timing),
        'median_online_ms_diagnostic': sorted(timing)[7:9],
        'online_ms_diagnostic': timing,
        'cold_wall_seconds_diagnostic': analysis['cold_wall_seconds_diagnostic'],
        'root_table_entries': analysis['root_table_entries'],
        'root_table_allocated_bytes': analysis['root_table_allocated_bytes'],
        'peak_sampled_rss_bytes': analysis['peak_sampled_rss_bytes'],
    }


def paired(first, second):
    differences = [a - b for a, b in zip(first, second)]
    mean = sum(differences) / len(differences)
    rng = random.Random(SEED + ':' + hashlib.sha256(str(differences).encode()).hexdigest())
    boot = sorted(sum(differences[rng.randrange(16)] for _ in range(16)) / 16
                  for _ in range(N_BOOT))
    return {
        'mean_difference_ms_first_minus_second': mean,
        'ratio_of_means_first_over_second': (sum(first) / sum(second)),
        'first_faster_count': sum(x < 0 for x in differences),
        'first_slower_count': sum(x > 0 for x in differences),
        'exploratory_bootstrap_95_interval_ms': [boot[2500], boot[97500]],
        'difference_ms_by_fixture_index': differences,
    }


def main():
    values = {name: arm(path) for name, path in ARMS.items()}
    fixtures = {row['fixture_sha256'] for row in values.values()}
    if len(fixtures) != 1:
        raise ValueError('arms used different fixture digests')
    if values['k4405_standard']['base_dump_sha256'] != values['k4405_wide3']['base_dump_sha256']:
        raise ValueError('wide3 changed the frozen K4405 factor base')
    if values['k4300_standard']['base_dump_sha256'] != values['k4300_wide3']['base_dump_sha256']:
        raise ValueError('wide3 changed the frozen K4300 factor base')
    results = {f'{a}_minus_{b}': paired(values[a]['online_ms_diagnostic'],
                                        values[b]['online_ms_diagnostic'])
               for a, b in PAIRS}
    output = {
        'schema': 'n83.highk16-exposed-paired-diagnostic/v2',
        'hardware_tier': 'local_correctness_diagnostic',
        'status': 'EXPLORATORY_ONLY',
        'selected_online_candidate_id': None,
        'point_count': 16,
        'fixture_sha256': next(iter(fixtures)),
        'bootstrap_resamples': N_BOOT,
        'bootstrap_seed_domain': SEED,
        'arms': values,
        'pairs': results,
    }
    path = ROOT / 'highk16_paired_diagnostic_v2.json'
    path.write_text(json.dumps(output, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'status': output['status'],
                      'means_ms': {k: v['mean_online_ms_diagnostic'] for k, v in values.items()},
                      'pairs': {k: v['exploratory_bootstrap_95_interval_ms']
                                for k, v in results.items()}}, sort_keys=True))


if __name__ == '__main__':
    main()
