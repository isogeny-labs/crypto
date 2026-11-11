# LEAD-TRF-ghs-binary-composite

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (reviewed 2026-10-29; 20261101 also closed_known). Family: transfer.
- Title: GHS / generalized GHS Weil descent (with Galbraith-Hess-Smart isogeny walks) for curves over F_{2^N}, N composite.
- Next-step excerpt (truncated in leads.json): "Revisit when: an end-to-end measured row is wanted for Oakley Group 3/4 or c2pnb176w1 (needs a g..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (line 327 only, a one-line tally). Other hits (atlas_tally.json, candidate_matches.json) contain only the id/title. No lead note body was found; the rest is from general knowledge of GHS (Gaudry-Hess-Smart 2002) and is [pcc:unverified].

## 2. Hypothesis
For a generic b in F_{2^N}, N = l*n, q = 2^l, the F_q-span of the Galois conjugates b^{q^i} has full dimension n with probability equal to the exact normal-element density prod_P (1 - q^{-deg P}) over the irreducible factors P of x^n - 1 over F_q (n odd). Hence the GHS cover has the generic maximal size (genus ~2^{n-1}), and the recorded obstruction (closed_known: no generic composite-N binary curve gains over rho except small n) is the genus blow-up, not a rare-event artefact. This is a proxy for the GHS parameter m, which this toy does not compute exactly.

## 3. Preregistered prediction and falsification criterion
- Statistic: per (l,n), the fraction of T=400 random b with F_q-span dimension n, averaged over arms A and B, against the exact normal density (odd n only).
- Threshold: |mean(A,B) - density| < 4*sqrt(0.25/T) + 0.02 for every odd-n config. Consistent: all within. Falsified: any odd-n config outside, with controls passing. Inconclusive: any control fails.
- Positive control: b chosen in F_q must give span dimension 1 (20/20, both arms).
- Null / negative control: uniformly random b is the null; the theoretical density is the independent reference.
- A/A arm: arm B uses an independent random modulus polynomial and seed; |fA - fB| < 2*tolerance.
- Secondary (reported, not decisive): an asymptotic proxy table (log2 rho = N/2 versus 2l + log2(g!) with g = 2^{n-1}) for N in {155,162,176,185,210}; ic_below_rho should be false for all n>=4. This proxy is an assumption, not a measured cost.

## 4. Method
Pure stdlib. GF(2^N) as F_2[x]/f with a random irreducible f (Rabin-type gcd test); F_q basis from powers of w^((2^N-1)/(q-1)) (rank l verified). Conjugates by q-power; F_q-span dimension = F_2-rank of {g*c_i}/l. Configs (l,n): (3,4),(4,3),(2,6),(5,3),(3,5),(4,4),(6,2); N<=20. Seeds: 1000+10l+n (A), 5000+10l+n (B). Cost: seconds to well under a minute, memory a few MB.

## 5. Classification
runnable_local. A stdlib toy decides the proxy statistic; the full GHS cover and index calculus need Magma/Sage-class tools, which are absent.

## 6. What a run cannot establish
It does not compute the true GHS m or genus, does not construct covers, does not run index calculus, and says nothing about Oakley 3/4 or c2pnb176w1 end-to-end rows or isogeny-walk gains. The cost table is a crude proxy. Even-n configs have no closed-form comparison here. All results are [pcc:unverified].
