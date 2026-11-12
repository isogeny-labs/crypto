# LEAD-EXO-elliptic-code-list-decoding

## 1. Lead as recorded
- Disposition: open_next_step. Family: exotic. Title: (empty in leads.json). Source: atlas_survey_20261107 --pursuit.
- Next-step excerpt (truncated): "Paper: the counting bound C(B,M) p^(k-M-1) >= 1 with M = sqrt(Bk), for B = p^beta and beta < 1."
- Sibling lead ICP-elliptic-code-list-decoding: AG-code min-weight codewords = k-subsets of the factor base summing to a prescribed point (Cheng cs/0507026; Zhang-Liu ePrint 2018/795). Desk heuristic H1: Guruswami-Sudan needs agreement ~sqrt(Bk) > k when B > k, so it cannot reach weight-k words; atlas expectation "reduces to enumeration".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 28-110 region). No hits in cryptanalysis.

## 2. Hypothesis
At toy sizes (|E(F_p)| = n prime, 2^11..2^14), the number of k-subsets (k=2,3) of a random B-point factor base summing to a uniformly random target has mean C(B,k)/n with Poisson-like dispersion (no excess structure a decoder could exploit), and the GS-style agreement M = ceil(sqrt(Bk)) exceeds k for every cell with B > k, so list decoding around a received word cannot return weight-k words: the lead reduces to enumeration.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio = (observed mean count over random targets) / (C(B,k)/n); also A/A ratio on a disjoint target set; dispersion = variance/mean; planted minimum count.
- Prediction: ratio and A/A ratio in [0.7, 1.3] in every cell; M_gs > k in all cells.
- Falsified: any cell ratio > 1.5 (excess weight-k words beyond the generic count). Inconclusive: positive control fails or ratios fall in (1.3,1.5] or < 0.7.
- Negative control / null: uniformly random targets (this is the null the ratio tests). Positive control: planted targets (sum of k FB points) must have count >= 1 and the scalar model must agree with real EC addition. A/A: two disjoint random-target sets, same seed family.
- Decides: consistent means the lead's recorded obstruction (enumeration in disguise) is supported at toy scale; falsified would reopen it.

## 4. Method
Stdlib only. For each cell (pbits, B, k, T) in {(12,32,2,600),(12,64,2,600),(12,32,3,400),(14,48,2,400),(14,64,3,300)}: random prime p near 2^pbits, naive-count a prime-order curve, FB = {s_i P} with distinct random scalars, count k-subsets with sum = target via dictionaries (k=2 hash, k=3 pair-sum table). Seeds: 20261110 + 17*cell_index; targets use seed xor 0xA5A5. Also evaluates ln of the counting bound C(B,M)p^(k-M-1) (reported, not decisive). Cost: curve search ~ seconds, counting < 1 min total; RAM < 50 MB.

## 5. Classification
runnable_local: a stdlib toy decides the counting/enumeration claim at small size.

## 6. What a run cannot establish
It does not test Guruswami-Sudan decoding itself (no AG code decoder is implemented), only the counting statistic and the M > k inequality; toy n is far below cryptographic size; the Zhang-Liu paper text was not read; scalar-index arithmetic is isomorphic to the group, so structure beyond group addition (e.g. coordinates) is not probed. Results are [pcc:unverified].
