#!/usr/bin/env python3
"""Red-team check (added after review; not a timed arm, not gate-scored).

Claim checked: for M^3 < p (and 3M^2 < p) the chart contract
    e2(x,y,z) = t0 S^2 + t1 S + t2,  e3(x,y,z) = u0 S^2 + u1 S + u2  (mod p),
    S = x+y+z, (x,y,z) in [0,M]^3
is solved exactly by scanning S in [0,3M]: the residues e2 mod p, e3 mod p are
then the true integers (e2 <= 3M^2 < p, e3 <= M^3 < p), so {x,y,z} are the
integer roots of t^3 - S t^2 + e2 t - e3. One integer cubic per S: 3M+1 cubics.
Integer roots are found exactly (integer critical points + integer bisection on
monotone pieces), so no floating point decides a root.

Compared against (a) the stored panel roots in data/panel.jsonl.gz (which
equalled the brute-force oracle), (b) a fresh brute force on a few charts, and
(c) the F4-visit root hashes stored in data/explore_posthoc.jsonl.gz, including
M = 1024 and 1365 where program B hit its node cap.

  python3 scripts/cubic_check.py > data/cubic_check.txt
"""
import gzip, hashlib, json, sys
from itertools import permutations
from math import isqrt
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import panel  # chart generator and brute force only (no subprocesses run)

D = HERE.parent / 'data'


def int_roots(S, e2, e3, lo, hi):
    """Distinct integer roots in [lo,hi] of g(t) = t^3 - S t^2 + e2 t - e3."""
    g = lambda t: ((t - S) * t + e2) * t - e3
    # g'(t) = 3t^2 - 2St + e2; critical points (S +- sqrt(S^2 - 3 e2)) / 3
    disc = S * S - 3 * e2
    cuts = [lo, hi]
    if disc >= 0:
        r = isqrt(disc)
        for c in ((S - r) // 3, (S + r) // 3):
            cuts += [c - 2, c + 3]          # monotone strictly between these
    cuts = sorted({min(max(c, lo), hi) for c in cuts})
    found = set()
    for c in cuts:                           # test every cut point directly
        for t in range(c - 2, c + 3):
            if lo <= t <= hi and g(t) == 0:
                found.add(t)
    for a, b in zip(cuts, cuts[1:]):        # bisection on monotone pieces
        ga, gb = g(a), g(b)
        if ga == 0 or gb == 0 or (ga > 0) == (gb > 0):
            continue
        while b - a > 1:
            m = (a + b) // 2
            gm = g(m)
            if gm == 0:
                found.add(m); break
            if (gm > 0) == (ga > 0):
                a, ga = m, gm
            else:
                b = m
    return found


def cubic_solve(c):
    p, M, (t0, t1, t2), (u0, u1, u2) = c['p'], c['M'], c['t'], c['u']
    assert M ** 3 < p and 3 * M * M < p, 'outside the M < p^(1/3) regime'
    out = set()
    for S in range(3 * M + 1):
        e2 = (t0 * S * S + t1 * S + t2) % p
        e3 = (u0 * S * S + u1 * S + u2) % p
        if e2 > 3 * M * M or e3 > M ** 3:
            continue
        rs = int_roots(S, e2, e3, 0, M)
        for x in rs:                         # deflate exactly by one root
            a = x - S
            b = e2 + x * a                   # g = (t - x)(t^2 + a t + b)
            d = a * a - 4 * b
            if d < 0 or isqrt(d) ** 2 != d:
                continue
            y, z = (-a - isqrt(d)) // 2, (-a + isqrt(d)) // 2
            if 0 <= y <= M and 0 <= z <= M and x + y + z == S \
               and x * y + x * z + y * z == e2 and x * y * z == e3:
                for q in set(permutations((x, y, z))):
                    out.add(q)
    return sorted(list(q) for q in out), 3 * M + 1


def sha(roots):
    return hashlib.sha256(json.dumps(sorted(roots)).encode()).hexdigest()[:16]


def main():
    ok = True
    print('# cubic_check.py: exact O(M) integer-cubic solver vs stored roots [pcc:unverified]')
    print(f'# p = {panel.P}; p^(1/3) = {panel.P ** (1/3):.1f}; M^3 < p holds for M <= 1625')
    # (a) Panel B, all 20 charts at M = 128, against stored oracle-matching roots
    stored = {}
    for l in gzip.open(D / 'panel.jsonl.gz', 'rt'):
        r = json.loads(l)
        if r['oracle_match'] and 'roots' in r:
            stored.setdefault(r['id'], r['roots'])
    print('\n## (a) Panel B, M=128: cubic vs stored panel roots (= brute-force oracle)')
    for s in range(1, 21):
        c = panel.chart(s, 128)
        got, n = cubic_solve(c)
        m = got == sorted(stored[c['id']])
        ok &= m
        print(f"{c['id']:>4} M=128 cubics={n} roots={len(got)} match={m}")
    # (b) fresh brute force on a few charts
    print('\n## (b) fresh brute force over [0,M]^3')
    for s, M in ((1, 64), (11, 64), (2, 128)):
        c = panel.chart(s, M)
        got, n = cubic_solve(c)
        bf = sorted(panel.brute(c))
        m = got == bf
        ok &= m
        print(f"{c['id']:>4} M={M} cubics={n} roots={len(got)} brute={len(bf)} match={m}")
    # (c) post-hoc ladder charts against F4-visit's stored root hashes
    f4 = {}
    for l in gzip.open(D / 'explore_posthoc.jsonl.gz', 'rt'):
        r = json.loads(l)
        if r['arm'] == 'F4-visit' and r.get('complete'):
            f4.setdefault((r['M'], r['id']), (r['n_roots'], r['roots_sha']))
    print('\n## (c) post-hoc ladder: cubic vs F4-visit stored root count and hash')
    for M in (128, 384, 768, 1024, 1365):
        for s in (1, 2, 11):
            c = panel.chart(s, M)
            got, n = cubic_solve(c)
            fn, fs = f4[(M, c['id'])]
            m = len(got) == fn and sha(got) == fs
            ok &= m
            print(f"{c['id']:>4} M={M} cubics={n} roots={len(got)} F4_roots={fn} sha_match={m}")
    print('\nALL MATCH' if ok else '\nMISMATCH')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
