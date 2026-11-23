# Protocol: is rho.signed_frobenius_strong the right matched reference on small Koblitz curves? (2026-11-15)

Frozen before any measured data. Class: **reference audit** (measurement of existing methods; no new method,
no IC change, no leaderboard edit). Every number is `[pcc:unverified]`.

## 1. Question
Validator VAL-20261115-f7df34 (D2) found `rho.signed_frobenius_v5` at about 0.25-0.30x `rho.signed_frobenius_strong`
on K1 n23/n29, mostly because strong's 32 full-size scalar multiplications (each charged 1.5 log2 r additions) dominate
its set-up at small r. (a) Is v5 a sound single-target cold method? (b) How does v5/strong move with log2 r?
(c) For the Koblitz rows that use a strong or tuned signed-Frobenius rho, how large is the distance to rho under each?

## 2. Arms (per curve, one ecbench session each, same binary, same 48 public targets, 3 rounds, 1 warm-up, alternating order)
| arm | method | role |
|:--|:--|:--|
| rho-strong, rho-strong-aa | rho.signed_frobenius_strong (twice) | current reference; A/A control |
| rho-v5, rho-v5-aa | rho.signed_frobenius_v5 (twice) | candidate reference; A/A control |
| rho-strong-v2 | rho.signed_frobenius_strong_v2 | strong walk with a fixed-base jump table (context: set-up fix on the strong walk) |
| rho-neg | rho.negation | context (A = 2) |
| gen-c<c> | ic.pipeline, `compact-orbit-scan:columns=c,raw_x_cap=1e8,generator_orbit=1`, `mitm-frobenius-layout-counted:m=2`, `targets=stride-generator` | strongest validated IC arm (research/ecbench_generator_orbit_20261022; validator-pre-declared gen-layout-c4 on n23) |
| k1-l<d> | ic.pipeline `koblitz-standard-subspace:dimension=d`, `pdp3-koblitz:m=3,engine=rootfind-negfold`, walk | K1 recipe rows of docs/ic/LEADERBOARD.md (n23 d=9, n29 d=10) |

Curves (a, n -> slug, log2 r; columns): (1,17) icv1-f2m17-tm101-00378d4e 16.00 c=1; (1,23) icv1-f2m23-tm5197-1f85e9e1 22.00 c=3,4;
(1,29) icv1-f2m29-tm40309-30c52b96 15.37 c=1; (0,31) icv1-f2m31-tm90707-c95f16f5 20.46 c=2; (0,37) icv1-f2m37-tm534059-32aad96b 27.78 c=8.
n31 and n37 are dropped, with the reason recorded, if a session would exceed the 3600 s timeout or the local cpu budget.
Plain methods only (no `*_full`): every arm's unpriced native work is charged alike in the section 5a band.

## 3. Pricing and statistics
`ecbench run --cpus none --allow-busy` (L0, no isolation claim), `verify --replay-all`, `band --ranks` at w = 0, 0.1, 1 GAE
per unpriced native operation (table work priced), `compare` paired by target with cluster-bootstrap 95% intervals
(seed 20261002, 4000 resamples) for v5/strong, strong-v2/strong, gen/strong, gen/v5 at each weight.

## 4. Predeclared reads
R1 soundness: every solve of every arm verified; A/A ratios of both rho pairs contain 1; median walk_steps / ideal steps
`sqrt(pi r / 2A)` for v5 and strong reported (a v5 median above 1.5x ideal, or any unverified solve, is a soundness flag).
R2 scaling: tabulate v5/strong at w = 0, 0.1, 1 per curve and fit ln(ratio) = alpha + beta * log2 r (ordinary least squares over
the curves measured; descriptive, the number of curves is at most five).
R3 distance: per IC arm, IC/strong and IC/v5 with intervals at the three weights; the rank holds only where it holds at all three.
R4 recommendation rule (owner decides): v5 is recommended as the additional matched reference at a row only if v5/strong < 1
with an interval excluding 1 at all three weights AND the audit finds no unpriced work or target-dependent information on the v5 side
that strong lacks. Otherwise strong stays. Either way both distances are reported.

## 5. Non-claims
No IC improvement, no change to docs/ic/*, scripts/build_ic_leaderboard.py, boundary_targets.json or src/. The binary is the one
already built (sha recorded in the README); the working tree has unrelated uncommitted edits in f4_fp.rs and pq_f4_f2.rs that no
measured method reads. The local host is a shared virtual machine: L0 only; counted GAE are deterministic, wall time is not used.

## Addendum A1 (written after the n17, n23 and n29 sessions had run and before any n31, n37, n41 or n53 data was read)
All five frozen sessions ran in minutes, so two larger Koblitz curves are added, rho arms only (no IC arm: no validated gen-layout column count
exists at these sizes), to extend the v5/strong scaling read R2: (0,41) icv1-f2m41-tm2308219-7f48b14a log2 r 39.00; (0,53)
icv1-f2m53-tm56619371-dac20a85 log2 r 44.26; same six rho arms, 48 targets, 3 rounds, seeds 20261115306 / 20261115307. The n17, n23 and n29
data had been seen when this addendum was written; the frozen read R2 is unchanged and these two curves do not alter R1, R3 or R4.
A supplementary session for the validated gen-layout section's own reference (rho.signed_frobenius, "rho-frob", the tuned walk) is not
in the frozen arm set and is labelled post hoc if run.
