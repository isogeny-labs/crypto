import sys, json, hashlib, math, random
SMOKE = "--smoke" in sys.argv
P_BITS = 11 if SMOKE else 14
NA, NG, NW = (150, 40, 8) if SMOKE else (3000, 2000, 150)
DBITS = 3 if SMOKE else 6          # hidden predicate density 2^-DBITS
KEY = b"hidden-key-fixed"

def isprime(n):
    if n < 2: return False
    i = 2
    while i*i <= n:
        if n % i == 0: return False
        i += 1
    return True
p = (1 << P_BITS) - 1
while not isprime(p): p -= 1
cnt = [0]*p
for y in range(p): cnt[y*y % p] += 1
def order(b):
    return 1 + sum(cnt[(x*x*x - 3*x + b) % p] for x in range(p))
def factors(n):
    f = []; d = 2
    while d*d <= n:
        while n % d == 0: f.append(d); n //= d
        d += 1
    if n > 1: f.append(n)
    return f
def b_of(seed):
    return int.from_bytes(hashlib.sha256(b"seed%d" % seed).digest(), "big") % p
def hidden(seed):  # keyed predicate independent of curve arithmetic
    h = hashlib.sha256(KEY + b"%d" % seed).digest()
    return int.from_bytes(h[:4], "big") >> (32 - DBITS) == 0
def valid(b): return (4*(-27) + 27*b*b) % p != 0
def screens(b):
    N = order(b); t = p + 1 - N; tw = p + 1 + t
    pf = factors(N); tf = factors(tw)
    k = 1; x = p % N
    if N > 1:
        while x != 1 and k <= 30: x = x*p % N; k += 1
    return {"order_prime": len(pf) == 1 and N > 3,
            "not_anomalous": N != p,
            "embedding_deg_gt_30": k > 30,
            "twist_large_factor": max(tf) > 50,
            "order_large_factor": max(pf) > 50}
NAMES = list(screens(1 if valid(1) else 2))
NAMES = ["order_prime","not_anomalous","embedding_deg_gt_30","twist_large_factor","order_large_factor"]
def rates(bs):
    c = dict.fromkeys(NAMES, 0)
    for b in bs:
        for k, v in screens(b).items(): c[k] += v
    return c
def z2(c1, n1, c2, n2):
    pp = (c1 + c2) / (n1 + n2)
    s = math.sqrt(max(pp*(1-pp)*(1/n1+1/n2), 1e-18))
    return (c1/n1 - c2/n2) / s if s > 1e-9 else 0.0

rnd = random.Random(20261110)
def rand_arm(n, lo):
    out = []; s = lo
    while len(out) < n:
        b = b_of(s); s += 1
        if valid(b): out.append(b)
    return out
A  = rand_arm(NA, 0)
A2 = rand_arm(NA, 10**7)
# grinding arm G: scan seeds until hidden predicate holds
G = []; trials = []; s = 2*10**7
while len(G) < NG:
    t = 0
    while True:
        t += 1; s += 1
        if hidden(s) and valid(b_of(s)): break
    trials.append(t); G.append(b_of(s))
mean_trials = sum(trials)/len(trials)
# positive control: grind for anomalous curves (N == p)
W = []; wt = []; s = 3*10**7
while len(W) < NW:
    t = 0
    while True:
        t += 1; s += 1; b = b_of(s)
        if valid(b) and order(b) == p: break
    wt.append(t); W.append(b)
cA, cA2, cG, cW = rates(A), rates(A2), rates(G), rates(W)
zAA = {k: z2(cA[k], NA, cA2[k], NA) for k in NAMES}
zGA = {k: z2(cG[k], NG, cA[k], NA) for k in NAMES}
thr = 3.3
aa_ok = all(abs(v) < thr for v in zAA.values())
null_ok = all(abs(v) < thr for v in zGA.values())
pos_ok = cW["not_anomalous"] == 0 and cA["not_anomalous"] > 0.9*NA
exp_trials = 2**DBITS
trial_ok = abs(mean_trials/exp_trials - 1) < 0.15
if not (aa_ok and pos_ok):
    verdict = "inconclusive"
elif null_ok and trial_ok:
    verdict = "consistent_with_prediction"
elif not null_ok:
    verdict = "falsified"
else:
    verdict = "inconclusive"
print(json.dumps({"lead": "LEAD-STR-curve-generation-provenance-seeds", "smoke": SMOKE,
  "p": p, "n_random": NA, "n_ground": NG, "n_anom": NW, "hidden_density_bits": DBITS,
  "seeds": {"rng": 20261110, "A": "0..", "A2": "1e7..", "G": "2e7..", "W": "3e7.."},
  "pass_counts": {"A": cA, "A2": cA2, "G": cG, "W": cW},
  "z_AA": zAA, "z_GvsA": zGA, "z_threshold": thr,
  "mean_grind_trials": mean_trials, "expected_trials": exp_trials,
  "mean_trials_to_anomalous": sum(wt)/len(wt),
  "checks": {"aa_ok": aa_ok, "null_ok": null_ok, "positive_control_ok": pos_ok, "trial_cost_ok": trial_ok},
  "verdict": verdict}))
