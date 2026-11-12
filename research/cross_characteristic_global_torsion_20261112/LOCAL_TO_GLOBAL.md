# CT-X — Local torsion lifting is automatic; the global bridge remains open

**Scope:** Mathematical reduction and a necessary residue-degree bound, not a scalable cross-characteristic DLP transfer. Scientific status: [pcc:unverified].

## The local theorem

For an abelian scheme A over a complete henselian DVR R of residue characteristic p, and an integer N coprime to p, A[N] is finite étale over R. Henselian lifting gives a group isomorphism A[N](R) → A_k[N](k). Therefore, if source points P,Q in A_k[N](k) obey Q=[x]P, their unique torsion lifts obey Q_tilde=[x]P_tilde. This equality does not require knowing x.

**This only constructs a local p-adic lift.** It does not construct a bounded-degree number field, two compatible global algebraic torsion points, or a second reduction to a different characteristic.

## Exact necessary target-size bound

If a globally defined N-torsion point (N coprime to q) reduces into a g-dimensional abelian variety over F_(q^f), injectivity of specialization and the Weil bounds force

N ≤ #A(F_(q^f)) ≤ (q^(f/2)+1)^(2g).

The stdlib script `degree_floor.py` checks this inequality exactly using integer binomial arithmetic, not floating-point approximations. For q=2, the least f *not ruled out by this bound* is:

| N (order; primality not asserted) | g=1 | g=2 | g=3 | g=4 |
|---|---:|---:|---:|---:|
| 5 | 1 | 1 | 1 | 1 |
| 17 | 4 | 1 | 1 | 1 |
| 257 | 8 | 4 | 2 | 1 |
| 65537 | 16 | 8 | 5 | 4 |
| 2^128 − 159 | 128 | 64 | 43 | 32 |
| 2^256 − 189 | 256 | 128 | 86 | 64 |

A number field L with a place of residue degree f above 2 must satisfy [L:Q]≥f. **This is logarithmic in N at fixed g; not a polynomial-time impossibility theorem.** The table demonstrates only necessary feasibility, never actual existence.

## Higher-dimensional torsion bound: a nonuniformity warning

Le Fourn--Lombardo--Zywina, *Torsion bounds for a fixed abelian variety and varying number field*, Theorem 9.1 (https://pi.math.cornell.edu/~zywina/papers/av_exponent.pdf), proves for a **fixed** nonzero A/K a point-degree lower bound [K(P):K] ≫_(A,epsilon) N^(delta_A−epsilon), where delta_A is defined using l-adic monodromy. The constants and exponent depend on A. This cannot be applied uniformly without more work when choosing A_N and K_N depending on N or the input instance.

The 2024 survey https://londmathsoc.onlinelibrary.wiley.com/doi/full/10.1112/blms.12952 emphasizes that uniform bounds for higher-dimensional abelian varieties remain much less understood.

## Strongest next proof obligation

Produce a **uniform encoded-input global algebraization algorithm**. Given arbitrary P,Q=[x]P on E/F_p with N prime to p, it must (without solving x):
1. Supply a global g-dimensional abelian variety and compatible source specialization.
2. Explicitly encode two N-torsion lifts agreeing with the local p-adic torsion sections, with field degree, coefficient heights, and bit complexity charged.
3. Supply a good second reduction at q≠p and preserve the relation under specialization.
4. Compare entire transfer and destination-solve costs against matched source rho.

Without step 2, the known local lift is not an algorithmic cross-characteristic bridge. Do not conflate the cost of an abstract lift with producing its coordinates.

## Research and validation receipt

Sources reviewed: `problems/lift-reduce-second-prime-transfer.md`, `research/cross_characteristic_global_torsion_20261112/README.md`, root PR #3877, and the two linked primary research sources. Shared research-KB MCP not available here, repository wrapper not executed; no novelty assertion. `python3 -B degree_floor.py` and `python3 -O -B degree_floor.py` passed locally (exact integer controls). No paid compute, no isolated performance benchmark, no signed PCC verdict.
