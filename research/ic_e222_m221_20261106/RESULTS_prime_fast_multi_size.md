# Multi-target IC vs the tuned BSGS at 33 and 35 bits (exploratory size-trend probe)

Status: [pcc:unverified] producer's own measurement; exploratory (prediction recorded before the run in `results/pf_size/PREDICTION.txt`, not a full frozen protocol); one instance per residue; same host, 1 thread, wall clock; not independently replayed.

Setup: batched multi-target IC with the bucketed pair table (`ic_multi_solve_ex`, table + batched), c = 1.0 (not retuned for the size), T = 1000, against the tuned multi-target BSGS (not compacted) on identical planted targets. Every target of every run verified (1000/1000).

| bits | residue | fb | IC ms | of which setup | tuned BSGS ms | IC / BSGS |
|---|---|---|---|---|---|---|
| 33 | 1 | 2048 | 818 | 550 | 253 | 3.23 |
| 33 | 3 | 2048 | 956 | 635 | 297 | 3.21 |
| 35 | 1 | 3251 | 1977 | 1435 | 495 | 3.99 |
| 35 | 3 | 3251 | 2162 | 1656 | 537 | 4.03 |

Prediction (written first): IC/BSGS <= 3.0 at 33 bits and <= 3.5 at 35 bits. **Both missed** (3.2 and 4.0). Together with the frozen evaluation (best constant per size: 1.9, 2.2, 2.4 at 25, 29, 31 bits) the distance to the tuned BSGS keeps growing with the field size, roughly 1.9 -> 2.4 -> 3.2 -> 4.0 from 25 to 35 bits (with c = 1.0 at 33 and 35 bits, not tuned there). The setup share rises to 67% to 77% of the run at 35 bits (the pair table is fb^2 entries and the factor-base solve grows as p^(2/3)), so at these sizes the setup, not the per-target phase, is again the dominant cost at T = 1000. The earlier "flatter than before" statement holds only up to 31 bits.

Limits: one instance per residue; c not retuned (a smaller constant may reduce the setup); BSGS arm not compacted (the true distance is about 1.1x to 1.3x larger); exploratory, not frozen; same host; no rho arm, §5a band, certificate file, bench record or `VALIDATION.json`.

## c-retune at 35 bits (exploratory; predictions in `results/pf_size/PREDICTION_c.txt` and `PREDICTION_c15.txt`, written before the runs)

T = 1000, one instance per residue, all targets verified (1000/1000), tuned (not compacted) BSGS = 495 ms (residue 1) and 537 ms (residue 3):

| c | fb | residue | IC ms | setup ms | IC / BSGS |
|---|---|---|---|---|---|
| 0.5 | 1625 | 3 / 1 | 2513 / 2531 | 1426 / 1294 | 4.68 / 5.11 |
| 0.7 | 2276 | 3 | 2569 | 1567 | 4.79 |
| 1.0 | 3251 | 1 / 3 | 1977 / 2162 | 1435 / 1656 | 3.99 / 4.03 |
| 1.5 | 4876 | 3 / 1 | 3278 / 2790 | 3006 / 2567 | 6.11 / 5.63 |

Both predictions **failed**: "c = 0.5 at least 15% lower than c = 1.0" (it is about 25% higher), and "c = 1.5 at most 0.9x of c = 1.0" (it is about 1.4x higher). c = 1.0 is the best of the four constants at 35 bits, as at 27 to 31 bits at T = 1000, and the setup has a floor of about 1.3 to 1.7 s (T-independent) that no constant removes: a smaller factor base lengthens the relation collection (about p / fb walk points in total), a larger one makes the fb^2-entry pair table build and scatter dominate (24 million entries at c = 1.5).

Reading (counted-operation reasoning, not a measurement): the setup scales like p^(2/3) (relations: p / fb points at fb ~ p^(1/3); pair table: fb^2 ~ p^(2/3) entries) while the tuned multi-target BSGS costs about sqrt(T n) additions, so for fixed T the ratio grows like p^(1/6), a factor of 2 per 6 bits. The measured ratios (1.9, 2.2, 2.4, 3.2, 4.0 at 25, 29, 31, 33, 35 bits) are consistent with that (about 2.1x over 10 bits) but are five points from different protocols. This is the Gaudry-Diem exponent gap (2/3 against 1/2) seen in the multi-target setup: the lead 'cheaper setup' improves constants (setup roughly 4x cheaper at 27 bits through the batched collection and the bucketed lookup) and not the exponent, so it does not lead to a crossover at larger sizes. The per-target phase (the T-dependent part) is not the wall; the T-independent setup is.

Limits: one instance per residue, exploratory, c on a four-point grid, not compacted BSGS, same host, not independently replayed.

## T-scaling at 35 bits (exploratory; prediction in `results/pf_size/PREDICTION_T.txt`, written before the run)

T = 10 000, one instance per residue, all 10 000 targets of every run verified; tuned (not compacted) BSGS = 1929 ms (residue 3) and 2011 ms (residue 1):

| c | fb | residue | IC ms | setup ms | IC / BSGS |
|---|---|---|---|---|---|
| 1.0 | 3251 | 3 / 1 | 7190 / 7051 | 1882 / 2193 | 3.73 / 3.51 |
| 1.5 | 4876 | 3 / 1 | 5227 / 5998 | 3088 / 3159 | 2.71 / 2.98 |

Prediction "IC/BSGS <= 2.0 at T = 10 000 for the better of c = 1.0 and 1.5" **failed** (best 2.71). The T-independent setup does amortize (setup is 26% to 59% of the run, against about 70% at T = 1000), but the per-target phase becomes the wall: about 0.53 ms per target at c = 1.0 (35 bits), against about 0.04 ms at 27 bits (c = 2.0) and 0.2 ms per target for the BSGS.

Reading (counted-operation reasoning, not a measurement): per target the IC walks about p / fb^2 = p^(1/3) / c^2 points (a 2-term relation has probability about fb^2 / p per point); the BSGS pays about sqrt(n / T) additions per target. The IC total is about p^(2/3) (setup) + T p^(1/3) / c^2; the BSGS total is about sqrt(T n). At T ~ p^(1/3) the two exponents coincide (both about p^(2/3)): below that the IC is setup-bound, above it per-target-bound, so for a fixed factor base of size ~p^(1/3) the IC can be at best a constant factor from the BSGS (here about 2.7 at the best constant at 35 bits), never below it asymptotically. This matches the observed minimum of the ratio near T ~ 1000 to 10 000 (about p^(1/3) = 3 000 at 35 bits) and is the multi-target form of the Gaudry-Diem barrier for this factor-base size. A different exponent would need a factor base of another size, a symmetry that shrinks the relation probability's denominator, or a decomposition into more summands with a smaller table; none of these was measured.

Limits: one instance per residue, exploratory, c on a short grid, BSGS not compacted (true distance 1.1x to 1.3x larger), same host, not independently replayed.

## Paper analysis of the remaining lever: 3-term decompositions with a smaller factor base (not run)

Counted-operation reasoning, no measurement. A random point R is a sum of k signed factor-base points with probability about fb^k / p (up to a constant). The two existing modes: (a) 2-term lookup in a pair table: p / fb^2 points per relation at O(1) each, table fb^2 entries; (b) 3-term, sweep over F_i with a pair-table lookup: p / fb^3 points per relation at O(fb) each, i.e. p / fb^2 operations per relation, table fb^2 entries (this is `ic_solve` summands 3). In both modes collecting fb relations costs about p / fb operations and the table fb^2, so the setup is minimised at fb ~ p^(1/3) with cost p^(2/3): a smaller factor base (fb ~ p^(1/4)) raises the collection to p^(3/4), and a larger one raises the table beyond p^(2/3). A 3-term lookup with a triple table (fb^3 entries) would cost p / fb^3 points per relation and fb^3 for the table: minimised at fb ~ p^(1/4)... with setup fb * p / fb^3 = p / fb^2 = p^(1/2) for collection but the table is fb^3 = p^(3/4): worse than p^(2/3). Hence no 'k-term with table' variant lowers the setup exponent below 2/3 while keeping the per-target cost at p^(1/3), consistent with the known absence of a sub-square-root index calculus for general prime-field curves (Gaudry-Diem exponent 2/3 against 1/2 for generic algorithms in the multi-target form measured here). I did not run this lever; the analysis predicts a worse exponent, not a better one, so it was not worth a frozen experiment. The research-kb prior-art check required by the repository rules could not be run in this container (no `research-kb` tools in the session); this paragraph is therefore my own derivation, not a cited result.
