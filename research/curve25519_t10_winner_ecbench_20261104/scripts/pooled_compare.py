#!/usr/bin/env python3
"""Paired compare pooled over several complete ecbench sessions (chunks).

Method = ecbench compare's 'ops' interval: per workload, sum S over rounds
for arm A and arm B; R = sum(B)/sum(A); linearised t on z_k = (B_k - R A_k)/mean(A),
se = sd(z)/sqrt(K), df = K-1, interval R*exp(+-t*se/R). S at weight w =
(total_gae + w * unpriced native ops)/sqrt(r), unpriced ops resolved as ecbench
does (solver F4 mul-adds from solver.ops, the rest from counters/phase natives).
Reproduces ecbench compare on a single session (see scripts/selftest output).
Usage: pooled_compare.py A B w dir1 [dir2 ...]  -> JSON on stdout.
"""
import json, math, statistics, sys, glob, os
from collections import defaultdict

def t975(df):
    # Student-t 0.975 quantile by bisection on the regularised incomplete beta (no scipy).
    def cdf(t):
        x = df / (df + t * t)
        # I_x(df/2, 1/2) via continued fraction (Numerical Recipes betacf)
        a, b = df / 2, 0.5
        def betacf(a, b, x):
            MAXIT, EPS, FPMIN = 400, 3e-16, 1e-300
            qab, qap, qam = a + b, a + 1, a - 1
            c, d = 1.0, 1 - qab * x / qap
            d = 1 / (d if abs(d) > FPMIN else FPMIN); h = d
            for m in range(1, MAXIT + 1):
                m2 = 2 * m
                aa = m * (b - m) * x / ((qam + m2) * (a + m2))
                d = 1 + aa * d; d = 1 / (d if abs(d) > FPMIN else FPMIN)
                c = 1 + aa / c; c = c if abs(c) > FPMIN else FPMIN; h *= d * c
                aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2))
                d = 1 + aa * d; d = 1 / (d if abs(d) > FPMIN else FPMIN)
                c = 1 + aa / c; c = c if abs(c) > FPMIN else FPMIN
                de = d * c; h *= de
                if abs(de - 1) < EPS: break
            return h
        lbeta = math.lgamma(a) + math.lgamma(b) - math.lgamma(a + b)
        front = math.exp(a * math.log(x) + b * math.log(1 - x) - lbeta) / a
        ib = front * betacf(a, b, x) if x < (a + 1) / (a + b + 2) else 1 - math.exp(a * math.log(x) + b * math.log(1 - x) - lbeta) / b * betacf(b, a, 1 - x)
        return 1 - 0.5 * ib
    lo, hi = 0.0, 100.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if cdf(mid) < 0.975: lo = mid
        else: hi = mid
    return (lo + hi) / 2

def unpriced(x):
    tot = 0
    for n in x['cost']['unpriced']:
        if n.startswith('solver_F_p_multiply-adds'):
            tot += x['solver']['ops']; continue
        v, hit = 0, False
        if n in x.get('counters', {}): v += x['counters'][n]; hit = True
        for ph in x['phases']:
            if n in ph.get('native', {}): v += ph['native'][n]; hit = True
        if not hit:
            s = n[:-len('_uncharged')]
            for ph in x['phases']:
                if s in ph.get('native', {}): v += ph['native'][s]; hit = True
        assert hit, (x['arm'], n)
        tot += v
    return tot

def load(dirs):
    recs = []
    for d in dirs:
        sj = json.load(open(os.path.join(d, 'session.json')))
        assert sj.get('status') == 'complete', (d, sj.get('status'))
        for line in open(os.path.join(d, 'records.jsonl')):
            recs.append(json.loads(line))
    return recs

def pooled(A, B, w, dirs):
    recs = load(dirs)
    # sqrt(r) from the record's own S = total_gae / sqrt(r)
    sqs = {round(x['cost']['total_gae'] / x['cost']['s'], 6) for x in recs if x['cost']['s']}
    assert len(sqs) == 1, sqs
    sq = sqs.pop()
    def S(x):
        return (x['cost']['total_gae'] + w * unpriced(x)) / sq
    ia = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == A}
    ib = {(x['workload']['workload_id'], x['round']): x for x in recs if x['arm'] == B}
    assert all(x['outcome']['status'] == 'verified' for x in list(ia.values()) + list(ib.values()))
    keys = sorted(set(ia) & set(ib))
    cl = defaultdict(lambda: [0.0, 0.0])
    for k in keys:
        cl[k[0]][0] += S(ia[k]); cl[k[0]][1] += S(ib[k])
    Ak = [v[0] for v in cl.values()]; Bk = [v[1] for v in cl.values()]
    R = sum(Bk) / sum(Ak); K = len(Ak); mA = statistics.fmean(Ak)
    z = [(bk - R * ak) / mA for ak, bk in zip(Ak, Bk)]
    se = statistics.stdev(z) / math.sqrt(K); t = t975(K - 1)
    return dict(a=A, b=B, weight=w, ratio=R, ci95=[R * math.exp(-t * se / R), R * math.exp(t * se / R)],
                pairs=len(keys), workloads=K, wins_b=sum(1 for ak, bk in zip(Ak, Bk) if bk < ak), sessions=len(dirs))

if __name__ == '__main__':
    A, B, w = sys.argv[1], sys.argv[2], float(sys.argv[3])
    print(json.dumps(pooled(A, B, w, sys.argv[4:])))
