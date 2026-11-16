#!/usr/bin/env python3
"""LEAD-TRF-padic-canonical-section toy protocol. stdlib only, one core, deterministic."""
import sys, json, random, math, time

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def next_prime(n):
    while not is_prime(n): n += 1
    return n

class Curve:
    """y^2=x^3+a x+b over Z/p^2 (projective, RCB complete addition)."""
    def __init__(s, p, a, b):
        s.p, s.M, s.a, s.b = p, p * p, a % (p * p), b % (p * p)
        s.b3 = 3 * s.b % s.M
    def add(s, P, Q):
        M, a, b3 = s.M, s.a, s.b3
        X1, Y1, Z1 = P; X2, Y2, Z2 = Q
        t0 = X1*X2 % M; t1 = Y1*Y2 % M; t2 = Z1*Z2 % M
        t3 = (X1+Y1)*(X2+Y2) % M; t3 = (t3-t0-t1) % M
        t4 = (X1+Z1)*(X2+Z2) % M; t4 = (t4-t0-t2) % M
        t5 = (Y1+Z1)*(Y2+Z2) % M; t5 = (t5-t1-t2) % M
        Z3 = (a*t4 + b3*t2) % M
        X3 = (t1-Z3) % M; Z3 = (t1+Z3) % M
        Y3 = X3*Z3 % M
        t1 = 3*t0 % M; t2 = a*t2 % M; t1 = (t1+t2) % M
        t2 = a*((t0-t2) % M) % M
        t4 = (b3*t4 + t2) % M
        Y3 = (Y3 + t1*t4) % M
        X3 = (t3*X3 - t5*t4) % M
        Z3 = (t5*Z3 + t3*t1) % M
        return (X3, Y3, Z3)
    def mul(s, k, P):
        R = (0, 1, 0)
        for bit in bin(k)[2:]:
            R = s.add(R, R)
            if bit == '1': R = s.add(R, P)
        return R
    def neg(s, P): return (P[0], (-P[1]) % s.M, P[2])
    def on(s, P):
        X, Y, Z = P; M = s.M
        return (Y*Y*Z - X**3 - s.a*X*Z*Z - s.b*Z**3) % M == 0
    def eq(s, P, Q):
        M = s.M
        return all((P[i]*Q[j]-P[j]*Q[i]) % M == 0 for i, j in ((0,1),(0,2),(1,2)))
    def lift(s, x0, y0, rng):
        """random lift of affine F_p point to Z/p^2 (x moves by p*u, y by Hensel)."""
        p, M = s.p, s.M
        x = (x0 + p*rng.randrange(p)) % M
        f = (x**3 + s.a*x + s.b) % M
        t = ((f - y0*y0) % M)
        assert t % p == 0
        t = (t//p) * pow(2*y0, -1, p) % p
        return (x, (y0 + p*t) % M, 1)
    def psi(s, P):
        """Smart functional on kernel of reduction: (X/Y)/p mod p."""
        X, Y, Z = P; p, M = s.p, s.M
        assert Z % p == 0 and Y % p != 0
        v = X * pow(Y, -1, M) % M
        assert v % p == 0
        return v // p

def count_points(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    n = 1
    for x in range(p): n += len(sq.get((x**3 + a*x + b) % p, ()))
    return n, sq

def find_curve(p, rng, want):
    """want='prime' (N prime, N!=p) or 'anomalous' (N==p). returns (a,b,N,sq)."""
    for _ in range(5000):
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p == 0: continue
        n, sq = count_points(p, a, b)
        if want == 'prime' and n != p and is_prime(n): return a, b, n, sq
        if want == 'anomalous' and n == p: return a, b, n, sq
    raise RuntimeError('no curve')

def rand_affine(p, a, b, sq, rng):
    while True:
        x = rng.randrange(p); ys = sq.get((x**3 + a*x + b) % p)
        if ys and ys[0] != 0: return x, rng.choice(ys)

def binom_tail_threshold(T, q, alpha):
    """smallest h with P(Bin(T,q) >= h) <= alpha."""
    def pmf(k):
        return math.exp(math.lgamma(T+1)-math.lgamma(k+1)-math.lgamma(T-k+1)+k*math.log(q)+(T-k)*math.log1p(-q))
    tail, k = 0.0, T
    # accumulate from the top only to modest k
    pm = [pmf(k) for k in range(0, min(T, 200)+1)]
    cdf = 0.0
    for k in range(len(pm)):
        cdf += pm[k]
        if 1.0 - cdf <= alpha * 0.999999 and k + 1 <= len(pm):
            return k + 1
    return len(pm)

def section(C, N, Pt):
    """canonical torsion section: s(P) = [1 - N c] P~, c = N^-1 mod p (m = 1 mod N, 0 mod p)."""
    c = pow(N, -1, C.p)
    m = 1 - N*c
    return C.mul(m % (N*C.p), Pt)

def run_size(bits, rng, T, H, lift_offset_seed):
    p = next_prime((1 << bits) + rng.randrange(1 << (bits-2)))
    a, b, N, sq = find_curve(p, rng, 'prime')
    out = {'bits': bits, 'p': p, 'a': a, 'b': b, 'N': N}
    # two lift curves (A/A): canonical coefficients and a perturbed lift b+p*t
    t2 = rng.randrange(1, p)
    curves = [Curve(p, a, b), Curve(p, a, b + p*t2)]
    exact = []
    for ci, C in enumerate(curves):
        ok_on = ok_tors = ok_hom = ok_red = 0
        nh_fail_rand = 0
        for _ in range(H):
            x, y = rand_affine(p, a, b, sq, rng)
            x2, y2 = rand_affine(p, a, b, sq, rng)
            P1, P2 = C.lift(x, y, rng), C.lift(x2, y2, rng)
            s1, s2 = section(C, N, P1), section(C, N, P2)
            ok_on += C.on(s1)
            ok_red += (s1[2] % p != 0 and s1[0]*pow(s1[2], -1, p) % p == x and s1[1]*pow(s1[2], -1, p) % p == y)
            ok_tors += (C.eq(C.mul(N, s1), (0, 1, 0)) and C.mul(N, s1)[2] == 0 and C.mul(N, s1)[0] == 0)
            # s(P1+P2) vs s(P1)+s(P2): P1+P2 over F_p via lifts reduced mod p
            Pm = C.add(P1, P2); Pmr = (Pm[0] % p, Pm[1] % p, Pm[2] % p)
            if Pmr[2] == 0:
                ok_hom += 1; continue  # P2=-P1 case, skip
            zi = pow(Pmr[2], -1, p)
            xs, ys = Pmr[0]*zi % p, Pmr[1]*zi % p
            sm = section(C, N, C.lift(xs, ys, rng))
            ok_hom += C.eq(sm, C.add(s1, s2))
            # negative control: random lifts are not homomorphic
            r1, r2 = C.lift(x, y, rng), C.lift(x2, y2, rng)
            rs = C.lift(xs, ys, rng)
            nh_fail_rand += (not C.eq(rs, C.add(r1, r2)))
        exact.append({'lift_curve': ci, 'n': H, 'on_curve': ok_on, 'reduces_to_P': ok_red,
                      'N_s_is_O_exact': ok_tors, 'homomorphic': ok_hom,
                      'random_lift_nonhom_rate': nh_fail_rand / H})
    # Smart functional on torsion section: psi(N s(Q)) must be 0; and ratio test on random lifts
    C = curves[0]
    d_hits = [0, 0]; degenerate = 0; psi_sec_nonzero = 0
    G0 = rand_affine(p, a, b, sq, rng)
    G0l = C.lift(G0[0], G0[1], rng)
    for i in range(T):
        d = rng.randrange(1, N)
        x, y = rand_affine(p, a, b, sq, rng)
        Gt = C.lift(x, y, rng)
        Qa = C.mul(d, Gt)                      # affine-reduction of dG
        zi = pow(Qa[2], -1, p); qx, qy = Qa[0]*zi % p, Qa[1]*zi % p
        Qt = C.lift(qx, qy, rng)               # independent random lift of Q
        pg, pq = C.psi(C.mul(N, Gt)), C.psi(C.mul(N, Qt))
        if pg == 0: degenerate += 1; continue
        if pq * pow(pg, -1, p) % p == d % p: d_hits[i & 1] += 1
        if i < H:
            sg, sq_ = section(C, N, Gt), section(C, N, Qt)
            psi_sec_nonzero += (C.mul(N, sg)[0] != 0 or C.mul(N, sq_)[0] != 0)
    thr = binom_tail_threshold(T, 1.0/p, 1e-4)
    out.update({'exact_arms': exact, 'T': T, 'degenerate': degenerate,
                'hits_half_A': d_hits[0], 'hits_half_B': d_hits[1], 'hits_total': sum(d_hits),
                'expected_hits': T/p, 'hit_threshold_alpha1e-4': thr,
                'psi_of_section_nonzero': psi_sec_nonzero})
    return out

def positive_control(rng, bits, T):
    p = next_prime((1 << bits) + rng.randrange(1 << (bits-2)))
    a, b, N, sq = find_curve(p, rng, 'anomalous')
    assert N == p
    C = Curve(p, a, b); rec = 0; tried = 0
    for _ in range(T):
        d = rng.randrange(1, p)
        x, y = rand_affine(p, a, b, sq, rng)
        Gt = C.lift(x, y, rng); Qa = C.mul(d, Gt)
        zi = pow(Qa[2], -1, p); Qt = C.lift(Qa[0]*zi % p, Qa[1]*zi % p, rng)
        pg, pq = C.psi(C.mul(p, Gt)), C.psi(C.mul(p, Qt))
        if pg == 0: continue
        tried += 1; rec += (pq*pow(pg, -1, p) % p == d)
    return {'p': p, 'N': N, 'a': a, 'b': b, 'trials': tried, 'recovered': rec}

def main():
    smoke = '--smoke' in sys.argv
    t0 = time.time()
    seed = 20261110
    if smoke: sizes, T, H, pb, PT = [10], 150, 20, 10, 60
    else: sizes, T, H, pb, PT = [12, 14, 16], 20000, 150, 11, 500
    rng = random.Random(seed)
    res = [run_size(b, random.Random(seed + b), T, H, 0) for b in sizes]
    pos = positive_control(random.Random(seed + 999), pb, PT)
    falsified = inconclusive = False; why = []
    for r in res:
        for e in r['exact_arms']:
            if not (e['on_curve'] == e['reduces_to_P'] == e['N_s_is_O_exact'] == e['homomorphic'] == e['n']):
                falsified = True; why.append('section property failed at bits %d' % r['bits'])
            if e['random_lift_nonhom_rate'] < 0.9:
                inconclusive = True; why.append('negative control (random lift non-hom) weak')
        if r['psi_of_section_nonzero']:
            falsified = True; why.append('psi(N s(P)) nonzero')
        if r['hits_total'] >= r['hit_threshold_alpha1e-4'] and not smoke:
            falsified = True; why.append('d recovered above chance at bits %d' % r['bits'])
    if pos['recovered'] != pos['trials'] or pos['trials'] == 0:
        inconclusive = True; why.append('positive control (anomalous Smart) failed')
    verdict = 'falsified' if falsified else ('inconclusive' if inconclusive else 'consistent_with_prediction')
    print(json.dumps({'lead': 'LEAD-TRF-padic-canonical-section', 'smoke': smoke, 'seed': seed,
        'params': {'sizes_bits': sizes, 'T_ratio_trials': T, 'H_exact_trials': H},
        'statistic': 'hits of dhat=psi(N Q~)/psi(N G~) == d mod p over random lifts vs Binomial(T,1/p); exact section properties',
        'sizes': res, 'positive_control_anomalous_smart': pos,
        'negative_control': 'random (non-homomorphic) lifts: see random_lift_nonhom_rate and hits_total vs expected_hits',
        'aa_arm': 'second lift curve b+p*t (exact_arms[1]) and hit halves A/B',
        'reasons': why, 'verdict': verdict, 'wall_s': round(time.time()-t0, 2)}))
    return 0
if __name__ == '__main__': sys.exit(main())
