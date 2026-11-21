# LEAD-EXO-holant-decomposition-count-network

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_dominated. Family: exotic. Title: empty in leads.json. Source: atlas_survey_20261107 --pursuit.
- Next-step excerpt (truncated): "Revisit when: A contraction model that is not a tensor train over F_p digits (e.g. a contraction ..."
- Recorded obstruction (atlas_survey.md lines 74, 154): digit-QTT rank of the x-labelled Semaev transfer matrix M at p=1021 is 1024 (ceiling), random relabelling 1023-1024, DL-labelling 109-121 (a structure statistic, not a cost); invertible bond basis changes cannot lower flattening/CP ranks.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context only, lines 68-80, 150-190, 300-320). The cryptanalysis repo was searched; hits there were not read beyond that. The lead's own note file was not found (only the survey mentions the id).

## 2. Hypothesis
For a toy cyclic prime-field curve with factor base FB = {x < p/4}, the count matrix M[x1][x2] = 1 iff x(P1+P2) or x(P1-P2) lies in FB (P_i any point with x-coordinate x_i) has bond-flattening rank, over base-2 digit pairs of the x label, indistinguishable from a random relabelling of the labels (|difference| <= 0.03 of ceiling) and at least 0.85 of ceiling at p in {251,509,1021}, while the DL labelling has rank <= 0.35 of the x-label rank at p=1021. That is: the recorded closure (tensor-train structure over x-digits gives no compression) re-derives on independent curves and FB.

## 3. Preregistered prediction and falsification criterion
- Statistic: s = max over the 3 middle bonds of rank_GF(2)(flattening) / min(4^k, 4^(n-k), #ones). Computed for x-label, 4 random relabellings, DL label.
- Prediction: for p in {251,509,1021}: s_x >= 0.85; |s_x - mean(s_rand)| <= 0.03; DL s <= 0.35 * s_x at p=1021.
- Falsified: s_x <= 0.8 * mean(s_rand) at any p in {251,509,1021} (digit structure exists in x-labelling; would reopen the lead). Consistent: all three prediction clauses hold and controls pass. Otherwise inconclusive.
- Null / negative control: random relabellings (injective maps into 2^n labels, seeds derived from p and t). A/A arm: spread (max - min) among the 4 random relabellings must be <= 0.03, else the instrument is too noisy and the verdict is inconclusive.
- Positive control: a planted matrix with exact digit structure (parity of top digits xor low digit, rank 2 in every bond) must read rank <= 4 in digit order and >= 3x that after a random permutation. Failure makes the verdict inconclusive. The DL labelling is a second, in-lead positive control (known to compress per the record).
- What it decides: consistent keeps the lead closed_dominated for the digit-tensor-train model; falsified reopens it.

## 4. Method
- Primes p = 127, 251, 509, 1021 (127 is a sanity row, excluded from the decision). Global seed 20261110; per-p RNG = seed*1000003+p chooses (a,b) and generator G until E(F_p) is cyclic with G of order N (brute-force point count and orbit walk). x(kG) table xs[k].
- Pairs (x1,x2) over x-coordinates of points: M=1 iff FB-membership of xs[k1+k2] or xs[k1-k2] (ks are any DL of x1,x2; the OR over signs makes this well defined). Labels: x (identity), random injection into [0,2^n) with n = bits of p-1, DL label min(k,N-k).
- Flattening at bond k: row = top k digit pairs of (i,j), column = remaining; rank over GF(2) by Python-int elimination. Bonds n/2-1, n/2, n/2+1.
- Cost estimate: about 1M pair lookups at p=1021, 6 rank computations per p over at most 1024x1024 bit matrices; expected well under 2 minutes, less than 50 MB RAM.
- Command: `python3 -I run.py` prints one JSON object; `--smoke` uses p=61.

## 5. Classification
runnable_local. A stdlib toy decides the question directly (re-derivation of the recorded rank ceiling on independent curves, with nulls).

## 6. What a run cannot establish
- GF(2) rank lower-bounds rank over R/Q/F_p only in the sense rank_GF2 <= rank_R for 0/1 matrices: a high GF(2) rank implies high real rank, but low GF(2) rank would not imply low real rank (the DL clause is therefore weak evidence only).
- The toy tests the specific model (base-2 digit tensor train, Semaev-2 matrix, FB = x<p/4) at p <= 1021. It says nothing about other contraction models (the lead's "revisit when" clause), larger p, higher summation arity, or the actual cost of a solver; DL-label rank is a structure statistic, not a cost.
- Rank at ceiling is a statement about exactness; approximate (truncated) low-rank compression is not tested.
- Single curve per p; curve-to-curve variation not sampled. Results are labelled [pcc:unverified].
