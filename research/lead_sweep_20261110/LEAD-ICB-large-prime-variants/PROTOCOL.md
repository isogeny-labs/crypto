# LEAD-ICB-large-prime-variants

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-29). Family: ic-binary.
- Title: "Single and double large-prime relation collection for binary and Koblitz IC (free-leg, GTTD on subspace bases)".
- Next-step excerpt (truncated): "Combine the double-large-prime collector with the aimed m = 3 collection of the Table A/C recipe".
- Files read: research/lead_sweep_20261110/leads.json (entry); research/prime_field_lead_assessment_20261107/atlas_survey.md (line 212, one-line listing only);
  docs/session-logs/2026-10-29.md lines 1320-1355 (ICB large-prime ladder at n = 53, 61).
- Recorded facts (unverified assertions here, [pcc:unverified]): the 1LP/2LP collector exists on main; at n = 53 and 61, 2LP beat the best plain
  column count by 0.604 and 0.559 (paired, counted unit), 1LP was worse than plain (1.47-1.73x); rho was ahead 30-65x. The aimed m = 3 recipe was
  NOT combined with large primes.

## 2. Hypothesis
Large-prime gain composes with aimed (structured) sampling: the 2LP-over-plain gain in raw samples per usable relation under an aimed sampler is at
least 0.8 times the gain under an unaimed sampler (ratio >= 0.8), and the aimed 2LP gain is itself > 1.2. Falsified if aimed sampling eats the large-prime benefit.
Toy stand-in (stdlib only): the engine is not available, so the mechanism is modelled on integer smoothness with a cycle-counting graph (vertex 1 for
full relations, large primes as vertices), which is the same accounting as the double-large-prime collector.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = G_aimed / G_unaimed, G = mean raw samples (plain) / mean raw samples (2LP) to reach K+10 usable relations, 20 seeds per cell.
- Prediction: R >= 0.8 and G_aimed > 1.2 -> "consistent_with_prediction" (composition plausible; motivates the real combined run).
  R < 0.8 or G_aimed <= 1.2 -> "falsified" (aimed collection already harvests most of what large primes give; combined step deprioritised).
- Negative control / null: 2LP with large-prime bound L = B (no large primes admitted): G must equal 1 exactly in both samplers.
- Positive control: unaimed 2LP gain G_unaimed > 1.2 (detector sees the known effect).
- A/A arm: same arm with disjoint seed set; relative deviation of mean cost must be < 0.15 for plain and for 2LP (unaimed).
- If any control fails the verdict is "inconclusive". 1LP is also reported (recorded finding: 1LP can be worse than plain in the real engine; the toy ignores
  factoring cost, so it cannot reproduce that).

## 4. Method
- Universe: integers in [2, 2^22] (aimed: uniform multiples of 30 in that range). Smallest-prime-factor sieve gives exact factorisation.
- Factor base: primes <= B = 200 (K = 46). Large-prime bound L = B*C, C = 50. Relation kinds: full (all primes <= B), 1LP (one prime in (B, L]), 2LP (two).
- Usable relations by union-find: full relation always usable; a large-prime edge (p-1 or p-q) is usable iff it closes a cycle. Stop at K+10 usable.
- Seeds: arm A seeds 1000..1019, A/A seeds 5000..5019, random.Random. Arm order is fixed, no state shared across arms.
- Cost: sieve a few seconds, collection well under 1 minute total; memory about 20 MB. Smoke: 2^16, B = 60, 3 reps, under 1 s.

## 5. Classification
runnable_local. A stdlib toy decides the composition question for the cost-per-relation model.

## 6. What a run cannot establish
Nothing about binary/Koblitz curves, the Semaev/GTTD decomposition, factoring cost of cofactors, linear algebra growth from large-prime merging, descent,
or the matched rho gap (rho stays far ahead). Aimed sampling here is a divisibility bias, not the m = 3 Table A/C recipe. A positive result does not
justify a leaderboard row; a negative one does not close the real combined step. Results are [pcc:unverified].
