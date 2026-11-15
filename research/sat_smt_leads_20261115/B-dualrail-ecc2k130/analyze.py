"""Tables, results.json and the figure from raw/d*.jsonl.  python3 analyze.py [files..]"""
import glob
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
files = sys.argv[1:] or sorted(glob.glob(str(HERE / 'raw/d*.jsonl')))
rows = []
for f in files:
    for line in open(f):
        if line.strip():
            rows.append(json.loads(line))
ARMS = ['a', 'b', 'c', 'a-scan', 'L-scan', 'L-plain-scan', 'L-free', 'L-rail-free']
ds = sorted(set(r['d'] for r in rows))


def fmt(x):
    if x is None:
        return '-'
    if x >= 1000:
        return '{:,.0f}'.format(x)
    if x >= 10:
        return '%.1f' % x
    return '%.3g' % x


cells = {}
for d in ds:
    for arm in ARMS:
        for stratum in ('uniform', 'planted'):
            rs = [r for r in rows if r['d'] == d and r['arm'] == arm and r['stratum'] == stratum]
            if not rs:
                continue
            n = len(rs)
            conf = [r['conflicts'] for r in rs]
            decided = [r for r in rs if r['status'] in ('SAT', 'UNSAT')]
            cens = n - len(decided)
            leaves = [r.get('leaves') for r in rs if r.get('leaves')]
            c = {'d': d, 'arm': arm, 'stratum': stratum, 'targets': n, 'decided': len(decided),
                 'not_decided': cens, 'statuses': sorted(set(r['status'] for r in rs)),
                 'mean_conflicts': sum(conf) / n, 'min_conflicts': min(conf), 'max_conflicts': max(conf),
                 'lower_bound': cens > 0, 'pairs': rs[0]['pairs'], 'A': rs[0]['A'],
                 'scan_count': rs[0]['leaves_total'], 'mean_wall': sum(r['wall'] for r in rs) / n,
                 'cnf_vars_clauses_xors': rs[0]['cnf']}
            if leaves:
                tl = sum(leaves)
                c['leaves_run'] = tl
                c['conflicts_per_leaf'] = sum(conf) / tl
                c['capped_leaves'] = sum(r['capped_leaves'] for r in rs)
                c['sampled'] = rs[0]['sampled']
                c['max_leaf_conflicts'] = max(max(r['per_leaf']) for r in rs)
            c['x_pairs'] = c['mean_conflicts'] / c['pairs']
            c['x_scan'] = c['mean_conflicts'] / c['scan_count']
            cells[(d, arm, stratum)] = c

out = []
W = out.append
for stratum, title in (('uniform', 'Non-decomposable (uniform) targets'), ('planted', 'Decomposable (planted) targets, first model')):
    W('### %s: mean conflicts per target [pcc:unverified]\n' % title)
    W('| d | A | pairs C(A,2) | scan 2^d-1 | arm | targets (decided) | mean conflicts | x pairs | x scan count | conflicts per leaf (leaves run, capped) | mean wall s |')
    W('|---|---|---|---|---|---|---|---|---|---|---|')
    for d in ds:
        for arm in ARMS:
            c = cells.get((d, arm, stratum))
            if not c:
                continue
            lb = '>= ' if c['lower_bound'] else ''
            if c.get('sampled'):
                tot = '(sampled leaves)'
                xp = xs = '-'
            else:
                tot = lb + fmt(c['mean_conflicts'])
                xp = lb + fmt(c['x_pairs'])
                xs = lb + fmt(c['x_scan'])
            pl = '-'
            if 'conflicts_per_leaf' in c:
                pl = '%s%s (%d, %d)' % ('>= ' if c['capped_leaves'] else '', fmt(c['conflicts_per_leaf']), c['leaves_run'], c['capped_leaves'])
            W('| %d | %d | %s | %d | %s | %d (%d) | %s | %s | %s | %s | %.2f |' % (
                d, c['A'], '{:,}'.format(c['pairs']), c['scan_count'], arm, c['targets'], c['decided'], tot, xp, xs, pl, c['mean_wall']))
    W('')

# decision
W('### Reduction against arm a (uniform targets) and the rule [pcc:unverified]\n')
W('| d | arm a mean | arm | arm mean | a / arm | arm / pairs | 5x leg | 10x-of-pairs leg |')
W('|---|---|---|---|---|---|---|---|')
dec = {}
for d in ds:
    a = cells.get((d, 'a', 'uniform'))
    if not a:
        continue
    for arm in ('b', 'c', 'L-scan', 'a-scan'):
        c = cells.get((d, arm, 'uniform'))
        if not c or c.get('sampled'):
            continue
        if a['mean_conflicts'] == 0:
            continue   # arm a already at 0 conflicts (linear certificate): no reduction defined
        red = a['mean_conflicts'] / c['mean_conflicts'] if c['mean_conflicts'] else float('inf')
        # censoring: arm censored -> reduction is an upper bound; a censored -> lower bound
        note = ''
        if c['lower_bound'] and a['lower_bound']:
            note = ' (both censored)'
        elif c['lower_bound']:
            note = ' (upper bound: arm not all decided)'
        elif a['lower_bound']:
            note = ' (lower bound: a censored)'
        leg5 = red >= 5 and not c['lower_bound']
        leg10 = c['x_pairs'] <= 10 and not c['lower_bound']
        dec[(d, arm)] = {'reduction': red, 'x_pairs': c['x_pairs'], 'leg5': leg5, 'leg10': leg10,
                         'lt2': red < 2 and not a['lower_bound'], 'note': note.strip()}
        W('| %d | %s%s | %s | %s%s | %s%s | %s | %s | %s |' % (
            d, '>= ' if a['lower_bound'] else '', fmt(a['mean_conflicts']), arm,
            '>= ' if c['lower_bound'] else '', fmt(c['mean_conflicts']), fmt(red), note, fmt(c['x_pairs']),
            'yes' if leg5 else 'no', 'yes' if leg10 else 'no'))
W('')
elig = ('b', 'c', 'L-scan')
transfers = [arm for arm in elig if all((d, arm) in dec and dec[(d, arm)]['leg5'] and dec[(d, arm)]['leg10'] for d in (7, 8))]
notT = all((8, arm) in dec and dec[(8, arm)]['lt2'] for arm in elig)
verdict = 'transfers' if transfers else ('does not transfer' if notT else 'inconclusive')
attr = None
if (8, 'c', 'uniform') in cells and (8, 'a-scan', 'uniform') in cells:
    cc, ca = cells[(8, 'c', 'uniform')], cells[(8, 'a-scan', 'uniform')]
    ratio = cc['mean_conflicts'] / ca['mean_conflicts'] if ca['mean_conflicts'] else None
    attr = {'c_over_a_scan_d8': ratio, 'a_scan_lower_bound': ca['lower_bound'],
            'attributed_to': 'rails' if ratio is not None and ratio <= 0.5 else 'outer loop'}
W('Decision by the pre-registered rule: **%s** (arms meeting both legs at d = 7 and d = 8: %s).' % (verdict, ', '.join(transfers) or 'none'))
if attr:
    W('Attribution leg: arm c / a-scan control at d = 8 = %s -> attributed to the %s.' % (fmt(attr['c_over_a_scan_d8']), attr['attributed_to']))
text = '\n'.join(out) + '\n'
(HERE / 'tables.md').write_text(text)
print(text)
json.dump({'tag': 'pcc:unverified', 'verdict': verdict, 'arms_meeting_rule': transfers, 'attribution': attr,
           'decision_table': {'%d/%s' % k: v for k, v in dec.items()},
           'cells': [cells[k] for k in sorted(cells)], 'rows': rows},
          open(HERE / 'results.json', 'w'), indent=1, sort_keys=True)

import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(7.2, 4.6))
style = {'a': ('#1f4e79', 'o', 'a: repository S\'4, free solve'),
         'b': ('#c0392b', 's', 'b: + dual rails, free solve'),
         'a-scan': ('#7f8c8d', 'v', 'a-scan: repository S\'4, x1 outer loop'),
         'c': ('#27ae60', '^', 'c: + dual rails, x1 outer loop'),
         'L-scan': ('#8e44ad', 'D', 'L-scan: chained linear-leaf + rails, x1 outer loop')}
for arm, (col, mk, lab) in style.items():
    xs, ys, cen = [], [], []
    for d in ds:
        c = cells.get((d, arm, 'uniform'))
        if c and not c.get('sampled'):
            xs.append(d)
            ys.append(max(c['mean_conflicts'], 0.5))
            cen.append(c['lower_bound'])
    if xs:
        ax.plot(xs, ys, color=col, marker=mk, label=lab, lw=1.6, ms=6)
        for x, y, ce in zip(xs, ys, cen):
            if ce:
                ax.annotate('', xy=(x, y * 2.2), xytext=(x, y), arrowprops=dict(arrowstyle='->', color=col))
pd = [d for d in ds if (d, 'c', 'uniform') in cells or (d, 'a', 'uniform') in cells]
get = lambda d, k: next(c[k] for (dd, _, s), c in cells.items() if dd == d)
ax.plot(pd, [get(d, 'pairs') for d in pd], 'k--', lw=1.2, label='reference: pairs C(A,2)')
ax.plot(pd, [get(d, 'scan_count') for d in pd], 'k:', lw=1.2, label='scan count 2^d - 1')
ax.set_yscale('log')
ax.set_xticks(ds)
ax.set_xlabel('factor-base dimension d (ECC2K-130 curve, n = 131, m = 3)')
ax.set_ylabel('mean CryptoMiniSat conflicts per non-decomposable target')
ax.set_title('Dual rails on the repository S\'4 instance (pilot, unverified)\narrows: censored, lower bound; 0 plotted as 0.5', fontsize=9)
ax.grid(True, which='major', alpha=0.3)
ax.legend(fontsize=7, loc='upper left')
fig.tight_layout()
fig.savefig(HERE / 'conflicts_vs_d.svg')
fig.savefig(HERE / 'conflicts_vs_d.png', dpi=150)
