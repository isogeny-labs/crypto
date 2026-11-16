import sys, json, random
from array import array
SMOKE = '--smoke' in sys.argv
LOGN = 16 if SMOKE else 22
N = 1 << LOGN
B = 60 if SMOKE else 200
C = 20 if SMOKE else 50
REPS = 3 if SMOKE else 20
AIM = 30  # aimed sampler: multiples of 2*3*5
spf = array('I', [0]) * (N + 1)
for p in range(2, N + 1):
    if spf[p] == 0:
        if p * p > N:
            # remaining zeros are primes: handled lazily in factor()
            break
        for m in range(p * p, N + 1, p):
            if spf[m] == 0:
                spf[m] = p
def factor(x):
    f = []
    while x > 1:
        p = spf[x] or x
        f.append(p); x //= p
    return f
FB = [p for p in range(2, B + 1) if (spf[p] or p) == p]
K = len(FB)
TARGET = K + 10
def classify(x, L):
    big = [p for p in factor(x) if p > B]
    if len(big) > 2 or any(p > L for p in big): return None
    return big
def collect(mode, aimed, seed, L):
    rng = random.Random(seed)
    par = {}
    def find(a):
        while par.setdefault(a, a) != a:
            par[a] = par[par[a]]; a = par[a]
        return a
    usable = n = 0
    while usable < TARGET and n < 3000000:
        n += 1
        x = rng.randrange(1, N // AIM) * AIM if aimed else rng.randrange(2, N + 1)
        if x < 2: continue
        big = classify(x, L)
        if big is None: continue
        if len(big) == 0: usable += 1; continue
        if mode == 'plain': continue
        if mode == '1lp' and len(big) != 1: continue
        a, b = (big[0], 1) if len(big) == 1 else (big[0], big[1])
        ra, rb = find(a), find(b)
        if ra == rb: usable += 1
        else: par[ra] = rb
    return n if usable >= TARGET else None
def mean_cost(mode, aimed, seeds, L):
    v = [collect(mode, aimed, s, L) for s in seeds]
    ok = [t for t in v if t]
    return (sum(ok) / len(ok) if len(ok) == len(v) else None), v
seedsA = [1000 + i for i in range(REPS)]
seedsB = [5000 + i for i in range(REPS)]
L = B * C
res = {}
for aimed in (False, True):
    tag = 'aimed' if aimed else 'unaimed'
    pl, _ = mean_cost('plain', aimed, seedsA, L)
    plb, _ = mean_cost('plain', aimed, seedsB, L)
    l1, _ = mean_cost('1lp', aimed, seedsA, L)
    l2, _ = mean_cost('2lp', aimed, seedsA, L)
    l2b, _ = mean_cost('2lp', aimed, seedsB, L)
    l2null, _ = mean_cost('2lp', aimed, seedsA, B)  # L=B: no large primes admitted
    res[tag] = dict(plain=pl, plain_B=plb, lp1=l1, lp2=l2, lp2_B=l2b, lp2_null=l2null)
def g(a, b): return (a / b) if a and b else None
out = dict(lead='LEAD-ICB-large-prime-variants', smoke=SMOKE, logN=LOGN, B=B, C=C, K=K, target=TARGET,
           reps=REPS, seeds_A=[seedsA[0], seedsA[-1]], seeds_B=[seedsB[0], seedsB[-1]], cost=res)
try:
    gu, ga = g(res['unaimed']['plain'], res['unaimed']['lp2']), g(res['aimed']['plain'], res['aimed']['lp2'])
    ratio = ga / gu
    null_u = g(res['unaimed']['plain'], res['unaimed']['lp2_null'])
    null_a = g(res['aimed']['plain'], res['aimed']['lp2_null'])
    aa = abs(res['unaimed']['plain'] / res['unaimed']['plain_B'] - 1)
    aa2 = abs(res['unaimed']['lp2'] / res['unaimed']['lp2_B'] - 1)
    ctrl_ok = (gu > 1.2 and abs(null_u - 1) < 1e-9 and abs(null_a - 1) < 1e-9 and aa < 0.15 and aa2 < 0.15)
    out.update(gain_unaimed=gu, gain_aimed=ga, ratio_aimed_over_unaimed=ratio, null_gain=[null_u, null_a],
               aa_dev=[aa, aa2], controls_ok=ctrl_ok)
    out['verdict'] = 'inconclusive' if not ctrl_ok else ('consistent_with_prediction' if ratio >= 0.8 and ga > 1.2 else 'falsified')
except Exception as e:
    out.update(error=repr(e), verdict='inconclusive')
print(json.dumps(out))
