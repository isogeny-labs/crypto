# LEAD-ICB-frobenius-orbit-factor-base

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-30). Family: ic-binary.
- Title: Frobenius/endomorphism-invariant factor bases and orbit columns on Koblitz and subfield curves, beyond the trivial orbit saving.
- Next-step excerpt: empty in leads.json (no next step recorded). The session log summarises the obstruction as "orbit credit only".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (grep context only); /home/user/crypto/docs/session-logs/2026-11-01.md (lines ~1405-1425). The lead's own atlas YAML is encrypted and was not read; the mechanism below comes from the one-line obstruction plus general knowledge.

## 2. Hypothesis
For a Frobenius-orbit-closed factor base F (union of k full orbits of size m, m prime) on the Koblitz curve y^2+xy=x^3+x^2+1 over GF(2^m), the 2-decomposition coverage c = |{P1+P2 : P1,P2 in F}| / #E equals that of a uniformly random point set of the same size, to within 5%. So the only gain is the trivial one: columns shrink by a factor m (orbit quotient) while relations per trial are unchanged. This is the recorded obstruction, re-derived at toy size.

## 3. Preregistered prediction and falsification criterion
- Statistic: R = mean coverage(orbit arm) / mean coverage(random arm), per m, with SE from the draws (delta method) and a 2-SE interval.
- Prediction: the 2-SE interval of R lies inside [0.95, 1.05] for every m in {11, 13, 17}.
- Falsified: for any m, the interval lies wholly outside [0.95, 1.05]. A value above 1 would be a beyond-trivial gain; a value below 1 would mean orbit closure costs yield. Either one decides against "orbit credit only".
- Inconclusive: otherwise, or if a control fails.
- Null / negative control: random same-size point sets (arm "random"), drawn from the same point list with Frobenius-fixed points excluded.
- A/A arm: a second independent random batch ("random_AA"); |A/A ratio - 1| must be <= 2 SE.
- Positive control: the arithmetic progression {jG}, whose sums collapse to about 2B points; its coverage ratio to the random arm must be < 0.5, showing the detector sees structure.
- Beyond-trivial factor is reported as R; the trivial factor is m. Native work of rho arms is not priced: no cross-method comparison is made (section 6).

## 4. Method
- Field GF(2^m) via log/antilog tables with primitive polynomials (m=11: x^11+x^2+1; m=13: x^13+x^4+x^3+x+1; m=17: x^17+x^3+1; primitivity asserted). Affine group law on y^2+xy=x^3+x^2+1; all points enumerated via the half-trace (m odd). Frobenius-fixed points are excluded, so every orbit has size m.
- Sizes: (m, orbits per base k, draws) = (11, 3, 200), (13, 3, 200), (17, 11, 200); base size B = k*m (33, 39, 187). Group orders are about 2^m (2^11 to 2^17).
- Each draw: coverage of all unordered pairs with repetition (including doubling and inverse pairs giving infinity), divided by #E.
- Seeds: random.Random(1000003*m+d) for orbit, 2000003*m+d for random, 3000003*m+d for A/A, d = 0..199.
- Cost: about 10^7 group additions in pure Python, estimated 1-3 minutes, well under 10 min and 200 MB (log tables for m=17 are about 260k ints). Smoke: m=7, 5 draws, under 1 s.
- Run: `python3 -I run.py` prints one JSON object with a "verdict" field.

## 5. Classification
runnable_local. A stdlib toy decides the question. The test is cheap, deterministic and fits the limits.

## 6. What a run cannot establish
- Results are labelled [pcc:unverified]. Coverage of 2-sums is a proxy for relation yield; it does not cover 3/4-sums, summation-polynomial decomposition cost, or linear-algebra cost.
- Toy m (11-17) says nothing about asymptotic behaviour or about the closed lead's cost accounting at cryptographic sizes.
- Orbit factor bases built on subfield curves, other endomorphisms (GLV) and negation-closure are not tested.
- It does not compare against matched rho, so it makes no statement about the distance to rho and is not an improvement claim (AGENTS.md section 5a pricing not applied).
- A "consistent" verdict only re-confirms the recorded obstruction in this model; it cannot prove that no orbit-aware scheme beats the baseline.
