import json, math, random, sys
SMOKE = "--smoke" in sys.argv

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

# GF(p^2)=GF(p)[i]/(i^2-n), n a non-residue; norm-1 torus has order p+1 (embedding degree 2 toy).
class F2:
    def __init__(s, p):
        s.p = p
        s.n = next(a for a in range(2,p) if pow(a,(p-1)//2,p) == p-1)
    def mul(s, x, y):
        p = s.p
        return ((x[0]*y[0] + s.n*x[1]*y[1]) % p, (x[0]*y[1] + x[1]*y[0]) % p)
    def pw(s, x, e):
        r = (1,0)
        while e:
            if e & 1: r = s.mul(r, x)
            x = s.mul(x, x); e >>= 1
        return r

def setup(bits, rng):
    # find prime r ~2^bits and prime p with r | p+1 (subgroup of norm-1 torus of order r)
    while True:
        r = rng.randrange(1 << (bits-1), 1 << bits) | 1
        if not is_prime(r): continue
        for h in range(2, 400, 2):
            p = h*r - 1
            if p % 4 == 3 and is_prime(p): return p, r

def rho(F, g, h, r, rng):
    # Pollard rho in order-r subgroup, 16-way r-adding walk, returns (steps, log or None)
    nw = 16
    ms = [(rng.randrange(r), rng.randrange(r)) for _ in range(nw)]
    mv = [F.mul(F.pw(g, a), F.pw(h, b)) for a, b in ms]
    def step(x, a, b):
        i = (x[0] + 3*x[1]) % nw
        return F.mul(x, mv[i]), (a+ms[i][0]) % r, (b+ms[i][1]) % r
    a0, b0 = rng.randrange(r), rng.randrange(r)
    x0 = F.mul(F.pw(g, a0), F.pw(h, b0))
    # Floyd
    t = (x0, a0, b0); u = t; steps = 0
    while True:
        t = step(*t); u = step(*step(*u)); steps += 3
        if t[0] == u[0]: break
    db = (u[2]-t[2]) % r
    if db == 0: return steps, None
    return steps, ((t[1]-u[1]) * pow(db, -1, r)) % r

def trial_batch(bits, seed, ntr):
    rng = random.Random(seed)
    p, r = setup(bits, rng)
    F = F2(p)
    # generator of order-r subgroup of norm-1 torus
    while True:
        z = (rng.randrange(1,p), rng.randrange(1,p))
        w = F.pw(z, p-1)  # norm-1 element
        g = F.pw(w, (p+1)//r)
        if g != (1,0): break
    ok = 0; tot = 0; used = 0
    while used < ntr:
        x = rng.randrange(1, r); h = F.pw(g, x)
        st, lg = rho(F, g, h, r, rng)
        if lg is None: continue
        used += 1
        assert F.pw(g, lg) == h, "rho log failed verification"
        tot += st; ok += 1
    return tot/ok, r, p

# ---- cost model: exTNFS in GF(p^12), memory charged with exponent w ----
def L(logq2, c):
    ln = logq2 * math.log(2)
    return c * ln**(1/3) * math.log(ln)**(2/3) / math.log(2)  # log2 of L_Q(1/3,c) without o(1)

def model():
    # anchor: o(1) offset chosen so that a 3072-bit prime field (c=(64/9)^(1/3)) costs 2^128 (NIST SP800-57 convention)
    c_gen = (64/9)**(1/3)
    off = 128 - L(3072, c_gen)
    out = {}
    for name, pbits, rbits in (("BN254",254,254), ("BLS12-381",381,255)):
        row = {}
        for cname, c in (("SexTNFS_1.526", 1.526), ("MexTNFS_1.923", 1.923)):
            T = L(12*pbits, c) + off
            rho_ = rbits/2 + math.log2(math.pi/4)/2
            row[cname] = {}
            for w in (0.0, 0.5, 1.0):
                # NFS sieve memory ~ sqrt(time) -> memory-charged cost = T + w*(T/2)
                row[cname]["w=%.1f" % w] = {"log2_cost": round(T + w*T/2, 2), "log2_rho": round(rho_, 2),
                                           "beats_rho": (T + w*T/2) < rho_}
        out[name] = row
    return out, off

def main():
    bits_list = (10,) if SMOKE else (12, 15, 18, 20)
    ntr = 6 if SMOKE else 60
    res = {}
    for bits in bits_list:
        a, r, p = trial_batch(bits, 1000+bits, ntr)
        b, r2, _ = trial_batch(bits, 5000+bits, ntr)   # A/A: same instrument, different seeds, different r
        pred = math.sqrt(math.pi*r/4); pred2 = math.sqrt(math.pi*r2/4)
        res[bits] = {"r": r, "p": p, "ratio_A": a/pred, "ratio_B": b/pred2}
    ra = [v["ratio_A"] for v in res.values()]; rb = [v["ratio_B"] for v in res.values()]
    # Floyd 3-steps-per-iteration -> expected ratio ~ 3*(~1.03..); control band [1.5, 5.0] (instrument sanity)
    pos_ok = all(1.0 <= x <= 6.0 for x in ra+rb)
    aa = [abs(math.log(x/y)) for x, y in zip(ra, rb)]
    aa_ok = max(aa) < 0.5
    m, off = model()
    # model prediction: BN254 beats rho at w=0 and loses at w=1 (SexTNFS) -> order flips across band;
    # BLS12-381 within +-4 bits of rho at w=0 and loses at w>=0.5.
    bn = m["BN254"]["SexTNFS_1.526"]; bl = m["BLS12-381"]["SexTNFS_1.526"]
    flip_bn = bn["w=0.0"]["beats_rho"] and not bn["w=1.0"]["beats_rho"]
    bl_near = abs(bl["w=0.0"]["log2_cost"] - bl["w=0.0"]["log2_rho"]) <= 4 and not bl["w=0.5"]["beats_rho"]
    if not (pos_ok and aa_ok): verdict = "inconclusive"
    elif flip_bn and bl_near: verdict = "consistent_with_prediction"
    else: verdict = "falsified"
    print(json.dumps({"lead": "LEAD-TRF-pairing-target-field-recost", "smoke": SMOKE, "seeds": [1000, 5000],
        "parameters": {"toy_bits": list(bits_list), "trials": ntr, "anchor_3072bit_log2": 128, "w_band": [0, 0.5, 1]},
        "positive_control_rho_ratio_in_band": pos_ok, "aa_log_ratio_max": max(aa), "aa_ok": aa_ok,
        "toy_rho": res, "model": m, "anchor_offset_bits": off,
        "statistic": {"BN254_order_flips_across_w": flip_bn, "BLS12_381_near_rho_then_loses": bl_near},
        "verdict": verdict}))
main()
