#!/usr/bin/env python3
"""Recompute descriptive comparison values from the complete r5 archive.

The independent Tier 0 audit is authoritative for qualification. This
script exposes the comparison distribution even when an A/A control fails.
"""

import argparse
import hashlib
import json
import math
import random
import statistics
import tarfile
from pathlib import Path


RESULT = 'queue-r3/results/a72dedd301a4490ebf3a5b39b80d7318'
SEED = 0x71CC20261017


def member_json(archive, path):
    handle = archive.extractfile(path)
    if handle is None:
        raise ValueError(f'missing archive member: {path}')
    return json.load(handle)


def member_lines(archive, path):
    handle = archive.extractfile(path)
    if handle is None:
        raise ValueError(f'missing archive member: {path}')
    return [json.loads(line) for line in handle if line.strip()]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('archive', type=Path)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    with tarfile.open(args.archive, 'r:gz') as archive:
        plan = member_json(archive, 'worker-plan-r3.json')
        audit = member_json(archive, 'audit-r5.json')
        runs = member_lines(archive, f'{RESULT}/runs.jsonl')
        pairs = member_lines(archive, f'{RESULT}/pairs.jsonl')
    if len(runs) != 768 or len(pairs) != 384 or audit['completed_pairs'] != 384:
        raise ValueError('incomplete r5 archive')
    cases = {case['id']: case for case in plan['cases']}
    by_q = {q: [] for q in range(32)}
    aa_failures = []
    for pair in pairs:
        case = cases[pair['case']]
        if case['kind'] == 'comparison':
            by_q[case['q_index']].append(pair['speedup'])
        elif case['kind'].startswith('aa_'):
            if not plan['claim_gate']['aa_ratio_min'] <= pair['speedup'] <= plan['claim_gate']['aa_ratio_max']:
                aa_failures.append({'case': pair['case'], 'repeat': pair['repeat'],
                                    'ratio': pair['speedup']})
    if any(len(by_q[q]) != 4 for q in by_q) or aa_failures != audit['aa_failures']:
        raise ValueError('Q clusters or A/A failures differ from independent audit')
    q_logs = [statistics.median(math.log(ratio) for ratio in by_q[q]) for q in range(32)]
    rng = random.Random(SEED)
    draws = sorted(math.exp(statistics.median(q_logs[rng.randrange(32)] for _ in range(32)))
                   for _ in range(10000))
    result = {
        'schema': 'rho-worker-tier0-r5-descriptive-v1',
        'archive_sha256': hashlib.sha256(args.archive.read_bytes()).hexdigest(),
        'audit_decision': audit['decision'],
        'gate_passed': audit['gate_passed'],
        'arms': len(runs),
        'pairs': len(pairs),
        'comparison_pairs': 128,
        'aa_pairs': 256,
        'aa_failures': aa_failures,
        'comparison_ratio_descriptive': math.exp(statistics.median(q_logs)),
        'comparison_q_cluster_bootstrap95_diagnostic': [draws[250], draws[9749]],
        'bootstrap_seed': SEED,
    }
    args.output.write_text(json.dumps(result, sort_keys=True, indent=2) + '\n')
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
