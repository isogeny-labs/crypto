# Walking genus-3 hyperelliptic Jacobians to a plane quartic (Smith): canonical map built, divisor push-forward verified, toy DLP pulled back

Status: `[pcc:unverified]`, local development check at small p, no isolab/benchrec, no improvement claimed.
Plan: `PLAN.md` (frozen at git HEAD 3c24520f; amendments A1, A2, A2b dated and ordered, see "Deviations"). This revision HEAD: 3c24520f7cdc1c47bac26cc7778d058e655b79a8.
Numbers: `RESULTS.json`; tables: `RESULTS.md`; graph: `graph.svg` (drawn by `plot.py`); raw rows: `results/*.jsonl`.

## Question
Does a route H (genus 3 hyperelliptic, F_p) -> Smith isogeny -> plane-quartic Jacobian -> DLP -> log pulled back and checked on H exist, and how far is its cost from the matched rho?

## What changed after the skeptic's verdict
The previous revision stopped because the projection (b0:b1:b2) of the R-triple Mumford coefficients is a degree-10 plane model, not the canonical quartic. The skeptic correctly noted that this tests only one projection. Fixes:
1. **Canonical map (new, `canon`).** For a point Q of Recillas' curve X let (x_k, y_k = b(x_k)), k = 1..3, be its R-triple on H over t = g(x). The trace of the holomorphic differentials x^i dx/y of H along the correspondence R is holomorphic on X, and since dt = g'(x) dx = (G_x/D) dx, it equals h_i dt with
   h_i = sum_k x_k^i D(x_k) / (b(x_k) G_x(x_k)) = [x^2] (x^i D b^-1 mod G(t,x))   (Euler's identity for the monic cubic G).
   So psi(Q) = (h_0 : h_1 : h_2) is computable with F_p / F_p^2 arithmetic at every known point of X, with no singularity analysis. Source of the construction: derived here from smith_trigonal.rs (eqs. (5)-(11) of Smith) by the trace-of-differentials argument; the independence of the three eta_i is not proved, it is tested by the fit below.
2. **Divisor push-forward (new, `pushfwd`).** Phi(P) = [Q1(P) + Q2(P) - 2 P0] in Pic^0(C)(F_p) for the R-pair of P; rational pairs use `point_class`, F_p^2-conjugate pairs use class(Q+Qbar+C) - class(A+B+C) + [A-P0] + [B-P0] through conics (`class_from_conics`, no K needed). phi([P-P']) = Phi(P) - Phi(P').
3. **Toy DLP with pullback (new, `dlp`).** BSGS on Pic^0(C) in `plane_quartic_pic`, log pulled back and checked with `prime_hyperelliptic` Cantor arithmetic on H.

## Results (all `[pcc:unverified]`)
**P1 (unchanged)** one-step Smith success 0.194 [0.186, 0.203] / 0.186 [0.178, 0.194] / 0.184 [0.177, 0.193] at p = 101 / 211 / 521: confirmed.

**P6 canonical model.** 60 instances (p = 37, 101, 3 seeds x 10): kernel dimensions of plane curves at d = 2..6 are (0,0,1,3,6) in 58 of 60 (a unique plane quartic, and quartic times 1,u,v at d = 5, 6), and (1,3,6,10,15) in 2 of 60 (image on a conic: X hyperelliptic for that curve, no quartic model; one per prime). P6 as preregistered (30 of 30 per p) is therefore confirmed in 29 of 30 per prime; the exceptions are a real feature of the curve family, not a formula failure. Points per instance 33 or more, image errors 0, 35 points skipped for h2 = 0 (chart). The 58 quartics passed `smoothness_certificate` (construction by `PlaneQuartic::new`). The earlier "degree 10" finding is thus explained: it was the wrong projection.

**P7 push-forward.** Of the 58 quartic instances, 57 had at least one testable pair. Principal-divisor test: P + iota P ~ P' + iota P' on H, so Phi(P) + Phi(iota P) must be one class independent of x. 1538 pairs were tested; 57 of 57 instances show exactly one value, 0 show several. Negative control (Phi(P) + Phi(P') with P, P' not related by iota): several distinct values in 57 of 57 instances. Limits: 254 test pairs hit a panic in `plane_quartic_pic::complete` ("generating space did not complete by degree 7", caught, counted as not tested) and 44 pairs had no Phi (image error or h2 = 0); one instance had no testable pair. These are library limits on some degree-3 divisors, not evidence against the map, but they mean the test is not exhaustive.

**P8 toy DLP and pullback** (p = 101, 211, 521; seeds 11-13; up to 4 instances per seed; 6 targets Q_H = [P3 - P4] each; G_H = [P1 - P2]; monic imaginary models; 2 of the 12 instances at p = 101 had a conic image and were dropped):

| p | instances | targets | log found on C | verified on H (k G_H = Q_H) | found, not verified | of those, 8(Q_H - m G_H) = 0 |
|---|---|---|---|---|---|---|
| 101 | 10 | 60 | 50 | 44 | 6 | 6 |
| 211 | 12 | 72 | 60 | 54 | 6 | 6 |
| 521 | 12 | 72 | 50 | 43 | 7 | 7 |

160 of 204 targets had a log on C (the rest are outside <G_C>); 141 of those 160 verified on H and the other 19 did not. Every found log either pulled back to a verified log on H (k = m + j ord(G_C), j in 0..7) or left a difference of order dividing 8 (the kernel S of phi), as the theory predicts: a log on C determines the log on H only modulo the kernel, and a target with Q_H not in <G_H> need not verify (A2b, stated honestly: this was not preregistered, the first pass showed the failures, the instrumented re-run reproduced the same counts exactly; pass 1 kept in `results/dlp_pass1.jsonl`). ord(G_H)/ord(G_C) is 1 or 2 in all instances. [pcc:unverified]

**Distance to matched rho (P10).** IC route cost per target = BSGS on the dense quartic, priced F_p mul+inv from `FqCounter`. Rho reference = analytic expectation sqrt(pi N / 4) group operations (negation map) times the measured Cantor field operations per add on H (about 600); **it is not a measured rho run** and the Cantor tally includes additions that the quartic counter does not, which favours the IC route, so the ratios below are if anything understated. Native unpriced work (hash probes, Phi in F_p^2 estimated at 800 ops per target, not instrumented) is priced at w = 0, 0.1, 1:

| p | median ord(G_H) | IC ops / target | rho ops (analytic) | S/rho w=0 | w=0.1 | w=1 |
|---|---|---|---|---|---|---|
| 101 | 134294 | 1.95e+06 | 1.95e+05 | 10.5 [8.9, 13.6] | 10.5 [8.9, 13.6] | 10.5 [8.9, 13.5] |
| 211 | 1166668 | 4.93e+06 | 5.05e+05 | 11.0 [7.4, 13.3] | 11.0 [7.4, 13.3] | 11.0 [7.4, 13.3] |
| 521 | 16948893 | 1.83e+07 | 2.07e+06 | 10.6 [6.2, 15.2] | 10.6 [6.2, 15.2] | 10.6 [6.2, 15.2] |

The route is about 10.6 times more expensive than the matched rho reference at every price and every prime (the native work is negligible next to the group operations, so w does not move it), with no trend in p: exponent fit over three primes (no interval) 1.37 for the quartic BSGS against 1.45 for rho, both near 1.5 as expected for a square-root method on a group of size p^3. This is BSGS, not Diem index calculus; the distance remaining is therefore to be closed by the sparse Diem solver on the now-available quartic (separate lead), not by this transfer. No improvement is claimed; the transfer itself costs little (the setup is a quartic fit and a few Phi evaluations). [pcc:unverified]

## Predictions against outcomes
| | prediction | outcome |
|---|---|---|
| P1 | one-step in [0.14, 0.24] | confirmed |
| P2 | monotone in W | existence ceilings monotone at p = 7; W-resolved walk untested (no generic (2,2,2)-isogeny walk code; p = 11 exhaustive run was killed) |
| P3 | S/rho < 1 | refuted for the BSGS toy: S/rho about 10.6 at all prices; Diem IC arm not run |
| P4 | (b0:b1:b2) is the quartic | refuted for that projection only (it is a degree-10 model); not a statement about X |
| P5 | about half rational pairs | 0.491 |
| P6 | canonical map gives a unique quartic | 58 of 60; 2 of 60 hyperelliptic X (conic) |
| P7 | push-forward of principal divisor is principal | confirmed in all testable instances, negative control separates; some pairs untestable (library panic) |
| P8 | found logs pull back | confirmed up to the kernel (see table); differences in 8-torsion in every failure |
| P9 | BSGS cost exponent 1.5 | fit 1.37 (three points) |
| P10 | S/rho >> 1 | S/rho about 10.6 (not >> but clearly > 1) |

## Verifier disagreement
- The verdict's major issue (no S, no transfer, no rho) is now partly resolved: S, the transfer and a rho reference exist. I disagree that "matched rho measured" was achieved: the rho figure is analytic (expected steps times measured Cantor cost per add), and the IC arm is BSGS, not Diem IC. I did not build a rho run (time box); the ratio is a labelled estimate.
- The minor P4 point is accepted and the text now says "this projection is not the canonical model".
- The verdict's suggestion to implement an explicit (2,2,2)-isogeny walk for generic H is not done: W > 1 success remains class-level only. Evidence: no code in `smith_kernel_coverage.rs` or `smith_elliptic_walk.rs` constructs it, and the new bin does not either.

## Deviations from the frozen plan
A1 (canonical map, P6, P7) was added before its first run; A2 (dlp) after one smoke run at p = 101 seed 1 (excluded); A2b after seeing the unverified targets of the first confirmatory pass. P6 and P8 wording were corrected as stated above. All earlier-numbered cells are unchanged.

## Limitations
Small p (101..521), toy BSGS on the quartic, rho analytic, native work partly estimated (Phi 800 ops), instances limited to monic degree-7 Smith-positive curves with a rational Smith kernel (about 19 percent of curves; the cost of finding such a curve is not charged), points over U only for building the quartic (the fit uses at least 33 points for 15 coefficients, and uniqueness at d = 4 plus smoothness are the evidence it is the canonical model), 254 library panics in test pairs, X hyperelliptic in 2 of 60. Walk W > 1 not built; p = 11 coverage not run. One session, no replay or validator; all values `[pcc:unverified]`.

## Next step
Run the Diem line-relation pipeline with sparse block Wiedemann on the quartic that `build_quartic` now produces (reuse `research/quartic_n4_la_20261021/ic.rs`), keep Phi as the transfer, and measure a real rho on the same Jacobian with the negation map. Then the gap above (about 10.6 for BSGS) can be restated for the exponent-1.4 to 1 pipeline.

## Sources read (repo paths)
src/cryptanalysis/smith_trigonal.rs, smith_prop24.rs, smith_kernel_coverage.rs, smith_elliptic_walk.rs, plane_quartic_pic.rs, diem_quartic.rs; src/prime_hyperelliptic/curve.rs, ops.rs, fp_poly.rs; src/bin/smith_near_prime.rs, smith_holdout_materialize.rs, quartic_ic.rs; research/smith_near_prime_20261114/RESULTS.md; research/smith_iso_granularity_20261015/REPORT.pdf; research/smith_class_invariant_20261015/REPORT.pdf; research/five_leads_e2e_20261019/REPORT.md; papers/ecdlp-index-calculus-treatise/survey/transfer.md.

## Repro
`cd /home/user/crypto && CARGO_TARGET_DIR=$PWD/target cargo build --release -j2 --bin smith_walk_dlp && bash research/isogeny-walk-quartic_20261115/repro.sh`
