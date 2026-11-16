# LEAD-EXO-minors-guess-and-determine

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step. Family: exotic. Source: atlas_survey_20261107 --pursuit. Title field in leads.json is empty.
- Next-step excerpt (truncated): "Read the full texts of arXiv:1703.07544, 2005.05039, 2310.04132 and 2607.09814 first (the draft ..."
- Atlas survey line: "Mahalanobis line; designed from abstracts only; papers say no security risk". README: "never charged against matched rho. Four arXiv papers to read, then a 20-40-bit implementation with the scaling exponent fitted."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 74, 195 only); .../README.md (lines 201-206 only). No cryptanalysis hits searched beyond these; no draft design file found.

## 2. Hypothesis
Charged in a counted unit (group operations plus unpriced native work at 0, 0.1, 1 GAE per op, AGENTS.md 5a), a minors-based guess-and-determine ECDLP solver has a fitted scaling exponent in log2(N) over 20-40 bits that is no better than the matched rho exponent 0.5 (i.e. cost/sqrt(N) is non-decreasing in N). This is the falsifiable form of "papers say no security risk".

## 3. Preregistered prediction and falsification criterion
- Statistic: fitted slope b of log2(charged cost) on log2(N), N in 2^20..2^40, at least 6 sizes, 10+ instances each, bootstrap 95% CI; and ratio to matched rho at each size.
- Threshold: falsified if the upper 95% CI bound of b is < 0.5 at all three pricing levels (0, 0.1, 1 GAE). Consistent if the CI lower bound >= 0.5 at all three. Otherwise inconclusive.
- Null/negative control: same solver on random-relabelled (DL-shuffled) group, expected b near or above 1 (no structure). Positive control: planted instance where the minors relation is known to leak (solver must recover with a measurably lower cost). A/A: same solver, two seed sets, slopes must agree within CI.
- Not decidable here: the instruments need the papers' construction, which is not available (see 5).

## 4. Method
Not specified in executable form. The mechanism (the minors guess-and-determine construction) is defined only in the four papers' full texts, none present in this environment (no network). Inventing a stand-in would test my construction, not the lead's, so no toy is preregistered. Phase-2 plan once texts are available: implement the construction in C or Rust at 20-40 bits on the Iso Lab route (AGENTS.md 9a), frozen task card, derived seeds, interleaved arms. Cost within local limits: none, no run is made here.

## 5. Classification: blocked
Reason: the next step is explicitly to read full texts of four arXiv papers (1703.07544, 2005.05039, 2310.04132, 2607.09814), which are not in the repositories reached and cannot be fetched (no network). The implementation at 20-40 bits with fitted exponent also exceeds the stdlib, 2^22-order toy limits (sizes up to 2^40).

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
Nothing was run. This protocol does not show the lead is dead or alive; the recorded "no security risk" is the papers' claim relayed second-hand and is an unverified assertion. A toy at 2^12..2^22 could not support an exponent claim over 2^20..2^40. Even a null fit would not rule out an advantage beyond tested sizes.
