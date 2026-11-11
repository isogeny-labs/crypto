# LEAD-ICP-char0-lifting-xedni-smoothness

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (disposition_20261101: closed_known); family: ic-prime; last_reviewed 2026-10-24.
- title: Lifting to characteristic zero - classical xedni, prescribed rational lifts (Cheon 2-isogeny form) and smoothness of integer lifts of x.
- next-step excerpt (truncated): "implement and freeze the bounded H-XEDN-4a5b50 search and either certify an exact dependent pres..."
- Files read: research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (item 11, lines 25 and 230); research/ecdlp_subexp_desk_20261101/README.md (lines 11-13, 61). Recorded context: JKSST C0/p obstruction conditional on a random-image model; Xedni V2 found 0 certified dependencies in 20 bounded pairs; function-field lift exponent alpha=3 versus needed < 1/2.
- H-XEDN-4a5b50 itself was not found in the files read (no Q-elliptic height/dependency engine here).

## 2. Hypothesis
The "smoothness of integer lifts of x" component of the lead carries no exploitable bias: for x-coordinates of random curve points over F_p (p ~ 2^16..2^22) lifted as integers x + k p (k in 1..63), the B-smooth rate (B=300) equals that of uniform random integers of the same size, within a ratio of 1.00 +/- 0.03 + 3 SE. This is the recorded obstruction (random-image model) restated for the smoothness step.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio R of smooth rate, point-lift arm / uniform null arm, SE by delta method; N = 150,000 draws per arm.
- Consistent: |R-1| <= 3 SE + 0.03. Falsified (bias exists): R - 3 SE > 1.10. Otherwise inconclusive.
- Null/negative control: uniform x in [0,p), same lift offsets (seed 202). A/A: second null with a different seed (303); must have |R_aa-1| < 3 SE.
- Positive control: planted x built from the 20 smallest primes (seed 404); must show R > 3, else the detector is invalid and the verdict is inconclusive.
- Falsifying decides: a smoothness-biased lift exists and the lead's smoothness leg deserves a real search; consistency decides only that this leg is generic.

## 4. Method
Pure stdlib. For each draw i: p = PS[i mod 3]; random a,b, rejection-sample x with x^3+ax+b a nonzero QR (Euler criterion); lift n = x + k p; trial-divide by primes <= 300. Fixed seeds 101/202/303/404. Cost: about 600k trial divisions per arm, estimated 1-3 min on one core, < 20 MB RAM.

## 5. Classification: runnable_local
A stdlib toy decides the smoothness-of-lift leg. The dependency-certification leg (H-XEDN-4a5b50) needs a Q-height engine and is not attempted.

## 6. What a run cannot establish
It does not test xedni rank-drop probability, Cheon 2-isogeny lifts, heights/discriminants of the lifted curve, or any asymptotics; toy sizes only; x is the only coordinate tested; the generic result is expected, so it cannot show the lead is closed, and results are [pcc:unverified].
