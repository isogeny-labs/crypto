# Protocol: does Pollard rho see a large prime conductor gap? Large-sample test

Frozen 2026-10-18 before any session of this study ran (sha256 of this file and of
`specs/*.json` are in `FROZEN.sha256`, written by `freeze.sh` right after this file).

## Question

Two ordinary curves over GF(q) in one F_q-isogeny class, with prime order N and
endomorphism rings O_K (crater, conductor 1) and Z + ℓ·O_K (floor, conductor the
prime ℓ), are joined only by isogenies of degree divisible by ℓ. Jao, Miller and
Venkatesan (ANTS 2005; JNT 2009) prove equal ECDLP difficulty up to polynomial
factors only across conductor gaps made of small primes. Question: does the
measured cost of Pollard rho (ecbench `rho.negation`, counted group operations)
differ between crater and floor when ℓ is large?

## Hypotheses

- H0 (null): the per-target distribution of S = charged group operations / √r is
  the same on the crater and on every floor of the same (q, N). Ratio
  mean S(floor) / mean S(crater) = 1.
- H1: the ratio differs from 1 by more than representation noise, defined below.
- Representation noise (negative control): the ratio between a curve and its
  isomorphic model (a·u⁴, b·u⁶), u = 3, which has the same endomorphism ring,
  the same group and a different point representation; rho's partition hashes
  the representation, so this ratio is the null distribution of "different
  coordinates, same everything else".
- Positive control: `rho.plain` against `rho.negation` on the same workloads must
  show the √2 ≈ 1.414 factor (interval excluding 1). If it does not, the
  detector is broken and nothing else in the session is interpreted.
- A/A control: a second `rho.negation` arm (`rho-neg-aa`) on the same workloads;
  its ratio to the reference is the harness noise floor (it should be 1 up to
  the rho draw, since seeds are shared per workload and round).

## Families (D = −19, h(D) = 1, Aut = ±1 everywhere; no automorphism confound)

| tier | spec | ℓ | q bits | kernel field degree k = ord(t/2 mod ℓ) | curves | targets/curve | arms |
|---|---|---|---|---|---|---|---|
| S1 | `s1-ell8209-q29` | 8209 | 29 | 2736 (hard gap) | crater, 3 floors, 2 twins | 4096 | rho-neg, rho-plain, rho-neg-aa |
| S2 | `s2-ell131101-q37` | 131101 | 37 | 65550 (hard gap) | crater, 1 floor, 2 twins | 1024 | rho-neg, rho-plain, rho-neg-aa |
| M1 | `m1-ell1211653-q45` | 1211653 | 45 | ℓ−1 (hard gap) | crater, 3 floors (native constructor, seeds 1–3) | 128 | rho-neg, rho-neg-aa |
| M2 | `m2-ell1138019-q45` | 1138019 | 45 | ℓ−1 (hard gap) | same | 128 | same |
| M3 | `m3-ell1609903-q45` | 1609903 | 45 | (ℓ−1)/2 (hard gap) | same | 128 | same |
| L1–L4 | `l*-pairs*` | 33,555,419 … (≈2^25) | 53 | 2 (easy gap: the quadratic twist has rational ℓ-torsion; t = ℓ−2) | 24 pairs crater+floor, 8 crater twins | 16 | rho-neg |

"Hard gap" means the kernel of every ℓ-isogeny between the levels is defined over
GF(q^k) with k large, so no known way to compute the connecting isogeny costs less
than ~ℓ^2.6 group operations (benchmarks/20261011-isogeny-vertical-margin-historical);
at ℓ = 8209 that is ≈ 2^34 against a rho cost of ≈ 2^14.5, at ℓ ≈ 2^20 it is ≈ 2^53
against ≈ 2^22.5. "Easy gap" (tier L) means the gap can be crossed by one Vélu
isogeny of cost O(ℓ) ≈ 2^25 (how `pairs.json` was built), comparable to the rho
cost 2^26; tier L therefore tests the large-ℓ end where equivalence is already
constructive, and tiers S and M test the end where it is not.

Tier S curves are the Sage-verified curves of
`research/conductor_gap_cm_pairs_20261010/specs/` (same q, a, b, generator). Tier M
parameters were found by `make_specs.py`'s parameter search (q = (t² + 19ℓ²)/4 and
N = q + 1 − t both prime, k computed exactly); the native `conductor_gap`
constructor certifies crater and floors in the session itself. Tier L pairs are
entries 0–23 of `research/conductor_gap_cm_orbits_20261015/inputs/pairs.json`
(all `status: PASS`).

## Measurement

- Harness: `target/release/ecbench` built from commit fb02c552bde56a609f167829183191d6be1c1319
  with cargo 1.99.1; method `rho.negation` (reference), `rho.plain` (positive control,
  tier S), `rho.negation` again as `control`. Targets `planted_shared` (the same scalars
  on every curve of a spec), one round, one warm-up, `order: alternate`,
  `measurement.seed = 20261018` (fresh: every earlier study used 1 or 20261012),
  `target_seed` distinct per spec (2026101801, 2026101802, 2026101811–13, 2026101821–24).
- Unit: S as recorded by ecbench (counted group operations over √r). Wall time is not
  a result of this study: the host is shared with two other running jobs (load ≈ 24 of
  32 cores) and every run is L0; wall time is not read.
- Sessions run concurrently, one per spec, each on its own CPU (25–31) and lock file
  (`run_sessions.sh`). Concurrency cannot change operation counts.
- Audit: `ecbench verify --replay-all` for tiers S and M; `--replay 24` for each tier L
  session (a full replay of a 53-bit session costs about as much as the session).
- Sample size and power (SD of S for rho ≈ 0.52 × mean): standard error of a
  floor/crater ratio with n targets per curve ≈ 0.52·√(2/n): S1 1.1 %, S2 2.3 %,
  M 6.5 % per floor (3.8 % pooled over 3 floors), L 3.7 % pooled over 24 pairs
  (per pair 18 %). The earlier lead at ℓ = 8209 was 1.22 [1.005, 1.46] from 4 targets;
  S1 resolves it at the 1 % level.
- Runtime estimate from the pilot (18 s per 53-bit solve on this host, scaling with √r):
  S1 ≈ 6 min, S2 ≈ 25 min, M ≈ 20 min each plus floor construction, L ≈ 60–80 min
  each; all nine sessions in about 90 minutes of wall time on 7 cores.

## Analysis (fixed before data)

1. Per spec, per arm: mean S per curve with 95 % bootstrap interval over targets;
   ratio floor_i / crater and twin / original with bootstrap intervals (4000
   resamples, seed 1) and, since targets are shared, the paired per-target
   log-ratio with its interval.
2. Pooled over tier S + M floors and over the 24 tier L pairs separately (never
   pooled across tiers).
3. Decision rule: H0 is rejected for a tier only if (a) a floor/crater interval
   excludes 1 after Bonferroni over the floors of that tier, (b) the twin/original
   intervals of the same tier contain 1 (otherwise the effect is representation
   noise), and (c) the positive control shows √2 and the A/A control is at 1.
4. Inadmissible after seeing data: changing targets, seeds, arms, unit, curves, or
   adding tiers. A failed audit voids the session.
5. Every outcome is reported, including incomplete and timed-out runs (`outcome.status`).

## Formal companion

`lean/RepresentationHashInvariance.lean` (Lean 4.34.1, no Mathlib, no axioms
beyond core): along any isomorphism of the groups of points, an r-adding walk whose
partition hashes the representation has a transported partner walk with the same
trajectory and the same first-collision cost, and the transport is a bijection of
the space of (partition, multiplier) choices. So, over a uniformly random partition,
rho's cost distribution is identical on crater and floor; what is measured here is
the one fixed partition ecbench uses.
