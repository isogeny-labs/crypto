# RESULT: LEAD-ICP-extension-field-embedding

## Outcome
- status: ran (single run, no retry). exit code 0, wall 7 s (run.status: exit=0 wall=7). run.stderr empty.
- run.py verdict field: consistent_with_prediction; aa_ok: true.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): consistent if min(s_0.1, s_1) >= 0.4 and ratio > 1 at both w for the larger half of sizes; falsified if min slope < 0.1 or ratio_0.1 < 1 at the largest size. Positive control: random-FB null reproduces Poisson model within 0.03. A/A mean-step ratio in (0.7, 1.43).
Measured:
- slope ln(ratio) vs ln N2: s_0.1 = 0.778, s_1 = 0.784 (threshold 0.4).
- ratio_0.1 / ratio_1 at p=607 (largest): 291 / 2897; at p=307,431,607 and all sizes ratio_w>1 for w=0.1,1 (10 to 291, 100 to 2897).
- ratio_w0 (report only): 0.32 to 1.59, crosses 1 at p>=307 (not ranked, per s5a).
- A/A ratios 0.886 to 1.096, all inside (0.7, 1.43): pass.
- Positive control: null f_null vs Poisson within 0.03 only at p=67,103,211 (0.865/0.871/0.853 vs 0.864/0.869/0.863). FAILS at p=151 (0.491), 307 (0.250), 431 (0.250), 607 (0.125): null is not Poisson there.
- Side claim: f_sub ~ 1.0 only at p=67,103,211 (0.9998 to 1.0). At p=151,307,431,607 f_sub = 0.5 exactly.
Verdict: consistent_with_prediction by the frozen rule; but controls did not fully pass (positive control failed at 4 of 7 sizes).

## Reading
- Cause of the control failure: ord(G) = r < N2 at p=151 (r=N2/2), 307 (N2/4), 431 (N2/4), 607 (N2/8). Coverage and the null were normalised by N2, not by r, so f_sub, f_null are deflated by the factor r/N2 (0.5, 0.25, 0.125 match f_null exactly). The f_sub = 0.5 values are therefore a normalisation artefact, not evidence about subfield-FB yield; at p with r=N2 the side claim holds (yield ~1 vs ~0.86).
- The IC arm is priced with attempts = p/f_sub, so f_sub=0.5 doubles ic_ops at those sizes, and rho runs in a subgroup of order r, so rho_ops are non-monotone (762 at p=607 vs 736 at 431). Both distort ratios, though the dominant term is natives (2p per attempt + 2p^2), which scale as p^2 ~ N2 regardless; ratio_w>1 and slope near 0.78 follow from the pricing formula (natives ~ N2 vs rho ~ N2^0.5, slope ~0.5 to 1 expected) more than from any measurement. The IC cost is a formula, the oracle is not implemented, and rho carries no native pricing (stated limit).
- The toy does not test the lead's real question (embedded E(F_p) subgroup DLP); G generates E(F_{p^2}) or a large subgroup of it.
- Discrepancy: run.py verdict does not account for the failed positive control; I would call the verdict valid for the slope question but the yield side claim only supported at 3 sizes.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (weakly): with a free oracle priced at 0.1 or 1 GAE the counted subfield-FB cost exceeds rho at n=2 and grows faster, as Gaudry's p^(2-2/n) vs sqrt(p) predicts; no sub-rho signal, and the result is largely built into the pricing formula.

## Standing
[pcc:unverified], single local toy run, no independent replay.
