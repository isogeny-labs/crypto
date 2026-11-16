#!/usr/bin/env python3
"""Validator recompute of every arm-row of remeasure.json from its raw counts.

Reads only the committed-to-be raw part (remeasure.json) and the bench record;
recomputes band totals, calibrated totals, ratios to negation / plain / best rho,
and classes, with the validator's own code; compares with the stored fields and
with the record's metrics. Writes a JSON summary to argv[1]."""
import json, sys, math, os

ROOT = '/home/user/crypto'
D = json.load(open(f'{ROOT}/research/negation_remeasure_20261113/remeasure.json'))
REC = json.load(open(f'{ROOT}/benchmarks/records/20261113T0320Z-negation-remeasure.json'))
BAND = [0.0, 0.1, 1.0]
CALF = [0.0, 1.0, 10.0]
TOL = 1e-9

def close(a, b, tol=TOL):
    if isinstance(a, str) or isinstance(b, str):
        return a == b
    if a is None or b is None:
        return a is b
    return abs(a - b) <= tol * max(1.0, abs(a), abs(b))

def rho_counts(m):
    return m['total_ops'], {'canon': m['canonicalisations'], 'hash': m['partition_hashes'],
                            'lookup': m['dp_lookups'], 'insert': m['dp_inserts'], 'word': m['word_ops']}

def band(ops, nat):
    s = sum(nat.values())
    return [ops + w * s for w in BAND]

def cal(ops, nat, times, tadd):
    return [ops + f * sum(times[c] / tadd * n for c, n in nat.items()) for f in CALF]

def cls(a, r, lb):
    above = all(x > y for x, y in zip(a, r))
    below = all(x < y for x, y in zip(a, r))
    if above: return 'above'
    if below and not lb: return 'below'
    if lb: return 'unresolved (lower bound)'
    return 'unresolved'

def tadd_key(target, arm):
    if target == 'v3':
        return 'J_recorded_impl' if arm in ('recorded', 'L') else 'J_word_impl'
    if target == 'cover_pushforward':
        if arm in ('A', 'B', 'C'): return 'J_word_impl'
        if arm.startswith('ic_jc'): return 'J_recorded_impl'
        return 'E1'
    return 'J_g2' if arm.startswith('g2') else 'J_g3'

def rho_tadd_key(target):
    return 'E' if target == 'tower_large' else 'E1'

out = {'arm_rows': 0, 'mismatches': [], 'classes': {}, 'rows': []}
for target, T in D['targets'].items():
    for row in T['rows']:
        p = row['p']
        times = row['class_times_ns']
        ta = row['t_add_ns']
        neg_ops, neg_nat = rho_counts(row['rho']['negation'])
        pl_ops, pl_nat = rho_counts(row['rho']['plain'])
        # check rho native and native_per_step
        for kind, (o, n) in (('negation', (neg_ops, neg_nat)), ('plain', (pl_ops, pl_nat))):
            m = row['rho'][kind]
            if not close(sum(n.values()), m['native']):
                out['mismatches'].append([target, p, 'rho_' + kind, 'native', m['native'], sum(n.values())])
            if not close(m['native'] / max(m['steps'], 1.0), m['native_per_step']):
                out['mismatches'].append([target, p, 'rho_' + kind, 'native_per_step'])
            if m['certified'] != m['runs']:
                out['mismatches'].append([target, p, 'rho_' + kind, 'certified<runs', m['certified'], m['runs']])
            if not close(m['walk_ops'] + m['other_ops'], m['total_ops']):
                out['mismatches'].append([target, p, 'rho_' + kind, 'walk+other!=total'])
            if not close(m['key_evaluations'], (2 * m['canonicalisations'] if kind == 'negation' else 0) + m['dp_lookups'] + m['partition_hashes']):
                out['mismatches'].append([target, p, 'rho_' + kind, 'key identity'])
        rt = ta[rho_tadd_key(target)]
        nb, pb = band(neg_ops, neg_nat), band(pl_ops, pl_nat)
        nc, pc = cal(neg_ops, neg_nat, times, rt), cal(pl_ops, pl_nat, times, rt)
        bb = [min(a, b) for a, b in zip(nb, pb)]
        bc = [min(a, b) for a, b in zip(nc, pc)]
        rrow = {'target': target, 'p': p, 'rho_neg_band': nb, 'rho_neg_cal': nc, 'arms': {}}
        for name, A in row['arms'].items():
            out['arm_rows'] += 1
            ops = A['group_ops']
            nat = A['native']
            lb = A['lower_bound']
            tk = tadd_key(target, name)
            tadd = ta[tk]
            if not close(tadd, A['t_add_ns']):
                out['mismatches'].append([target, p, name, 't_add mapping', tk, tadd, A['t_add_ns']])
            tb = band(ops, nat)
            tc = cal(ops, nat, times, tadd)
            calc = {
                'total_band': tb, 'total_cal': tc,
                'ratio_neg_band': [a / b for a, b in zip(tb, nb)],
                'ratio_neg_cal': [a / b for a, b in zip(tc, nc)],
                'ratio_plain_band': [a / b for a, b in zip(tb, pb)],
                'ratio_best_band': [a / b for a, b in zip(tb, bb)],
                'class_neg_band': cls(tb, nb, lb), 'class_neg_cal': cls(tc, nc, lb),
                'class_plain_band': cls(tb, pb, lb), 'class_best_band': cls(tb, bb, lb),
                'class_best_cal': cls(tc, bc, lb),
            }
            for k, v in calc.items():
                s = A[k]
                if isinstance(v, list):
                    if not all(close(x, y) for x, y in zip(v, s)):
                        out['mismatches'].append([target, p, name, k, s, v])
                elif v != s:
                    out['mismatches'].append([target, p, name, k, s, v])
            if 'certified' in A:
                c = A['certified']
                runs = A.get('runs', A.get('seeds'))
                if isinstance(c, bool):
                    if not c: out['mismatches'].append([target, p, name, 'not certified'])
                elif runs is not None and c != runs:
                    out['mismatches'].append([target, p, name, 'certified<runs', c, runs])
            if A.get('exhausted'):
                out['mismatches'].append([target, p, name, 'exhausted', A['exhausted']])
            out['classes'].setdefault(calc['class_neg_band'], 0)
            out['classes'][calc['class_neg_band']] += 1
            rrow['arms'][name] = {'group_ops': ops, 'lower_bound': lb, **calc}
        out['rows'].append(rrow)

# record metrics
idx = {}
for r in out['rows']:
    idx[(r['target'], r['p'])] = r
tmap = {'v3': 'v3', 'cover': 'cover_pushforward', 'tower': 'tower_large'}
mm = []
checked = 0
for m in REC['metrics']:
    n = m['name']
    parts = n.split('_')
    tgt = tmap[parts[0]]; p = int(parts[1][1:])
    row = idx[(tgt, p)]
    if 'native_per_step' in n:
        kind = parts[3]
        raw = [x for x in D['targets'][tgt]['rows'] if x['p'] == p][0]['rho'][kind]
        val = raw['native_per_step']
        ok = close(val, m['value'])
        checked += 1
        if not ok: mm.append([n, m['value'], val])
        continue
    w = n.rsplit('_w', 1)[1]
    wi = {'0': 0, '0.1': 1, '1': 2}[w]
    armkey = n.split('_', 2)[2].rsplit('_total', 1)[0]
    if tgt == 'tower_large':
        g, a = armkey.split('_')
        armkey = f"{g}_{a.upper() if a == 'c' else a}"
    elif armkey == 'c':
        armkey = 'C'
    A = row['arms'][armkey]
    ok = close(A['total_band'][wi], m['value']) and close(row['rho_neg_band'][wi], m['reference'])
    checked += 1
    if not ok: mm.append([n, m['value'], A['total_band'][wi], m['reference'], row['rho_neg_band'][wi]])
out['record_metrics_checked'] = checked
out['record_metric_mismatches'] = mm
json.dump(out, open(sys.argv[1], 'w'), indent=1)
print('arm rows', out['arm_rows'], 'mismatches', len(out['mismatches']), 'classes', out['classes'])
print('record metrics checked', checked, 'mismatches', len(mm))
for x in out['mismatches'][:20]: print(x)
for x in mm[:10]: print(x)
