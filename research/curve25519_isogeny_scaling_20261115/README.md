# Pairs law, scaling in t, deeper seed screen, distance to rho

All claims `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md) (`1bd45ffc3b`); the deep-screen choice (seed 1550) was committed before its arm ran.
Same engine, binary and conditions as PR #4213, blind arm, 20,000 targets. Output: [summary.txt](summary.txt); `analyse.py`, `select_deep.py`, `runs_t6/`, `runs_deep/`.
Large candidate dump `deep/candidates_t5.jsonl` is in the artifact store (see `deep/.artifacts.json`).

## Results (3 of 7 hypotheses failed or partly failed)

| | outcome |
|---|---|
| H22 pairs law, existing blind arms | hits per target = b(b-1)/(2 x 1049624), no fitted constant. Seeds 0-20: 218 observed vs 206.2 expected (interval 179..235), **supported**. Bottom 8: 51 vs 45.3 (33..59), **supported**. Top 8: 146 vs 122.2 (101..144), **not supported**. Every one of the six blind and lifts group totals is 2% to 20% above the law (obs/exp 1.02 to 1.20), so the law under-predicts a little everywhere. Lifts arms scale with 7.99 lift targets per target. |
| H23 t = 6 rate beats t = 5 | pooled t = 6 rate lower bound 0.574 vs 3.334. **Not supported**: the rate is 0.22x the t = 5 rate. |
| H24 pairs law at t = 6 | 315 observed vs 306.7 expected (273..341). **Supported** (a second t: the law holds at b = 56 to 70). |
| H25 cost per target within 2x | 3.06e6 mul-adds per target at t = 6 vs 1.77e5 at t = 5: 17.3x. **Not supported** (a record of how cost scales). |
| H26 deeper screen reaches c >= 25 | seed 1550, c = 26 of 32, from 3,000 candidates (maximum otherwise 25). **Supported**. |
| H27 screening cost small | 7,725,157 tower operations vs 2.12e7 (1e-2 x 18 x muladds per hit), unit weight only. **Supported**. |
| H28 pairs law out of sample | seed 1550: 30 hits vs 25.3 expected (16..36). **Supported**. |

## What it says
- **t = 6 is a loss.** Hits per target rise 3.82x (the law's 4x), but the F4 cost per target rises 17.3x, so hits per mul-add fall to 0.22x. Raising t does not pay on this engine. This corrects the optimism of my suggestion to try t = 6; the pilot (500 targets, 3.07e6 per target) already showed it.
- **The mechanism holds.** Hits track the number of unordered pairs of base points, with no fitted constant, at t = 5 and t = 6 (and with lifts). The law is a bit low (about 5% overall); `b(b-1)/2` versus `b^2/2` accounts for 2% to 5% of that (not fitted here).
- **A deeper screen helps once more.** Seed 1550 gave 30 hits in 3.54e9 mul-adds = 8.476 per 1e9 [5.719, 12.101], about 2.9x the unscreened rate and 1.6x the top-8 rate, but it is one arm and one target set. The extra yield is predicted by the law (b = 52 vs 44 at best before).

## Distance to rho (Part D; descriptive, indicative only)
On seed 1550: 26 columns, 27 relations at 1.18e8 mul-adds per hit = 3.2e9 F4 mul-adds, against 321 expected rho group operations: a ratio of about 1e7. A mul-add is not a group addition and the unpriced native work is not charged (0, 0.1, 1 GAE band), so this is an order of magnitude, not a measured gap. Even a factor of 100 per group operation leaves about 1e5. Screening seeds moves this by a factor of a few; the distance to rho remains five to seven orders of magnitude on this toy. A measured matched-rho arm needs the isolab-eks route, which is absent here.

## Decisions
Item 5 (drop lifts) and item 6 (skip descent/torsion tricks) are decisions, not experiments, per the protocol: lifts are 8x the cost for 0.9x the rate; T2 is closed and descent class did not predict the rate. Item 3 is closed by the pairs law; item 2 by the t = 6 result: not a lever on this engine.
