# Fused residual step with distinguished points: preregistered protocol (before data)

Date: 2026-11-01. Branch `claude/ic-gap-20261031`. Follows `research/ic_residual_fused_20261031/` (study 1), whose
preregistered rule (fused beats baseline at w = 0, 0.1 AND 1) was NOT met: one addition per step won at w = 0
(0.56-0.92) but the priced native work (about 2 table lookups and 56 stored bytes per step) lost at w >= 0.1.

## Candidate (`targets=residual-fused-dp`, src/cryptanalysis/ic_framework/residual.rs, mode 3)
Study-1 fused step (one addition per step) plus distinguished points: every step pays one DP test
(`dp_tests`, one native op per step, the analogue of rho's per-step canonicalisation); only residuals whose
x-key hashes into a 2^-b fraction (b = floor(log2(r)/2) - 9, so b = 0 for log2 r < 20, b = 8 for the 36-37 bit
curves) are tested against F, probed and stored; walks longer than 40*2^b steps without a DP are restarted.
Collisions are found when the merged walks reach a DP, about 2^b steps late. b is a function of r only, fixed here.
Not part of this study (recorded, exploratory): a negation-equivariant step (so that L and -L merge) fell into short
cycles and lost; the candidate does not merge L with -L after a collision, so it needs about 1.5x more steps than
the fused no-DP walk (exploratory, 16-24 targets).

## Claims
- C1 (engineering, confirmatory), on the DP-active curves p36 and p37: for each B in {2, 4, 8} the paired 95% interval of
  S(res{B}dp)/S(res{B}) lies below 1 at w = 0, 0.1 and 1 (harness weights; counted unit; unpriced native work priced alike).
- C2 (descriptive): distance to matched rho at w = 0, 0.1, 1 (section 5a band); rank only where the order holds across the band.
- C3 (control): on p16 and p18 b = 0, so the candidate is the study-1 fused walk; no claim is made there beyond A/A = 1 and verified answers.
- Classification: the gain is lower priced native work (table lookups/bytes) of a generic collision search, not structural IC.

## Design
Curves p16, p18, p36, p37 (same group data as study 1). Arms: rho (rho.negation), baseline res2/res4/res8 (`residual`),
candidate res2dp/res4dp/res8dp (`residual-fused-dp`), res4aa (A/A copy of res4). 16 targets per curve, target seed 20261401,
measure seed 20261402, 3 rounds + 1 warm-up, alternate_measured. Counts are deterministic; run locally at L0
(isolab-eks route unavailable, as in study 1 Amendment 1); `specs/` (L2) are kept for a later isolab-eks replay.
Positive control: `ecbench verify --replay 12` and harness verification of every recovered log.

## Decision rule (frozen)
C1 holds iff every one of the six (curve in {p36,p37}) x (B in {2,4,8}) cells has its whole interval below 1 at w = 0, 0.1, 1.
A/A interval must contain 1, all records verified, audits OK; otherwise the session is void. Cells that fail are reported as failures.
Next step after this study (not decided here): equivariant step with a larger step set, and DP on the baseline's two-addition step.
