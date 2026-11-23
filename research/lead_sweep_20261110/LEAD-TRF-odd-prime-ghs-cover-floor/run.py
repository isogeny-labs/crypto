#!/usr/bin/env python3
"""Toy for LEAD-TRF-odd-prime-ghs-cover-floor. stdlib only, deterministic."""
import sys, json, random, itertools, time
P = 5
def build_field(n):
    Q = P**n
    for cs in itertools.product(range(P), repeat=n):
        if cs[0] == 0: continue
        # x^n = -(c0 + c1 x + ...)  ; element = digit list
        cur = [0]*n; cur[0] = 1
        exp = []; seen = 0; ok = True
        for k in range(Q-1):
            code = 0
            for d in reversed(cur): code = code*P + d
            exp.append(code)
            top = cur[-1]
            cur = [0]+cur[:-1]
            for i in range(n): cur[i] = (cur[i]-top*cs[i]) % P
            if k < Q-2 and cur == [1]+[0]*(n-1): ok = False; break
        if ok and len(set(exp)) == Q-1: break
    log = [0]*Q
    for k, c in enumerate(exp): log[c] = k
    lo = n//2; hi = n-lo
    def mk(m):
        S = P**m
        def dg(x): return [(x//P**i) % P for i in range(m)]
        return [[sum(((a+b) % P)*P**i for i, (a, b) in enumerate(zip(dg(x), dg(y)))) for y in range(S)] for x in range(S)]
    return Q, exp, log, mk(lo), mk(hi), P**lo
class F:
    def __init__(s, n):
        s.n = n; s.Q, s.exp, s.log, s.alo, s.ahi, s.S = build_field(n)
    def add(s, x, y):
        S = s.S
        return s.alo[x % S][y % S] + S*s.ahi[x//S][y//S]
    def mul(s, x, y):
        if x == 0 or y == 0: return 0
        return s.exp[(s.log[x]+s.log[y]) % (s.Q-1)]
    def inv(s, x): return s.exp[(-s.log[x]) % (s.Q-1)]
    def pw(s, x, e):
        if x == 0: return 0
        return s.exp[(s.log[x]*e) % (s.Q-1)]
    def neg(s, x): return s.mul(x, s.exp[(s.Q-1)//2])
def trace(f, a, b):
    Q = f.Q; ex = f.exp; lg = f.log; M = Q-1; add = f.add
    la = lg[a] if a else None
    tot = 0
    for x in range(Q):
        if x:
            lx = lg[x]
            c = ex[(3*lx) % M]
            ax = ex[(lg[a]+lx) % M] if a else 0
            r = add(add(c, ax), b)
        else:
            r = b
        if r:
            tot += 1 if lg[r] % 2 == 0 else -1
    return -tot            # #E = Q+1-t
def bset(n):
    B = {}
    for s in range(-4, 5):                    # |s| <= 2*sqrt(5)
        u0, u1 = 2, s
        for _ in range(n-1): u0, u1 = u1, s*u1-P*u0
        B.setdefault(u1, []).append(s)
    return B
def divides(s, t, n):                          # x^2 - s x + P  | x^{2n} - t x^n + P^n  ?
    poly = [0]*(2*n+1); poly[2*n] = 1; poly[n] -= t; poly[0] += P**n
    for d in range(2*n, 1, -1):
        c = poly[d]
        if c:
            poly[d] = 0; poly[d-1] += c*s; poly[d-2] -= c*P
    return poly[0] == 0 and poly[1] == 0
def sample(f, rng, N, subfield):
    out = []
    while len(out) < N:
        if subfield: a, b = rng.randrange(P), rng.randrange(P)
        else: a, b = rng.randrange(f.Q), rng.randrange(f.Q)
        a3 = f.pw(a, 3); b2 = f.mul(b, b)
        disc = f.add(f.mul(4 % P, a3), f.mul(27 % P, b2))
        if disc == 0: continue
        if subfield:
            if a == 0 and b == 0: continue
            out.append((a, b, True))
            continue
        j = f.mul(f.mul(1728 % P, f.mul(4 % P, a3)), f.inv(disc)) if a3 else 0
        inF = (f.pw(j, P) == j) and (j in range(P))   # j in prime field: codes 0..P-1 are constants
        out.append((a, b, inF))
    return out
def experiment(n, Nrand, Nctrl, seed):
    f = F(n); B = bset(n); Bt = set(B)
    rng = random.Random(seed)
    # positive control: curves over F_5 viewed over F_{5^n}: t must lie in B and divisibility must hold
    pos = 0; posdiv = 0; tot = 0
    for a, b, _ in sample(f, rng, Nctrl, True):
        t = trace(f, a, b); tot += 1
        if t in Bt: pos += 1
        if t in Bt and any(divides(s, t, n) for s in B[t]): posdiv += 1
    # random curves, j outside F_5
    ts = []
    for a, b, inF in sample(f, rng, Nrand, False):
        if not inF: ts.append(trace(f, a, b))
    def frac(T, S): return sum(1 for t in T if t in S)/max(1, len(T))
    hits = sum(1 for t in ts if t in Bt)
    div_ok = all(any(divides(s, t, n) for s in B[t]) for t in ts if t in Bt)
    fB = frac(ts, Bt)
    dec = []
    for d in list(range(-6, 0))+list(range(1, 7)):
        Bd = {t+d for t in Bt}
        if Bd & Bt: continue
        dec.append(frac(ts, Bd))
    md = sum(dec)/len(dec)
    h = len(ts)//2
    aa = (frac(ts[:h], Bt), frac(ts[h:], Bt))
    return dict(n=n, Q=f.Q, B_traces=sorted(Bt), n_random_j_not_in_F5=len(ts), hits=hits,
                f_B=fB, decoy_mean=md, decoy_count=len(dec), ratio=(fB/md if md > 0 else None),
                AA_halves=aa, pos_control=dict(n=tot, in_B=pos, divisible=posdiv),
                hits_divisible=div_ok)
def main():
    smoke = '--smoke' in sys.argv
    t0 = time.time()
    if smoke: cfg = [(3, 60, 8, 1)]
    else: cfg = [(5, 3000, 40, 20261110), (7, 300, 10, 20261111)]
    res = [experiment(*c) for c in cfg]
    p = res[0]
    pc_ok = all(r['pos_control']['in_B'] == r['pos_control']['n'] == r['pos_control']['divisible'] for r in res)
    div_ok = all(r['hits_divisible'] for r in res)
    if not (pc_ok and div_ok): verdict = 'falsified'
    elif p['hits'] < 5 or p['ratio'] is None: verdict = 'inconclusive'
    elif 0.5 <= p['ratio'] <= 2.0 and abs(p['AA_halves'][0]-p['AA_halves'][1]) <= 0.5*max(p['f_B'], 1e-9)+0.01: verdict = 'consistent_with_prediction'
    elif not (0.5 <= p['ratio'] <= 2.0): verdict = 'falsified'
    else: verdict = 'inconclusive'
    if smoke and verdict != 'falsified': verdict = 'inconclusive' if verdict != 'consistent_with_prediction' else verdict
    print(json.dumps(dict(lead='LEAD-TRF-odd-prime-ghs-cover-floor', smoke=smoke, char=P,
        seeds=[c[3] for c in cfg], params=[dict(n=c[0], n_random=c[1], n_control=c[2]) for c in cfg],
        results=res, positive_control_ok=pc_ok, divisibility_ok=div_ok,
        statistic='ratio = f_B / mean decoy fraction (n=5 primary)', threshold='0.5..2.0, hits>=5',
        verdict=verdict, seconds=round(time.time()-t0, 2), label='[pcc:unverified]')))
main()
