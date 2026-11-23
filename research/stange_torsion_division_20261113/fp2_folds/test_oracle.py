#!/usr/bin/env python3
"""Positive control: the symmetric 2-torsion oracle against exhaustive enumeration of the line."""
import random, sys
import ic_fp2_folds as F
B = F.B
for p in (503, 1019):
    B.p = p
    rng = random.Random(p)
    tot = dict(targets=0, decomposable=0, agree=0, mismatch=0, rels=0, sign_ok=0)
    for cv in range(3):
        a, b, x0, c, r, G, _ = F.find_tors_curve(rng)
        orc = F.TorsOracle(a, b, x0, c)
        # line points
        line = {}
        for t in range(1, p):
            Pt = None
            x = B.f2add(x0, B.f2mulfp(c, t))
            rhs = B.f2add(B.f2add(B.f2mul(B.f2sqr(x), x), B.f2mul(a, x)), b)
            y = B.f2sqrt(rhs)
            if y is None or t in (1, p - 1):
                continue
            line[x] = t
        for _ in range(100):
            R = B.smul(rng.randrange(1, r), G, a)
            truth = set()
            for x, t in line.items():
                rhs = B.f2add(B.f2add(B.f2mul(B.f2sqr(x), x), B.f2mul(a, x)), b)
                y = B.f2sqrt(rhs)
                for P1 in ((x, y), B.pneg((x, y))):
                    D = B.padd(R, B.pneg(P1), a)
                    if D is not None and D[0] in line:
                        t2 = line[D[0]]
                        s1 = (t + pow(t, p - 2, p)) % p; s2 = (t2 + pow(t2, p - 2, p)) % p
                        truth.add(frozenset((s1, s2)))
            got = orc.decompose(R)
            gs = set(frozenset((g[0], g[3])) for g in got)
            tot['targets'] += 1
            tot['decomposable'] += bool(truth)
            if gs == truth:
                tot['agree'] += 1
            else:
                tot['mismatch'] += 1
                print('mismatch', p, cv, sorted(map(sorted, truth)), sorted(map(sorted, gs)))
            for (s1, e1, b1, s2, e2, b2) in got:
                tc1, _, w1 = orc.memo[s1]; tc2, _, w2 = orc.memo[s2]
                P1 = orc.point(tc1 if b1 == 'c' else (s1 - tc1) % p, w1)
                P2 = orc.point(tc2 if b2 == 'c' else (s2 - tc2) % p, w2)
                S = B.padd(P1 if e1 == 1 else B.pneg(P1), P2 if e2 == 1 else B.pneg(P2), a)
                tot['rels'] += 1
                tot['sign_ok'] += (S == R)
                # [4]P(s,'f') == -[4]P(s,'c')
                Pc = orc.point(tc2, w2); Pf = orc.point((s2 - tc2) % p, w2)
                assert B.smul(4, Pf, a) == B.pneg(B.smul(4, Pc, a))
    print(p, tot)
