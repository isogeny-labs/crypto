# L1-pkm protocol (frozen 2026-11-13 before any measurement)

Lead: LEAD-ICP-pkm-tower-algebraic-oracle. Everything is `[pcc:unverified]`.

## Question
For the PKM Kummer chain (V = g·μ_{2^t}, split Semaev S3 chain with m−2 auxiliary
unknowns, N = m·t tower unknowns), what is the exact polynomial-calculus (PC) solving
degree D(m,t), and how fast does it grow with t at fixed m?

## Instrument
`scripts/gen.py` (generator; checked polynomial-for-polynomial against two committed repo
systems, m3 t6 and m4 t4 at P1, before this protocol) and `closure/` (Rust): the exact
degree-D PC closure in the tower normal form F_p[Z/2^t]^{⊗m}[u]. A normal-form block monomial
y_{i,0}^e has degree popcount(e). W_D = smallest F_p-space containing the S3 generators
(normal form) and closed under h -> NF(v·h) for every variable v and every h ∈ W_D with
deg h ≤ D−1. (This equals the raw-presentation PC closure at degree D ≥ 4; F4's solving degree
is an upper bound for it.) Certified D = least D with 1 ∈ W_D; the certificate of exactness is
1 ∉ W_{D−1} (computed exactly, full closure). A run stopped by memory/time at degree D' with
1 ∉ W_{D'−1} complete reports D ≥ D' (censored), never more.

Targets: random x_R with no decomposition (checked by exhaustive enumeration over E(F_{p^2})
of all multisets of V and all signs), so 1 ∈ ideal. Prime p = 40961 (2^13 | p−1),
prime-order curve, 2 random targets per cell (seeds 1, 2); replication of a subset at p = 12289.

## Cells
m=2: t=1..6; m=3: t=1..4; m=4: t=1..4 (t=4 is the never-run SURVIVE route); m=5: t=1..3;
m=6: t=1..3; m=7: t=1..2; m=8: t=1..2. Run in increasing size; stop a cell at the resource caps.

## Controls
- Positive control (detector sees degree): reproduce committed/certified D: m=2 → 5
  (t=6..), m=3 t=3 → 6, m=4 t=2 → 6, m=4 t=3 → 7 (closure note Table 4.2, repo 03-pkm-degree).
  If any of these disagrees, the instrument is suspect and no verdict is drawn.
- Negative control (no false refutation): planted (consistent) targets must never yield
  1 ∈ W_D up to the cap; at least one planted target per m ≤ 6 at t=2.
- Replication: D at p = 12289 equals D at p = 40961 on ≥ 3 cells.

## Hypotheses and predictions
- H1 (closure note scenario): D rises by one per t-step at fixed m (σ = ΔD/ΔN = 1/m).
- H0 (KILL-type growth): σ ≥ α*(m,1) = H⁻¹(1 − 3/m).
- Hflat: D bounded in t (σ → 0).
Prediction recorded now: m=4 t=4 gives D = 7 or 8; m=5 t=3 gives D = 8; m=6 t=3 D ≥ 8.

## Cost model (closure note §4; re-derived): with |V| = 2^t = 2^{τn}, relations cost
2^{n(1−(m−1)τ)} attempts × M^w, M ≈ 2^{N·H(σ)}, linear algebra 2^{2τn}. Implied exponent
E_w(m,σ) = 2/(m+1 − w·m·H(σ)) when the denominator > 2, else ≥ 1 (no gain); break-even
E = 1/2 at σ = α*(m,w) = H⁻¹((1 − 3/m)/w). Distance to rho reported as E − 0.5.

## Slope estimate
σ̂(m) = least-squares slope of certified D on N over all exact t ≥ 2 cells of that m (and
separately the t ≥ 1 fit, and the last-step slope). Uncertainty: interval [σ_lo, σ_hi] from
the min/max of all pairwise slopes among t ≥ 2 cells, with censored values entering as bounds
(D ≥ c widens σ_hi to +∞ side only on that endpoint). D is an integer, so with two exact cells
the interval is that single slope ± 1/ΔN (integer-rounding uncertainty).

## Decision rule (per m, frozen)
- KILL (toy scale) if σ_lo − (1/ΔN_max) ≥ α*(m,1), i.e. even the rounding-pessimistic slope
  gives E_1 ≥ 0.5.
- SURVIVE (toy scale, i.e. the lead stays alive) if σ_hi + (1/ΔN_max) < α*(m,1).
- INCONCLUSIVE otherwise, or if fewer than two exact t ≥ 2 cells exist for that m (then the
  t ≥ 1 two-point slope is reported but no verdict is drawn).
- The m=4 t=4 route: SURVIVE for m=4 requires D(4,4) = D(4,3) = 7 AND the rule above; it
  cannot by itself give SURVIVE since α*(4,1) = 0.042 needs a flat ladder over > 6 t-steps.
Global: the lead is KILL at toy scale only if every m with a verdict is KILL and the fitted
σ̂(m)·m does not decrease with m; otherwise INCONCLUSIVE for m ≥ 8 extrapolation.

## Theory sub-question (desk; D)
Attempt a PC-degree lower bound for the chain growing with t at fixed m. Judged against the
measured D: a claimed bound that exceeds any certified D is refuted.

## Resource limits
One core, `prlimit --as=3221225472`, `timeout 1200` per command, ~3 h total lab wall.
