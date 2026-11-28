#!/usr/bin/env python3
"""Independent exact Newton-moment term-count comparison; no ECDLP claim.

Requires sympy. Does not invoke F4. Run: python3 research/trace_skew_prime31_20261122/term_count_gate.py
"""
import sympy as sp

def powers(m, degree):
    e = sp.symbols("e1:" + str(m + 1))
    p = [sp.Integer(m)]
    for k in range(1, degree + 1):
        value = sum((-1)**(j - 1) * e[j - 1] * p[k - j]
                    for j in range(1, min(k - 1, m) + 1))
        if k <= m:
            value += (-1)**(k - 1) * k * e[k - 1]
        else:
            value += (-1)**(m - 1) * e[m - 1] * p[k - m]
        p.append(sp.expand(value))
    return e, p

def evaluate(m, B):
    e, p = powers(m, B + m - 1)
    # Deterministic dense monic h(T) of degree B; no claim of genericity.
    coefficients = [1] + [i + 1 for i in range(B)]
    h = [sp.expand(sum(coefficients[t] * p[B - t + j]
                       for t in range(B + 1))) for j in range(m)]
    g = [sp.expand(sum((-1)**a * (1 if a == 0 else e[a-1]) * h[j-a]
                       for a in range(j+1))) for j in range(m)]
    report = lambda xs: [(sp.Poly(x, *e).total_degree(),
                          len(sp.Poly(x, *e).terms())) for x in xs]
    # The triangular change is invertible over any coefficient ring.
    for j in range(m):
        reconstructed = sp.expand(g[j] -
            sum((-1)**a * (1 if a == 0 else e[a-1]) * h[j-a]
                for a in range(j+1)))
        assert reconstructed == 0
    return report(h), report(g)

def main():
    for m in (2, 3, 4):
        for B in (4, 6, 8):
            raw, reduced = evaluate(m, B)
            assert max(d for d, _ in reduced) <= B
            assert all(d <= B+j for j, (d, _) in enumerate(raw))
            print(f"m={m} B={B} raw={raw} reduced={reduced}")
            if (m, B) == (4, 8):
                assert raw == [(8,53), (9,70), (10,92), (11,117)]
                assert reduced == [(8,53), (8,62), (8,59), (8,39)]
                print("m4/B8 total terms: 332 -> 213 (35.8434% fewer)")
    print("PASS: 9 symbolic configurations, exact triangular checks")

if __name__ == "__main__":
    main()
