# LEAD-ICP-retained-setup-online

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_dominated (reviewed 2026-10-24; 20261101 same). Family: ic-prime.
- Title: "Retained-setup online IC (compact orbit n61, dense K1798 root table n83, orbit-folded m = 2) as a per-target win".
- Next-step excerpt (truncated): "Revisit when: An IC online row that beats BL preprocessing rho at equal advice bytes and equal s..."
- Recorded obstruction (atlas_survey.md): claimed rho/IC online ratios 13.9x (n61) and 18.5x (n83) were against empty-table rho; Bernstein-Lange (BL) preprocessing rho at equal advice is 53-317x faster online. Sibling-lead note: revisit only for an online IC with S*T^2 <= r/|Aut| (non-generic oracle).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep hits, lines 33-35, 141-143, 306-311);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 150).
  (atlas_tally.json matched by grep, not read. Cryptanalysis repo not searched.)

## 2. Hypothesis
For a generic cyclic group of prime order N (counted-operation model), the recorded obstruction is the accounting identity of a retained-table online solver: with S stored random points of known discrete log, a per-target random walk finds a table hit in T = Theta(N/S) steps (T*S/N in [0.6,1.6]), so log T falls with slope -1 in log S, and the online speedup over an empty-table rho (about sqrt(pi N/2) steps) is about sqrt(pi/2)*S/sqrt(N) (within a factor 0.7-1.4). A "ratio vs empty-table rho" therefore measures advice size S, not an algorithmic gain; any retained-setup IC online row must be compared with this BL baseline at equal S.

## 3. Preregistered prediction and falsification criterion
Statistic: (a) T*S/N per size; (b) OLS slope of log mean T on log S at N~2^16, S = N^e, e in {0.5,0.6,0.7,0.8}; (c) ratio rho_steps/online_steps divided by the formula sqrt(pi N/2)*S/N.
Thresholds: (a) all four sizes in [0.6,1.6]; (b) slope in [-1.15,-0.85]; (c) all in [0.7,1.4].
Outcomes: all three hold and controls pass -> consistent_with_prediction (obstruction re-derived: baseline cost law holds, lead stays closed). Any fails with controls passing -> falsified (the BL-at-equal-advice law does not describe the toy baseline; recorded obstruction's accounting needs re-audit). Controls failing -> inconclusive.
Controls:
- Positive control: S=N/4 must give mean T in [2.5,6] (expected about 4), showing the detector sees the S-dependence.
- Negative control / null: table dlog labels randomly permuted; recovered x must be wrong in >90% of trials (verification detects a fake table). Honest runs must have wrong_rate = 0.
- A/A arm: same protocol, independent seed; ratio of mean T between the two arms must be in [0.7,1.4] (noise floor).
Seeds: 20261110+k (rho/table seeds 1000..6000+k offsets as in code), fixed.

## 4. Method
Group Z_N (N = next prime above 2^14, 2^16, 2^18, 2^20), element w = (known-coefficient) with dlog tracked as a pair, so only group additions are counted (GGM-style). Table: S = round(N^(2/3)) random points a*g stored with a. Online per target h=x*g: r-adding walk (R=32 step multiples, hashed partition) from h + c*g with c tracked; stop at first table point, recover x = a - c and verify; restart with fresh offset after 3N/S steps (restart steps counted) to escape walk cycles. 300 targets per arm per size. Empty-table reference: single r-adding walk until first revisit (stored visited set), 150 trials per size, expected sqrt(pi N/2). Cost: about 1e6-1e7 dict steps total, roughly 1-3 minutes, memory < 100 MB (S <= 10^4 entries, visited sets <= ~2000).
Not priced: native-operation weights (0/0.1/1 GAE band of AGENTS.md 5a): both arms use one group addition per step and the same hash, so the ratio is invariant across the band here; stated, not tested.

## 5. Classification: runnable_local
A stdlib toy decides whether the BL-at-equal-advice cost law (the content of the recorded obstruction) holds in the counted model. run.py supports --smoke.

## 6. What a run cannot establish
- Nothing about n61/n83 IC rows, the 53-317x figures, real curves, or any IC algorithm: the toy is a generic-group BL-style table, not index calculus; no IC online row is built or compared.
- Does not reproduce the recorded 13.9x/18.5x numbers; only the shape of the equal-advice baseline.
- Table advice is counted in entries, not bytes; DP compression and Hellman-chain tradeoffs (S*T^2 >= N regime) are not tested.
- Single group family (Z_N, trivial structure, no Aut folding); small sizes where restarts and cycles matter.
- Results are labelled [pcc:unverified].
