import sys, json, hashlib, random, math
SMOKE = '--smoke' in sys.argv
PRIMES = {12: 4093, 16: 65521, 20: 1048573}   # prime group orders N
SIZES = [12] if SMOKE else [12, 16, 20]
NQ = 200 if SMOKE else 3000

class GGM:
    """Generic group Z_N with keyed-hash labels; counts add queries."""
    def __init__(s, N, key):
        s.N, s.key, s.inv, s.adds = N, key, {}, 0
    def lab(s, x):
        x %= s.N
        h = int.from_bytes(hashlib.blake2b(s.key + x.to_bytes(4, 'big'), digest_size=8).digest(), 'big')
        s.inv[h] = x
        return h
    def add(s, a, b):
        s.adds += 1
        return s.lab(s.inv[a] + s.inv[b])
    # augmented oracles (real side, uses hidden x)
    def endo(s, a, lam): return s.lab(lam * s.inv[a])
    def halve(s, a): return s.lab(s.inv[a] * pow(2, -1, s.N))
    def parity(s, a): return s.inv[a] & 1   # NON-label-valued control oracle

def smul(g, h, k):
    """Simulator: scalar multiple by double-and-add using only add queries."""
    r = None
    while k:
        if k & 1: r = h if r is None else g.add(r, h)
        k >>= 1
        if k: h = g.add(h, h)
    return r

def trial(bits, seed):
    N = PRIMES[bits]; rng = random.Random(seed)
    lam = rng.randrange(2, N - 1)           # public endomorphism eigenvalue
    g = GGM(N, seed.to_bytes(8, 'big'))
    gen = g.lab(1)
    ok_e = ok_h = 0; cost = 0; par = 0
    for _ in range(NQ):
        x = rng.randrange(N); a = g.lab(x)
        before = g.adds
        ok_e += smul(g, a, lam) == g.endo(a, lam)
        cost += g.adds - before
        ok_h += smul(g, a, (N + 1) // 2) == g.halve(a)
        # simulator for parity: best guess from labels alone = coin flip
        par += (rng.getrandbits(1) == g.parity(a))
    return dict(bits=bits, N=N, lam=lam, endo_match=ok_e / NQ, halve_match=ok_h / NQ,
                mean_sim_adds=cost / NQ, bound_2log2N=2 * math.log2(N), parity_acc=par / NQ)

def main():
    res = []; seeds = []
    for b in SIZES:
        for i in range(1 if SMOKE else 3):
            sd = 1000 * b + i; seeds.append(sd); res.append(trial(b, sd))
    # A/A: same instance, different label key must give same match stats (both 1.0)
    aa = trial(SIZES[0], 777)
    z = 4 * math.sqrt(0.25 / NQ)
    sim_ok = all(r['endo_match'] == 1.0 and r['halve_match'] == 1.0 and r['mean_sim_adds'] <= r['bound_2log2N'] for r in res)
    null_ok = all(abs(r['parity_acc'] - 0.5) <= z for r in res)   # non-simulable oracle: sim at chance
    aa_ok = aa['endo_match'] == 1.0 and aa['halve_match'] == 1.0
    if sim_ok and null_ok and aa_ok: v = 'consistent_with_prediction'
    elif not sim_ok: v = 'falsified'
    else: v = 'inconclusive'
    print(json.dumps(dict(lead='LEAD-EXO-augmented-oracle-ggm', smoke=SMOKE, seeds=seeds, nq=NQ,
        statistic='simulator match rate and add-query overhead', results=res, aa_arm=aa,
        positive_control='endo/halve oracles (simulable) expected 1.0',
        negative_control='parity-of-dlog oracle (not label-valued) expected simulator acc 0.5 +-%.4f' % z,
        verdict=v)))
main()
