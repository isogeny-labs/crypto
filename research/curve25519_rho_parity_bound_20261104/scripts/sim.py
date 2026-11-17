"""Monte Carlo check of the k-th-collision cost: draw uniform classes from N = r/2
(negation-folded) and record the number of draws n at which the k-th colliding pair
appears (a new draw equal to m earlier draws adds m pairs). Compare E[n_k]/sqrt(r)
with Gamma(k+1/2)/Gamma(k), and the ratio to k = 1 with Gamma(k+1/2)/(Gamma(k)Gamma(3/2))."""
import json, math, random, sys
r = 131203; N = r // 2 + 1; K = 4; T = int(sys.argv[1]) if len(sys.argv) > 1 else 4000
rng = random.Random(20261104)
acc = [0.0] * (K + 1); sq = [0.0] * (K + 1)
for _ in range(T):
    seen = {}; pairs = 0; n = 0; got = 1
    while got <= K:
        n += 1; c = rng.randrange(N); m = seen.get(c, 0); seen[c] = m + 1; pairs += m
        while got <= K and pairs >= got:
            acc[got] += n; sq[got] += n * n; got += 1
out = {"r": r, "classes": N, "trials": T, "seed": 20261104, "k": {}}
base = acc[1] / T
for k in range(1, K + 1):
    mean = acc[k] / T; sd = math.sqrt(sq[k] / T - mean ** 2)
    out["k"][k] = {"mean_n": mean, "sem": sd / math.sqrt(T), "mean_over_sqrt_r": mean / math.sqrt(r),
                   "model_over_sqrt_r": math.gamma(k + .5) / math.gamma(k),
                   "ratio_to_k1": mean / base, "model_ratio": math.gamma(k + .5) / (math.gamma(k) * math.gamma(1.5))}
json.dump(out, open(sys.argv[2] if len(sys.argv) > 2 else "/dev/stdout", "w"), indent=1)
