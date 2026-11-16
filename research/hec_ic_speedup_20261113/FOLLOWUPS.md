# Follow-ups, protocol dated 2026-11-14

Baseline: S2a Solver::Fast, full factor base, same round-eight instances and seeds 20260916..20260935.
Sources: PLAN.md; oracle.rs; sparse elimination in solve.rs; KB IC_TECHNIQUE_INTEGRATION_20261012
(large-prime exact elimination and filtering). GTTD 2007 primary PDF fetch remains blocked (403).
Existing methods are reused; no novelty claim.

## S2b: sparse columns followed by a dense residual

Move the walk's step columns and k into a dense block. Eliminate the other columns with minimum
column weight / shortest row pivots, then forward-eliminate the residual densely, preserving free
variables. Return k only when a row determines it; the driver verifies k D1 = D2.
Correctness gate: same verified k as Fast over 11 rows x 5 seeds, plus planted rank-deficient and
inconsistent systems. Pivot/count identity is not expected: this is a different elimination order.
Timing: alternating Fast/Hybrid, 20 seeds per row; ratio of median solve times and total times.
Success bar: median solve <=80% of Fast at g2 p>=503, with no total regression >10%.
Keep Hybrid opt-in if any gate fails or the bar is missed.

## T2: squarefree split test

For nonzero discriminant, complete splitting is equivalent to x^p mod u = x. Compare directly
instead of computing gcd(u,x^p-x); retain gcd for repeated-root polynomials. On accepted squarefree
polynomials all multiplicities are one, so skip repeated exact division. Gate against the original
oracle on exhaustive small fields and identical walks. Same seeds/instances as S2b; success bar
median oracle time <=90% of the original at genus 3 and 4. Keep selectable reference.

## T1: reduced-base two-large-prime baseline

Build the full place catalogue for these bounded prime fields; truncate its first m entries for
the factor base. Walk only on that base. Keep split relations with at most two distinct off-base
places, canonicalized by x with signed multiplicities. Eliminate off-base columns exactly mod ell
using sparse pivot rows until only factor-base columns and k remain. This handles general edge
coefficients and repeated roots without assuming every unsigned graph cycle cancels.
This first baseline uses exact partial elimination rather than claiming the optimized GTTD
forest complexity. Bound trials at 100000 and partial pivots at the full catalogue size.
Gate each retained/recombined relation as an identity in the full Jacobian before timing. Compare
full base / half base, 0/1/2 large primes on g4 p=31,61,127, three seeds; report successes and caps.
All construction, filtering, verification and solve time is charged. No superiority hypothesis
until the baseline's rank and partial-memory behavior are measured.

Measurements require a fresh S3 checkpoint and bench.record/v1. Unit correctness tests may run
before the experiment. Raw data and reports will retain null/capped arms.
