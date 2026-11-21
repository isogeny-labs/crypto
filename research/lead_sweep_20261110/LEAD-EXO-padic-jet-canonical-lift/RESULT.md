# RESULT: LEAD-EXO-padic-jet-canonical-lift

## Outcome
- status: ran; exit code 0; wall 10 s (run.status: exit=0 wall=10); run.stderr empty.
- Single run of run.py, no retries. RESULT.json verdict field: "falsified" (max_z = 5.579).

## Statistic vs preregistered prediction
- Prediction (PROTOCOL.md s3): "z < 4.0 in every cell (6 prime-size cells x 2 variants)"; falsified if any cell z >= 4.0; inconclusive if a control fails.
- Measured (16 cells; 8 at p~2^11, 6 at p~2^13, 2 at p~2^15; z of first-difference mode frequency vs shuffle null):
  - p~2^11 (2063): z in 2.47..3.40, all < 4.
  - p~2^13 (8219): z = 4.88, 5.22, 4.51, 5.49 for two curves (both variants); 2.00, 1.99 for the third curve.
  - p~2^15 (32771, N=32467): z = 5.58 (variant 0), 4.92 (variant 1).
  - 7 of 16 cells have z >= 4.0, so the preregistered criterion is met: falsified.
- Real mode frequencies are only about 1.3-1.5x the null mean (e.g. 0.001192 vs 0.000794 at N=8389). The null sd is tiny, which inflates z.
- Controls: positive control M = 1.0 in all cells (pass); A/A batch-mean |z| <= 0.25 in all cells (pass); null is the matched-multiset shuffle. All controls pass, so the verdict is not "inconclusive" by the protocol's own rule.
- Note: protocol text says 6 prime-size cells; run.py produced 16 (4+3+1 curves, x2 variants = 16 cells, not 12). Count is as in section 4.

## Reading
- Falsification as defined is real, but I doubt it shows linear-in-k scalar structure. Likely cause is a symmetry the shuffle null does not preserve. Hensel lift in y commutes with negation, so s(-P) = -s(P), and z = -x/y flips sign, giving t(-k) = -t(k) (my derivation, not tested by run.py). Then t(k+1)-t(k) equals t(-k)-t(-k-1), so every first difference occurs in matched pairs. This inflates the mode count relative to a random permutation that has no such pairing.
- Supporting but indirect evidence: distinct_t is about 0.63*N (e.g. 5223/8389), as expected for a random map, so t looks otherwise unstructured. The z excess is modest in ratio terms, and scales with how small the null sd is.
- Variant 1 (x-shift) also shows it. Its map is not obviously odd, but x -> x+p(x^2+3) keeps x(-P)=x(P), so negation symmetry should hold too (unverified).
- The toy therefore tests "first-difference mode vs exchangeable null", not additive scalar leakage. The 2^13 cell with z~2 shows large cell-to-cell variation of the mode statistic. The shuffle null is mis-specified for an odd function t, which PROTOCOL s6 flags only as an exchangeability caveat.
- run.py verdict ("falsified") matches the numbers under the preregistered rule. The discrepancy is interpretive, not computational.
- A follow-up (not done, not allowed here) would use a symmetry-preserving null (shuffle t on k=1..(N-1)/2 and mirror) or check whether t(-k) = -t(k) holds exactly.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction (tentative): the excess is plausibly explained by the negation symmetry of t, which carries no information about d beyond the known +-k ambiguity; the prediction was formally falsified, but this does not show a usable scalar signal.

## Standing
[pcc:unverified], single local toy run, no independent replay.
