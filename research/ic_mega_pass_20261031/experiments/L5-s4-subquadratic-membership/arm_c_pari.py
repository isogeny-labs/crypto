# Arm C: algebraic third step (two-summand gcd oracle) for S_4 membership, via cypari2. Single thread.
import cypari2, random, time, json, sys
pari = cypari2.Pari(); pari.default("nbthreads", 1) if hasattr(pari,'default') else None
def run(p, n, ntg, npl, seed):
    random.seed(seed); pari.setrand(seed)
    al, be = random.randrange(p), random.randrange(p)
    A2, A4 = (-al-be) % p, (al*be) % p
    E = pari.ellinit([0, A2, 0, A4, 0], p)
    def rp():
        while True:
            P = pari.random(E)
            if P != [0] and P[1] != 0: return P
    F = []; xs = set()
    while len(F) < n:
        P = rp(); x = int(P[0])
        if x not in xs: xs.add(x); F.append(P)
    Z = pari('Z'); Fp = lambda v: pari.Mod(v, p)
    # validate S3 on 300 random pairs
    def S3(x1, x2, x3):
        return (x1-x2)**2*x3**2 - 2*((x1+x2)*(x1*x2+A4) + 2*A2*x1*x2)*x3 + (x1*x2-A4)**2
    bad = 0
    for _ in range(300):
        P1, P2 = rp(), rp()
        if P1[0] == P2[0]: continue
        for Q in (pari.elladd(E, P1, P2), pari.ellsub(E, P1, P2)):
            if Q != [0] and S3(int(P1[0]), int(P2[0]), int(Q[0])) % p != 0: bad += 1
    fF = pari.lift(pari.Mod(1,p)*pari('prod(i=1,#v,Z-v[i])'.replace('v', 'V')) ) if False else None
    V = [Fp(int(P[0])) for P in F]
    fF = pari.prod if False else None
    fF = Fp(1)
    for v in V: fF *= (Z - v)
    # brute-force truth via explicit triple enumeration restricted to target: check x(R -+ P3) in x(F+-F)
    pairX = set()
    for i in range(n):
        for j in range(i, n):
            for Q in (pari.elladd(E, F[i], F[j]), pari.ellsub(E, F[i], F[j])):
                if Q != [0]: pairX.add(int(Q[0]))
    X1 = set(int(P[0]) for P in F)
    def truth(R):
        if int(R[0]) in X1: return True
        for P3 in F:
            for Q in (pari.elladd(E, R, P3), pari.ellsub(E, R, P3)):
                if Q != [0] and int(Q[0]) in pairX: return True
        return False
    tg = [rp() for _ in range(ntg)]
    for _ in range(npl):
        a, b, c = random.sample(F, 3); tg.append(pari.elladd(E, pari.ellsub(E, a, b), c))
    mism = 0; t_alg = 0.0; ngcd = 0
    for R in tg:
        if R == [0]: continue
        tr = truth(R)
        t0 = time.process_time(); dec = int(R[0]) in X1
        for P3 in F:
            if dec: break
            for Q in (pari.elladd(E, R, P3), pari.ellsub(E, R, P3)):
                if Q == [0]: continue
                xq = Fp(int(Q[0]))
                # (v1 bug: accepted xq in x(F), i.e. 2-sums R=+-P3+-Pk, outside the 3-summand convention; removed)
                g = Fp(1)
                for v in V: g *= S3(xq, v, Z)   # deg 2n product: roots = x(Q +- Pk)
                ngcd += 1
                if pari.poldegree(pari.gcd(g, fF)) > 0: dec = True; break
        t_alg += time.process_time() - t0
        mism += (dec != tr)
    return dict(p=p, n=n, S3_validation_bad=bad, targets=len(tg), mismatch=mism, cpu_s_per_target=t_alg/len(tg),
                gcd_calls_per_target=ngcd/len(tg),
                formula_M_per_target=f"~2n*(M(2n)log2(2n) + M(n)log2 n) = O~(n^2), n={n}")
out = []
for p, n in [(1048571, 8), (1048571, 16), (1048571, 32), (16777199, 32), (16777199, 64)]:
    r = run(p, n, 40, 10, 7); print(json.dumps(r), flush=True)
