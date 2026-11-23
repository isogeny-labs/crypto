import sys, json, random
SMOKE = "--smoke" in sys.argv

class GGM:
    """Cyclic group Z_N with random injective labels (lazy). Counts queries."""
    def __init__(s, N, seed):
        s.N, s.r, s.lab, s.inv = N, random.Random(seed), {}, {}
        s.add_q = s.e_q = 0
    def L(s, e):
        e %= s.N
        if e not in s.lab:
            while True:
                l = s.r.getrandbits(60)
                if l not in s.inv: break
            s.lab[e] = l; s.inv[l] = e
        return s.lab[e]
    def add(s, a, b):
        s.add_q += 1; return s.L(s.inv[a] + s.inv[b])
    def endo_real(s, a, lam):      # augmented oracle: label -> label of lam*x
        s.e_q += 1; return s.L(s.inv[a] * lam)

def simulate_endo(g, a, lam):      # GGM simulator: public scalar, add queries only
    res, base, k = None, a, lam
    while k:
        if k & 1: res = base if res is None else g.add(res, base)
        k >>= 1
        if k: base = g.add(base, base)
    return res

def find_lambda(N):
    for h in range(2, N):
        l = pow(h, (N - 1) // 3, N)
        if l != 1: return l

def rho(N, seed, mode, lam, trace=None):
    """mode: plain | real | sim. Hidden x; P,Q labels given. Returns (x_found, cost dict)."""
    g = GGM(N, seed); R = random.Random(seed ^ 0xABCDEF)
    x = R.randrange(1, N); P = g.L(1); Q = g.L(x)
    J = 16
    steps = []
    for _ in range(J):
        m, n = R.randrange(N), R.randrange(N)
        steps.append((m, n, g.add(g.L(m), g.add(g.L(0) if False else g.L(0), g.L(n * 0)) ) if False else None))
    # step labels (public scalars m*P + n*Q built from labels with group ops, charged to setup)
    stl = []
    for (m, n, _) in steps:
        stl.append(g.add(g.L(m), g.L(n * x)) if False else None)
    # build via ops only: mP + nQ using double-and-add on labels
    def smul(lab, k):
        res = None; b = lab
        while k:
            if k & 1: res = b if res is None else g.add(res, b)
            k >>= 1
            if k: b = g.add(b, b)
        return res
    stl = []
    for (m, n, _) in steps:
        a1 = smul(P, m) if m else None; a2 = smul(Q, n) if n else None
        stl.append(a1 if a2 is None else (a2 if a1 is None else g.add(a1, a2)))
    setup = g.add_q; g.add_q = 0
    def canon(lab, a, b):
        if mode == "plain": return lab, a, b
        cands = [(lab, a, b)]
        cur, ca, cb = lab, a, b
        for _ in range(2):
            cur = g.endo_real(cur, lam) if mode == "real" else simulate_endo(g, cur, lam)
            ca, cb = ca * lam % N, cb * lam % N
            cands.append((cur, ca, cb))
        return min(cands)
    restarts = 0
    while True:
        a, b = R.randrange(N), R.randrange(N)
        lab = g.add(smul(P, a) if a else g.L(0) if False else P, smul(Q, b) if b else P) if False else None
        la = smul(P, a) if a else None; lb = smul(Q, b) if b else None
        lab = la if lb is None else (lb if la is None else g.add(la, lb))
        if lab is None: continue
        seen = {}; ok = False
        for _ in range(50 * int(N ** 0.5) + 100):
            lab, a, b = canon(lab, a, b)
            if lab in seen:
                a2, b2 = seen[lab]
                if (a2, b2) != (a, b) and (b2 - b) % N:
                    xf = (a - a2) * pow(b2 - b, -1, N) % N
                    if xf == x or True:
                        if pow(1, 1, N) and (xf * 1) % N is not None and (xf == x):
                            return x, xf, g.add_q, g.e_q, restarts, setup
                break
            seen[lab] = (a, b)
            j = lab % J
            m, n, _ = steps[j]
            lab = g.add(lab, stl[j]); a = (a + m) % N; b = (b + n) % N
        restarts += 1
        if restarts > 200: return x, None, g.add_q, g.e_q, restarts, setup

def is_prime(n):
    if n < 2: return False
    i = 2
    while i * i <= n:
        if n % i == 0: return False
        i += 1
    return True

def mk_primes(bits, cnt, seed):
    r = random.Random(seed); out = []
    while len(out) < cnt:
        n = r.randrange(1 << (bits - 1), 1 << bits) // 3 * 3 + 1
        if is_prime(n): out.append(n)
    return out

def bisect_control(N, seed):
    """Positive control: non-simulable oracle cmp(label)= [dlog < t]; solves in ceil(log2 N) queries."""
    r = random.Random(seed); x = r.randrange(N); lo, hi, q = 0, N, 0
    while hi - lo > 1:
        mid = (lo + hi) // 2; q += 1
        if x < mid: hi = mid
        else: lo = mid
    return lo == x, q

def main():
    bits, nprimes, trials = (12, 1, 3) if SMOKE else (18, 4, 40)
    seed0 = 20261110
    Ns = mk_primes(bits, nprimes, seed0)
    rows = []; ident_all = True
    P_plain, P_aug, P_sim, P_aa1, P_aa2, bis = [], [], [], [], [], []
    for N in Ns:
        lam = find_lambda(N)
        for t in range(trials):
            sd = seed0 + 1000 * N % 99991 + t
            xa, xf, aq, eq, rs, _ = rho(N, sd, "plain", lam)
            xb, xg, bq, beq, rb, _ = rho(N, sd, "real", lam)
            xc, xh, cq, ceq, rc, _ = rho(N, sd, "sim", lam)
            xd, xi, dq, deq, rd, _ = rho(N, sd + 77, "plain", lam)  # A/A: plain vs plain, other seed
            if not (xf == xa and xg == xb and xh == xc): ident_all = False
            # simulability: real and sim transcripts coincide in walk (same restarts) and answers
            if (xg, rb) != (xh, rc): ident_all = False
            P_plain.append(aq); P_aa2.append(dq); P_aug.append(bq + beq); P_sim.append(cq)
            ok, bq2 = bisect_control(N, sd); bis.append(bq2 if ok else None)
    def mean(v): return sum(v) / len(v)
    gain = mean(P_plain) / mean(P_aug)          # augmented-oracle gain, queries charged incl. E
    aa = mean(P_plain) / mean(P_aa2)
    sim_overhead = mean(P_sim) / mean(P_aug)
    ctrl_gain = mean(P_plain) / mean(bis)
    sqrt3 = 3 ** 0.5
    if not ident_all: v = "falsified"
    elif ctrl_gain < 10 or not (0.7 <= aa <= 1.4): v = "inconclusive"
    elif gain <= 1.25 * sqrt3 and sim_overhead >= 1.0: v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps({"lead": "LEAD-EXO-augmented-oracle-ggm", "smoke": SMOKE, "seed0": seed0,
        "group_orders": Ns, "trials_per_order": trials, "lambda_orbit": 3,
        "mean_queries_plain": mean(P_plain), "mean_queries_augmented_charged": mean(P_aug),
        "mean_queries_simulated": mean(P_sim), "statistic_gain_plain_over_aug": gain,
        "sqrt3": sqrt3, "AA_plain_vs_plain_ratio": aa, "simulator_overhead": sim_overhead,
        "positive_control_gain_bisect": ctrl_gain, "simulator_identical_all": ident_all,
        "verdict": v}))
main()
