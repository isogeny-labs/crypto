#!/usr/bin/env python3
"""Main-term threshold model for the Anthropic #k-SAT clique-sum recursion.

Reimplementation of the model in R. Majdoddin, "Lowered crossover points of
Anthropic's subexponential 3-SAT algorithm", arXiv:2610.41622, Section 3
(recursion (5), admissibility (S1)-(S4), Definition 3.1, Proposition 3.9).

One level:  K_{l+1} = (s_l K_l)^2,   e_{l+1} = (rho + oz*zeta_l) e_l + oK / K_l
            with oz = 3, oK = 3 in the paper's accounting.
Admissible:  (S1) 2 zeta^2 s >= ln(2 s K^2)   (S2) 1/(sK) <= zeta
             (S3) holds for x0 >= 4 with c = 30     (S4) e <= 1
Threshold:   crude top (q): log2 m* = q log2 K_L, first L with e_L < bar
             Lemma C top:  log2 m* = log2((4 + 2 ln2 g^2 (log2(4k g^2)+1))/eps),
                           g = sK at the top, largest zeta with
                           e(1+2 kappa zeta (1+eps)) + 2 kappa (1+eps)/K + nu e < bar.
Everything is in log2 K to allow log2 K in the tens of thousands.
"""
from __future__ import annotations
import math
from dataclasses import dataclass, field

LN2 = math.log(2.0)
KAPPA = 1.0 / LN2


def least_s(zeta: float, log2K: float) -> int:
    """Least integer s with 2 zeta^2 s >= ln(2 s K^2) and 1/(sK) <= zeta."""
    lnK2 = 2.0 * log2K * LN2
    s = 1
    for _ in range(200):
        need = (LN2 + math.log(s) + lnK2) / (2.0 * zeta * zeta)
        s_new = max(1, math.ceil(need))
        if s_new <= s:
            break
        s = s_new
    # (S2): 1/(sK) <= zeta  <=>  log2 s >= -log2 zeta - log2K
    s2 = 2.0 ** max(0.0, -math.log2(zeta) - log2K)
    return max(s, math.ceil(s2))


def bar(density: float, exact_U: bool = True, counting_only: bool = False) -> float:
    """Definition 3.1: bar per edge bit for k=3 at clause density Delta.
    exact_U: U = 2(km - n) = (6 - 2/Delta) n  instead of U < 2km = 6 Delta ... per m.
    counting_only: compare only against the counting bar 0.711 n (Kutzkov), not PSX 1.4423^m.
    """
    per_m = 0.711 / density  # 0.711 n per clause
    if not counting_only:
        per_m = min(0.5283, per_m)
    denom = (6.0 - 2.0 / density) if exact_U else 6.0
    return per_m / denom


@dataclass
class State:
    log2K: float
    e: float
    path: list = field(default_factory=list)  # (log2zeta, log2s)


def thin(front, keep=250):
    """Keep at most `keep` Pareto points, evenly spaced in log2K (always keep the ends)."""
    if len(front) <= keep:
        return front
    step = (len(front) - 1) / (keep - 1)
    return [front[round(i * step)] for i in range(keep)]


def pareto(states: list[State]) -> list[State]:
    states = sorted(states, key=lambda t: (t.log2K, t.e))
    out = []
    best_e = float("inf")
    for st in states:
        if st.e < best_e - 1e-15:
            out.append(st)
            best_e = st.e
    return out


def run_levels(x0: int, e0: float, rho: float, bar_: float, oz: float = 3.0, oK: float = 3.0,
               zgrid=None, max_levels: int = 14, keep: int = 4000):
    """Return list of per-level Pareto fronts; each state may be used as top."""
    if zgrid is None:
        zgrid = [2.0 ** (-j / 2.0) for j in range(2, 40)]
    fronts = [[State(float(x0), e0, [])]]
    for lev in range(max_levels):
        nxt = []
        for st in fronts[-1]:
            if st.e < bar_:
                continue  # already below bar; no need to recurse further
            for z in zgrid:
                s = least_s(z, st.log2K)
                nl = 2.0 * (st.log2K + math.log2(s))
                ne = (rho + oz * z) * st.e + oK * (2.0 ** (-st.log2K))
                if ne >= st.e:  # (S4)-style: require decrease
                    continue
                nxt.append(State(nl, ne, st.path + [(math.log2(z), math.log2(s))]))
        nxt = thin(pareto(nxt), min(keep, 250))
        if not nxt:
            break
        fronts.append(nxt)
    return fronts


def crude_threshold(fronts, bar_, q=9.0):
    best = None
    for lev, fr in enumerate(fronts):
        for st in fr:
            if st.e < bar_:
                val = q * st.log2K
                if best is None or val < best[0]:
                    best = (val, lev, st)
    return best


def lemmaC_threshold(fronts, bar_, k=3, nu=1e-3, eps_grid=None, zgrid_top=None):
    """Proposition 3.9 top level. Returns (log2 m*, level, state, zeta, eps, log2 s, q)."""
    if eps_grid is None:
        eps_grid = [2.0 ** (-j / 4.0) for j in range(0, 40)]
    if zgrid_top is None:
        zgrid_top = [2.0 ** (-j / 10.0) for j in range(10, 300)]
    best = None
    for lev, fr in enumerate(fronts):
        for st in fr:
            if st.e >= bar_:
                continue
            K_inv = 2.0 ** (-st.log2K)
            for eps in eps_grid:
                # largest zeta with e(1+2 kappa zeta (1+eps)) + 2 kappa (1+eps)/K + nu e < bar
                zeta_ok = None
                for z in zgrid_top:  # descending
                    val = st.e * (1 + 2 * KAPPA * z * (1 + eps)) + 2 * KAPPA * (1 + eps) * K_inv + nu * st.e
                    if val < bar_:
                        zeta_ok = z
                        break
                if zeta_ok is None:
                    continue
                s = least_s(zeta_ok, st.log2K)
                log2g = st.log2K + math.log2(s)
                # (T1): m >= (4 + 2 ln2 g^2 (log2(4k g^2) + 1)) / eps
                inner = 2 * LN2 * (math.log2(4 * k) + 2 * log2g + 1)  # times g^2
                log2m = 2 * log2g + math.log2(inner) - math.log2(eps)  # the +4 is negligible
                if best is None or log2m < best[0]:
                    best = (log2m, lev, st, zeta_ok, eps, math.log2(s), log2m / st.log2K)
    return best


def paper_schedule(bar_, x0=18, q=9.0, rho=1 / math.sqrt(2), e0=0.5, oz=3.0, oK=3.0, max_levels=20):
    """Column (P): s_l = 2^(3l+x0), zeta_l = 2^(-l-5)."""
    log2K, e = float(x0), e0
    for lev in range(max_levels):
        if e < bar_:
            return q * log2K, lev, log2K, e
        z = 2.0 ** (-lev - 5)
        log2s = 3 * lev + x0
        e = (rho + oz * z) * e + oK * 2.0 ** (-log2K)
        log2K = 2 * (log2K + log2s)
    return None


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--density", type=float, default=1.0)
    ap.add_argument("--exactU", action="store_true")
    ap.add_argument("--counting-only", action="store_true")
    ap.add_argument("--x0", type=int, default=7)
    ap.add_argument("--e0", type=float, default=0.5)
    ap.add_argument("--rho", type=float, default=1 / math.sqrt(2))
    ap.add_argument("--oz", type=float, default=3.0)
    ap.add_argument("--oK", type=float, default=3.0)
    ap.add_argument("--q", type=float, default=9.0)
    ap.add_argument("--top", choices=["crude", "lemmaC"], default="crude")
    ap.add_argument("--paper", action="store_true", help="column (P) schedule")
    ap.add_argument("--fine", action="store_true", help="finer zeta grid 2^(-j/8)")
    a = ap.parse_args()
    b = bar(a.density, a.exactU, a.counting_only)
    print(f"bar = {b:.4f}")
    if a.paper:
        print("paper schedule:", paper_schedule(b, x0=a.x0, q=a.q, rho=a.rho, e0=a.e0, oz=a.oz, oK=a.oK))
        raise SystemExit
    zg = [2.0 ** (-j / 8.0) for j in range(8, 140)] if a.fine else None
    fronts = run_levels(a.x0, a.e0, a.rho, b, a.oz, a.oK, zgrid=zg)
    if a.top == "crude":
        val, lev, st = crude_threshold(fronts, b, a.q)
        print(f"crude q={a.q}: log2 m* = {val:.1f}  levels={lev}  log2K_L={st.log2K:.1f} e_L={st.e:.4f}")
    else:
        val, lev, st, z, eps, l2s, qeff = lemmaC_threshold(fronts, b)
        print(f"lemmaC: log2 m* = {val:.1f} levels={lev} log2K_L={st.log2K:.1f} e_L={st.e:.4f} "
              f"zeta=2^{math.log2(z):.1f} eps={eps:.3f} log2s={l2s:.1f} q_eff={qeff:.3f}")
    # print the schedule
    lk, ee = float(a.x0), a.e0
    print(f"  l=0 log2K={lk:.1f} e={ee:.4f}")
    for i, (lz, ls) in enumerate(st.path):
        ee = (a.rho + a.oz * 2 ** lz) * ee + a.oK * 2 ** (-lk)
        lk = 2 * (lk + ls)
        print(f"  l={i+1} log2K={lk:.1f} e={ee:.4f}  log2zeta={lz:.1f} log2s={ls:.1f}")
