"""Part 1 analysis. Reads raw_l*.jsonl, computes per-instance best-k ratios xor/lit and sym/lit,
pooled geometric means with bootstrap 95% intervals, the within-lit A/A floor (cube-order), and
applies the pre-registered decision rule. Writes results.json and figure.svg/png."""
import json, glob, math, os
import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
SEEDS = [101, 102, 103]


def load():
    rows = []
    for f in sorted(glob.glob(os.path.join(HERE, 'raw_l*.jsonl'))):
        for line in open(f):
            line = line.strip()
            if line:
                rows.append(json.loads(line))
    return rows


def geomean(xs):
    xs = [x for x in xs if x > 0]
    return math.exp(sum(math.log(x) for x in xs) / len(xs)) if xs else float('nan')


def boot_ci(logratios, n=10000, seed=0):
    """basic bootstrap 95% CI of the geometric mean (exp of mean of log ratios)."""
    a = np.array(logratios, dtype=float)
    if len(a) == 0:
        return (float('nan'), float('nan'))
    rng = np.random.default_rng(seed)
    means = a[rng.integers(0, len(a), size=(n, len(a)))].mean(axis=1)
    lo, hi = np.percentile(means, [2.5, 97.5])
    return (math.exp(lo), math.exp(hi))


def conflicts_by(rows, l, pop, arm):
    """dict[(idx)] -> dict[k] -> mean-over-seeds conflicts (censored rows excluded for lit/xor;
    for sym kept with a flag). Returns also per-seed for A/A."""
    byinst = {}
    perseed = {}
    for r in rows:
        if r['l'] != l or r['arm'] != arm:
            continue
        if pop == 'UNSAT' and r['want_sat']:
            continue
        if pop == 'SAT' and not r['want_sat']:
            continue
        idx, k, sd = r['idx'], r['k'], r['order_seed']
        byinst.setdefault(idx, {}).setdefault(k, {})[sd] = (r['conflicts'], r['censored'])
        perseed.setdefault((idx, k), {})[sd] = (r['conflicts'], r['censored'])
    return byinst, perseed


def best_k_conflicts(byinst):
    """per instance: (best_k, mean conflicts at best k, any_censored). best k = min mean conflicts
    over seeds among k's with no censored seed; if all k censored, take min with censored flag."""
    out = {}
    for idx, ks in byinst.items():
        best = None
        for k, seeds in ks.items():
            vals = [c for (c, cen) in seeds.values()]
            cens = any(cen for (c, cen) in seeds.values())
            m = geomean(vals)
            cand = (cens, m, k)  # prefer non-censored, then lower conflicts
            if best is None or cand < best:
                best = cand
        out[idx] = dict(best_k=best[2], conflicts=best[1], censored=best[0])
    return out


def arm_ratio(rows, l, pop, num, den):
    bI_n, _ = conflicts_by(rows, l, pop, num)
    bI_d, _ = conflicts_by(rows, l, pop, den)
    bk_n = best_k_conflicts(bI_n)
    bk_d = best_k_conflicts(bI_d)
    logs = []
    ncens = 0
    for idx in sorted(set(bk_n) & set(bk_d)):
        cn, cd = bk_n[idx]['conflicts'], bk_d[idx]['conflicts']
        if bk_n[idx]['censored'] or bk_d[idx]['censored']:
            ncens += 1
            continue
        if cn > 0 and cd > 0:
            logs.append(math.log(cn / cd))
    gm = math.exp(sum(logs) / len(logs)) if logs else float('nan')
    lo, hi = boot_ci(logs)
    return dict(n=len(logs), n_censored=ncens, geomean=gm, ci=[lo, hi])


def aa_floor(rows, l, pop, arm='lit'):
    """within-arm ratio across order seeds: for each (idx,k) all seed pairs i/j; pool over instances
    at each instance's best k (best k defined on that arm)."""
    byinst, perseed = conflicts_by(rows, l, pop, arm)
    bk = best_k_conflicts(byinst)
    logs = []
    for idx, info in bk.items():
        k = info['best_k']
        seeds = perseed.get((idx, k), {})
        vals = [(sd, c) for sd, (c, cen) in seeds.items() if not cen and c > 0]
        for i in range(len(vals)):
            for j in range(len(vals)):
                if i != j:
                    logs.append(math.log(vals[i][1] / vals[j][1]))
    gm = math.exp(sum(logs) / len(logs)) if logs else float('nan')
    lo, hi = boot_ci(logs)
    return dict(n=len(logs), geomean=gm, ci=[lo, hi])


def main():
    rows = load()
    ls = sorted(set(r['l'] for r in rows))
    res = dict(sizes=ls, cells={}, note='[pcc:unverified] pilot; see PROTOCOL.md and README.md')
    for l in ls:
        cell = {}
        for pop in ('UNSAT', 'SAT'):
            n_targets = len(set(r['idx'] for r in rows if r['l'] == l and
                                (r['want_sat'] == (pop == 'SAT'))))
            cell[pop] = dict(
                n_targets=n_targets,
                xor_vs_lit=arm_ratio(rows, l, pop, 'xor', 'lit'),
                sym_vs_lit=arm_ratio(rows, l, pop, 'sym', 'lit'),
                aa_lit=aa_floor(rows, l, pop, 'lit'),
                aa_xor=aa_floor(rows, l, pop, 'xor'),
            )
        cell['seeds'] = SEEDS
        cell['n'] = next(r['n'] for r in rows if r['l'] == l)
        res['cells'][l] = cell

    # decision rule on UNSAT (primary population), largest l
    two_largest = ls[-2:]
    largest = ls[-1]
    xl = res['cells'][largest]['UNSAT']['xor_vs_lit']
    aa = res['cells'][largest]['UNSAT']['aa_lit']
    # "lies inside the A/A interval" read as: the xor/lit CI is not separated below the A/A band,
    # i.e. the two intervals overlap (ratio indistinguishable from the cube-order noise floor).
    inside_aa = not (xl['ci'][1] < aa['ci'][0] or xl['ci'][0] > aa['ci'][1])
    includes_or_exceeds_1 = xl['ci'][1] >= 1.0 or xl['ci'][0] >= 1.0
    kill = includes_or_exceeds_1 or inside_aa
    ubs = [res['cells'][l]['UNSAT']['xor_vs_lit']['ci'][1] for l in two_largest]
    alive = all(u < 0.8 for u in ubs)
    decision = 'kill' if kill else ('alive' if alive else 'inconclusive')
    res['decision'] = dict(
        rule='kill if xor/lit best-k 95% CI includes/exceeds 1.0 at largest l or lies inside A/A; '
             'alive if upper bound < 0.8 at two largest l; else inconclusive',
        two_largest_l=two_largest, largest_l=largest,
        xor_vs_lit_largest=xl, aa_lit_largest=aa,
        includes_or_exceeds_1=includes_or_exceeds_1, inside_aa=inside_aa,
        upper_bounds_two_largest=ubs, verdict=decision)
    json.dump(res, open(os.path.join(HERE, 'results.json'), 'w'), indent=1)
    print(json.dumps(res['decision'], indent=1))
    for l in ls:
        for pop in ('UNSAT', 'SAT'):
            c = res['cells'][l][pop]
            print(f"l={l} {pop} N={c['xor_vs_lit']['n']} "
                  f"xor/lit={c['xor_vs_lit']['geomean']:.3f} {c['xor_vs_lit']['ci']} "
                  f"sym/lit={c['sym_vs_lit']['geomean']:.3f} {c['sym_vs_lit']['ci']} "
                  f"(sym cens {c['sym_vs_lit']['n_censored']}) "
                  f"A/A={c['aa_lit']['geomean']:.3f} {c['aa_lit']['ci']}")
    return res


if __name__ == '__main__':
    main()
