"""Galois-orbit (trace) factor base toy test. [pcc:unverified]
E/F_p, Tr: E(F_{p^k}) -> E(F_p), Tr(Q)=sum_i pi^i(Q). Factor base F=Tr(S) for an
F_p-algebraic restriction S of E(F_{p^k}). Preset kill rule (written before running):
 K1 size: if |Tr(S)|/#E(F_p) >= 0.25 for every positive-dim S tested, F ~ p >> sqrt(p): killed.
 K2 budget: with free preimage the relation Tr(Q1)+Tr(Q2)=R carries e_eff ~ 1 F_p equation
    (not k): killed (no equations added over plain F_p Semaev).
 K3 fixed preimage Q1+Q2=k^-1 R is Weil descent route 2 (closed by equation-budget-B).
"""
import sys, random, math, json, flint
O = None
def add(P, Q, a, F):
    if P is O: return Q
    if Q is O: return P
    x1,y1 = P; x2,y2 = Q
    if x1 == x2:
        if (y1 + y2).is_zero(): return O
        l = (3*x1*x1 + a) / (2*y1)
    else:
        l = (y2 - y1) / (x2 - x1)
    x3 = l*l - x1 - x2
    return (x3, l*(x1 - x3) - y1)
def neg(P): return O if P is O else (P[0], -P[1])
def mul(n, P, a, F):
    R = O; Q = P
    while n:
        if n & 1: R = add(R, Q, a, F)
        Q = add(Q, Q, a, F); n >>= 1
    return R
def key(P): return 'O' if P is O else (tuple(int(c) for c in P[0].to_list()), tuple(int(c) for c in P[1].to_list()))
def frob(P, i): return O if P is O else (P[0].frobenius(i), P[1].frobenius(i))
def trace(P, k, a, F):
    R = O
    for i in range(k): R = add(R, frob(P, i), a, F)
    return R
def run(p, k, seed, nv=2, nfix=20):
    rnd = random.Random(seed)
    F = flint.fq_default_ctx(p, k)
    while True:
        a = F(rnd.randrange(p)); b = F(rnd.randrange(p))
        if not (4*a**3 + 27*b**2).is_zero(): break
    f = lambda x: x**3 + a*x + b
    # E(F_p)
    EFp = [O]
    for t in range(p):
        x = F(t); r = f(x)
        if r.is_zero(): EFp.append((x, F(0)))
        else:
            # y in F_p iff r is square in F_p: Euler criterion on the integer value
            rv = int(r.to_list()[0])
            if pow(rv, (p-1)//2, p) == 1:
                y = F(int(flint.nmod(rv, p).sqrt()))
                EFp += [(x, y), (x, -y)]
    n = len(EFp); tr = p + 1 - n
    # #E(F_{p^k}) via a_i recurrence
    s0, s1 = 2, tr
    for _ in range(k - 1): s0, s1 = s1, tr*s1 - p*s0
    Nk = p**k + 1 - s1
    out = dict(p=p, k=k, seed=seed, a=int(a.to_list()[0]), b=int(b.to_list()[0]), nEFp=n, nEFpk=Nk, restrictions=[])
    vs = [('x in F_p', F(1))]
    while len(vs) < nv + 1:
        v = F.random_element()
        if v.frobenius(1) != v: vs.append(('x in F_p*v', v))
    kinv = pow(k, -1, n) if math.gcd(k, n) == 1 else None
    for name, v in vs:
        S = []
        for t in range(p):
            x = F(t) * v; r = f(x)
            if r.is_zero(): S.append((x, F(0)))
            elif r.is_square():
                y = r.sqrt(); S += [(x, y), (x, -y)]
        T = {}
        for Q in S:
            kk = key(trace(Q, k, a, F)); T[kk] = T.get(kk, 0) + 1
        rec = dict(restriction=name, v=str(v), S=len(S), TrS_distinct=len(T),
                   coverage=len(T) / n, Tr_is_O=T.get('O', 0))
        # K2 (m=2, free preimage): avg ordered solutions of Tr(Q1)+Tr(Q2)=R over all R in E(F_p)
        pts = {}
        for Q in S:
            P = trace(Q, k, a, F); pts[key(P)] = P
        cnt = {}
        items = list(T.items())
        for k1, c1 in items:
            P1 = pts[k1]
            for k2, c2 in items:
                kk = key(add(P1, pts[k2], a, F)); cnt[kk] = cnt.get(kk, 0) + c1*c2
        avg = sum(cnt.values()) / n
        rec['K2_avg_solutions_free'] = avg
        rec['K2_pred_1eq'] = len(S)**2 / n
        rec['K2_pred_keq'] = len(S)**2 / Nk
        rec['K2_R_hit_fraction'] = len(cnt) / n
        rec['K2_e_eff'] = math.log(len(S)**2 / avg, p) if avg > 0 else None
        # K3 fixed preimage: Q1+Q2 = k^-1 R in E(F_{p^k}), Q1,Q2 in S
        if kinv is not None:
            Sk = set(key(Q) for Q in S); tot = 0
            for R in rnd.sample(EFp[1:], min(nfix, n - 1)):
                Qp = mul(kinv, R, a, F)
                tot += sum(1 for Q in S if key(add(Qp, neg(Q), a, F)) in Sk)
            rec['K3_avg_solutions_fixed'] = tot / min(nfix, n - 1)
        out['restrictions'].append(rec)
    return out
if __name__ == '__main__':
    res = []
    for p, k in [(101,2),(211,2),(401,2),(809,2),(101,3),(211,3),(401,3)]:
        for seed in (1, 2):
            r = run(p, k, seed); res.append(r)
            for q in r['restrictions']:
                print(p, k, seed, r['nEFp'], q['restriction'], 'S', q['S'], 'TrS', q['TrS_distinct'],
                      'cov %.3f' % q['coverage'], 'K2avg %.2f pred1 %.2f predk %.4f eeff %s hit %.3f' % (
                      q['K2_avg_solutions_free'], q['K2_pred_1eq'], q['K2_pred_keq'],
                      None if q['K2_e_eff'] is None else round(q['K2_e_eff'], 3), q['K2_R_hit_fraction']),
                      'K3', q.get('K3_avg_solutions_fixed'), flush=True)
    json.dump(res, open(sys.argv[1], 'w'), indent=1)
