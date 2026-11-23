#!/usr/bin/env python3
"""R13-C analysis: la_mul exponent and log-factor fit. Reads genus_ladder JSONL."""
import json, math, statistics, random, sys

FILES = {
    262147: 'research/r13c_la_logfactor_20261123/run_262147_anchor.jsonl',
    1048583: 'research/r13c_la_logfactor_20261123/run_1048583.jsonl',
    1572869: 'research/r13c_la_logfactor_20261123/run_1572869.jsonl',
}
SGE = 'sge(8,3)'

def load(path):
    inst, rows = {}, []
    for line in open(path):
        r = json.loads(line)
        if r.get('kind') == 'instance':
            inst[r['curve_seed']] = r
        elif r.get('kind') == 'run' and SGE in (r.get('arm') or ''):
            rows.append(r)
    return inst, rows

def cell(rows, seed=None):
    """mean la_mul per curve, then mean over curves; also FPM_mid."""
    by = {}
    for r in rows:
        if seed is not None and r['seed'] != seed:
            continue
        by.setdefault(r['curve_seed'], []).append(r)
    per_curve = []
    for c, rs in by.items():
        la = statistics.mean(x['la_mul'] for x in rs)
        fpm = statistics.mean(x['cantor_mul'] + 10*x['cantor_inv'] + x['rel_mul'] + 9*x['la_mul'] for x in rs)
        per_curve.append((c, la, fpm))
    return per_curve

def slope(a, b, pa, pb):
    return math.log(b/a)/math.log(pb/pa)

def main():
    data = {}
    for p, f in FILES.items():
        inst, rows = load(f)
        assert rows, f
        bad = [r for r in rows if not r.get('certified')]
        print(f'p={p}: {len(inst)} instances, {len(rows)} SGE rows, uncertified={len(bad)}')
        pc = cell(rows)
        la = statistics.mean(x[1] for x in pc)
        fpm = statistics.mean(x[2] for x in pc)
        data[p] = (inst, rows, pc, la, fpm)
        print(f'  la_mul {la:.6e}  FPM_mid {fpm:.6e}  LA share {9*la/fpm:.3f}  curves {sorted(x[0] for x in pc)}')

    ps = sorted(data)
    for a, b in zip(ps, ps[1:]):
        s = slope(data[a][3], data[b][3], a, b)
        print(f'local la_mul slope {a}->{b}: {s:.4f}')

    # fit la_mul = A p^(4/3) (ln p)^c
    xs = [math.log(math.log(p)) for p in ps]
    ys = [math.log(data[p][3]) - (4/3)*math.log(p) for p in ps]
    n = len(xs); mx = statistics.mean(xs); my = statistics.mean(ys)
    c = sum((x-mx)*(y-my) for x, y in zip(xs, ys))/sum((x-mx)**2 for x in xs)
    print(f'log-factor fit c = {c:.3f} over {ps}')

    # bootstrap over curves, resample within each prime
    rng = random.Random(13)
    bs = []
    for _ in range(4000):
        means = {}
        for p in ps:
            pc = data[p][2]
            draw = [pc[rng.randrange(len(pc))] for _ in pc]
            means[p] = statistics.mean(x[1] for x in draw)
        xs2 = [math.log(math.log(p)) for p in ps]
        ys2 = [math.log(means[p]) - (4/3)*math.log(p) for p in ps]
        mx2 = statistics.mean(xs2); my2 = statistics.mean(ys2)
        den = sum((x-mx2)**2 for x in xs2)
        bs.append(sum((x-mx2)*(y-my2) for x, y in zip(xs2, ys2))/den if den else float('nan'))
    bs = sorted(x for x in bs if not math.isnan(x))
    lo, hi = bs[int(0.025*len(bs))], bs[int(0.975*len(bs))]
    print(f'c bootstrap 95%: [{lo:.3f}, {hi:.3f}]')

if __name__ == '__main__':
    main()
