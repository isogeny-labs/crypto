import sys, json, random, math, statistics
SMOKE = '--smoke' in sys.argv
LOGN = 12 if SMOKE else 20
TR = 20 if SMOKE else 300
N = (1 << LOGN) - 3  # fixed modulus (group Z/N, additive walks)
R = 16

def make_sel(kind, rng):
    if kind == 'hash':
        s = rng.getrandbits(32) | 1
        return lambda x: ((x * s) >> 7) & (R - 1)
    if kind == 'popc':  # 4 parity planes of masked popcount
        ms = [rng.getrandbits(LOGN) | 1 for _ in range(4)]
        return lambda x: sum((bin(x & m).count('1') & 1) << i for i, m in enumerate(ms))
    if kind == 'popc2':  # null/positive control: only 2 classes
        m = rng.getrandbits(LOGN) | 1
        return lambda x: bin(x & m).count('1') & 1
    if kind == 'low':  # low-bits selection, known-structured
        return lambda x: x & (R - 1)

def rho_len(sel, adds, x0):
    seen = {}
    x = x0; i = 0
    while x not in seen:
        seen[x] = i
        x = (x + adds[sel(x)]) % N
        i += 1
    return i

def arm(kind, seed):
    rng = random.Random(seed)
    L = []
    for t in range(TR):
        sel = make_sel(kind, rng)
        adds = [rng.randrange(1, N) for _ in range(R if kind in ('hash', 'popc', 'low') else 2)]
        L.append(rho_len(sel, adds, rng.randrange(N)))
    m = statistics.mean(L)
    se = statistics.stdev(L) / math.sqrt(len(L))
    return m, se

base = math.sqrt(math.pi * N / 2)
res = {}
for k, seed in [('hash', 1), ('hash', 2), ('popc', 3), ('popc2', 4), ('low', 5)]:
    key = k + ('_AA' if (k, seed) == ('hash', 2) else '')
    m, se = arm(k, seed)
    res[key] = {'mean_len': m, 'se': se, 'ratio_to_sqrt_piN_2': m / base}
d = lambda a, b: (res[a]['mean_len'] - res[b]['mean_len'], math.hypot(res[a]['se'], res[b]['se']))
dp, sp = d('popc', 'hash')
da, sa = d('hash_AA', 'hash')
rel = res['popc']['mean_len'] / res['hash']['mean_len']
aa_ok = abs(da) <= 3 * sa
pos_ok = res['popc2']['mean_len'] > 1.15 * res['hash']['mean_len']
if not (aa_ok and pos_ok):
    verdict = 'inconclusive'
elif abs(dp) <= 3 * sp and 0.9 <= rel <= 1.1:
    verdict = 'consistent_with_prediction'
else:
    verdict = 'falsified'
if SMOKE and verdict == 'inconclusive':
    pass
print(json.dumps({'lead': 'LEAD-RHO-ecc2k130-gpu-fpga-throughput', 'smoke': SMOKE, 'N': N, 'trials': TR,
  'sel_classes': R, 'seeds': {'hash': 1, 'hash_AA': 2, 'popc': 3, 'popc2': 4, 'low': 5},
  'arms': res, 'stat_popc_over_hash': rel, 'diff_popc_hash': dp, 'diff_se': sp,
  'aa_ok': aa_ok, 'positive_control_ok': pos_ok, 'verdict': verdict}))
