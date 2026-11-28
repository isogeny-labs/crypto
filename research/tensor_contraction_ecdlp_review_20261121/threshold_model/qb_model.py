#!/usr/bin/env python3
"""Quantile-balancing variant of the clique-sum recursion (lever A).

Standard level (Anthropic Lemma 8 case (ii) + Theorem A step), with the proved
re-reading of the overheads (README):
    e_{l+1} = (rho + 2 zeta + 1/g) e_l + 1/K_l,   s >= ln(2 s K^2)/(2 zeta^2)   (S1)

Quantile balancing (this file). Instead of padding every pair of merged groups to the
maximum load L_max = (U + 2 zeta g P)/K^2 guaranteed by Hoeffding + union bound, set the
clique edge length to L = mu + d*sigma, where mu = U/K^2 is the mean pair load under a
uniformly random balanced grouping and sigma a bound on its standard deviation, and
enumerate the excess bits (Y - L)^+ of the heavy pairs outside the clique call. Cost:
    exponent = e K^2 L + sum_pairs (Y - L)^+
             <= e U + sigma_rel U [ e d + (sqrt(1+d^2) - d)/2 ]            (two-point tail bound)
with sigma_rel U <= N (1/(2 sqrt s) + 1/s)  (first-stage Bhatia-Davis variance, worst
case rho' = 1/2; second stage <= 1/s). A grouping achieving the average exists and is found
by the same brute-force search over groupings the paper already uses. No (S1), no union bound.
    e_{l+1} = rho e_l + beta(e_l) (1/(2 sqrt s) + 1/s) + 1/K_l,   beta(e) = min_d [e d + (sqrt(1+d^2)-d)/2]
    log2 K_{l+1} = 2 (log2 K_l + log2 s_l),  s_l >= 1 free.
Top level (Prop. 3.9 with Lemma 8 case (ii)): rho'_top = U/(gP) with gP <= 2 kappa (1+eps) k m,
U = 2(km - n) = (6 - 2/Delta) m, so sigma_rel U <= U sqrt((1-rho')/(rho' s)) + U/s, and
    main term / U = e + beta(e) (sqrt((1-rho')/(rho' s)) + 1/s) + kappa (1+eps)/K,
    (T1) unchanged: log2 m* = 2 log2(sK) + log2(2 ln2 (log2(4k s^2 K^2) + 1)/eps).
"""
from __future__ import annotations
import math, sys
from dataclasses import dataclass, field

LN2 = math.log(2.0)
KAPPA = 1.0 / LN2


def beta(e: float) -> float:
    """min over d>=0 of e d + (sqrt(1+d^2) - d)/2.  Derivative: e + (d/sqrt(1+d^2) - 1)/2 = 0."""
    if e >= 0.5:
        return 0.5  # d = 0
    t = 1.0 - 2.0 * e            # d/sqrt(1+d^2) = t
    d = t / math.sqrt(1.0 - t * t)
    return e * d + (math.sqrt(1.0 + d * d) - d) / 2.0


@dataclass
class State:
    log2K: float
    e: float
    path: list = field(default_factory=list)


def thin(front, keep=250):
    """Keep at most `keep` Pareto points, evenly spaced in log2K (always keep the ends)."""
    if len(front) <= keep:
        return front
    step = (len(front) - 1) / (keep - 1)
    return [front[round(i * step)] for i in range(keep)]


def pareto(states):
    states = sorted(states, key=lambda t: (t.log2K, t.e))
    out, best = [], float("inf")
    for st in states:
        if st.e < best - 1e-15:
            out.append(st); best = st.e
    return out


def run_levels_qb(x0, e0, rho, bar_, oK=1.0, max_levels=12, sgrid=None):
    if sgrid is None:
        sgrid = [2 ** j for j in range(0, 40)]
    fronts = [[State(float(x0), e0, [])]]
    for _ in range(max_levels):
        nxt = []
        for st in fronts[-1]:
            if st.e < bar_:
                continue
            for s in sgrid:
                # Var Y <= 2PU/K^3 + PU/(gK^2) for a random equipartition; sigma_rel U = U sqrt(2/(rho' s) + 1/(rho' s^2)),
                # absolute slack <= beta(e) N sqrt(2 rho'/s + rho'/s^2) with rho' = U/(gP) <= rho + 1/g.
                slack = beta(st.e) * math.sqrt(2.0 * rho / s + rho / (s * s))
                ne = rho * st.e + slack + oK * 2.0 ** (-st.log2K)
                if ne >= st.e:
                    continue
                nl = 2.0 * (st.log2K + math.log2(s))
                nxt.append(State(nl, ne, st.path + [math.log2(s)]))
        nxt = thin(pareto(nxt), 250)
        if not nxt:
            break
        fronts.append(nxt)
    return fronts


def top_qb(fronts, bar_, k=3, density=1.0, nu=1e-3, eps_grid=None, sgrid=None):
    """Lemma C top with quantile balancing. Returns (log2 m*, lev, state, s_top, eps, q)."""
    if eps_grid is None:
        eps_grid = [2.0 ** (-j / 4.0) for j in range(0, 40)]
    if sgrid is None:
        sgrid = [2 ** j for j in range(0, 60)]
    best = None
    for lev, fr in enumerate(fronts):
        for st in fr:
            if st.e >= bar_:
                continue
            Kinv = 2.0 ** (-st.log2K)
            for eps in eps_grid:
                gP_over_U = 2 * KAPPA * (1 + eps) * k / (6.0 - 2.0 / density)
                rhop = 1.0 / gP_over_U
                for s in sgrid:
                    srel = math.sqrt(2.0 / (rhop * s) + 1.0 / (rhop * s * s))  # relative sd of pair loads, same covariance bound
                    val = st.e + beta(st.e) * srel + KAPPA * (1 + eps) * Kinv + nu * st.e
                    if val >= bar_:
                        continue
                    log2g = st.log2K + math.log2(s)
                    inner = 2 * LN2 * (math.log2(4 * k) + 2 * log2g + 1)
                    log2m = 2 * log2g + math.log2(inner) - math.log2(eps)
                    if best is None or log2m < best[0]:
                        best = (log2m, lev, st, s, eps, log2m / st.log2K)
                    break  # larger s only costs more once feasible
    return best


if __name__ == "__main__":
    sys.path.insert(0, __file__.rsplit("/", 1)[0] if "/" in __file__ else ".")
    from threshold_model import bar, run_levels, lemmaC_threshold
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--rho", type=float, default=1 / math.sqrt(2))
    ap.add_argument("--e0", type=float, default=0.2554)
    ap.add_argument("--density", type=float, default=1.0)
    a = ap.parse_args()
    b = bar(a.density, exact_U=True)
    print(f"bar = {b:.4f}  rho = {a.rho:.4f}  e0 = {a.e0}")
    # baseline: proved re-reading (oz=2, oK=1), Hoeffding balancing
    best_std = None
    for x0 in range(5, 15):
        fr = run_levels(x0, a.e0, a.rho, b, 2.0, 1.0)
        r = lemmaC_threshold(fr, b)
        if r and (best_std is None or r[0] < best_std[0]):
            best_std = (r[0], x0, r[1], r[2].log2K, r[2].e, r[6])
    print(f"Hoeffding balancing (oz=2,oK=1): log2 m* = {best_std[0]:.1f} x0={best_std[1]} levels={best_std[2]} log2K={best_std[3]:.1f} e_L={best_std[4]:.4f} q={best_std[5]:.2f}")
    best_qb = None
    for x0 in range(4, 15):
        fr = run_levels_qb(x0, a.e0, a.rho, b, 1.0)
        r = top_qb(fr, b, density=a.density)
        if r and (best_qb is None or r[0] < best_qb[0]):
            best_qb = (r[0], x0, r[1], r[2], r[3], r[4], r[5])
    val, x0, lev, st, s_top, eps, q = best_qb
    print(f"Quantile balancing:              log2 m* = {val:.1f} x0={x0} levels={lev} log2K={st.log2K:.1f} e_L={st.e:.4f} s_top=2^{math.log2(s_top):.0f} eps={eps:.2f} q={q:.2f}")
    lk, ee = float(x0), a.e0
    print(f"  l=0 log2K={lk:.1f} e={ee:.4f}")
    for i, ls in enumerate(st.path):
        s = 2 ** ls
        ee = a.rho * ee + beta(ee) * math.sqrt(2 * a.rho / s + a.rho / (s * s)) + 2 ** (-lk)
        lk = 2 * (lk + ls)
        print(f"  l={i+1} log2K={lk:.1f} e={ee:.4f}  log2s={ls:.0f}")
