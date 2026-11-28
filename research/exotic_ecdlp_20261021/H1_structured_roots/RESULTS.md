# H1 results: symmetrized Semaev S_3 small roots (C1)

Verdict: **KILL**. The symmetrized system reaches no further than the raw system. Its best root-box exponent is 0.1125 at p = 2^28, which is below the generic 1/4 and far below the 1/3 needed.

Authority: `PROTOCOL.md`, `../PROOFS.md` (C1). Program: `run.py`. Data: `out/result.json` (isolab job `exotic-h1-structured-roots-r5`). Earlier run `out/result_r3_nmax110.json` used smaller lattices and gave the same betas, except raw at 2^24 (0.1000 vs 0.1125 here).

## Setup actually run
- Curves: E/F_p with full rational 2-torsion and a rational point of order 4 (`E.abelian_group()` has a Z/4 factor), all four cells.
  - Full E[4] only at p = 65537 (p = 1 mod 4). The other primes are 3 mod 4, so full E[4] is impossible there.
- S_3 was verified on random collinear triples at every p. The rewrite g(e1,e2) = f(x1,x2) was also verified.
- Degrees: raw f has bidegree (2,2), total degree 4. g(e1,e2) has total degree 2 (weighted (1,2) degree 4). The FGHR invariant system has bidegree (2,2) in xi (xi = sum of x over the E[2]-orbit), which is total degree 2 in (sigma1,sigma2).
- Reach is measured with a hand-built modular Coppersmith lattice. Shifts are x^i y^j f^k p^(m-k) over a downward-closed monomial support (triangle, rectangle or weighted triangle), reduced with LLL.
  - A root counts as recovered only if it comes out of the Howgrave-Graham-valid short vectors via resultant and integer root finding.
  - A beta level passes if at least 2 of 3 planted true roots are recovered.
  - Raw box: x_i <= p^beta. Symmetrized box: (e1,e2) <= (2p^beta, p^(2 beta)), so beta stays in x-box units and is directly comparable with the raw system.
  - Lattice size is capped at n = 180, m = 9. Reach is therefore lower-bounded by the lattice family and may grow slightly with a larger one.
- Generic references. Spec (1/total degree of raw f): beta_generic = 0.25. Product-box heuristic for raw f: 1/8 = 0.125. The same heuristic for g in x-box units: 1/6.

## Per-p table (job r5)
| bits | p | beta_raw | beta_sym (swap, e1/e2) | beta_sym (FGHR) | beta_generic (1/deg = 1/4) | sym degree (e1,e2) | count ratio (200 R) |
|---|---|---|---|---|---|---|---|
| 16 | 65537 | none (box too small) | none | not testable | 0.25 | 2 | 1.10 |
| 20 | 1048583 | 0.0875 | 0.0875 | not testable | 0.25 | 2 | 0.72 |
| 24 | 16777259 | 0.1125 | 0.1000 | not testable | 0.25 | 2 | n/a |
| 28 | 268435459 | 0.1125 | 0.1125 | not testable | 0.25 | 2 | n/a |

- The count-control ratio at 40000 random R is 0.996 (2^16) and 1.006 (2^20). The 1/3! model holds (within 2x).
- `beta_sym` = max over the swap and FGHR variants. FGHR was never recovered: below xi-box about 15 there are fewer than two small-xi points with a rational preimage, and where it was testable (beta 0.15 and up at 2^28) no lattice was predicted to work.
- Median lattice time at successes: 0.05 to 0.7 s.
- Primary field: `beta_sym` = 0.1125 (p = 2^28).
- Trend: beta_sym - 1/3 is about -0.22 and does not grow. beta_sym creeps up at about 0.009 per bit. That is consistent with finite-size and rounding effects approaching the asymptotic 1/8 to 1/6, not a threshold change. Extrapolating that slope linearly to 131 bits is not meaningful and `result.json` clips it at 1/3; it is not used below.

## Verdict against the rule
- Kill rule: beta_sym <= beta_generic + 0.02 at every p. 0.1125 <= 0.27 holds at every p, so Kill.
- Win rule needs beta_sym > 1/3 and growing with log p. It is not met.
- The symmetrized system never outperforms the raw one by more than one grid step (0.0125). Raw even leads at 2^24.
- Controls (S_3 verified, identity verified, 4-torsion present, count control within 2x) all passed.
- All cells finished uncensored; the whole job took 111 s.

## Implied cost at 131 bits and distance to rho
One relation needs about 6 * p^(1-3 beta) target tries, one lattice call each. This leaves out the polynomial lattice cost.

| beta used | bits per relation |
|---|---|
| 0.1125 (measured, 2^28) | (1 - 0.3375) * 131 + log2(6) = **89.4** |
| 1/3 (what a Win would need) | about 2.6 |
| 0.2865 (needed to hit the 2^21 gate) | 21 |

- Gate: 2^21. The measured cost is 2^89.4, which is 68.4 bits above it.
- Rho at 131 bits: about 2^65. Each relation here already costs more than a whole rho run (2^89.4 against 2^65), by about 24 bits.
- Null box enumeration costs about 2^43.7 per relation (131/3 bits), so this lattice route is worse than the null by about 46 bits.
- 0.0125 per bit of growth, even if it held, would not close this gap at any cryptographic size.

## Caveats
- The sweep grid step is 0.0125 and boxes are ceil(p^beta). Rounding dominates at p = 2^16, where nothing was recovered. beta values at 2^16 to 2^28 are therefore coarse.
- The lattice family (n <= 180, m <= 9, rectangle/triangle/weighted-triangle supports) is hand-built, not the Jochemsz-May optimal Newton-polytope lattice. The `sym2` runs used the n = 91 weighted triangle `w11_D12`; no larger sym lattice was selected.
- `run.py` was executed on isolab via a 273-byte bootstrap that fetches the program from `s3://isolab-results-104186811093-us-east-2/inputs/exotic-h1/run.py`. The wrapper rejects command arguments over 8 KB, and the 33 KB script exceeded that.
- Job history: `exotic-h1-structured-roots` ran an empty program, because its stdin path equals the wrapper's own temp path. `-r1` failed at submit (8 KB limit). `-r2` and `-r3` were first valid runs (n <= 72, then n <= 110). `-r4` was a repeat of r3 (my edit failed to apply). `-r5` is the final run.
- No performance claim follows from these isolab timings.

## Framework rubric (`ECDLP_EXPERIMENT_FRAMEWORK.md`)
| Field | Entry |
|---|---|
| Object | E/F_p with full 2-torsion and a rational order-4 point, p = next_prime(2^b), b = 16..28; S_3(X1,X2,x(R)), raw and symmetrized |
| Claim | A symmetry-aware lattice recovers root boxes of exponent above 1/3 that grow with log p |
| Observable | Largest recovered exponent beta (raw vs symmetrized), count-control ratio |
| Null | Generic Coppersmith reach (1/4 spec, about 1/8 product-box); box enumeration about p^(1/3) per relation |
| Win | beta_sym > 1/3 and growing with log p; relation cost < 2^21 |
| Kill | beta_sym <= beta_generic + 0.02 at every p. **Met.** |
| Scale test | beta_sym = 0.0875, 0.100, 0.1125 for 2^20, 2^24, 2^28: slow creep, nowhere near 1/3 |

Gap to rho at 131 bits, using the idealized 0.886*sqrt(N) = 2^65.2 floor: this route costs about 2^89.4 per relation, so its distance to rho is about +24 bits per relation, before linear algebra. TotalCost is not below rho, and the relation step alone exceeds the whole rho run.
