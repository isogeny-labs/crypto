# LEAD-ICB-generic-table-oracle

## 1. Lead as recorded
- Disposition: closed_obstructed (atlas_tally_20261101; last reviewed 2026-10-30). Family: ic-binary.
- Title: "Generic decomposition oracles for Koblitz IC: pair tables, compact-orbit 4-sum, MITM and enumeration, cap-locus arity, known-log pools".
- next_step_excerpt: empty in leads.json. next_step_cost / why_it_might_beat_sqrt_p were not needed for this protocol.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id, title, disposition only).
  The rg search found no other hit; no note text was available, so the mechanism below is from general knowledge, not the atlas note.

## 2. Hypothesis
Obstruction (reconstructed, not read from the atlas): a generic pair-table 4-sum oracle (build table of all sums of two factor-base
points, then MITM lookup of R - s) costs, per full relation set of size ~|F|/2, strictly more group additions than a matched
Pollard rho walk at the same group order, at every toy size 2^11..2^17, so it cannot supply an improvement over rho.
Falsifiable form: for every tested n, min over FB sizes of (oracle group additions) / (mean rho steps) >= 1.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio_c0(n) = best oracle cost in group additions (table build + one subtraction per lookup) / mean rho evaluations, n in {11,13,15,17}.
  Also reported with lookups priced at 0.1 and 1 GAE (AGENTS.md 5a band): cost = adds + c*lookups.
- Prediction: ratio_c0(n) >= 1 for all n (expected well above 1), and non-decreasing trend in n.
- Falsified if any ratio_c0(n) < 1 with controls passing. Consistent if all >= 1 with controls passing. Inconclusive if a control fails.
  A falsification only says a generic table oracle beat a conservative rho at that toy size; it does not extend to cryptographic sizes.
- Positive control: every relation found is re-summed and verified to equal R (the detector sees real decompositions); measured
  per-R success rate p_hat is reported next to the predicted |T|^2/(2r).
- Null / negative control: a random factor base of equal size (no subspace structure); its ratio is reported as the structureless reference.
- A/A arm: the subspace oracle run twice with different seeds (101, 202); require best-cost relative difference < 35%.
- Rho reference is deliberately conservative (cheap): first repeat of a 16-way additive walk, stored visited set, mean of 200 seeds (1000..1199);
  it is not a distinguished-point rho, which would cost more, so the test is biased toward falsifying the prediction.

## 4. Method
- Group: Koblitz curve y^2+xy=x^3+x^2+1 over GF(2^n), n in {11,13,15,17}, field via generator log tables; G = 2Q for first point Q;
  r = order of G found by enumerating multiples (pts[k] = kG), so group ops become integer arithmetic on logs.
- Factor base F: points with x < 2^k (GF(2)-subspace of x-coordinates), closed under negation; k swept 2..n/2.
- Oracle: table of all pair sums (i<=j); for random R, scan table keys s and test R - s in table (4-sum). Need |F|//2+1 distinct
  +/- classes of R decomposed; cap 200000 R tries. Best k = minimum adds among k with a completed set.
- Seeds: oracle 101 and 202 (A/A), null 101, rho 1000..1199. Deterministic.
- Cost: pure Python, enumeration of 2^17 curve points plus tables; estimated well under 10 minutes, < 200 MB (tables of <= a few 10^5 entries). Not executed in full before check-in; the smoke test only runs n=9.
- Entry: `python3 -I run.py` (full) or `--smoke`; prints one JSON object with field "verdict".

## 5. Classification
runnable_local. A stdlib toy decides the recorded ratio directly; no GPU, large memory or missing records needed.

## 6. What a run cannot establish
- That the real lead's oracles (compact-orbit 4-sum with Frobenius orbits, cap-locus arity, known-log pools) were exactly this one; the atlas note was not read.
- Anything about sizes beyond 2^17 or true asymptotics; toy ratios only sharpen the constant, not the exponent.
- Group ops are simulated through a full discrete-log table, so native Koblitz arithmetic costs and Frobenius savings are not priced.
- The x<2^k subspace FB at small k is tiny; the factor base is not co-designed. All results are labelled [pcc:unverified].
