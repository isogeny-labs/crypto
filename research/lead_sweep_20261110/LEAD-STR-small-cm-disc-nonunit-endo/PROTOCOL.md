# LEAD-STR-small-cm-disc-nonunit-endo

## 1. Lead as recorded
- Disposition: open_next_step (2026-11-01 survey; disposition_20261101 closed_obstructed; last_reviewed 2026-10-24). Family: structure.
- Title: Small |D_0| or a cheap non-unit endomorphism beyond the automorphism constant.
- Next step (truncated in leads.json): "Someone constructs a prime-field curve where a norm-2 or norm-3 endomorphism has eigenvalue orde..." (atlas survey completes it: eigenvalue order near log_n N and canonical representative cost o(h)). Cost: cheap (measure equality case). Why it might win: net factor sqrt(h)/kappa only if canonical rep costs o(h).
- Recorded: orbit-size bound h >= log N / log n (elementary); net factor <= sqrt(h)/kappa, kappa ~ h c(alpha); the net-factor clause is a cost-model estimate never measured at the equality case.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 53, 54); research/ecdlp_subexp_desk_20261101/README.md (lines 58-60, 103). Not read: research-kb (not run in this phase).

## 2. Hypothesis
For alpha = a+b*sqrt(-d) in Z[sqrt(-d)] of norm n (2<=n<=8), lambda a root of x^2-2a x+n mod prime N: (i) h=ord(lambda) satisfies h >= 2 log(sqrt(N)-1)/log n always (since N | Norm(alpha^h-1) <= (n^(h/2)+1)^2); (ii) the equality regime (h within a small factor of log_n N) is constructible, but only for sparse N; (iii) in that regime, charging canonicalisation at h-1 endomorphism applications (c=1, no o(h) shortcut) gives net factor sqrt(h)/h <= 0.71 for every h>=2, so no gain over plain rho.

## 3. Preregistered prediction and falsification criterion
Statistics: (S1) count of (alpha,N) pairs with h below the bound; (S2) min over positive-control pairs of h/log_n N; (S3) max net = sqrt(h)/h over positive-control pairs with h>=2; (S4) median h/log_n N over random N versus the control.
- Prediction: S1 = 0; S2 in [1,3]; S3 <= 0.71; S4 median > 20 x S2.
- Falsified if S1 > 0 (the recorded bound or this derivation is wrong). Inconclusive if S1 = 0 but the positive control is empty or S4 fails (generic h not large, so the detector could not discriminate).
- Consistent: all four hold. This decides only that, with c=1 charging, the cost-model clause fails at equality; it does not test an o(h) canonical-rep algorithm.
- Negative control / null: random lambda in (Z/N)^*, median order ratio ~ N/log N scale. Positive control: primes N dividing Norm(alpha^h-1) for h=2..13 (forces lambda^h=1). A/A: not applicable, exact integer arithmetic, deterministic, no noise.

## 4. Method
Python stdlib. Seed 20261110. Elements: d in squarefree-ish 1..40, b 1..5, a 0..7, norm 2..8. Primes: 400 random odd primes in [2^12, 2^22], all elements with a root mod N; order via factoring N-1 (trial division). Positive control: factor Norm(alpha^h-1) (< 1e12 only) for 30 elements, h 2..13, keep primes p>256 with measured order <= h. Cost estimate: well under 1 min, <20 MB.

## 5. Classification
runnable_local: pure number-theoretic toy decides the bound and the c=1 net-factor arithmetic with stdlib. `python3 -I run.py --smoke` passed (verdict consistent_with_prediction at trivial size).

## 6. What a run cannot establish
- It uses only the ring-level eigenvalue order, not actual curves or point arithmetic; no prime-field curve with these endomorphisms and prime order N is constructed (CM discriminant, order N prime and p are not matched).
- Net factor uses an assumed unit cost per endomorphism application; real kappa/c(alpha) (isogeny evaluation degree n cost) is not measured, and an o(h) canonicalisation is not excluded, only unsupported.
- No rho comparison or section 5a pricing band; no replay certificate; labelled [pcc:unverified].
