# Exact Semaev-only decomposition harness (experimental)

Requires Python >=3.10 and sympy, no factor-base map. From repository root:

    PYTHONPATH=. python -m summation_split.harness --samples 50 --timeout 4 --out summation_split
    PYTHONPATH=. python -m unittest -q summation_split.test_split

The harness refuses fewer than 50 random instances per parameter. It completes the full exactness gate before any GB timing, comparing (1) fixed-degree Sylvester modular resultant, (2) projective Fp² root propagation, and (3) an independent E(Fp²) group-law sign oracle. core.py interpolates specialized higher summation polynomials and requires p > final X-degree. chain.py provides the original S3 equations; projective.py handles infinity charts; solver.py sends all charts through SymPy Buchberger lex, with explicit Fp membership polynomials; symmetric.py gives elementary-symmetric S4 with quadratic ties; linear.py supplies a negative linear-coordinate control.

bench.csv contains all measured, censored, and unimplemented cases and leaves d_F4/first-fall fields blank. gate.csv holds all six exactness cells. solving_degree_vs_m.svg **is only a generator-degree chart, not a measured F4-degree plot**. FINDINGS.md gives counterexamples and negative timing. All x_i are specialized fixed constants, leaving one X variable; this qualifies correctness controls, **not** the unspecialized positive-dimensional Eq. (4) problem.

All evidence producer-owned, [pcc:unverified]; no claimed degree-bound theorem or speedup.
