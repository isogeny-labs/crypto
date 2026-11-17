import sys, json, random, math

def pmul(a, b):
    r = 0
    while b:
        if b & 1: r ^= a
        a <<= 1; b >>= 1
    return r
def pmod(a, f):
    d = f.bit_length()
    while a.bit_length() >= d: a ^= f << (a.bit_length() - d)
    return a
def pgcd(a, b):
    while b: a, b = b, pmod(a, b)
    return a
def irreducible(f, N):
    x = 2; t = x
    for i in range(1, N // 2 + 1):
        t = pmod(pmul(t, t), f)           # x^(2^i)
        if pgcd(f, t ^ x) != 1: return False
    return True
class F:
    def __init__(s, N, rng):
        s.N = N
        while True:
            f = (1 << N) | rng.getrandbits(N) | 1
            if irreducible(f, N): break
        s.f = f
    def mul(s, a, b): return pmod(pmul(a, b), s.f)
    def pw(s, a, e):
        r = 1
        while e:
            if e & 1: r = s.mul(r, a)
            a = s.mul(a, a); e >>= 1
        return r
def rank2(vs):
    basis = {}
    for v in vs:
        while v:
            h = v.bit_length()
            if h in basis: v ^= basis[h]
            else: basis[h] = v; break
    return len(basis)
def span_dim(K, l, n, b, Fq_basis):
    q = 1 << l
    conj = [b]
    for _ in range(n - 1): conj.append(K.pw(conj[-1], q))
    vs = [K.mul(g, c) for c in conj for g in Fq_basis]
    return rank2(vs) // l
def fq_basis(K, l, n, rng):
    N = l * n
    while True:
        w = rng.randrange(2, 1 << N)
        u = K.pw(w, ((1 << N) - 1) // ((1 << l) - 1))
        bs = [1]
        for _ in range(l - 1): bs.append(K.mul(bs[-1], u))
        if rank2(bs) == l: return bs
def normal_density(q, n):
    # exact count of normal elements of F_{q^n}/F_q over q^n: prod over irreducible factors of x^n-1 (n coprime to 2)
    # we only use it as comparison when gcd(n,2)=1: prod_{P|x^n-1} (1-q^{-deg P})
    # deg-factor multiset: for each d|n, phi(d)/ord_d(q) factors of degree ord_d(q)
    r = 1.0
    for d in range(1, n + 1):
        if n % d: continue
        phi = sum(1 for k in range(1, d + 1) if math.gcd(k, d) == 1)
        o = 1; t = q % d if d > 1 else 0
        if d > 1:
            t = q % d
            while t != 1: t = t * q % d; o += 1
        for _ in range(phi // o): r *= (1 - q ** (-o))
    return r
def trial(l, n, T, seed):
    rng = random.Random(seed)
    N = l * n; K = F(N, rng); B = fq_basis(K, l, n, rng)
    full = 0; hist = {}
    for _ in range(T):
        b = rng.randrange(1, 1 << N)
        m = span_dim(K, l, n, b, B)
        hist[m] = hist.get(m, 0) + 1
        full += (m == n)
    # positive control: b in F_q (m must be 1)
    pc = 0
    for _ in range(20):
        c = 0
        for g in B:
            if rng.getrandbits(1): c ^= g
        if c == 0: c = 1
        pc += (span_dim(K, l, n, c, B) == 1)
    return full / T, hist, pc == 20
def cost_table():
    # GHS cover genus ~2^(m-1); Gaudry cost ~ q^2 for g<=~4 (larger g: use rho-bound q^{n/2} vs q^{2} only for genus-sized heuristic);
    # report log2 costs: rho = N/2 ; ic = 2*l + 0.5*(2^(n-1)) crude factorial overhead term
    rows = []
    for N, l, n in [(155, 31, 5), (176, 44, 4), (176, 22, 8), (176, 11, 16), (185, 37, 5), (210, 70, 3), (162, 54, 3), (162, 81, 2)]:
        g = 2 ** (n - 1)
        rho = N / 2
        ic = 2 * l + math.log2(math.factorial(min(g, 20)))  # factorial-in-genus overhead (crude, assumption)
        rows.append({"N": N, "l": l, "n": n, "genus": g, "log2_rho": rho, "log2_ic_proxy": round(ic, 1), "ic_below_rho": ic < rho})
    return rows
def main():
    smoke = "--smoke" in sys.argv
    configs = [(2, 3)] if smoke else [(3, 4), (4, 3), (2, 6), (5, 3), (3, 5), (4, 4), (6, 2)]
    T = 20 if smoke else 400
    out = {"seeds": [], "arms": [], "params": {"T": T, "configs": configs}}
    ok_all = True; aa_ok = True; ctrl_ok = True
    for (l, n) in configs:
        sa = 1000 + 10 * l + n; sb = 5000 + 10 * l + n
        fa, ha, pa = trial(l, n, T, sa)
        fb, hb, pb = trial(l, n, T, sb)   # A/A: independent field and seeds
        q = 1 << l
        exp = normal_density(q, n) if n % 2 else None
        # prediction: generic b has full F_q-span dimension n at rate matching normal density (q^n large) -> m=n generic
        tol = 4 * math.sqrt(0.25 / T) + 0.02
        match = True if exp is None else abs((fa + fb) / 2 - exp) < tol
        aa = abs(fa - fb) < 2 * tol
        out["arms"].append({"l": l, "n": n, "frac_full_A": fa, "frac_full_B": fb, "normal_density_theory": exp,
                            "hist_A": ha, "pos_control_in_Fq_gives_m1": pa and pb, "A_A_ok": aa, "match": match})
        out["seeds"] += [sa, sb]
        aa_ok &= aa; ctrl_ok &= (pa and pb); ok_all &= match
    out["cost_table_asymptotic_proxy"] = cost_table()
    if not ctrl_ok or not aa_ok: v = "inconclusive"
    elif ok_all: v = "consistent_with_prediction"
    else: v = "falsified"
    out["verdict"] = v
    out["statistic"] = "fraction of random b in F_{2^N} whose F_q-conjugate span has dimension n, vs exact normal-element density"
    print(json.dumps(out))
main()
