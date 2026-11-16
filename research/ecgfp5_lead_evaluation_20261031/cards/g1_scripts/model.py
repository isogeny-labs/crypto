#!/usr/bin/env python3
"""G1 desk model: cost of the Gaudry/Diem/Joux-Vitse family on E(F_{p^5}), p = 2^64-2^32+1.
Analysis only.  All quantities are carried as log2 of 64-bit word operations unless stated.

Sources for formulas (frozen texts, line numbers in paper_fulltext.md):
  JV 2010/157 l.324-353 (full base: n! c(n,q) q/2), l.361 (LA ~ n q^2), l.364-370 (large primes,
  q^{2-2/n} relations, n! c q^{2-2/n}), l.396-418 ((n-1)-variant: (n-1)! c~ q^2/2), l.534-575 (hybrid chart).
  Pornin 2022/274 l.138-149 (p^{2-2/5} = 2^102.4 'systems', D^2 = 2^40 -> 2^142), l.88 (10M group add),
  l.244-266 (F_{p^5} mul = 49 cycles = 29 mul + 20 add), l.370-376 (inversion 128 cycles).
"""
import math

lg = math.log2


def lg_add(a, b):
    """log2(2^a + 2^b)"""
    if a < b:
        a, b = b, a
    return a + lg(1 + 2.0 ** (b - a))


# ---------------------------------------------------------------- parameters (central values)
P = 2 ** 64 - 2 ** 32 + 1
LGP = lg(P)                      # 63.99999999...
LGL = LGP - 1                    # L = #x in F_p carrying a point of E, up to sign ~ p/2
LGN = 319.0                      # prime order n of EcGFp5 (Pornin), 319 bits
LG_RHO_OPS = 0.5 * (lg(math.pi / 4) + LGN)   # sqrt(pi n /4) group ops (negation map)

# unit prices in 64-bit word operations (assumed; sensitivity in sensitivity())
U_FP = 4.0          # one F_p multiply-accumulate in an F4/FGLM inner loop (mul, add, adc, amortised reduce)
U_ADD_N = 12.0      # add/sub mod the 319-bit n (5 limbs + conditional subtract)
U_MUL_N = 200.0     # mul-mod n (5x5 limbs + Montgomery reduction)
M_FP5 = 29.0 * 9 + 20 * 3   # one F_{p^5} mult done naively (29 mul at 9 words each incl. reduction, 20 add at 3) = 321
GROUP_OP_M = 5.7    # affine add with Montgomery batch inversion: 5M + 1S (S = 34/49 M), derived
GROUP_OP_WORDS = GROUP_OP_M * 140.0   # ~800 words with lazy reduction (140 words per M); naive 1830


def rho_words(group_op_words=GROUP_OP_WORDS):
    return LG_RHO_OPS + lg(group_op_words)


def kappa(w, a=U_ADD_N, m=U_MUL_N):
    """word ops per B^2 for Wiedemann on a B x B system with row weight w, +-1 coefficients,
    normal-equations form (2 matvecs per iteration):
      sequence : 2B iterations x 2wB adds        = 4 w B^2 adds
      recovery : B iterations  x 2wB adds        = 2 w B^2 adds
      Berlekamp-Massey 2B^2 mulmods, recovery scalar*vector B^2 mulmods = 3 B^2 mulmods"""
    return 6 * w * a + 3 * m


def la_mem_bytes(B, w):
    """matrix: w indices of 8 bytes per row; vectors mod n: 40 bytes each, 4 live vectors + BM sequence 2B*40"""
    return B * (8 * w + 4 * 40 + 80)


# ---------------------------------------------------------------- generic helpers
def minimise(fun, lo, hi, step=0.02):
    best = (1e9, None)
    x = lo
    while x <= hi + 1e-9:
        v = fun(x)
        if v < best[0]:
            best = (v, x)
        x += step
    # refine
    x0 = best[1]
    x = x0 - step
    while x <= x0 + step + 1e-9:
        v = fun(x)
        if v < best[0]:
            best = (v, x)
        x += step / 50
    return best


# ---------------------------------------------------------------- the five members
def full_base(lgc, u=U_FP, w=5):
    """Gaudry/Diem, full base B = L: relation search 120 c L, LA kappa L^2.  JV l.353, l.361."""
    cw = lgc + lg(u)
    R = lg(120) + cw + LGL
    LA = lg(kappa(w)) + 2 * LGL
    return lg_add(R, LA), R, LA


def subset_nolp(lgc, u=U_FP, w=5, cap_lgB=None):
    """Gaudry/Diem, factor-base subset of B <= L x-values (no large primes).
    attempts per relation = 120 / (B/L)^5, B relations needed:  R = 120 c L^5 / B^4,  LA = kappa B^2."""
    cw = lgc + lg(u)
    k = lg(kappa(w))

    def f(lgB):
        R = lg(120) + cw + 5 * LGL - 4 * lgB
        LA = k + 2 * lgB
        return lg_add(R, LA)

    hi = LGL if cap_lgB is None else min(LGL, cap_lgB)
    v, lgB = minimise(f, 20, hi)
    R = lg(120) + cw + 5 * LGL - 4 * lgB
    LA = k + 2 * lgB
    return v, lgB, R, LA


def cycle_rank_per_node(d):
    """cycle-space rank / L of G(L, M = dL/2): E_giant - V_giant, giant fraction S = 1 - exp(-d S)."""
    if d <= 1:
        return 0.0
    S = 0.5
    for _ in range(200):
        S = 1 - math.exp(-d * S)
    return (d / 2) * S * (2 - S) - S


def theta_for(beta):
    """smallest d with cycle_rank(d) * L >= B  <=>  r(d) >= beta; returns d/2 = edges per node"""
    lo, hi = 1.0 + 1e-9, 60.0
    for _ in range(200):
        mid = (lo + hi) / 2
        if cycle_rank_per_node(mid) >= beta:
            hi = mid
        else:
            lo = mid
    return hi / 2, hi


def lp2(lgc, u=U_FP, w_lp=150.0, lgLp=None, lgB_cap=None):
    """Gaudry/Diem + double large primes.  Large-prime set = L' = 2^lgLp points (<= L), small base B.
    A decomposition into 5 points of the L'-set has prob (L'/L)^5/120 per attempt; it is a usable
    2-LP edge iff >= 3 of its 5 points are small: P2 = C(5,2) b^3 (1-b)^2, b = B/L'.
    Need edges = theta(b) L' with cycle rank >= B.  Row weight after cycle elimination w_lp."""
    cw = lgc + lg(u)
    k = lg(kappa(w_lp))
    lgLp = LGL if lgLp is None else lgLp

    def parts(lgB):
        b = 2.0 ** (lgB - lgLp)
        if b >= 0.5:
            return None
        th, d = theta_for(b)
        P2 = 10 * b ** 3 * (1 - b) ** 2
        edges = th * 2.0 ** lgLp
        attempts = 120 * edges / P2 / (2.0 ** (lgLp - LGL)) ** 5
        R = lg(attempts) + cw
        LA = k + 2 * lgB
        return R, LA, lg(edges), d

    def f(lgB):
        pr = parts(lgB)
        if pr is None:
            return 1e9
        return lg_add(pr[0], pr[1])

    hi = lgLp - 1.0 if lgB_cap is None else min(lgLp - 1.0, lgB_cap)
    v, lgB = minimise(f, 20, hi)
    R, LA, lgE, d = parts(lgB)
    return v, lgB, R, LA, lgE, d


def lp1(lgc, u=U_FP, w_lp=8.0):
    """single large prime: usable pairs U(M1) = M1 - L(1-exp(-M1/L)) >= B."""
    cw = lgc + lg(u)
    k = lg(kappa(w_lp))

    def parts(lgB):
        b = 2.0 ** (lgB - LGL)
        if b >= 0.5:
            return None
        B = 2.0 ** lgB
        L = 2.0 ** LGL
        # solve M1 - L(1-exp(-M1/L)) = B ; x = M1/L -> x - 1 + exp(-x) = b
        lo, hi = 0.0, 5.0
        for _ in range(200):
            mid = (lo + hi) / 2
            if mid - 1 + math.exp(-mid) >= b:
                hi = mid
            else:
                lo = mid
        M1 = hi * L
        P1 = 5 * b ** 4 * (1 - b)
        attempts = 120 * M1 / P1
        return lg(attempts) + cw, k + 2 * lgB, lg(M1)

    def f(lgB):
        pr = parts(lgB)
        return 1e9 if pr is None else lg_add(pr[0], pr[1])

    v, lgB = minimise(f, 20, LGL - 1)
    R, LA, lgM = parts(lgB)
    return v, lgB, R, LA, lgM


def jv_nminus1(lgct, u=U_FP, B_lg=None):
    """JV (n-1)=4-point variant, factor-base subset B (default full L).  JV l.396-418, l.435.
    success per attempt (2B)^4/(24 p^5) ; B relations needed: R = 1.5 c~ p^5 / B^3."""
    cw = lgct + lg(u)
    lgB = LGL if B_lg is None else B_lg
    R = lg(1.5) + cw + 5 * LGP - 3 * lgB
    LA = lg(kappa(5)) + 2 * lgB
    return lg_add(R, LA), R, LA


def hybrid(lgch, u=U_FP):
    """Gaudry/Diem with 'guess one variable': n! q systems per relation (chart l.547-575); p/2 relations"""
    cw = lgch + lg(u)
    return lg(120) + LGP + lg(0.5) * 0 + cw + LGL  # 120 * p * c_h * (p/2)


if __name__ == "__main__":
    print("log2 p = %.6f   L = 2^%.3f   rho = 2^%.3f group ops" % (LGP, LGL, LG_RHO_OPS))
    print("rho in words (800/op) = 2^%.2f   (500/op: 2^%.2f, 1830/op: 2^%.2f)" %
          (rho_words(), rho_words(500), rho_words(1830)))
    print("group op = %.1f M = %.0f F_p mult + %.0f adds (Pornin 29 mul + 20 add per M)" %
          (GROUP_OP_M, GROUP_OP_M * 29, GROUP_OP_M * 20))
