# LEAD-ICP-full-width-arity-grid

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_blocked (reviewed 2026-10-29). Family: ic-prime.
- Title: Full-width complete-solve IC arity grid (m = 2..8) at 50-63-bit subgroups - prime coset/small-x bases, Koblitz orbit and ordinary-binary bases.
- Next step (truncated): "Run the approved 63-bit three-base m = 2..8 solver-to-certificate feasibility panel (H-ICPERF-05..." Cost: expensive (63-bit solves; launcher/RSS repairs). Why it might beat sqrt(p): "Measures rather than beats; generic model predicts the gap widens".
- Survey note: prime complete-solve ladder stops at 32 bits and m=7; 63-bit second run had six time-censored planted controls and one monitor-error cell; no mathematical obstruction claimed.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 29, 291); docs/session-logs/2026-11-05.md (lines 313-328, 770-785; only an empty atlas-match, no content). No other hits read.

## 2. Hypothesis
The full 63-bit panel cannot run here. Its core premise is that a small-x prime factor base is generic at every arity: the m-fold sumset of the base (the set of targets that admit an arity-m decomposition) covers a fraction of the group equal to that of a random symmetric subset of the same size, so the relation yield per target follows the generic count f^m/(m! N). Claim: at toy group orders N ~ 4e4..6.6e4 (prime order, p ~ 40k..65k), with a small-x base of about 40 signed points, the coverage ratio (actual / generic null) of the m-fold sumset is within 5% of 1 for m = 2, 3, 4.

## 3. Preregistered prediction and falsification criterion
- Statistic: R_m = coverage(small-x base logs, m-fold sumset) / mean coverage over 8 random symmetric subsets of equal size, per curve and m.
- Prediction: |R_m - 1| <= 0.05 in every cell with null coverage < 0.9 (cells near saturation carry no information; they are still reported).
- Falsified if any informative cell has |R_m - 1| > 0.05 (a non-generic structure in small-x bases, in either direction). Consistent otherwise. Cells reported with z against the null sd.
- Null / negative control: random symmetric subsets of Z_N of equal size (8 draws per curve).
- Positive control: structured base {-k..k} of the same size (arithmetic progression of logs); it must deviate from the null by |ratio-1| > 0.2 at m=2 on every curve, otherwise the detector is blind and the verdict is inconclusive.
- A/A arm: a ninth independent null draw scored by the same code against the null mean; must satisfy |ratio-1| <= 0.05 in informative cells, otherwise inconclusive.
- Decides: consistent means the generic yield model is not contradicted at toy scale. It does not validate the 63-bit panel or any speedup.

## 4. Method
Pure stdlib Python, seed 20261110, one core. For each of three primes (starting search at 40009, 52837, 65521) draw a random short Weierstrass curve with prime order N (point count by quadratic-residue table). Build the full discrete-log table of E(F_p) from a random generator (N affine additions). Factor base = all points with x < B, B minimal with >= 40 points, closed under negation (f ~ 40 logs in Z_N). Sumsets S_m = {s + g : s in S_{m-1}, g in F} in Z_N for m = 2..4 (sums of exactly m elements, repetition allowed). Nulls, A/A and positive control as in section 3. Cost estimate: 3 curves x (about 10 sumset chains of up to 4e6 integer ops) well under 5 minutes, under 100 MB. `--smoke` uses p ~ 1031, a 12-point base, 3 null draws (< 1 s).

## 5. Classification: runnable_local
The 63-bit solver-to-certificate panel is blocked (needs launcher admission, RSS monitor repair, long solves). A stdlib toy of its generic-coverage premise can decide whether small-x bases show non-generic sumset coverage, so the lead is classed runnable_local for this sharpened question.

## 6. What a run cannot establish
- Nothing about 50-63-bit solves, linear algebra, certificates, rho matching, or the GAE-priced comparison of section 5a; no speedup claim.
- Sumset coverage with repetition and no sign/ordering weights is only a proxy for relation yield; coset, Koblitz-orbit and binary bases are not tested.
- Three small curves of prime order only; a 5% threshold is a preregistered choice, not a derived bound. Results will be labelled [pcc:unverified].
