# LEAD-RHO-negation-map-fruitless-cycles
## 1. Lead as recorded
- disposition: closed_known (last_reviewed 2026-10-24); family: generic; source atlas_survey_20261107 --pursuit.
- title: Negation-map rho on {+-P} classes with fruitless-cycle escape (prime curves).
- next-step excerpt (truncated): "Revisit when: A medium-tier (log2 r >= 32) prime rho.negation bound shows a gain over rho.plain".
- Recorded numbers (atlas_survey.md l.66): rho.negation 1.453 [1.184,1.772] x sqrt(pi/4) vs rho.plain 1.920 [1.633,2.221], ratio 0.757 vs 0.707 ideal, log2 r 15.7-25.3.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 63-67, 317-323);
  /home/user/crypto/research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md (lines 1865-1996, grep context);
  /home/user/crypto/research/notes/index-calculus/round15/reports/run-20261024/report.typ (lines 17-21, grep context).
## 2. Hypothesis
On toy prime-order prime-field curves (group order 2^18..2^20), a-adding rho on +-classes (16-way partition by x) with fruitless-cycle escape (on a trivial same-class revisit, double the point) needs mean steps to a verified discrete-log collision of 0.707x plain rho (1/sqrt2) with a bootstrap 95% CI inside [0.62, 0.85]; without escape, a nonzero fraction of walks dies on a fruitless 2-cycle.
## 3. Preregistered prediction and falsification criterion
- Statistic: ratio of pooled means of steps/sqrt(n), neg_escape over plain, sizes with bits>=18; 95% bootstrap CI (400 resamples).
- consistent_with_prediction: CI within [0.62, 0.85]. falsified: CI entirely above 0.85 or below 0.62 (negation gain absent or implausibly large at toy size). Otherwise inconclusive.
- Validity gates (else inconclusive): A/A arm (plain vs plain, independent derived seeds) ratio CI contains 1; positive control = ideal birthday on n/2 vs n states gives 0.707 +-0.12 (shows the detector sees a sqrt2 effect).
- Negative control / null: plain vs plain (A/A); neg_noescape arm reports fail rate (expected > 0).
- Every collision is checked by recovering x against the known key; failures are counted, not dropped silently.
## 4. Method
Curves y^2=x^3+ax+b over random primes p ~ 2^14,2^16,2^18,2^20 (p%3==1), prime group order n by naive point count (squares table), random generator, Q=xG. Walk: 16 adding steps with random (u,v), collision dict on x (negation arms, canonical y<=p/2, coefficients negated with sign) or on (x,y) (plain). Arms interleaved per run with seeds derived from f"{20261110}-{bits}-{i}-{arm}". Runs per size 600/500/400/250. Stdlib only, one core; expected a few minutes (inversions via pow(-1)); dominated by point counting at 2^20 and ~2.5k steps x 1000 walks x 4 arms. Cap 60 sqrt(n)+2000 steps.
## 5. Classification
runnable_local: a stdlib toy decides whether the recorded 0.757 ratio (and the 1.453/1.920 excess) is reproduced at group orders 2^18-2^20, a cheap re-derivation of the closed lead's recorded gain. Does not touch the lead's actual reopen condition (log2 r >= 32).
## 6. What a run cannot establish
Nothing at log2 r >= 32, nothing about counted GAE cost, matched-rho band (section 5a), replay certificates or the validation gate. Toy-size small-group effects (cycle penalties) may bias the ratio. Single partition size r=16, no distinguished points, no inversion cost pricing. Results are [pcc:unverified]; reproduction of a known result, not an improvement.
