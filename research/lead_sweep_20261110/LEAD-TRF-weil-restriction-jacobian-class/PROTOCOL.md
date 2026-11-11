# LEAD-TRF-weil-restriction-jacobian-class

## 1. Lead as recorded
- disposition: open_next_step (last_reviewed 2026-10-24); family: transfer; source: atlas_survey_20261107 --pursuit.
- title: Genus-n Jacobians in the isogeny class of a Weil-restricted elliptic curve.
- next-step excerpt (truncated in leads.json): "Design and run the proposed n=2 control and exhaustive n=3 point-count census for q=2,3, with fi..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-62, 272-274: lists the lead
  among extension-field/Jacobian transfers that "apply to F_{p^k} curves, not E(F_p)");
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id/title only).
  No lead-specific note was found; the design below is from the recorded fields and the standard mechanism.
  The lead atlas YAML and research-kb were not consulted (encrypted / not available here).

## 2. Hypothesis
Let E/F_{q^n} have Frobenius trace t. Res E has Frobenius polynomial x^(2n) - t x^n + q^n, so a genus-n curve C/F_q has
Jac(C) isogenous to Res E iff N_k(C)=q^k+1 for k<n with s_1=...=s_(n-1)=0 and N_n = q^n+1-n t (Tate).
Claim H: for q=3, among ALL hyperelliptic genus-3 curves y^2=f(x), deg f in {7,8}, f squarefree over F_3 (exhaustive,
4374+13122 coefficient vectors before the squarefree filter), at least one has N_1=4, N_2=10 and N_3=28-3t with t an
admissible trace for E/F_27 (Waterhouse), i.e. a hyperelliptic Jacobian lies in a Weil-restriction class. Control
claim: the same test for n=2 (genus 2, N_1=4, N_2=10-2t, t admissible over F_9) has >=1 hit.

## 3. Preregistered prediction and falsification criterion
- Statistic: H = number of (f) vectors passing the n=3 criterion; C = number passing the n=2 criterion.
- Prediction: C >= 1 and H >= 1. Rough expectation: P(s1=s2=0) ~ 1e-2..1e-3 per curve over ~1.2e4 squarefree f gives H of
  order 10-100 (heuristic only, not preregistered as a number).
- Decisions: C=0, or any A/A mismatch, or any Hasse-Weil violation => inconclusive (instrument broken, no claim).
  C>=1 and H>=1 => consistent_with_prediction (hyperelliptic genus-3 Jacobians exist in the class at q=3).
  C>=1 and H=0 => falsified for the hyperelliptic locus only (non-hyperelliptic plane quartics untested).
- Negative control / null: curves with N_1=q+2 (not of Weil-restriction form, N_2 left at q^2+1 for n=3) are counted and
  reported (null_hits); they must not be used to decide, they show the filter is selective.
- Positive control: the n=2 genus-2 census (Weil restrictions of E/F_9 are known to contain Jacobians); C>=1 required.
- A/A arm: 300 random f recounted with a second independent method (explicit y enumeration over F_3 vs the log-table
  character count); mismatches must be 0. Also Hasse-Weil bound check on every N_k.

## 4. Method
Pure Python 3 stdlib. Build GF(3^k), k=1,2,3, via a searched primitive polynomial with exp/log and addition tables.
For each f (leading coeff 1 or 2, deg 5,6 for n=2; deg 7,8 for n=3, all tails in F_3^deg) test squarefree by gcd(f,f')
over F_3; count points as sum over x in F_{3^k} of 1+chi(f(x)) plus the points at infinity (1 if deg odd, 1+chi(lead) if
even; lead in F_3). Seed 20261110 (only the A/A sample is random; the census is deterministic and exhaustive).
Cost: about 2e4 curves x 39 evaluations x 8 Horner steps, estimated < 30 s, memory < 10 MB. q=2 is NOT run
(needs y^2+hy=f smoothness testing and non-hyperelliptic genus 3 needs plane quartics; both outside this protocol).
Command: `python3 -I run.py` (full) or `--smoke` (n=2 control on deg 5 only, A/A on 30 curves).

## 5. Classification: runnable_local
A stdlib exhaustive toy decides the q=3 hyperelliptic part; no GPU, Groebner engine or build needed.

## 6. What a run cannot establish
- Nothing about q=2 or non-hyperelliptic (plane quartic) genus-3 curves; a falsified verdict says only that the
  hyperelliptic locus misses the class at q=3.
- Matching N_1..N_3 gives equality of L-polynomials; existence of a Jacobian in the class is thereby established (Tate)
  but says nothing about ECDLP cost: transfer to Jac(C) needs a cover/GHS-type map from the original E(F_{q^n}), and
  the atlas records same-field genus-g embeddings as costing >= q (IC O~(q^(2-2/g))). No speedup or rho gap is shown.
- Toy field q=3 only; no extrapolation to cryptographic sizes. Results are labelled [pcc:unverified].
