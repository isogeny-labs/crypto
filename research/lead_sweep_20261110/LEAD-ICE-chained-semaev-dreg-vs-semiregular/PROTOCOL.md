# LEAD-ICE-chained-semaev-dreg-vs-semiregular

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-engine. Source: atlas_survey_20261107 --pursuit.
- Title: Degree of regularity of the Boolean chained Semaev system (m = 3, t = 3) against a semi-regular null.
- Next step (truncated excerpt): "Re-run item C1 (n = 15, D = 6, seeds as frozen) with a streaming native Macaulay builder (IDEA-2..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 43-49, 215-223; the lead is only listed, as "mostly binary (GF(2) descent) engine leads");
  /home/user/crypto/docs/session-logs/2026-11-06.md (line 859: "re-run C1 n=15 D=6 with a streaming Macaulay builder").
  Not read: research/ecdlp_subexp_desk_20261101/data/atlas_tally.json; the C1 frozen spec and IDEA-2... are not in the hits, so C1 details are unknown here.
  No cryptanalysis-repo search was made.

## 2. Hypothesis
For a toy binary-field chained Semaev system (two S_3 summation polynomials sharing an intermediate full-field variable u, Weil-descent to bits, factor-base variables restricted to l free bits), the rank of the degree-D Macaulay matrix (multilinear ring, D = 5) is lower than that of a random cubic system of the same shape (N variables, M bit-equations) by at least 0.02 in rank/semi-regular-predicted-rank, i.e. the chained structure falls below the semi-regular yardstick (earlier degree fall).

## 3. Preregistered prediction and falsification criterion
- Statistic: mean over K = 14 instances of r = rank(M_D) / pred, pred = sum_{k<=D} C(N,k) minus the partial sum of [z^k] (1+z)^N/(1+z^3)^M. Statistic at D = 5 (D = 3, 4 reported).
- diff = mean r(null) - mean r(chain). thr = max(0.02, 3*|mean r(aa1) - mean r(aa2)|).
- consistent_with_prediction: diff > thr AND positive-control effect > thr. falsified: diff <= A/A noise (and positive control passes). Otherwise inconclusive (including positive control failing, which means the detector is blind).
- Null/negative control: random dense cubic systems (same N, M, top degree), the semi-regular surrogate. Positive control: null system whose last equation is replaced by (first equation x a random linear form), a planted syzygy. A/A arm: two independent null batches (seeds 2000+, 3000+).
- Decides: consistent means the toy shows chained structure beating the semi-regular yardstick, a reason to run C1 natively; falsified means no toy-scale deviation. Neither is an ECDLP cost claim.

## 4. Method
- GF(2^6), modulus x^6+x+1 (0b1000011). n=6, l=2: variables x1,x2,x3 (2 free bits each), u (6 bits): N = 12; equations S3(x1,x2,u)=b-shifted and S3(u,x3,R)=0, S3 = (x1x2+x1x3+x2x3)^2 + x1x2x3 + b, expanded symbolically to bit-polynomials in the multilinear ring; b, R random nonzero from per-instance seed.
- Macaulay matrix degrees 3,4,5: rows = monomial multiples of each equation up to degree D, rank by python-int bitset elimination. Seeds: chain 1000..1013, aa1 2000.., aa2 3000.., pos 4000..; the null arm reuses the chain seeds for shape.
- Cost: roughly N=12, D=5 gives 1586 columns and under 1000 rows per instance, estimated well under 10 minutes, under 200 MB. `python3 -I run.py --smoke` uses n=4, l=1, 2 instances.

## 5. Classification: runnable_local
Stdlib toy decides or sharpens the question at toy scale; the n=15, D=6 native run itself is not required here.

## 6. What a run cannot establish
- Not the C1 item (n=15, D=6, frozen seeds): different n, D, and a custom m=3,t=3 chain.
- Rank ratio vs a Hilbert-series heuristic for non-homogeneous systems is a proxy for degree of regularity, not the solving degree; random-cubic null is not exactly semi-regular.
- Systems are random instances, mostly inconsistent or without planted solution; the count of inconsistent chain instances is reported.
- Nothing about prime fields or ECDLP cost versus rho. All results are [pcc:unverified].
