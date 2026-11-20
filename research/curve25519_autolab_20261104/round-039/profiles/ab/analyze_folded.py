#!/usr/bin/env python3
"""Split the folded stacks of a `tools/profile.sh cpu` run by phase and by leaf cost.
usage: analyze_folded.py <folded.txt> [<folded.txt> ...]   (prints one JSON object per file)
Only samples whose stack contains `solve_ic_prime` (the ecbench solve root) are counted as the solve."""
import collections, json, sys

def phase(stack):
    if 'solve_ic_prime' not in stack: return None
    if 'FactorBaseBuilder' in stack or 'PrimeProjectedAbscissaBase' in stack: return 'factor_base'
    if '7prepare' in stack or 'build_lcheck' in stack or 'TargetWalk' in stack: return 'oracle_setup'
    if 'collect_and_solve_relations' in stack: return 'relations'
    if 'verify' in stack.lower(): return 'verify'
    return 'solve_other'

def leaf(name):
    for key, cat in (('add_raw', 'curve add'), ('hashbrown', 'hashbrown'), ('exc_page_fault', 'page-fault'),
                     ('clear_page', 'page-fault'), ('memcg', 'page-fault'), ('alloc', 'alloc'),
                     ('malloc', 'alloc'), ('memmove', 'memmove'), ('memset', 'memset')):
        if key in name: return cat
    return 'other'

for path in sys.argv[1:]:
    total = solve = 0
    phases, leaves = collections.Counter(), collections.Counter()
    for line in open(path):
        stack, _, n = line.rstrip('\n').rpartition(' ')
        n = int(n); total += n
        p = phase(stack)
        if p is None: continue
        solve += n; phases[p] += n; leaves[leaf(stack.split(';')[-1])] += n
    print(json.dumps({'file': path.rsplit('/', 1)[-1], 'samples_total': total, 'samples_in_solve': solve,
                      'phase_samples': dict(phases), 'leaf_samples': dict(leaves)}, sort_keys=True))
