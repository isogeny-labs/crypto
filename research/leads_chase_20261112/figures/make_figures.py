#!/usr/bin/env python3
"""Figures for research/leads_chase_20261112 (plots recorded values; ratios are derived from the records, as labelled).
Run from the study directory with a Python that has matplotlib."""
import json, glob, os, re, statistics as st
import matplotlib; matplotlib.use('Agg')
import matplotlib.pyplot as plt
from math import comb
plt.rcParams.update({'font.size': 9, 'svg.fonttype': 'none'})
def recs(f): return [json.loads(l) for l in open(f) if l.startswith('{')]
def save(fig, name):
    fig.tight_layout(); fig.savefig(f'figures/{name}.svg'); fig.savefig(f'figures/{name}.png', dpi=150); plt.close(fig)

# ---------- J2
U = 'j2/units'; PH = ('fb', 'relations', 'matrix', 'linear_algebra', 'log_recovery', 'descent', 'verify')
calib = {}
for q in (65479, 262111, 1048507, 4194271):
    rs = [r for i in (1, 2, 3) for r in recs(f'{U}/calib_{q}_{i}.jsonl')]
    calib[q] = (st.median(r['r_inv'] for r in rs), st.median(r['r_lmul'] for r in rs))
def meq(r, q): ri, rl = calib[q]; return sum(r.get(f'{p}.mul', 0) + ri * r.get(f'{p}.inv', 0) + rl * r.get(f'{p}.lmul', 0) for p in PH)
J = {}
for f in glob.glob(f'{U}/ic_*_c*.jsonl'):
    q, sd, c = os.path.basename(f)[3:-6].split('_'); q, sd, c = int(q), int(sd), float(c[1:])
    rs = [r for r in recs(f) if r.get('arm') == 'DL1']
    J[(q, c, sd)] = (rs[0]['ok'] and rs[0]['attempt'] == 1, sum(meq(r, q) for r in rs))
kof = {65479: 16, 262111: 18, 1048507: 20, 4194271: 22}
fig, ax = plt.subplots(1, 2, figsize=(9, 3.4))
summary = {}
for q in sorted(kof):
    cs = sorted({c for (qq, c, s) in J if qq == q})
    succ = [sum(J[(q, c, s)][0] for s in (1, 2, 3)) / 3 for c in cs]
    rat = [[J[(q, c, s)][1] / J[(q, 2.0, s)][1] for s in (1, 2, 3)] for c in cs]
    mean = [st.mean(r) for r in rat]
    ax[0].plot(cs, succ, 'o-', label=f'k = {kof[q]}')
    ax[1].errorbar(cs, mean, yerr=[[m - min(r) for m, r in zip(mean, rat)], [max(r) - m for m, r in zip(mean, rat)]], fmt='o-', capsize=3, label=f'k = {kof[q]} (q = {q})')
    summary[kof[q]] = {f'{c:.2f}': dict(success=s, ratio_mean=round(m, 3), ratio_min=round(min(r), 3), ratio_max=round(max(r), 3)) for c, s, m, r in zip(cs, succ, mean, rat)}
ax[0].set_xlabel('factor-base constant c_D'); ax[0].set_ylabel('first-attempt success (3 seeds)'); ax[0].set_ylim(-0.05, 1.05)
ax[0].axvspan(1.40, 1.75, color='0.9', zorder=0); ax[0].text(1.42, 0.5, 'predicted\nc50 band', fontsize=7); ax[0].legend(fontsize=7)
ax[1].axhline(1.0, color='0.5', lw=0.8); ax[1].fill_between([1.72, 1.78], 0.70, 0.85, color='tab:green', alpha=0.2, label='predicted at c = 1.75')
ax[1].set_xlabel('factor-base constant c_D'); ax[1].set_ylabel('charged DL1 M-eq / same seed at c = 2.00 (derived)')
ax[1].legend(fontsize=6.5); fig.suptitle('J2: N4v2 plane-quartic DL1 (weak GHS family), every attempt charged, w = 0; 3 seeds per cell', fontsize=9)
save(fig, 'j2_threshold_cost')
json.dump(summary, open('j2/summary.json', 'w'), indent=1)

# ---------- P1
cols = lambda k, D: sum(comb(2 * k, j) for j in range(D + 1))
base = {3: 2, 4: 6, 5: 10, 6: 14}
committed = {3: dict(zip(range(2, 6), [5, 7, 9, 11])), 4: dict(zip(range(6, 10), [605, 829, 1061, 1301])),
             5: {10: 18569, 11: 25033, 12: 32057, 13: 39657}, 6: {14: 448922}}
fig, ax = plt.subplots(figsize=(5.6, 3.4))
for D, d in committed.items():
    js = [k - base[D] for k in sorted(d)]; ys = [(cols(k, D) - d[k]) / 2 ** k for k in sorted(d)]
    ax.plot(js, ys, 'o-', label=f'committed D = {D} (k = {min(d)}..{max(d)})')
fresh = {}
for f in glob.glob('p1/ideal-D*.json'):
    for r in json.load(open(f)):
        D, k = r['D'], r['k']; j = k - base[D]
        if 0 <= j <= 3: fresh.setdefault(D, []).append((j, (cols(k, D) - r['dim_V']) / 2 ** k))
for D, pts in fresh.items():
    ax.plot([p[0] + 0.06 * (D - 3.5) for p in pts], [p[1] for p in pts], 'kx', ms=6, label=f'fresh primes D = {D} ({len(pts) // 4} instances)')
ax.set_xlabel('plateau position j = k - (4D - 10)'); ax.set_ylabel('q / 2^k  (q = |M<=D| - dim V_D, derived)')
ax.set_ylim(0, 6.5); ax.legend(fontsize=6.5); ax.set_title('P1: penultimate-degree quotient over the base size', fontsize=9)
save(fig, 'p1_quotient_law')

# ---------- S1
fr = recs('s1/freq_probe.jsonl'); op = recs('s1/order_probe.jsonl')
ls = sorted({r['p'] for r in fr}); xs = list(range(len(ls) + 1))
dep = [sum(r['departs'] for r in fr if r['p'] == l) for l in ls]; tot = [sum(1 for r in fr if r['p'] == l) for l in ls]
op2 = [r for r in op if r['target_order'] != 2]
dep.append(sum(r['departs'] for r in op2)); tot.append(len(op2))
fig, ax = plt.subplots(1, 2, figsize=(9, 3.2))
ax[0].bar(xs, [d / t for d, t in zip(dep, tot)], color=['tab:orange'] * len(ls) + ['tab:blue'])
for x, d, t in zip(xs, dep, tot): ax[0].text(x, d / t + 0.005, f'{d}/{t}', ha='center', fontsize=7)
ax[0].set_xticks(xs); ax[0].set_xticklabels([str(l) for l in ls] + ['40961\n(small-order\ntargets)'], fontsize=7)
ax[0].set_ylabel('fraction of unit instances departing'); ax[0].set_xlabel('modulus l')
ax[0].set_title('dim V4 at k = 6 or 7 differs from the large-char value', fontsize=8)
cls = {}
for r in fr:
    if r['departs']: cls.setdefault((r['dimV4_k6'], r['dimV4_k7']), []).append(r['p'])
labels = [f'({a}, {b})' for (a, b) in sorted(cls)]; counts = [len(v) for _, v in sorted(cls.items())]
ax[1].bar(range(len(cls)), counts, color='tab:orange'); ax[1].set_xticks(range(len(cls))); ax[1].set_xticklabels(labels, fontsize=7)
ax[1].set_xlabel('(dim V4(6), dim V4(7)); generic (605, 829); full at k = 6 is 794'); ax[1].set_ylabel('instances')
ax[1].set_title('departure classes (all moduli)', fontsize=8)
fig.suptitle('S1 exploratory: small-characteristic departures from the generic closure (non-order-2 targets, unit instances)', fontsize=9)
save(fig, 's1_departures')
print('figures written')
