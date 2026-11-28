# EXP12d: check the simulator's cycle-rank count against the random-graph formula for G(n, M=d n/2):
# cycle rank / n -> d/2 - 1 + kappa(d), kappa(d) = (t - t^2/2)/d, t <= 1 solving t e^-t = d e^-d.
import math, numpy as np, sys
sys.argv = ["x", "--none"]
from exp12c_balanced import cycle_rank
rng = np.random.default_rng(5)
def kappa(d):
    if d <= 1: return 1 - d / 2
    lo, hi, target = 0.0, 1.0, d * math.exp(-d)
    for _ in range(100):
        m = (lo + hi) / 2
        if m * math.exp(-m) < target: lo = m
        else: hi = m
    return (lo - lo * lo / 2) / d
n = 2_000_000
for d in (0.8, 1.0, 1.1, 1.3, 1.6, 2.0, 3.0):
    M = int(d * n / 2); u = rng.integers(0, n, M); v = rng.integers(0, n, M)
    print(f"d={d}: simulated cycle rank/n {cycle_rank(n, u, v)/n:.5f}  formula {d/2 - 1 + kappa(d):.5f}")
