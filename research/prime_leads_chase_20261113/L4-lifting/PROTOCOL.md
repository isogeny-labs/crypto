# L4-lifting PROTOCOL (frozen 2026-11-13, before any measurement)

Lab dir: $SCR/chase/L4-lifting. All outputs [pcc:unverified].
Prior lab outputs (`.../ad79db08.../finish/lifting/`) no longer exist; everything is rebuilt.

## Q1 (paper proof, no measurement)
Extend the closure-note §5 Theorem to growing rank r = r(p) in two models:
- Model X (xedni: fresh lift per trial, relations among the k sampled lifted points);
- Model F (fixed lift / Silverman–Suzuki / "other rational points": a lift E'/Q fixed
  independent of the mask, target represented by a rational point of bounded height).
Derive the threshold r*(p) where each bound stops excluding per-trial success >= q^{-1/2},
and combine with rank results (GRH/Mestre, PPVW heuristic, known records). Verdict per (i),(iii),(iv).

## Q2 (measurement: height ratios on actual rank 7..9 subgroups)
Curves: Mestre-type y^2 = r(x) from prod_{i=1..8}(x-a_i) = q(x)^2 - r(x) (8 points, sum relation,
rank <= 7 from these), plus extra rational points found by small-height search to reach rank 8, 9.
Two size regimes: small a_i (|a_i| <= 30) and "lift-like" a_i random of b bits (b = 16, 32, 64).
Observable: rho_G = max_i hhat(P_i) / lambda_G, lambda_G = min nonzero hhat on <P_J>/tors
(SVP on the Gram matrix), per configuration J of k = s+1 points spanning rank s, s in {7,8,9}.
Heights by x-only doubling limit, n doublings (n >= 5); error estimate = |e_n - e_{n-1}|.
Positive control for the height code: hhat(2P) = 4 hhat(P) and hhat(P+Q)+hhat(P-Q) = 2hhat(P)+2hhat(Q)
to relative error < 1e-3; a torsion point (if present) gets hhat < 1e-3.
Output: margins at 256 bits (log2 sqrt(q) = 128) for s = 7..9 using the Cramer-form bound
(derived in PROOF.md) with median and worst measured rho_G, and the threshold rho*(s).
Rule: margin(s) > 0 at the WORST measured rho_G -> "excluded at measured rho"; < 0 at the MEDIAN ->
"not excluded by this argument"; otherwise "INCONCLUSIVE". (This only conditions on rho; it is not
a proof that real xedni lifts have such rho.)

## Q3 (toy: growing-rank xedni, 10..20 bits)
Arm H (honest): E/F_p, y^2 = x^3+Ax+B with prime order q (p of 10,12,14,16,20 bits).
Draw fresh masks (s_i,t_i), R_i = s_i P + t_i Q (Q = xP), k = 2..8 points. Centered integer lifts
P_i of R_i (z=1). Fit a plane cubic C over Z through O=(0:1:0) and P_1..P_k with C == lambda*E mod p
(LLL on the congruence lattice; k <= 8 since 9 coefficients). Group law on C with base O.
Detector: meet-in-the-middle over box n in {-2..2}^k mod a 61-bit prime l (C smooth mod l),
confirmed mod a second 61-bit prime and exactly over Q (projective integer chord-tangent).
Classify relations: trivial (implied by identical lifted points / torsion) vs non-trivial; useful
if sum n_i t_i != 0 mod q; recovered log checked against x.
Arm B (box bound, no lift): Pr[exists n in box\0: sum n_i R_i = 0 in E(F_p)] measured by the same
MITM on E(F_p). Any lift's non-trivial in-box rate is <= this (Lemma: Q-relations reduce).
Arm P (planted positive control): C fitted through O, P_1, P_2, P_4..P_k; P_3 := P_1 (+) P_2 over Q
(so R_3 = R_1 + R_2, planted). Detector must find (1,1,-1,0..) in >= 95% of trials.
A/A: Arm H repeated with a second seed set at 12 bits for k = 4, 8.
Trials: 300 per (bits,k) for H (fewer if time-limited, reported), 100 for B, 30 for P.
Predictions: fixed-r theory (closure §5.4 toy) predicts honest non-trivial relations only at
<= 10-12 bits and 0 at 14-20 bits for every k; growing-rank hypothesis H_grow predicts the
honest non-trivial rate tracks Arm B (ratio H/B bounded below, e.g. >= 0.1) as k grows.
KILL (H_grow at toy scale): pooled over 14..20 bits and k = 6..8, honest non-trivial count = 0
while Arm B predicts >= 30 expected in-box mod-p relations for the same trial count
(i.e. realisation fraction < 0.1 with ~95% confidence), AND positive control >= 95%.
SURVIVE: honest/B ratio >= 0.1 at >= 14 bits for some k >= 6 with >= 5 non-trivial verified relations.
INCONCLUSIVE: otherwise, or positive control < 95%.
Resources: single core, prlimit 3 GB, timeout 1200 s per command, total <= ~3 h wall.
