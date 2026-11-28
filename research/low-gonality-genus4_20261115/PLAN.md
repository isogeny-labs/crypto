# PLAN: genus-4 non-hyperelliptic images, trigonal-pencil and plane-quintic relations

Frozen 2026-11-15 before the first measurement run. Local development check at small parameters, not a lab
benchmark. Every number produced is `[pcc:unverified]`. No improvement is claimed. git HEAD at planning time:
`3c24520f7cdc1c47bac26cc7778d058e655b79a8` (working tree has untracked files only).

## Question

Diem-Kochinke predict exponent 2 - 2/(g-1) = 4/3 for non-hyperelliptic genus-4 curves (against 3/2 for hyperelliptic
genus 4 and 2 for rho), using low-degree pencils/plane models. What do the two relation sources (fibres of the g^1_3 from
the rulings of the quadric; lines of the degree-5 plane model obtained by projecting the canonical curve from one of its
points) actually yield at small q, can they close a DLP in `Pic^0` with sparse linear algebra, and where does that sit
against a matched rho in the same group?

## Model and tool

- Curve: smooth bidegree-(3,3) curve C on P^1 x P^1 over F_p (p prime, 61 <= p <= 509 in the cells; p = 7..13 in group-law
  tests). K = O(1,1), degree 6, genus 4, canonical model on the split quadric. Smoothness is certified by
  (G_s,G_t,G_u,G_v) containing all (8,8)-forms. Split quadric only: the non-split quadric and the cone are not covered.
- Pic^0: classes D - D0 with D0 four fixed rational points, D effective of degree 4 carried by W_D = H^0(2K - D)
  (5-dim subspace of the 9-dim biquadratic forms); `combine(Da,Db,Dc) ~ Da+Db-Dc` by ideal-quotient linear algebra
  (Khuri-Makdisi style), level-3 fallback if the product space does not span. Exceptional classes (D coplanar,
  h^0(D)=2) are keyed through the residual pair, so keys are canonical.
- Group order: N_1, N_2 from fibrewise root counting, then interval BSGS in the Weil range of (a3,a4), verified on
  random points. Brute-force N_1..N_4 (zeta) for p = 7, 11 in unit tests.
- Relations (unknown classes lambda(P) = phi(P - Pa), plus kappa = phi(K - 6Pa), tau_i = phi(T_i - 3Pa)):
  - B (trigonal): fibre of either ruling with 3 distinct rational points all in F: sum lambda = tau_i; K = T1 + T2.
  - A (plane quintic): plane through Pa, P1, P2 (P_i in F) with 3 further rational points all in F: sum lambda = kappa.
    Equivalent to lines through two factor-base points on the projection from Pa.
  - C (general hyperplane section, **not** part of the lead): plane through 3 F points, other 3 in F. Used only as a
    labelled supplement when A+B is insufficient at F = all rational points.
- Linear algebra: Wiedemann on A^T D A mod ell (ell the largest prime factor of N, ell exactly dividing N), cross-checked
  against a dense kernel (kernel dimension must be 1). Individual log: random walk a g + b h decomposed into 4 rational
  points in F; two decompositions eliminate the unknown scale; the answer is verified by x g = h.
- Matched rho: Pollard rho with distinguished points, 16-adding walk, same `add`, same group, same g, h. The negation
  map is not free in this arithmetic (one full `combine`), so plain rho is the measured arm; a free-negation reference
  is reported only as plain/sqrt(2), labelled an assumption.

## Preregistered predictions (formulas fixed now)

| id | prediction | tolerance / decision rule |
|---|---|---|
| P1 | fraction of fibres (either pencil) with 3 distinct rational points = 1/6 | within +-25% (0.125..0.208), pooled over p >= 61; per-p reported |
| P1b | fibre histogram by #rational points: 0 -> 1/3, 1 -> 1/2, 3 -> 1/6 (2 -> ~0) | informational |
| P2 | A-line yield per pair = phi^3/6, phi = |F|/#points | within +-30%, cells with |F| >= 60 |
| P2b | the same with hypergeometric correction (m-3)(m-4)(m-5)/((n-3)(n-4)(n-5))/6 | informational |
| P3 | unique A relations = hits/10 (each plane found by C(5,2) pairs) | within +-15% |
| P4 | A+B alone are insufficient at p = 61 for every phi <= 1 and sufficient for phi* ~ 1.0, 0.8, 0.67 at p = 127, 251, 509 (minimal phi with rows >= columns + 3); implied m*(p) ~ p^0.75 | exponent of m* in [0.6, 0.9] (informational: 3 points) |
| P5 | combine needs the level-3 fallback in < 1% of operations | counted |
| P6 | DLP solved and verified by IC at each cell where P4 permits; rho solved and verified in all runs | hard check (x g = h) |
| P7 | S_IC / S_rho < 1 at w = 0 and at w = w_cal; > 1 at w = 0.1 and w = 1 (so the order flips inside the band: unresolved) | w = GAE per native op |
| P8 | A/A: second IC pass on the same instance within +-30% in GAE at w = 0 and within +-30% in native ops; rho first-half vs second-half means within 2 standard errors | |
| P9 | the measured native cost of the A search grows like m^3 (direct scan of F per pair), exponent of native_p vs p in [2.0, 2.6]; group-op count of IC is roughly flat in p | informational |

## Cells and stopping rules

- Yield study (task item b): p in {31, 61, 127, 251, 509}; curves K = 16, 16, 16, 12, 8; curve i from seed
  `20261115*1000003 + 7919 i + 1` (see `yield` code); phi in {0.3, 0.5, 0.7, 1.0}, F = D0 plus a random subset.
  Command: `genus4_pencil_ic yield P K 20261115 OUT`.
- DLP ladder (task item c): p in {61, 127, 251} with seeds {1, 2, 3} (three distinct curves each), p = 509 with seed 1 if
  wall time allows. Curve filter: N has a prime factor ell >= N^0.8 with ell^2 not dividing N (up to 80 random smooth
  curves). IC ladder phi = 0.4, 0.5, ..., 0.9, 1.0 (A+B only), then phi = 1.0 with C supplement; stop at the first rung
  that is solved and verified. The ladder is run twice (A/A). Rho: 8 runs at p = 61, 127, 6 at p = 251, 3 at p = 509;
  distinguished-point bits = floor(log2(sqrt(ell)) - 4); cap 60 sqrt(ell) steps per run.
- Costs: one GAE = one `combine` (counted exactly). Native work is counted in field multiplications (IC: F_p mulmods for
  point enumeration, relation search, decomposition tests; mod-ell word operations for Wiedemann, +1 per matrix entry
  touched) and, for rho, one native operation per walk step (partition hash and DP test) plus one per DP insert.
  Reported at w = 0, 0.1, 1 GAE per native op and at calibrated w_cal = (mulmod time)/(combine time) measured in the same
  binary (best of 5, labelled). Failed ladder rungs are reported separately: S_final (oracle rung) and S_all (all rungs).
- Unit tests (must pass before measuring): ring dimensions, group-law (commutative, associative, inverse, identity),
  point-class additivity against direct four-point divisors, order of random classes kills them with N from the zeta
  function (p = 7, 11), relations are true in Pic (two relations of one type give the same class sum), Wiedemann vs
  dense kernel, small rho.

## Known limitations declared up front

- Smoke runs of `dlp` at p = 31 and p = 61 (seed 1) were made during development before this plan was frozen; they showed
  that A+B are insufficient at p = 31 and p = 61 and are not used in any table.
- Split quadric only; one trigonal pencil pair, no large primes, no sieving, no Diem two-large-prime variant: the 4/3
  exponent needs those and cannot be read off these cells. At p <= 509 the factor base is a constant fraction of all
  points, so any exponent fit has 3-4 points and wide uncertainty.
- Rho is plain (no free negation). The `combine` arithmetic is generic linear algebra (about 2.6e5 mulmods per
  operation); an optimised Jacobian arithmetic would make a GAE cheaper and w_cal larger. A sensitivity row prices
  a GAE at 2.6e3 mulmods.

## Amendments (dated 2026-11-15, made after a first batch of DLP cells was started and stopped; the batch is kept in
`results_discarded_prefix/` and used in no table)

1. First batch (p = 61 seeds 1-3, p = 127 seed 1, p = 251 seed 1) showed that the C-supplement trigger "rows >=
   columns + 8" fired too early: at p = 127, phi = 1 the A+B system had enough rows but kernel dimension 5 (rank
   deficient), so the supplement never ran. The ladder now decides by the dense kernel dimension (must be exactly 1;
   dense check is a diagnostic monitor and is **not priced**), and the C supplement is added in chunks of 2000 random
   triples while the kernel dimension exceeds 1. A rung is rejected without a dense check only if rows + 1 < used
   columns. Consequence: phi* is now the smallest rung with kernel dimension 1 (previously rows >= columns + 3).
2. The curve filter is tightened from ell >= N^0.8 to ell >= N/16 (prime ell exactly dividing N), because in the first
   batch the cofactor N/ell reached 30 and moved rho by a factor sqrt(30) between instances. Cells are re-run with the
   rebuilt binary; seeds, rho runs and all other settings are unchanged.
3. The first batch's p = 251 seed 1 solved at phi = 0.9 under the old row-count rule (0.8 was 3 rows short), the p = 127
   seed 1 failed at all rungs; both are reported here only as the reason for the amendment.
