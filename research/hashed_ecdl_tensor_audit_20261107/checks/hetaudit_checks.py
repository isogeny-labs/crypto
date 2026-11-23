#!/usr/bin/env python3
"""Small exact checks for the hashed-ECDL tensor audit (hetaudit, 2026-11-07).

Every check is a few seconds of integer arithmetic.  Nothing here is a
measurement; the outputs are derivations from the stated theorem parameters.
Run:  python3 -I checks/hetaudit_checks.py
"""
import math
from fractions import Fraction


def theorem3_params(d):
    """Bochum v0.2 Theorem 3 / Lemma 5 parameter choice for d-bit inputs."""
    J = 16
    while J * J < 64 * (d + 1):
        J *= 2
    k = J // 4
    n = -(-d // k)
    L = 1
    while L < 2 * n:
        L *= 2
    mu = L.bit_length() - 1
    L1 = 1 << (mu // 2)
    L2 = 1 << (mu - mu // 2)
    return J, k, n, L, L1, L2


def lemma5_checks(dmax):
    bad = []
    for d in range(1, dmax + 1):
        J, k, n, L, L1, L2 = theorem3_params(d)
        ok = (J >= 16 and J * J < 256 * (d + 1)
              and 2 * n <= L <= J // 2 and (2 * J) % L == 0
              and max(L1, L2) ** 2 <= J
              and n * 2 ** (2 * k) < 2 ** J)          # part 4: c_l < 2^J
        if not ok:
            bad.append(d)
    return bad


def principal_root_check(J, L):
    """omega = 2^(2J/L) is a principal L-th root of unity in Z/(2^J+1)."""
    A = 2 ** J + 1
    w = pow(2, 2 * J // L, A)
    if pow(w, L, A) != 1:
        return False
    for j in range(1, L):
        if sum(pow(w, j * i, A) for i in range(L)) % A != 0:
            return False
    return True


def as_built_mult_edges(d):
    """Edges of the Theorem 3 network exactly as constructed (Section 2.3):
    4d interface bits + words: 4L (forward) + L (products) + L (inverse stage 1)
    + (2n-1) coefficients c_l + (2n-1) carries kappa_1..kappa_{2n-1}, each J+1 bits."""
    J, k, n, L, L1, L2 = theorem3_params(d)
    words = 4 * L + L + L + (2 * n - 1) + (2 * n - 1)
    return 4 * d + words * (J + 1)


def lemma7_bound(d):
    J, k, n, L, L1, L2 = theorem3_params(d)
    return 4 * d + 8 * L * (J + 1), 2 * max(L1, L2) * (J + 1)


def main():
    print("== Lemma 5 inequalities, d = 1..5000:", "all hold" if not lemma5_checks(5000) else lemma5_checks(5000))
    for J, L in [(16, 8), (32, 16), (64, 32), (128, 16), (128, 64)]:
        print(f"   principal root check J={J} L={L}: {principal_root_check(J, L)}")

    print("\n== Theorem 3 parameters and sizes")
    for d in (131, 132, 160, 256, 521):
        J, k, n, L, L1, L2 = theorem3_params(d)
        e_built = as_built_mult_edges(d)
        e_bound, deg = lemma7_bound(d)
        print(f"   d={d}: J={J} (ring {J+1} bits) k={k} n={n} L={L} L1={L1} L2={L2}; "
              f"transform-vertex degree 2*max(L1,L2)*(J+1)={deg} (<136(d+1)^0.75={136*(d+1)**0.75:.0f}); "
              f"edges as built={e_built}, Lemma 7 bound={e_bound} (<1100(d+1)={1100*(d+1)})")
    print("   ECCp-131 receipt: ring_bits 129, transform_length 16, max degree 1032 -> matches d=131 row above.")

    print("\n== Multiplications per Corollary 2 point addition (paper's own constraint list)")
    # validity of P1, P2: x.x, u1.x, ca.x, y.y  -> 4 modmul each (paper: Corollary 1 example)
    # zero tests zeta_x, zeta_y: a*zeta, a*e, zeta*e -> 3 modmul each
    # slopes: lambda(x2-x1), lambda*2y1, x1*x1 (for 3x1^2), lambda^2, lambda(x1-x3) -> 5 modmul
    # validity of P3: 4 modmul (needed so that the representation of P3 is canonical downstream)
    modmul = 4 + 4 + 3 + 3 + 5 + 4
    intmul = 2 * modmul  # Lemma 8: xy = theta*q + z needs two integer products
    print(f"   modular multiplications per addition >= {modmul}; integer multiplications >= {intmul}")
    for b in (131, 256):
        per_add = intmul * as_built_mult_edges(b)
        print(f"   b={b}: >= {per_add} edges per point addition from multiplications alone "
              f"(= {per_add/b:.0f} edges per bit); hash slot adds 2 x {as_built_mult_edges(b+1)} edges")
    print("   ECCp-131 m=2 implementation: 1,564,506 edges / (2*131) = "
          f"{1564506/(2*131):.0f} edges per slot-bit; receipt lists 34 x 131-bit + 4 x 132-bit integer products.")
    print(f"   Lemma 7 upper bound for 38 products at d<=132: {38*1100*133} edges (>= 1,564,506: consistent).")

    print("\n== D.4 admission thresholds (OpenAI preprint, quantitative-contraction.tex eqs. parameters/bounds)")
    eps_log = 17  # epsilon = 2^-17
    # u_0 >= 1  <=> lambda >= 2^17
    # u_0 >= lambda^(4/5) <=> lambda^(1/5) >= 2^17 <=> lambda >= 2^85
    print("   u_0 = floor(lambda/2^17) >= 1            <=> lambda >= 2^17   (Bochum 'size 2^(2^17)')")
    print("   u_0 >= lambda^(4/5) at depth 0           <=> lambda >= 2^85   (clustering note item 1)")
    # full depth d_max < log2(lambda)/6 (from M <= s 8^-d and M > H = s/sqrt(lambda), C=1):
    # u_d >= eps*lambda/2^d >= eps*lambda^(5/6); need >= lambda^(4/5): lambda^(1/30) >= 2^17 <=> lambda >= 2^510
    print("   u_d >= lambda^(4/5) at the deepest active call d ~ log2(lambda)/6  <=> lambda >= 2^510 (C=1)")
    lam = 2 ** 85
    print(f"   check: lambda=2^85: u0={lam // 2**17}, lambda^0.8={2**68}: equal -> boundary correct")
    lam = 2 ** 510
    d = int(math.floor(math.log2(lam) / 6))  # = 85
    u = lam // 2 ** 17 // 2 ** d
    print(f"   check: lambda=2^510: d_max={d}, u_d=2^{u.bit_length()-1}, lambda^0.8=2^{int(0.8*510)} -> equal: boundary correct")
    print("   L09 margin formula: 0.2*log2(lambda) - 17 at lambda=10.21 ->", round(0.2 * math.log2(10.21) - 17, 2),
          "; at lambda=21.96 ->", round(0.2 * math.log2(21.96) - 17, 2), "(table2.json: -16.33 .. -16.11)")
    print("   ECCp-131 graph: s=1,564,506 -> lambda=21, u_0=0, exact leaf 2^s; the receipt agrees (root_u 0).")

    print("\n== Lemma 10 constant")
    print("   (98/99)/(1+e^0.1) =", round((98 / 99) / (1 + math.exp(0.1)), 4), "> 2/5")

    print("\n== Exponent comparison (bits), taking D.1 as the formula c*s/sqrt(log2 s) with c=1 at every size")
    print("   s = a*m*b edges; a=1 is the impossible floor (interface only), a=2600 is the paper's own gadget floor,")
    print("   a=6000 is the implementation at b=131.  rho = b/2.  Naive enumeration per attempt = (m-1)b/m.")
    for b in (128, 192, 256):
        for a in (1, 2600, 6000):
            best = None
            for m in range(2, 9):
                s = a * m * b
                e_solver = s / math.sqrt(math.log2(s))
                e_calls = math.log2(m * m * b) + b / m
                e_la = 3 * b / m + 2 * math.log2(b) + 3 * math.log2(m)
                e_total = max(e_calls + e_solver, e_la)
                if best is None or e_total < best[1]:
                    best = (m, e_total, e_solver, e_calls, e_la)
            m, e_total, e_solver, e_calls, e_la = best
            print(f"   b={b:3d} a={a:5d}: best m={m}: solver/attempt {e_solver:.3e} bits, calls {e_calls:.1f}, "
                  f"lin.alg {e_la:.1f}; total {e_total:.3e} bits vs rho {b/2:.0f}, enumeration/attempt {(m-1)*b/m:.1f}")
    print("   Real cost below admission (u_0 = 0, direct summation): 2^s with s = 1,564,506 at ECCp-131.")

    print("\n== Crossover conditions (c=1)")
    print("   Bochum below rho (blackbox balance E_min = 2b sqrt(a c)/(log2 b)^(1/4) < b/2): log2 b > 256 (a c)^2")
    for a in (1, 2600, 6000):
        print(f"     a={a}: log2 b > {256*a*a:.3e}  (b > 2^{256*a*a:.3e} bits)")
    print("   Solver beats naive enumeration per attempt at fixed m: sqrt(log2 s) > a c m^2/(m-1)")
    for a in (1, 2600):
        for m in (2, 3, 4):
            t = (a * m * m / (m - 1)) ** 2
            print(f"     a={a} m={m}: log2 s > {t:.3e}")
    print("   LEAD-ICB form of the obstruction (N = omega(l sqrt(log l)) variables): s = a m^2 l is LINEAR in l,")
    print("   so the literal obstruction does NOT apply; the closure rests on admission and on the constants above.")


if __name__ == "__main__":
    main()
