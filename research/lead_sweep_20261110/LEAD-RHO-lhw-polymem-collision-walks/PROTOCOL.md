# LEAD-RHO-lhw-polymem-collision-walks

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24; source atlas_survey_20261107 --pursuit). Family: generic.
- Title: "Low-Hamming-weight DLP with polynomial memory: collision walks on the weight-w class".
- Next-step excerpt (truncated in leads.json): "Re-run EXP-LHW-b49bc6 Stage 1 (hash-realisable jump sums on the frozen (m, w) ladder, m in {20,"
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 67, 253: lists it among
  "LHW low-weight leads ... not generic-key leads"). /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json
  matched but was not read. No cryptanalysis hit searched beyond this; EXP-LHW-b49bc6's frozen ladder is not available here,
  so the ladder below is my own toy stand-in, not the frozen one.

## 2. Hypothesis
For a secret k of m bits with Hamming weight w, split into low/high halves of weight w/2 each, a memory-O(1)
(Floyd cycle) collision walk over a hash-selected jump set realising g^a (low-half masks) and T*g^-b (high-half
masks) recovers k with expected cost c*sqrt(S) walk evaluations, S = C(m/2, w/2), with constant c in [2, 40]
(Floyd + golden-collision restarts overhead) and a log-log slope of cost vs S in [0.35, 0.65]; i.e. cost scales as
sqrt(S), independent of the group order q (sqrt(q) >> sqrt(S) at every rung).

## 3. Preregistered prediction and falsification criterion
- Statistic: per rung, mean f-evaluations over 240 planted instances (arms A and B pooled); c = mean/sqrt(S);
  slope = least-squares slope of ln(mean) on ln(S) over the 4 rungs.
- Consistent: all rungs c in [2,40] AND slope in [0.35,0.65], with all controls passing.
- Falsified: controls pass but c leaves [2,40] at any rung or slope is outside [0.35,0.65].
- Inconclusive: any control fails (positive-control solve failures, null hit, A/A gap), or smoke mode.
- Negative control: target with k' whose low half has weight w/2+2 (outside the class); must yield 0 verified
  solutions in 20 restarts per instance (20 instances per rung).
- Positive control: every planted instance must be solved and the recovered k verified by g^k == T (0 failures allowed).
- A/A arm: two disjoint seed sets (101,102 vs 201,202) with identical code; relative difference of rung means < 0.35.
- Decides: consistent = the poly-memory collision walk has the sqrt(C(m/2,w/2)) toy scaling the lead assumes (supports
  re-running Stage 1); falsified = the mechanism's cost model is wrong at toy scale.

## 4. Method
- Group: order-q subgroup of Z_P^*, P = 2q+1 safe prime, q the first safe-prime-compatible prime >= 2^23, g = 4.
- Ladder (m,w): (16,4), (18,6), (20,6), (22,8); S = 28, 84, 120, 330. 2^m < q asserted.
- Instance: k = a + (b << m/2), a,b uniform weight-w/2 masks; T = g^k.
- Walk f(z): h = mix64(z xor salt); type t = h&1, index i = (h>>1) mod S; f = g^lo[i] (t=0) or T*g^-(lo[i]<<m/2) (t=1).
  Tables of size S are precomputed (the hash-realisable jump sums); memory is O(S), walk memory O(1).
- Collision: Floyd cycle detection from random x0, locate the two distinct predecessors of the merge point; golden iff
  types differ and g^(a+b) == T. Else restart with a fresh salt (max 60 restarts; 20 for null).
- Seeds: arms 101,102 / 201,202 (120 trials per seed per rung), null 301.
- Cost: pure-Python small ints; estimated under 1 minute total, <20 MB RAM, one core. Full run is not executed in phase 1.

## 5. Classification
runnable_local: a stdlib toy decides the scaling question.

## 6. What a run cannot establish
- Only toy sizes (S <= 330, q ~ 2^23); slope fit over four points is coarse, and Floyd's constant is a toy artifact.
- Only the balanced split (w/2, w/2) is planted; the unknown-split search (extra factor over splits) is not tested.
- It does not reproduce EXP-LHW-b49bc6 or its frozen ladder, says nothing about GAE pricing vs matched rho (section 5a),
  and cannot show an improvement over the index-calculus baseline. Tables cost O(S) memory. Results are [pcc:unverified].
