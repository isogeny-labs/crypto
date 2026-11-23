import sys, json, random, math
SMOKE = '--smoke' in sys.argv
P = 251 if SMOKE else 4093
K = 8 if SMOKE else 60          # curves per population
NS = 500 if SMOKE else 15000     # random (x1,x2) pairs per curve for split fraction
FB = 16 if SMOKE else 96         # factor-base size (x-coords with rational points)
NT = 50 if SMOKE else 150        # targets x3 per curve for relation rate
SEEDS = {"A": 1101, "B": 2202, "perm": 3303, "bias": 4404}

def chi(v, p):
    v %= p
    if v == 0: return 0
    return 1 if pow(v, (p-1)//2, p) == 1 else -1

def curve(rng, p):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b**2) % p: return a, b

def disc(a, b, p, x1, x2):
    # disc/4 of S_3 in x3
    s = (x1+x2)*(x1*x2+a)+2*b
    return (s*s - (x1-x2)**2*((x1*x2-a)**2 - 4*b*(x1+x2))) % p

def census(rng, a, b, p, bias=0.0):
    sp = 0
    for _ in range(NS):
        d = disc(a, b, p, rng.randrange(p), rng.randrange(p))
        ok = chi(d, p) == 1
        if bias and rng.random() < bias: ok = not ok if rng.random() < 0.5 else ok
        sp += ok
    return sp / NS

def relrate(rng, a, b, p):
    f = lambda x: (x**3 + a*x + b) % p
    rat = [x for x in range(p) if chi(f(x), p) >= 0]
    fb = rng.sample(rat, min(FB, len(rat)))
    tot = 0
    for _ in range(NT):
        x3 = rng.choice(rat)
        for i, u in enumerate(fb):
            for v in fb[i:]:
                # S_3(u,v,x3)=0 evaluated directly
                s = ((u-v)**2*x3*x3 - 2*((u+v)*(u*v+a)+2*b)*x3 + (u*v-a)**2 - 4*b*(u+v)) % p
                if s == 0: tot += 1
    return tot / NT

def pop(seed, bias_half=False):
    rng = random.Random(seed)
    F, R = [], []
    for c in range(K):
        a, b = curve(rng, P)
        bi = 0.06 if (bias_half and c % 2) else 0.0
        F.append(census(rng, a, b, P, bi))
        R.append(relrate(rng, a, b, P) if not bias_half else 0.0)
    return F, R

def mean(x): return sum(x)/len(x)
def var(x):
    m = mean(x); return sum((t-m)**2 for t in x)/(len(x)-1)
def corr(x, y):
    mx, my = mean(x), mean(y)
    sx = math.sqrt(sum((t-mx)**2 for t in x)); sy = math.sqrt(sum((t-my)**2 for t in y))
    return 0.0 if sx*sy == 0 else sum((a-mx)*(b-my) for a, b in zip(x, y))/(sx*sy)

def disp(F):  # observed variance across curves / binomial variance at s=mean
    s = mean(F); return var(F)/(s*(1-s)/NS)

FA, RA = pop(SEEDS["A"])
FB_, RB = pop(SEEDS["B"])
FP, _ = pop(SEEDS["A"]+7, bias_half=True)
dA, dB, dPos = disp(FA), disp(FB_), disp(FP)
r_obs = corr(FA, RA)
rng = random.Random(SEEDS["perm"]); n = 0; NP = 200 if SMOKE else 2000
for _ in range(NP):
    s = RA[:]; rng.shuffle(s)
    if abs(corr(FA, s)) >= abs(r_obs): n += 1
pperm = (n+1)/(NP+1)
THR = 1.6  # dispersion threshold; chi2-like ratio with K-1 df
aa_ok = abs(math.log(dA/dB)) < 0.5
pos_ok = dPos > 2*THR
unif = dA < THR and dB < THR
nocorr = pperm > 0.01
if not (pos_ok and aa_ok): verdict = "inconclusive"
elif unif and nocorr: verdict = "consistent_with_prediction"
elif (not unif) or (not nocorr): verdict = "falsified"
print(json.dumps({"lead": "LEAD-ICE-semaev-cover-monodromy", "smoke": SMOKE, "p": P, "K": K,
  "NS": NS, "FB": FB, "NT": NT, "seeds": SEEDS,
  "dispersion_A": dA, "dispersion_B_AA": dB, "dispersion_positive_control": dPos,
  "split_mean_A": mean(FA), "corr_split_vs_relrate": r_obs, "perm_p": pperm,
  "thresholds": {"dispersion": THR, "perm_p": 0.01, "positive_control": 2*THR, "AA_logratio": 0.5},
  "negative_control": "label-shuffled relation rates (permutation null)",
  "verdict": verdict}))
