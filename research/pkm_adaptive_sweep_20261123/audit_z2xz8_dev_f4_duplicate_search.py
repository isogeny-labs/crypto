#!/usr/bin/env python3
"""Account for the second deterministic selected-tower construction."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
RAW = Path('/Volumes/SSD990-2/pkm-z2xz8-dev-f4-fullrank-20261128')
REPLAY = HERE / 'VERIFY_Z2XZ8_DEV_F4_FULLRANK_20261128.json'
REPLAY_SHA = 'e8f5c7294eb51dc48e592129134a0d006f07db3cf60f8ee5e4fb82c4dd1100cc'
OUT = HERE / 'AUDIT_Z2XZ8_DEV_F4_DUPLICATE_SEARCH_20261128.json'
SOURCE = Path('/Volumes/SSD990-2/pkm-oracle-parity-full-20261128')
SOURCE_COMMIT = '5f8ddfd0ff43d151d7b35c5d81003905c59d12fc'
ORACLE_SHA = 'b016eadafeabae244a7980ea9ae14488bd90892722e1581f3bfd135b049393aa'
TOWER_SHA = 'ee3b7f81c520618b1d902657c6b706612dc60a0e0397892e9406b409c3c378e2'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    if sha(REPLAY.read_bytes()) != REPLAY_SHA:
        raise ValueError('independent full-rank replay differs')
    oracle_path = SOURCE / 'src/cryptanalysis/ic_framework/pkm_semaev.rs'
    tower_path = SOURCE / 'src/cryptanalysis/ic_framework/prime_tower.rs'
    if sha(oracle_path.read_bytes()) != ORACLE_SHA or sha(tower_path.read_bytes()) != TOWER_SHA:
        raise ValueError('pinned construction source differs')
    oracle_source = oracle_path.read_text()
    tower_source = tower_path.read_text()
    if ('z2xz8_equal_size_tower(&inst.curve, &mut scratch)' not in oracle_source
            or 'z2xz8_equal_size_tower(&self.instance.curve, &mut aux_ops)' not in tower_source):
        raise ValueError('deterministic duplicate call path differs')
    replay = json.loads(REPLAY.read_bytes())
    receipts_path = RAW / 'receipts.jsonl'
    if sha(receipts_path.read_bytes()) != replay['receipts_sha256']:
        raise ValueError('raw receipt store differs')
    receipts = {r['id']: r for r in map(json.loads, receipts_path.read_text().splitlines())}
    if len(receipts) != 13 or receipts['header']['source_commit'] != SOURCE_COMMIT:
        raise ValueError('receipt coverage or source differs')
    rows = []
    for curve in ('t5_c0', 't5_c1', 't5_c2'):
        for arm in ('candidate_a', 'interval_a'):
            case = f'{curve}:{arm}'
            saved = receipts['result:' + case]
            report = json.loads(Path(receipts['launch:' + case]['stdout_path']).read_bytes())
            if sha(Path(receipts['launch:' + case]['stdout_path']).read_bytes()) != saved['stdout']['sha256']:
                raise ValueError(f'raw hash differs: {case}')
            row = report['rows'][0]
            fb = row['factor_base']['cost']
            native = fb['native']
            repeated = 0
            if arm == 'candidate_a':
                repeated = native['aux_search_gae'] + native['aux_tower_build_gae']
                if repeated != fb['gae'] or repeated != fb['group_ops']['adds'] + fb['group_ops']['doubles']:
                    raise ValueError(f'search group ledger differs: {case}')
            elif fb['gae'] != 0:
                raise ValueError(f'interval base has unexpected group work: {case}')
            rows.append({'curve': curve, 'arm': arm, 'reported_partial_gae': row['total_gae'],
                         'second_tower_group_gae': repeated,
                         'adjusted_partial_gae_minimum': row['total_gae'] + repeated,
                         'second_tower_native_counters_duplicated': ({k: v for k, v in native.items()
                             if k.startswith('aux_') or k == 'velu_evaluations_uncharged'} if repeated else {}),
                         'peak_rss_kib': saved['peak_rss_kib'],
                         'process_cpu_s_contended': saved['process_cpu_s'],
                         'stdout_sha256': saved['stdout']['sha256']})
    pairs = []
    for curve in ('t5_c0', 't5_c1', 't5_c2'):
        candidate = next(r for r in rows if r['curve'] == curve and r['arm'] == 'candidate_a')
        interval = next(r for r in rows if r['curve'] == curve and r['arm'] == 'interval_a')
        pairs.append({'curve': curve,
                      'adjusted_partial_candidate_over_interval': candidate['adjusted_partial_gae_minimum']/interval['adjusted_partial_gae_minimum']})
    output = {'schema': 'pkm-z2xz8-dev-f4-duplicate-search-audit/v1',
              'source_commit': SOURCE_COMMIT, 'oracle_source_sha256': ORACLE_SHA,
              'tower_source_sha256': TOWER_SHA, 'independent_replay_sha256': REPLAY_SHA,
              'raw_receipts_sha256': sha(receipts_path.read_bytes()),
              'method': 'from_spec and factor-base build each execute the same deterministic selected-tower search; add the factor-base auxiliary group ledger once more to the reported lower bound',
              'qualification': 'This is still a partial lower bound: duplicate field arithmetic, coefficient and Jacobi work, Velu evaluations, rank normalization and other positive native work remain unpriced. Development curves on a contended host.',
              'complete_cold_cost': False, 'rows': rows, 'pairs': pairs}
    data = (json.dumps(output, sort_keys=True, indent=2) + '\n').encode()
    if OUT.exists() and OUT.read_bytes() != data:
        raise ValueError('existing audit differs')
    OUT.write_bytes(data)
    print(json.dumps({'sha256': sha(data), 'pairs': pairs}, sort_keys=True))


if __name__ == '__main__':
    main()
