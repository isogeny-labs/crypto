# PLAN: walk genus-3 hyperelliptic Jacobians to a plane quartic (Smith) and solve there

Frozen 2026-11-15 before any measurement run. git HEAD at freeze: 3c24520f7cdc1c47bac26cc7778d058e655b79a8.
All numbers produced are `[pcc:unverified]` local development checks (no isolab, no benchrec).

## Question
Does the repo contain an end-to-end route  H (genus 3 hyperelliptic over F_p, prime-order subgroup)
-> Smith isogeny -> plane quartic Jacobian -> Diem IC -> log pulled back and verified on H ?
If not, what is the exact missing piece, and what do the walk statistics say?

## Stages (stage gate: a later stage runs only if the earlier one passes)
- S0 audit: what smith_trigonal / smith_prop24 / smith_kernel_coverage / smith_elliptic_walk build (file+function names).
- S1 walk statistics (explicit Smith one step, class-level ceilings for W > 1).
- S2 model test: do the points of Recillas' curve X, projected by the Mumford coefficients
  (b0 : b1 : b2) of the R-triple, lie on a plane curve of degree 4 (the canonical model)?
- S3 (only if S2 passes) pushforward of divisor classes into Pic^0 of the plane quartic and DLP.
- S4 (only if S3 passes) Diem relations + sparse solve, pullback, verification on H, S / rho at 0, 0.1, 1 GAE.

## Cells
S1a one-step success: p in {101, 211, 521}; seeds {1, 2, 3}; n = 3000 random squarefree octics per (p, seed)
    via smith_prop24::random_row(p, seed, index), index 0..n. Metric: full_success rate (Wilson 95%),
    plus the cycle-type tabulation. A/A: seeds are the replicates; the three seeds must agree within their Wilson intervals.
S1b walk ceilings, exhaustive (no seeds): smith_kernel_coverage::prime_report for p in {7, 11}
    (monic octics with zero x^7 term, curve + twist). Tiers: W=1 explicit-Smith necessary condition
    (stable pair kernel on the curve), W=1 any stable Lagrangian, W=inf in the isogeny class: pair-kernel member,
    any-Lagrangian member. These are existence statistics, NOT constructed isogenies.
S2 model test: p in {37, 101}; instances: Smith Example 1 (p = 37) and 10 random Smith-positive degree-7 curves
    per (p, seed in {1,2,3}) (generator: SplitMix64(seed), first normal-form valid map). Points: all X points
    produced by Trigonal::image_points over every affine H(F_p) point over U (F_p and F_{p^2} points, the latter
    split into F_p equations). Metric: dimension of the space of plane curves of degree d in {2,3,4,5} vanishing on the
    projected points (b0/b2, b1/b2); minimal d with nonzero kernel; number of points used (must exceed C(d+2,2)).
    Also: fraction of H(F_p) points whose R-pair is F_p-rational.
S3/S4 cells (only if S2 passes): p in {101, 211}, 3 seeds, 3 instances each; matched negation rho on
    Jac(H) in the same counted unit (Mumford/Cantor F_p multiplications), quartic side in F_p multiplications
    via plane_quartic_pic's FqCounter; native work priced at 0, 0.1, 1 GAE per operation.

## Preregistered predictions
- P1: one-step Smith success on random curves is in [0.14, 0.24] at every p (point prediction 0.186 = Smith 18.57%;
  the lab's prior census gives 0.169-0.173).
- P2: success within W steps is monotone in W: direct_pair <= direct_any <= class_any and direct_pair <= class_pair <= class_any
  for every prime (monotone by construction at the class level; an inversion would be an instrument error).
  Explicit multi-step hyperelliptic walks are NOT measurable for generic curves from repo code (see S0), so W > 1 is class-level only.
- P3: total S/rho < 1 at the largest prime for instances that reach a quartic (tested only if S4 runs; else untested).
- P4: for every S2 instance the projected X points lie on a unique plane quartic (kernel dim 1 at d = 4, 0 at d <= 3).
- P5: the R-image pair of an H(F_p) point is F_p-rational for about half of the points (conjugate pair otherwise) -- exploratory, no pass/fail.

## Stopping rule
Time-box about 3 h. S3 requires an explicit divisor-class map into Pic^0 of the quartic; if S2 fails or S3 cannot be
built in the box, stop and report the blocker with exact errors. No claim of improvement is made; distances to matched rho are reported.

## Amendment A1 (2026-11-15, written BEFORE the first `canon` run; reaction to the skeptic verdict)
The skeptic rated P4 as testing one projection only. New stage S2b: the canonical map by the trace of the H differentials.
For Q in X with R-triple (x_k, y_k = b(x_k)), eta_i(Q) = sum_k x_k^i dx_k / y_k = h_i dt with
h_i = sum_k x_k^i D(x_k) / (b(x_k) G_x(x_k)) = [x^2] (x^i D b^{-1} mod G). The trace of holomorphic forms along a correspondence is holomorphic,
so (h0:h1:h2) is the canonical map of X if the eta_i are independent. Subcommand: `smith_walk_dlp canon`.
- P6: for every instance in the S2 cells (p in {37,101}, seeds 1..3, 10 instances each, Smith Example 1 not separately re-run) the points
  (h0/h2, h1/h2) have plane-curve kernel dimension 0 at d = 2, 3 and exactly 1 at d = 4 (a unique plane quartic). Decisive: 30/30 at d=4 dim 1 confirms; any instance with dim 0 at d=4 refutes the formula.
- P7 (S3a, only if P6 holds): the pushforward of a principal divisor of H (div(x - a)) lands on a principal divisor of the quartic (checked in plane_quartic_pic).
Stopping rule unchanged; S4 (DLP, S/rho) is run only if S3 yields a verified divisor map within the 90 minute box.

## Amendment A2 (2026-11-15, written after the S2b `canon` and S3a `pushfwd` runs, after ONE smoke run of `dlp` at p=101 seed 1 want 2 (2/2 instances verified; those two are excluded), before the confirmatory `dlp` cells)
S3b/S4 (toy, BSGS on the quartic, not Diem IC): `smith_walk_dlp dlp`. Instances: monic degree-7 Smith-positive curves (the Cantor code needs monic imaginary models).
G_H = [P1 - P2], targets Q_H = [P3 - P4] (point differences over U); Phi maps both to Pic^0(C); BSGS on Pic^0(C) (plane_quartic_pic, FqCounter) gives m with m G_C = Q_C;
candidates k = m + j ord(G_C), j = 0..7, accepted iff k G_H = Q_H on H (prime_hyperelliptic Cantor). Confirmatory cells: p in {101, 211, 521}, seeds {11, 12, 13}, up to 4 instances each, 6 targets per instance.
- P8: every target whose log exists on C is pulled back and verified on H (verified/found = 1), with ord(G_H) = ord(G_C) * e, e in 1..8.
- P9 (exponent, exploratory): BSGS group-op count on C scales as sqrt(ord) i.e. p^{1.5}; priced F_p cost per target grows about p^{1.5}.
- P10: S/rho >> 1 at all three prices (BSGS on a dense quartic is not Diem IC; no win expected). Rho reference: analytic expectation sqrt(pi N / 4) group ops (negation map) times the MEASURED Cantor cost per add on H (field ops per add from prime_hyperelliptic::ops), NOT a measured rho run; label accordingly.
Native unpriced work (hash probes of BSGS, Phi evaluation in F_{p^2}, quartic-fit linear algebra) is priced at w in {0, 0.1, 1} per operation in the post-processing script; rho's native work is one hash probe per step.

Amendment A2b: the first confirmatory dlp pass showed unverified targets (6, 6, 7 at p = 101, 211, 521). Correct expectation (should have been preregistered): m G_C = Q_C only gives Q_H - m G_H in ker(phi), of order dividing 8, so a target with Q_H not in <G_H> need not verify. P8 is restated as: every found-on-C target either verifies or has 8 (Q_H - m G_H) = 0. Instrumentation added and the SAME cells re-run (deterministic seeds); numbers of pass 1 are kept in results/dlp_pass1.jsonl.
