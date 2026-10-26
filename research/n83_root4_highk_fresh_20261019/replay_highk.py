#!/usr/bin/env python3
"""Independent exact-base, witness, scalar, and resource replay for one high-K child."""

import importlib.util
import json
import math
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    'n83_highk_replay_group', ROOT / 'research/n83_indexed5_20261013/analyze.py')
group = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(group)
RSS_CAP_BYTES = 24 * 1024**3


def read(path):
    return json.loads(path.read_text())


def counter(usage, section, key):
    def value(side):
        for line in (usage.get(side, {}).get(section) or '').splitlines():
            parts = line.split()
            if len(parts) == 2 and parts[0] == key:
                return int(parts[1])
        return None
    before, after = value('cgroup_before'), value('cgroup_after')
    return after - before if before is not None and after is not None else None


def verify(folder, k, point, scalar, manifest):
    usage_path = folder / 'usage.json'
    if not usage_path.exists():
        return {'status': 'NOT_RUN'}
    usage = read(usage_path)
    if usage.get('status') != 'PASS_PRODUCER':
        return {'status': usage.get('status', 'FAILED'), 'usage': usage}
    summary = read(folder / 'stdout.jsonl')
    rows = [json.loads(line) for line in
            (folder / 'targets_result.jsonl').read_text().splitlines() if line]
    if len(rows) != 1:
        raise ValueError('one target row required')
    slot = manifest['point_decomposition']['root_slot']
    bloom = manifest['point_decomposition']['root_bloom_filter']
    state_bound = 83 * k * (k - 1) // 2 + 42 * k
    request = ((4 * state_bound + 2) // 3 if slot['state_id_bits'] == 30
               else 1 << 29)
    expected = {
        'n': 83, 'a': 1, 'targets': 1, 'targets_solved': 1,
        'targets_failed': 0, 'orbit_columns': k, 'rank': k,
        'factor_base_points': 166 * k, 'regular_states': state_bound,
        'root_table_layout': ('indexed83_5byte_exact_id30'
                              if slot['state_id_bits'] == 30 else
                              'indexed83_5byte_exact'),
        'root_table_slot_bytes': 5, 'root_table_indexed5_enabled': True,
        'root_table_indexed4_enabled': False,
        'root_state_layout': 'root_only83_11byte_states_reconstructed_second_root',
        'root_state_slot_bytes': 11,
        'root_table_request_entries': request,
        'root_table_allocated_bytes': 5 * (1 << (2 * request - 1).bit_length()),
        'indexed5_load75_enabled': slot['state_id_bits'] == 30,
        'indexed5_fixed30_enabled': slot['state_id_bits'] == 29,
        'root_table_indexed5_id30_enabled': slot['state_id_bits'] == 30,
        'root_only11_enabled': True, 'symmetric_s3_states_enabled': True,
        'sparse_canonical_enabled': True, 'root_bloom_enabled': True,
        'root_bloom_profile': 'wide3', 'root_bloom_hashes': 3,
        'root_bloom_bits_per_request': 16,
        'root_bloom_bytes': bloom['allocated_bytes'],
        'target_threads': 2, 'target_cancel': True,
    }
    mismatches = {key: (summary.get(key), value) for key, value in expected.items()
                  if summary.get(key) != value}
    if mismatches:
        raise ValueError(f'exact layout, rank or factor base differs: {mismatches}')
    if summary['root_table_entries'] >= summary['root_table_allocated_bytes'] // 5:
        raise ValueError('root table occupancy exceeds slots')
    dump = read(folder / 'base_dump.jsonl')
    base = group.replay_base(dump, summary)
    if len(base) != 166 * k:
        raise ValueError('factor-base replay count differs')
    target = rows[0]
    indices = target['point_indices']
    if (target['fixture_index'] != 0 or
            group.point_from_json(target['target']) != point or
            int(target['recovered_scalar']) != scalar or
            target['group_verified'] is not True or
            group.multiply(group.G, scalar) != point or
            len(indices) != 4 or any(not isinstance(i, int) or
                                     i < 0 or i >= len(base) for i in indices)):
        raise ValueError('target, scalar or four-point indices differ')
    relation = None
    for index in indices:
        relation = group.add(relation, base[index])
    if relation != point:
        raise ValueError('four-point relation differs')
    phase_keys = ('target_query_ms', 'target_pdp_ms',
                  'target_relation_check_ms', 'target_descent_ms',
                  'target_recovery_check_ms')
    phases = {key: float(target[key]) for key in phase_keys}
    online = float(target['target_ms'])
    if (not math.isfinite(online) or online <= 0 or
            any(not math.isfinite(v) or v < 0 for v in phases.values()) or
            abs(math.fsum(phases.values()) - online) > 1):
        raise ValueError('exclusive online phases differ')
    timing = summary['timing_ms']
    cold = float(timing['process_total'])
    setup = float(timing['setup_total'])
    rank = float(timing['rank_stage'])
    targets = float(timing['targets_total'])
    residual = cold - setup - rank - targets
    if (not all(math.isfinite(v) and v >= 0 for v in
                (cold, setup, rank, targets)) or
            not 0 <= residual <= 1000 or not 0 <= targets - online <= 1000):
        raise ValueError('cold phase accounting differs')
    throttled = counter(usage, 'cpu.stat', 'nr_throttled')
    oom = counter(usage, 'memory.events', 'oom')
    oom_kills = counter(usage, 'memory.events', 'oom_kill')
    resources = {'peak_rss_bytes': usage['peak_rss_bytes'],
                 'major_faults': usage['major_faults'],
                 'cpu_throttled_periods_delta': throttled,
                 'oom_events_delta': oom, 'oom_kills_delta': oom_kills,
                 'status': 'PASS' if usage['peak_rss_bytes'] <= RSS_CAP_BYTES and
                 (throttled, oom, oom_kills) == (0, 0, 0) else 'NOISY_OR_OVER_CAP'}
    return {'status': 'PASS_INDEPENDENT_REPLAY',
            'actual_factor_base_points': len(base), 'effective_columns': k,
            'base_hash': summary['base_hash'],
            'root_table_entries': summary['root_table_entries'],
            'root_table_allocated_bytes': summary['root_table_allocated_bytes'],
            'rank_attempts': summary['rank_attempts'],
            'rank_relations': summary['rank_relations'],
            'cold_process_ms_exploratory': cold, 'setup_ms': setup,
            'rank_stage_ms': rank, 'targets_total_ms': targets,
            'cold_unattributed_ms': residual,
            'one_target_online_ms': online, 'online_phases_ms': phases,
            'target_probes': target['probes'],
            'full_child_wall_seconds': usage['wall_seconds'],
            'resources': resources}
