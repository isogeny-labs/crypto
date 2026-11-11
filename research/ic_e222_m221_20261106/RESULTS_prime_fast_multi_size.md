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
