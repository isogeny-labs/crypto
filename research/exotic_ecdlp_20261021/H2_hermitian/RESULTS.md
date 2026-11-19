# H2 results: Hermitian-surface line relations on a Legendre fibre (C2)

Verdict: **KILL** (under the surrogate stated below). Job `exotic-h2-hermitian-r4` (isolab, 4 CPU / 8Gi; the protocol's 8 CPU / 16Gi did not schedule on Spot, so this is a documented deviation). Runtime 156 s. Seed 20261022. Raw data: `out/result.json`. Program: `run.py`. Earlier attempts: the base job and `-r1` never scheduled (harness bug and pending pods); `-r2` failed on a JSON serialisation error; `-r3` was OOM-killed by the p=31 brute-force point count, which was then removed.

## What was actually built (read this before the numbers)
- `H: sum x_i^(p+1) = 0` over `F_{p^2}`. Lines were generated as one orbit of a seed totally isotropic line under generators of `GU(4,p)` (permutations, a norm-1 diagonal, and rotations). The orbit size is the line count and was checked against the formula.
- The point and line counts come out as `(p^3+1)(p^2+1)` points and `(p+1)(p^3+1)` lines. **The protocol's `lines_count_expected=(p^3+1)(p^2+1)` is the number of points, not lines.** The correct line formula matches at every finished p. The protocol formula does not, so that validation flag is False by design. `result.json` carries both.
- Each line has `p^2+1` points (not `p+1`).
- Each line was independently verified to lie in H, each point lies on `p+1` lines, and the incidence is GQ(p^2,p). At p=7 the line count was cross-checked by brute-force counting of collinear pairs.
- **Surrogate fibre (no faithful H -> E_lambda map exists).** With `lambda = N(u0)`, the fibre `N(x0) = lambda N(x3)` is the union of the `p+1` planes `x0 = zeta x3` with `N(zeta) = lambda` (the norm circle). Every line meets the fibre in exactly `p+1` points, one per plane (coincident if the line meets the axis `x0 = x3 = 0`; those entries are dropped).
- Each fibre point goes to a Legendre x-coordinate `X` of `E_lambda: y^2 = x(x-1)(x-lambda)` over `F_{p^2}`, in two variants.
  - Surrogate A: `X = x1/x2`.
  - Surrogate B: `X = x1 x2^p / x3^(p+1)`.
- Columns are the `<±1>` classes of `E_lambda(F_{p^2})` (x-coordinates with square right-hand side, plus O); the count equals `#E/2 + 2`. Entries are **unsigned** multiplicities, because the sign of y is undetermined. Entries whose X is not an x-coordinate of an `F_{p^2}`-point are dropped.
- Tautological relations are the span of the pencil differences `S_P - S_P0`, where `S_P` is the sum of the rows of the `p+1` lines through P (the tangent hyperplane section pushed down). `nontaut_corank = min(#lines, #hit columns) - rank_Q(M)`. Rank over Q is the exact rank of the Gram matrix `M^T M`; ranks mod ell and mod a 31-bit prime are also reported.
- `E_lambda(F_{p^2})` is smooth (order `2^a * small`), so ell is tiny (3 or 5) and the mod-ell rank is not cryptographically meaningful. The mod-ell rank still equals the Q rank.

## Per-p table (surrogate A, primary)
| p | lines | ncols (hit) | rank_Q | taut dim | nontaut corank | M-rank vs random (10x) | surrogate B corank | incidence NN^T spectrum residual vs derived GQ(p^2,p) |
|---|---|---|---|---|---|---|---|---|
| 7  | 2752  | 26 (24) | 24 | 24 | 0 | 24.0 ± 0 | 0 (17 of 17) | 0.0 (exact) |
| 11 | 15984 | 74 (68) | 68 | 68 | 0 | 68.0 ± 0 | 0 (39 of 39) | 0.0 |
| 13 | 30772 | 82 (76) | 76 | 76 | 0 | 76.0 ± 0 | 0 (45 of 45) | 0.0 |
| 31 | censored | 514 | - | - | - | - | - | - |

- **Never-hit columns.** A few columns are never in the image of the surrogate (6 for A, 35-37 for B at p=11, 13). They are a property of the map, not of E, and are reported as `ncols_never_hit`, not counted as corank. The corank over all columns is `ncols - rank`, e.g. 6 for A at p=13.
- Density-matched Bernoulli rank is 24, 68, 76 (A); an alternative random baseline with `p+1` draws per row gives the same. M matches the random baselines in rank at every p.
- **The tautological pencil relations alone already span the whole row space** (taut dim = rank). The system is heavily overdetermined: about `p^4` rows against about `p^2/2` columns. So there is no room for a non-tautological rank deficit.
- Corank per hit column is 0.0 at every p, so it is flat and does not grow.

## Spectrum
- The incidence matrix `N N^T` (lines x lines) was verified exactly against the GQ(p^2,p) prediction `{(p+1)(p^2+1), p^2+p, 0}`. The annihilator `A(A-(p^2+p))(A-(p+1)(p^2+1))` gives residual 0 on random vectors, and the trace and trace-of-square multiplicities match. At p=7 the dense spectrum is `400 x1, 56 x2450, 0 x301`.
- **The spectrum list stated in the task and protocol, `{(p^2+1)(p+1), p^2-p-1, -(p+1)}`, is not the spectrum of `N N^T`.** Only its top element matches. The residual against that list as stated (15, 23, 27 for p = 7, 11, 13) is reported as `spectrum_residual_vs_srg_as_stated_in_protocol`. I judged the decision on the derived spectrum.
- The pushed-down matrix `M = N Phi` is a push-forward through a many-to-one map, so its `M M^T` spectrum is *not* expected to be the GQ spectrum. The result is a generic-looking spectrum with about `ncols_hit` distinct eigenvalues. Taking the unmodified rule literally ("off-SRG eigenvalue appears => Win") would give a vacuous Win; I did not use it for that reason.
- Diagnostic: the second-largest eigenvalue of `M^T M` is about 2.5-3x the matching random baseline's (e.g. p=13 A: 7415 vs 2750). The minimum eigenvalue is bounded well away from 0 (59, 59, 449 for A), so there is no near-singular direction. This does not change rank.

## Decision
Kill rule: at every finished p the non-tautological part has full rank (corank 0, for both surrogates) and the incidence spectrum residual is exactly 0 (< 1e-6). Win rule (corank growing with p, or a genuine structural eigenvalue anomaly): not met. Validation flags all pass, except the protocol's mislabelled line formula noted above. Caveats:
1. This is a surrogate, not a faithful `H -> E_lambda` restriction, so the result bounds this combinatorial model only.
2. p=31 is **censored** (953,344 lines x 962 points would give about 9e8 nonzeros). The scale trend is p = 7, 11, 13 only.
3. The relations are unsigned and carry no group-law check on E.
