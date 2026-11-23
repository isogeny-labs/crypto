# LEAD-ICP-representation-subset-sum

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (theorem-kind; last_reviewed 2026-10-29). Family: ic-prime.
- Title: Representation technique (Howgrave-Graham-Joux style) on the base-B digit subset-sum form of ECDLP.
- next_step_excerpt: empty in leads.json. From the survey: "generic, Shoup T>~sqrt N, S T^2>~N; filter has no non-generic handle; reopen with filter predicate reading coordinate structure with measured bias".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 33-35, 135-149);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (line 55);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 152).
  The cryptanalysis hits were atlas input JSON only (not read).

## 2. Hypothesis
Any representation-style subset-sum algorithm that touches the group only through addition and equality
(including hash filters on element labels) needs T >= c*sqrt(N) group operations (c >= 0.5) on a cyclic
group of order N. Its measured log-log slope of T against N is >= 0.40, no better than BSGS or rho.
The recorded obstruction is that the filter has no handle on coordinate structure.

## 3. Preregistered prediction and falsification criterion
- Statistic: per arm, slope of log2(mean ops) vs log2 N (N = prime near 2^k, k = 12,14,...,22; 60 trials per size;
  bootstrap 95% CI over trials), and min over arms and sizes of mean ops / sqrt(N).
- Prediction: all three generic arms (rho, BSGS, representation two-list sampler) have slope >= 0.40
  and min ops/sqrt(N) >= 0.5.
- Falsified if any generic arm has CI upper bound < 0.45 or min ops/sqrt(N) < 0.3 (a sub-sqrt generic method).
  That would contradict the Shoup bound and indicate an accounting bug or a missing coordinate leak.
- Consistent if neither holds and the controls are valid.
- Positive control: Pohlig-Hellman on a group of order 2^k reading coordinate structure. It must show CI upper
  bound < 0.45, otherwise the detector cannot see sub-sqrt scaling and the verdict is inconclusive.
- Negative control / null: the generic arms themselves, with the hidden d uniform; success rate must be >= 0.95.
- A/A arm: rho with two seed sets (11, 22); slope difference must be < 0.1, else inconclusive.

## 4. Method
Black-box model: Z_N additive, generator 1, target Q = d. Algorithms use only addition (each counted as 1 op)
and equality/dictionary lookup (free). Arms: Floyd rho with 16-way partition on label; BSGS with m = ceil(sqrt N);
representation arm: d = x + y, two independent additive random-walk lists (16 random steps, 1 op per sample),
collision of xP with Q - yP among the N representations. Seeds 11, 22 (rho), 11 (others), 33 (PH);
per-size seeds derived as seed*1000+k. Cost: a few seconds, <20 MB, stdlib only.
The sampler is the unfiltered limit of the HGJ list construction; a hash filter on labels only
reduces list storage, not the number of group operations needed to generate candidates.

## 5. Classification
runnable_local: a stdlib toy re-derives the recorded generic bound and sharpens it with a falsifiable scaling test.

## 6. What a run cannot establish
- It is a generic-model toy on Z_N (labels equal values), not elliptic curves; it proves no lower bound.
- It does not test filters that read coordinates (the only recorded reopen route), nor HGJ's actual weighted digit lists.
- Slopes at N <= 2^22 carry finite-size effects (rho constants are large), so a slope above 0.5 is expected.
- Results are [pcc:unverified].
