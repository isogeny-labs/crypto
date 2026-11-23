# PLAN (round 2): Smith route H -> plane quartic with Diem relations, charged group order, measured negation rho

Frozen 2026-11-21 before any measurement run of this round. git HEAD at freeze: 473c4064294aea5131fb869b8aaa30810f9ed228.
All numbers are `[pcc:unverified]` local development checks (no isolab, no benchrec). No improvement is claimed.
Prior round: `research/isogeny-walk-quartic_20261115/` (kept unchanged). Binary: `src/bin/smith_walk_dlp_ic.rs` (new; the first-round bin is untouched).

## Question
After the verifier's major issue (S omitted the group-order phase of the quartic Jacobian) and two minor ones (rho analytic, BSGS not Diem), how far is the full Smith route from a MEASURED negation-map rho on the same J_H, with every phase charged?

## Design (what is built)
1. Instances: random monic degree-7 H over F_p that reach a quartic at one Smith step (generator `positives_lc`, copied), canonical quartic C refit from the first 20 distinct canonical points (20 > 16 = deg C * 4, so the quartic through them is unique by Bezout), checked by the cubic kernel (dimension 0) and quartic kernel (dimension 1).
2. Group order (charged): #J_C(F_p) = #J_H(F_p) (isogenous). Route Z3: point counts N1, N2, N3 of H over F_p, F_p^2, F_p^3 by exhaustive enumeration (F_p^3 by finite differences, character through the norm), L-polynomial of genus 3, N = P(1). Route Z2B (sensitivity, also measured): N1, N2 plus a BSGS on the one unknown coefficient of the window |s3| <= 6 p^1.5, using Cantor additions on H. Both charged in counted units; their agreement is a check. Both verified by N*G = 0 on H and on C.
3. Instance for the DLP: ell = largest prime with ell || N and N/ell <= 64 (instances without one are skipped and counted). G_H = [P1 - P2], Q_H = [P3 - P4] etc. for rational points P_i of H whose R-pair is pushable; G' = (N/ell) G_H, Q' = (N/ell) Q_H (so Q' in <G'>, prime order ell, odd, hence no 8-torsion ambiguity in the pullback). Unknown k with Q' = k G'.
4. Push-forward: P -> R-pair (Q1, Q2) on C (F_p or conjugate F_p^2 pair); divisor Q1 + Q2 + O, with O a fixed rational non-flex point of C, is a degree-3 class [Q1 + Q2 + O - 3 O] = Phi(P) - [... differences only]. Phi(P1) - Phi(P2) is the image of [P1 - P2] (homomorphism, tested 57/57 in round 1 and re-tested end-to-end here by the pullback check).
5. Solver on C: `qds_core.rs` of research/quartic-diem-sparse_20261115 (copy with a driver appended; group law, sparse arm, descent unchanged): factor base about 2 sqrt(p) points, double large primes, row mode `light` (c = 2.0), sparse Wiedemann. Several targets share one relation/LA phase; per-target descent charged separately.
6. Pullback and end-to-end check [5]: k from C, verified as k G' = Q' on the ORIGINAL H with Cantor arithmetic (`scalar_mul_fast`), counted.
7. Rho reference: negation-map rho with r = 32 adding walk, distinguished points (2^-5), two-cycle escape and restart on long walks, on J_H in <G'> with the same Cantor arithmetic (`prime_hyperelliptic`), same G', Q', same ell. Runs per target: 6 (two seed sets of 3 for A/A). Cost in the same counted unit.
8. Plane_quartic_pic panic: module copied to `src/plane_quartic_pic.rs` with the completion bound raised from 7 to 12 (a catch and a retry with a different divisor is also kept); the first-round P7 test (principal divisor sums single valued, negative control) is re-run (`pushfwd2`) on the same instances and the untested pairs are counted again.

## Unit and prices
Counted unit: GAE_H = the cost in F_p operations (mults M plus 3 per inversion I) of one Cantor addition on H, measured as the mean over 300 random additions in each instance. Field adds are not counted on either side (same convention as qds_core and the Cantor tally here, `M = FieldMul + FieldSqr`, `I = FieldInv`; first round counted adds on H only, which favoured IC; the first-round reference is recomputed below in its own convention).
Native work (hash probes, table lookups, mod-ell multiply-adds in linear algebra, union-find, comparisons, the Legendre-table lookups of the point count, negation and compare in rho) is priced at w in {0, 0.1, 1} GAE_H per operation, on every arm, rho included. S(w) = (M + 3 I) / GAE_H + w * native. For the transfer, F_p mults are counted by hooks in copies of smith_trigonal.rs and planefit.rs; modular square roots, Legendre symbols and inversions inside the library helpers are modelled (2 log2 p mults, log2 p mults and 1 I) and labelled as modelled. Smoothness certificate of the fitted quartic is run by qds_core (counted).

## Cells
Ladder p in {101, 211, 521} and 1021 if the measured F_p^3 count at 521 times 8 is below 3 minutes per curve. Per p: 3 curves (first three that pass the filters in generator order, seed 31; every rejected candidate and the reason is recorded), 3 targets per curve, IC arm run twice per curve (seeds s and s xor 1, A/A), rho 6 runs per target. Metrics per curve: transfer M,I,native, group order phase M,I,native (Z3 and Z2B), relation+LA on C (setup, relations, elimination, Wiedemann, BM), descent per target, pullback (Cantor), total S at w = 0, 0.1, 1, S/rho at each w (ratio of means, bootstrap 95% over curves), shares of each phase.
Exponent fits: log-log slope of mean S (and of rho) vs p over the ladder, only with >= 4 points, bootstrap over curves; otherwise "not available".
Stopping rule: about 3 h wall; a smaller completed ladder is preferred to an unfinished one; the p = 1021 cell is dropped first.

## Preregistered predictions
- Q1: At p = 521 the Diem arm (relation phase + descent, field operations M + I per target, w = 0, group order excluded) costs less than the first-round BSGS arm (1.83e7 mul+inv per target at p = 521) by a factor >= 3. Expected: confirmed at w = 0, refuted at w >= 0.1 once the mod-ell linear algebra is priced (the previous Diem study had the LA at 97% of S at w = 0.1). Judged at w = 0; the other prices are reported.
- Q2: The group-order phase (route Z3) is below 10% of S at p = 521. Expected: refuted at every w (brute-force F_p^3 counting is p^3 work against p^1.5 for rho).
- Q3: The measured negation rho (mean field operations incl. adds, same convention as the first-round analytic reference sqrt(pi N / 4) * Cantor field ops per add, with ell in place of N) is within a factor 2 of that reference at every p (ratio in [0.5, 2]). Expected: borderline; the earlier quartic study measured negation rho at 1.32 sqrt(ell) steps with 2 additions per step.
- Q4: S/rho at w = 0 is below 3 at p = 521 (first round: about 10.6 at every price). Judged for the headline S that charges Z3; reported separately for S charging Z2B and for S without any group-order phase (ell public). Expected: refuted for Z3, confirmed for Z2B and for the given-ell variant.
- Q5 (exploratory): the log from C pulls back and verifies on H for every found target (end-to-end [5]); the P7 test has no untested pairs after the extended completion bound.

## Out of scope
W > 1 walk. Walk statistics only if everything above is done early.

## Amendment A1 (2026-11-21, written after the p = 101 smoke run and BEFORE any run of record)
The smoke run (p = 101, seed 31, excluded from the results) showed the factor-base size c = 2.0 of the plan fails at p = 101 (`fail=cycles(21,101)`: all 231 pairs of a 21-point factor base used, too few cycles). Rule added: each curve's Diem arm starts at c = 2.0 and escalates to c = 4.0, then 6.0 only after a failed attempt; every failed attempt is charged in full (setup included) to S. The A/A repeat uses the same rule with the other seed. The smoke run also showed that the verification `k G' = Q'` on C can be inconclusive (the scalar-multiplication chain of the quartic group law returns `None` for some k); the verification on H is the authoritative end-to-end check and both are reported. No prediction changes.
Selection filter unchanged (cof <= 64); candidates rejected for any reason (no prime ell with small cofactor, quartic not fitting, driver cannot form the group instance) are listed with the reason and are not charged (instance selection was not charged in round 1 either); acceptance rate is reported.
