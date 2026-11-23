import sys, json, math, random
SMOKE = "--smoke" in sys.argv
M64 = (1 << 64) - 1

def is_prime(n):
    if n < 2: return False
    for p in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % p == 0: return n == p
    d = n-1; s = 0
    while d % 2 == 0: d //= 2; s += 1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a, d, n)
        if x in (1, n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def safe_prime(start):
    q = start | 1
    while True:
        if is_prime(q) and is_prime(2*q+1): return q
        q += 2

def combos(n, w):
    from itertools import combinations
    return [sum(1 << i for i in c) for c in combinations(range(n), w)]

def mix(z, salt):
    x = ((z ^ salt) * 0x9E3779B97F4A7C15) & M64
    x ^= x >> 29; x = (x * 0xBF58476D1CE4E5B9) & M64
    x ^= x >> 32
    return x

def make_instance(m, w, q, P, g, rng, planted=True):
    half = m // 2; w1 = w // 2
    lo = combos(half, w1)
    S = len(lo)
    if planted:
        a = lo[rng.randrange(S)]; b = lo[rng.randrange(S)] << half
        k = a + b
    else:  # null: weight w1+2 in low half, so outside the balanced split class
        while True:
            a = sum(1 << i for i in rng.sample(range(half), w1 + 2))
            b = lo[rng.randrange(S)] << half
            k = a + b
            break
    return lo, S, k, half

def solve(m, w, q, P, g, k, lo, S, half, rng, max_restarts, counter):
    T = pow(g, k, P)
    Ginv = pow(g, q - 1, P)
    E0 = [pow(g, a, P) for a in lo]
    E1 = [T * pow(Ginv, b << half, P) % P for b in lo]
    def f(z, salt):
        counter[0] += 1
        h = mix(z, salt)
        t = h & 1; i = (h >> 1) % S
        return (E1[i] if t else E0[i]), t, i
    for _ in range(max_restarts):
        salt = rng.getrandbits(64)
        x0 = pow(g, rng.randrange(1, q), P)
        # Floyd with O(1) memory
        tort, _, _ = f(x0, salt); hare = f(f(x0, salt)[0], salt)[0]
        guard = 0
        while tort != hare:
            tort = f(tort, salt)[0]; hare = f(f(hare, salt)[0], salt)[0]
            guard += 1
            if guard > 50 * S + 1000: break
        else:
            tort = x0; prev_t = None; prev_h = hare
            # find mu
            hare_prev = None; tort_prev = None
            while tort != hare:
                tort_prev, hare_prev = tort, hare
                tort = f(tort, salt)[0]; hare = f(hare, salt)[0]
            if tort_prev is None: continue  # x0 on cycle, no predecessor pair
            _, t1, i1 = f(tort_prev, salt); _, t2, i2 = f(hare_prev, salt)
            if t1 != t2 and tort_prev != hare_prev:
                a_idx, b_idx = (i1, i2) if t1 == 0 else (i2, i1)
                kk = lo[a_idx] + (lo[b_idx] << half)
                if pow(g, kk, P) == T: return kk
    return None

def run_rung(m, w, q, P, g, seeds, trials, null=False):
    res = []; found = 0
    for sd in seeds:
        rng = random.Random(sd)
        for _ in range(trials):
            lo, S, k, half = make_instance(m, w, q, P, g, rng, planted=not null)
            c = [0]
            kk = solve(m, w, q, P, g, k, lo, S, half, rng, 20 if null else 60, c)
            if kk is not None and (null or kk == k): found += 1
            res.append(c[0])
    return res, found, S

def main():
    q = safe_prime(1 << (14 if SMOKE else 23)); P = 2*q + 1
    g = pow(2, 2, P)
    ladder = [(10, 4)] if SMOKE else [(16, 4), (18, 6), (20, 6), (22, 8)]
    trials = 3 if SMOKE else 120
    seedsA, seedsB = [101, 102], [201, 202]
    if SMOKE: seedsA, seedsB = [101], [201]
    rows = []; xs = []; ys = []; ok_c = True; null_hits = 0; pos_fail = 0
    for (m, w) in ladder:
        assert (1 << m) < q
        ra, fa, S = run_rung(m, w, q, P, g, seedsA, trials)
        rb, fb, _ = run_rung(m, w, q, P, g, seedsB, trials)
        rn, fn, _ = run_rung(m, w, q, P, g, [301], max(1, trials // 6), null=True)
        ma, mb = sum(ra)/len(ra), sum(rb)/len(rb)
        mean = (ma + mb) / 2
        c = mean / math.sqrt(S)
        aa = abs(ma - mb) / mean
        n_inst = len(ra) + len(rb)
        pos_fail += n_inst - fa - fb; null_hits += fn
        xs.append(math.log(S)); ys.append(math.log(mean))
        rows.append(dict(m=m, w=w, half_domain_S=S, mean_f_evals=round(mean, 2), ratio_to_sqrtS=round(c, 3),
                         aa_rel_diff=round(aa, 3), solved=fa+fb, of=n_inst, null_hits=fn, null_runs=len(rn),
                         sqrt_q=round(math.sqrt(q), 1)))
        if not (2 <= c <= 40): ok_c = False
    slope = None
    if len(xs) >= 3:
        mx, my = sum(xs)/len(xs), sum(ys)/len(ys)
        slope = sum((x-mx)*(y-my) for x, y in zip(xs, ys)) / sum((x-mx)**2 for x in xs)
    aa_ok = all(r["aa_rel_diff"] < 0.35 for r in rows) if not SMOKE else True
    if pos_fail > 0 or null_hits > 0 or not aa_ok:
        verdict = "inconclusive"
    elif SMOKE:
        verdict = "inconclusive"
    elif ok_c and 0.35 <= slope <= 0.65:
        verdict = "consistent_with_prediction"
    else:
        verdict = "falsified"
    print(json.dumps(dict(lead="LEAD-RHO-lhw-polymem-collision-walks", smoke=SMOKE, q=q, P=P,
        seeds_A=seedsA, seeds_B=seedsB, null_seed=301, trials_per_seed=trials, rows=rows,
        log_log_slope=slope, positive_control_failures=pos_fail, null_spurious_hits=null_hits,
        aa_ok=aa_ok, verdict=verdict)))
main()
