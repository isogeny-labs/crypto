"""Frozen analysis of PRESET.txt.  [pcc:unverified]"""
import json, math, random, sys, collections
BKK_PICK = {3: 'C3', 4: 'H4a'}   # from out/bkk.txt by the preset tie-break rule
ALT = {3: ['C3', 'C3u'], 4: ['C4', 'B4', 'H4a', 'H4b']}
recs = []
for f in sys.argv[1:]:
    recs += [json.loads(l) for l in open(f)]
by = collections.defaultdict(dict)
for r in recs:
    by[(r['m'], r['bits'], r['idx'])][r['arm']] = r

def gm(xs):
    return 2 ** (sum(math.log2(x) for x in xs) / len(xs))

def fit(pts):
    n = len(pts)
    if n < 2: return None
    xb = sum(x for x, _ in pts) / n; yb = sum(y for _, y in pts) / n
    sxx = sum((x - xb) ** 2 for x, _ in pts)
    return sum((x - xb) * (y - yb) for x, y in pts) / sxx

def pair_ratio(cell, pick, metric, m):
    """list of per-instance ratios D/X, or None if the cell is censored."""
    out = []
    for idx, arms in sorted(cell.items()):
        d = arms.get('D')
        if pick == 'oracle':
            alts = [arms[a] for a in ALT[m] if a in arms and arms[a]['status'] == 'ok']
            x = min(alts, key=lambda r: r[metric]) if alts else None
        else:
            x = arms.get(pick)
        if not d or not x or d['status'] != 'ok' or x['status'] != 'ok':
            return None
        out.append(max(d[metric], 1e-3) / max(x[metric], 1e-3))
    return out

report = {'cells': [], 'fits': []}
for m in (3, 4):
    bits = sorted({k[1] for k in by if k[0] == m})
    cells = {b: {k[2]: by[k] for k in by if k[0] == m and k[1] == b} for b in bits}
    for b in bits:
        for arm in ['D'] + ALT[m]:
            rs = [cells[b][i][arm] for i in cells[b] if arm in cells[b][i]]
            if not rs: continue
            ok = [r for r in rs if r['status'] == 'ok']
            report['cells'].append(dict(m=m, bits=b, L=rs[0]['L'], arm=arm, n=len(rs), n_ok=len(ok),
                                        T_gm=round(gm([r['T'] for r in ok]), 3) if ok else None,
                                        Msum_gm=round(gm([r['Msum'] for r in ok])) if ok else None,
                                        Mmax_max=max((r['Mmax'] for r in ok), default=None),
                                        Dmax=sorted({r['Dmax'] for r in ok}) if ok else None,
                                        statuses=[r['status'] for r in rs],
                                        n_gb_one=sum(1 for r in ok if r['gb_is_one'])))
    for pick in (BKK_PICK[m], 'oracle'):
        for metric in ('T', 'Msum'):
            per_b = {}
            censored = []
            for b in bits:
                pr = pair_ratio(cells[b], pick, metric, m)
                if pr is None: censored.append(b)
                else: per_b[b] = pr
            pts = [(b, math.log2(gm(v))) for b, v in per_b.items()]
            beta = fit(pts)
            rng = random.Random(f'boot-{m}-{pick}-{metric}')
            bs = []
            if beta is not None:
                for _ in range(2000):
                    pp = [(b, math.log2(gm([rng.choice(v) for _ in v]))) for b, v in per_b.items()]
                    bs.append(fit(pp))
                bs.sort()
            ci = (bs[int(0.025 * len(bs))], bs[int(0.975 * len(bs)) - 1]) if bs else None
            report['fits'].append(dict(m=m, pick=pick, metric=metric,
                                       R_gm={b: round(gm(v), 3) for b, v in per_b.items()},
                                       censored_bits=censored, n_bits=len(per_b),
                                       beta=None if beta is None else round(beta, 4),
                                       ci95=None if ci is None else [round(ci[0], 4), round(ci[1], 4)],
                                       survives=bool(beta is not None and len(per_b) >= 3 and beta >= 0.05 and ci[0] > 0)))
# verdict
surv = {}
for m in (3, 4):
    for pick in (BKK_PICK[m], 'oracle'):
        f = {x['metric']: x for x in report['fits'] if x['m'] == m and x['pick'] == pick}
        surv[(m, pick)] = f['T']['survives'] and f['Msum']['survives']
enough = any(x['n_bits'] >= 3 for x in report['fits'])
report['verdict'] = 'ALIVE' if any(surv.values()) else ('KILL' if enough else 'INCONCLUSIVE')
report['survival'] = {f'm{m}:{p}': v for (m, p), v in surv.items()}
print(json.dumps(report, indent=1))
