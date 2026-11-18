#!/usr/bin/env python3
"""Render the validator's tables (markdown) from evidence/recompute.json, evidence/certcheck.jsonl and the raw records.
usage: make_tables.py <J2 dir> <item dir> > evidence/tables.md"""
import json, sys, os, glob, statistics as st
J2, D = sys.argv[1:3]
E = os.path.join(D, 'evidence')
RC = json.load(open(os.path.join(E, 'recompute.json')))
CERT = [json.loads(l) for l in open(os.path.join(E, 'certcheck.jsonl'))]
def recs(f): return [json.loads(l) for l in open(f) if l.startswith('{')]
K = {65479: 16, 262111: 18, 1048507: 20, 2097091: 21, 4194271: 22}
def fmt(x, d=3): return f'{x:.{d}f}'
print('## T1 first-attempt success and paired DL1 cost ratio to c = 2.00 (M-eq, recomputed, w = 0; min-max over seeds)\n')
print('| k (q) | set | seeds | c | first-attempt success | attempts per seed | ratio w=0 | ratio w=0.1 | ratio w=1 |')
print('|---|---|---|---|---|---|---|---|---|')
for q in (65479, 262111, 1048507, 2097091, 4194271):
    for kind in ('grid', 'held'):
        for c in ('1.40', '1.50', '1.60', '1.75', '2.00'):
            key = f'{q}|{kind}|{c}'
            if key not in RC['first_attempt_success']: continue
            s = RC['first_attempt_success'][key]
            r = [RC['paired_ratio_to_c2.00'].get(f'{key}|w={w}') for w in (0.0, 0.1, 1.0)]
            rr = ' | '.join(f"{fmt(x['lo'])}-{fmt(x['hi'])}" if x else '-' for x in r)
            print(f"| {K[q]} ({q:,}) | {'held-out' if kind == 'held' else 'grid'} | {'4-8' if kind == 'held' else '1-3'} | {c} | {s['succ']}/{s['n']} | {','.join(str(a) for a in s['attempts'])} | {rr} |")
print('\n## T2 per-seed order c=1.60 < 1.75 < 2.00 (seeds where it holds / seeds), by pricing reading\n')
print('| k | set | w=0 frozen | w=0.1 frozen | w=1 frozen | w=0 strict | w=0.1 strict | w=1 strict |')
print('|---|---|---|---|---|---|---|---|')
for q in (65479, 262111, 1048507, 2097091, 4194271):
    for kind in ('grid', 'held'):
        if f'{q}|{kind}|w=0.0' not in RC['band_frozen_rule']: continue
        f = [RC['band_frozen_rule'][f'{q}|{kind}|w={w}'] for w in (0.0, 0.1, 1.0)]; s = [RC['band_strict_add_ladd'][f'{q}|{kind}|w={w}'] for w in (0.0, 0.1, 1.0)]
        print(f"| {K[q]} | {'held-out' if kind == 'held' else 'grid'} | " + ' | '.join(f"{x['order_holds_seeds']}/{x['n']}" for x in f + s) + ' |')
print('\n## T3 price at which an addition (F_q add and mod-ell add, M-eq each) erases the c = 1.60 gain; ratio c1.60/c2.00 at selected prices\n')
print('| k | set | break-even M-eq per addition | ratio at 0.05 | at 0.25 | at 0.5 | at 1.0 | at 2.0 |')
print('|---|---|---|---|---|---|---|---|')
for key, v in sorted(RC['add_ladd_weight_sweep'].items(), key=lambda kv: (int(kv[0].split('|')[0]), kv[0])):
    q, kind = key.split('|'); q = int(q)
    cells = ' | '.join(f"{fmt(v[f'wx={w}|c=1.60'][2], 3)}" for w in (0.05, 0.25, 0.5, 1.0, 2.0))
    print(f"| {K[q]} | {'held-out' if kind == 'held' else 'grid'} | {fmt(v['breakeven_wx_c1.60_M-eq_per_add_or_ladd'], 3)} | {cells} |")
print('\n## T4 unpriced counters per arm (mean per solve, every attempt, all phases)\n')
print('| arm | q | c | mul | inv | lmul | hash | cmp | add (F_q) | ladd (mod ell) |')
print('|---|---|---|---|---|---|---|---|---|---|')
PH = ['fb', 'relations', 'matrix', 'linear_algebra', 'log_recovery', 'descent', 'verify']
for q in (2097091, 4194271):
    for c in ('1.60', '1.75', '2.00'):
        tot = {k: [] for k in ('mul', 'inv', 'lmul', 'hash', 'cmp', 'add', 'ladd')}
        for sd in (4, 5, 6, 7, 8):
            rs = recs(os.path.join(J2, 'heldout', f'ic_{q}_{sd}_c{c}.jsonl'))
            for k in tot: tot[k].append(sum(r.get(f'{p}.{k}', 0) for r in rs if r.get('rec') == 'ic' and r['arm'] == 'DL1' for p in PH))
        print(f"| DL1 held-out seeds 4-8 | {q:,} | {c} | " + ' | '.join(f"{st.mean(tot[k]):.3e}" for k in tot) + ' |')
for q in (2097091, 4194271):
    for arm in ('plain', 'neg'):
        v = RC['rho'][f'{arm}|{q}|all|j2_calib']; u = v['unpriced_per_run_mean']
        print(f"| rho-{arm} (N4v2 raw, {v['runs']} runs) | {q:,} | - | - | - | - | {u['hash']:.3e} | {u['cmp']:.3e} | {u['add']:.3e} | {u['ladd']:.3e} |")
print('\n(rho mul/inv/lmul per run: see evidence/recompute.json `rho`; every rho counter listed in the N4v2 raw records is emitted.)\n')
print('## T5 IC (DL1) vs rho, mean M-eq per solve, J2 calibration, frozen rule (unpriced = hash + cmp), w = 0 / 0.1 / 1 GAE\n')
print('| k | arm | w=0 | w=0.1 | w=1 |')
print('|---|---|---|---|---|')
for q in (2097091, 4194271):
    for c in ('1.60', '1.75', '2.00'):
        v = RC['ic_vs_rho'][f'{q}|j2_calib|frozen_rule|c={c}|neg']; print(f"| {K[q]} | DL1 c={c} | " + ' | '.join(f'{x:.3e}' for x in v['ic']) + ' |')
    for arm in ('plain', 'neg'):
        v = RC['ic_vs_rho'][f'{q}|j2_calib|frozen_rule|c=1.60|{arm}']; print(f"| {K[q]} | rho-{arm} | " + ' | '.join(f'{x:.3e}' for x in v['rho']) + ' |')
print('\n| k | DL1 c | vs rho-plain w=0/0.1/1 | vs rho-neg w=0/0.1/1 | vs rho-plain strict | vs rho-neg strict |')
print('|---|---|---|---|---|---|')
for q in (2097091, 4194271):
    for c in ('1.60', '1.75', '2.00'):
        g = lambda arm, mode: ' / '.join(f"{x:.4f}" for x in RC['ic_vs_rho'][f'{q}|j2_calib|{mode}|c={c}|{arm}']['ratio'])
        print(f"| {K[q]} | {c} | {g('plain', 'frozen_rule')} | {g('neg', 'frozen_rule')} | {g('plain', 'strict')} | {g('neg', 'strict')} |")
print('\n## T6 measured CPU and wall of the same DL1 solves, ratio c / c=2.00 (per-seed median [min-max]); L0 host\n')
print('| source | k | set | c | total CPU | total wall | LA-phase CPU | relations-phase CPU |')
print('|---|---|---|---|---|---|---|---|')
T = RC['cpu_wall_ratios_to_c2.00']
def cell(key):
    v = T.get(key); return f"{st.median(v):.3f} [{min(v):.3f}-{max(v):.3f}]" if v else '-'
for tag in ('producer_records', 'validator_replay'):
    for q, kind in ((262111, 'grid'), (1048507, 'grid'), (2097091, 'held'), (4194271, 'grid'), (4194271, 'held')):
        for c in ('1.60', '1.75'):
            b = f'{tag}|{q}|{kind}|c={c}'
            print(f"| {tag} | {K[q]} | {'held-out' if kind == 'held' else 'grid'} | {c} | {cell(b + '|cpu_total')} | {cell(b + '|wall_total')} | {cell(b + '|cpu_LA_phase') if c == '1.60' else '-'} | {cell(b + '|cpu_relations_phase') if c == '1.60' else '-'} |")
print('\n## T7 independent certificates (own C code), 22 instances\n')
print('| q | seed | tries until match (committed N4v2 tries) | ell | order = 4 ell | k planted (from seed) | recorded k_found checked | j in F_q | pass |')
print('|---|---|---|---|---|---|---|---|---|')
inst = {}
for l in open('/home/user/crypto/research/quartic_n4v2_20261020/results/instances.jsonl'):
    r = json.loads(l); inst[(r['q'], r['seed'])] = r['tries']
for c in sorted(CERT, key=lambda x: (x['q'], x['seed'])):
    ct = inst.get((c['q'], c['seed']))
    print(f"| {c['q']:,} | {c['seed']} | {c['tries_until_match']} ({ct if ct is not None else 'n/a, seed not in N4v2 list'}) | {c['ell']} | {c['order']} | {c['k_planted']} | {c['logs_ok']}/{c['distinct_ok_logs']} real + {c['logs_checked'] - c['distinct_ok_logs']} negative controls rejected | {str(c['j_in_Fq']).lower()} | {'PASS' if c['pass'] else 'FAIL'} |")
