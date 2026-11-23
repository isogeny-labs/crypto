# EXP12c: balanced cost model for n = m = 4 index calculus on a line factor base, case-P vs random line.
#
# Factor base B = S x-values inside ONE 2-descent class c* of the line (4 points of one class always sum
# into 2E, so this is the best subset for either line type). Targets are taken in 2E.
# Per decomposition trial, expected relation counts (exact multiset counts, #E ~ q^4, 8 sums per multiset):
#   full : 64*C(S,4)/q^4
#   1-LP : 64*C(S,3)*(L*-S)/q^4            (the large prime must lie in class c*)
#   2-LP : 64*C(S,2)*sum_c C(pool_c,2)/q^4  (two large primes of equal class; pool_c* = L*-S)
# Relations are combined through the large-prime graph (vertex 0 = "1" for 1-LP edges); usable relations
# = full + cycle rank (E - V + components), computed exactly with union-find on a simulated graph.
# Cost = N_trials * T + LA(S), LA(S) = 2S * S * (w*t_add + 3*t_mul) (Lanczos/Wiedemann, weight w).
# Also the unrestricted "whole line" strategy of Exp 11i: S = whole line, yield 64*N0/q^4, no large primes.
import sys, math, numpy as np
from scipy.sparse import coo_matrix
from scipy.sparse.csgraph import connected_components

rng = np.random.default_rng(12)
KC = 64.0  # 8 sums per multiset / #(2E up to sign); 64 for odd q with E/2E = (Z/2)^2, 32 in char 2 (E/2E = Z/2)
LINES = None  # set by --sweep2
TDM = 1.0     # per-trial cost multiplier for line "D" (Exp 14), optional 5th arg of --sweep2

def C(n, k): return math.comb(int(n), k) if n >= k else 0

def la_cost(S, q, w=8):
    limbs = max(1, math.ceil(4 * math.log2(q) / 64))
    t_add, t_mul = {1: (12e-9, 20e-9), 2: (3.3e-9, 65e-9), 3: (5e-9, 100e-9), 4: (6e-9, 144e-9)}[min(limbs, 4)]
    return 2.0 * S * S * (w * t_add + 3 * t_mul)

def N0(n):  # 4-multisets from class counts n[0..3] (class index = bit pattern) whose classes XOR to 0
    s = 0
    for k in [(a, b, c, d) for a in range(5) for b in range(5) for c in range(5) for d in range(5) if a + b + c + d == 4]:
        x = 0
        for ci in range(4):
            if k[ci] % 2: x ^= ci
        if x == 0: s += math.prod((math.comb(n[ci] + k[ci] - 1, k[ci]) if k[ci] else 1) for ci in range(4))
    return s

def cycle_rank(nv, eu, ev):
    if len(eu) == 0: return 0
    g = coo_matrix((np.ones(len(eu), dtype=np.int8), (eu, ev)), shape=(nv, nv))
    ncomp, _ = connected_components(g, directed=False)
    return len(eu) - nv + ncomp

def relations_needed_trials(q, pools, S, lp, T):
    """Smallest N (trials) with full + cycle rank >= S, by simulation; returns N."""
    cs = int(np.argmax(pools)); Lc = pools[cs]
    pool = list(pools); pool[cs] = Lc - S
    f = KC * C(S, 4) / q**4
    g = KC * C(S, 3) * pool[cs] / q**4 if lp >= 1 else 0.0
    pairw = np.array([C(p, 2) for p in pool], dtype=float)
    h = KC * C(S, 2) * pairw.sum() / q**4 if lp >= 2 else 0.0
    if f == 0: return math.inf
    N_hi = S / f  # no-LP bound
    if lp == 0: return N_hi
    # simulate edges for N_top trials (start where the large-prime graph has ~3 edges per vertex, double if
    # that is not enough), arrival order random; then binary search on the prefix
    off = np.concatenate([[1], 1 + np.cumsum(pool)[:-1]]).astype(np.int64)  # vertex offsets per class
    nv = 1 + int(sum(pool))
    N_top = min(N_hi, 3.0 * nv / (g + h))
    while True:
        if (g + h) * N_top > 3e7: return None
        n1 = rng.poisson(g * N_top); n2 = rng.poisson(h * N_top)
        t1 = rng.random(n1); u1 = np.zeros(n1, dtype=np.int64); v1 = off[cs] + rng.integers(0, pool[cs], n1)
        cls = rng.choice(4, size=n2, p=pairw / pairw.sum()) if n2 else np.zeros(0, dtype=np.int64)
        a = rng.random(n2); b = rng.random(n2)
        sz = np.array(pool, dtype=np.int64)[cls]
        u2 = off[cls] + (a * sz).astype(np.int64)
        v2 = off[cls] + ((u2 - off[cls]) + 1 + (b * (sz - 1)).astype(np.int64)) % sz
        t2 = rng.random(n2)
        t = np.concatenate([t1, t2]); o = np.argsort(t)
        U = np.concatenate([u1, u2])[o]; V = np.concatenate([v1, v2])[o]; t = t[o]
        if f * N_top + cycle_rank(nv, U, V) >= S or N_top >= N_hi: break
        N_top = min(N_hi, 2 * N_top)
    lo, hi = 0.0, 1.0
    for _ in range(11):
        mid = (lo + hi) / 2; k = np.searchsorted(t, mid)
        got = f * N_top * mid + cycle_rank(nv, U[:k], V[:k])
        if got >= S: hi = mid
        else: lo = mid
    return hi * N_top

def best(q, pools, T, lp, nS=18):
    Lc = max(pools); out = (math.inf, None, None)
    for j in range(nS):
        S = int(Lc * 0.75 ** j) if j else Lc - 1
        if S < 8: break
        N = relations_needed_trials(q, pools, S, lp, T)
        if N is None: continue
        c = N * T + la_cost(S, q)
        if c < out[0]: out = (c, S, N)
    return out

def wholeline(q, pools, T):
    S = sum(pools); y = KC * N0(pools) / q**4
    N = S / y
    return (N * T + la_cost(S, q), S, N)

if __name__ == "__main__" and sys.argv[1:2] not in (["--sweep"], ["--sweep2"]):
    T = float(sys.argv[1]) if len(sys.argv) > 1 else 6.9
    qs = [int(x) for x in sys.argv[2].split(",")] if len(sys.argv) > 2 else [2**10, 2**14, 2**18, 2**22]
    print(f"T = {T} s per trial; LA = 2 S^2 (8 t_add + 3 t_mul), measured GMP mpn unit costs")
    for q in qs:
        lines = {"P": [q // 2, q // 2, 0, 0], "R": [q // 8] * 4}
        res = {}
        for name, pools in lines.items():
            r = {"whole": wholeline(q, pools, T)}
            for lp in (0, 1, 2): r[f"{lp}LP"] = best(q, pools, T, lp)
            res[name] = r
            print(f"q=2^{math.log2(q):.1f} {name}: " + " | ".join(
                f"{k}: cost {v[0]:.3g}s S={v[1]} trials {v[2]:.3g} (LA share {la_cost(v[1], q)/v[0]:.2f})" if v[1] else f"{k}: -"
                for k, v in r.items()))
        bP = min(res["P"].values(), key=lambda v: v[0]); bR = min(res["R"].values(), key=lambda v: v[0])
        kP = min(res["P"], key=lambda k: res["P"][k][0]); kR = min(res["R"], key=lambda k: res["R"][k][0])
        print(f"   BEST q=2^{math.log2(q):.1f}: P {kP} {bP[0]:.3g}s | R {kR} {bR[0]:.3g}s | gain R/P {bR[0]/bP[0]:.2f}"
              + "  | same-strategy gains: " + ", ".join(f"{k} {res['R'][k][0]/res['P'][k][0]:.2f}" for k in res['P']), flush=True)

# ---- Scaled sweep (python3 exp12c_balanced.py --sweep T q_sim q_real1,q_real2,...) ----
# All per-trial rates depend on sigma = S/q only, graph sizes and the relation target scale with q, so
# N = q*n(sigma) and cost = q*(T*n(sigma) + kappa*q*sigma^2). A run at q_sim with T_eff = T*q_sim/q_real
# has the same optimal sigma as the real q, and cost_real = cost_sim*(q_real/q_sim)^2.
def sweep(T, q_sim, q_reals):
    for qr in q_reals:
        Te = T * q_sim / qr; out = {}
        kappa = la_cost(1, qr)  # LA cost per S^2 at the real limb size
        for name, pools in (LINES(q_sim) if LINES else {"P": [q_sim // 2, q_sim // 2, 0, 0], "Q": [q_sim // 2, 0, 0, 0], "R": [q_sim // 8] * 4}).items():
            Lc = max(pools); cand = []
            Te = T * q_sim / qr * (TDM if name == "D" else 1.0)
            S = sum(pools); y = KC * N0(pools) / q_sim**4
            cand.append(("whole", S / y * Te + kappa * S * S, S))
            for j in range(0, 60):
                S = int(Lc * 0.7 ** j) if j else Lc - 1
                if S < 64: break
                f = KC * C(S, 4) / q_sim**4
                cand.append(("0LP", S / f * Te + kappa * S * S, S))
                N = relations_needed_trials(q_sim, pools, S, 2, Te)
                if N is not None: cand.append(("2LP", N * Te + kappa * S * S, S))
            b = min(cand, key=lambda c: c[1]); out[name] = b
            b2 = min((c for c in cand if c[0] == "2LP"), key=lambda c: c[1], default=None)
            sc = (qr / q_sim) ** 2
            print(f"q_real=2^{math.log2(qr):.0f} {name}: best {b[0]} cost {b[1]*sc:.3g}s sigma=S/q {b[2]/q_sim:.2e} "
                  f"(LA share {kappa*b[2]**2/b[1]:.2f}); best 2LP sigma {b2[2]/q_sim if b2 else float('nan'):.2e}"
                  + (" [edge of S grid]" if b[0] != "whole" and b[2] == min(c[2] for c in cand) else ""), flush=True)
        ref = "D" if "D" in out else "P"
        print(f"   GAIN q_real=2^{math.log2(qr):.0f}: " + "  ".join(f"{k}/{ref} = {out[k][1]/out[ref][1]:.2f}" for k in out if k != ref), flush=True)

if __name__ == "__main__" and sys.argv[1:2] == ["--sweep"]:
    sweep(float(sys.argv[2]), int(sys.argv[3]), [int(eval(x)) for x in sys.argv[4].split(",")])

# char 2 (Exp 13): D = lift-dense (one class of Q), Q = pinned (one class of Q/2), R = random (two classes of Q/4)
if __name__ == "__main__" and sys.argv[1:2] == ["--sweep2"]:
    KC = 32.0
    TDM = float(sys.argv[5]) if len(sys.argv) > 5 else 1.0
    LINES = lambda q: {"D": [q, 0, 0, 0], "Q": [q // 2, 0, 0, 0], "R": [q // 4, q // 4, 0, 0]}
    sweep(float(sys.argv[2]), int(sys.argv[3]), [int(eval(x)) for x in sys.argv[4].split(",")])
