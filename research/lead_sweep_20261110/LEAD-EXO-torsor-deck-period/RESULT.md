# RESULT: LEAD-EXO-torsor-deck-period

## Outcome
- status: ran (single run, no retries). exit=0, wall=45 s (run.status). run.stderr tail empty.
- Output: RESULT.json (single object), seed 20261110, 3000 pairs/cell, |z|>5 threshold.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): each of the five cheap selectors has <=2 strong cells over 20 curves; falsified if any has >=3; consistent if controls pass.

Measured strong cells (|z|>5), max |z|:
- y_parity 0 (2.23); y_high 0 (2.67); x_parity 0 (2.30); x_high 0 (4.20, below threshold); leg_y 0 (2.23; 20 cells).
- hash_null: 0 strong, max |z| 3.21 (40 cells).
- ORACLE_dl_parity: 20/20 strong in N=2 mod 4 cells, max |z| 54.77. N=0 cells: 0 strong (not applicable to its gate).
- leg_x_minus_x0 (expected positive, not gated): 20/20 strong in N=2 class, 0 in N=0 class, max |z| 54.77.
- A/A: 0 of 220 reruns exceeded |z|>5.
- leaking_cheap_selectors: [].

Verdict (run.py): consistent_with_prediction. Controls passed: null yes, positive yes, A/A yes.

## Reading
- The numbers match the verdict field; no discrepancy. x_high max |z| 4.20 is the nearest approach to threshold, with 40 cells, which is plausible noise for the tail.
- The toy tests a narrow claim: no cheap coordinate selector is a homomorphic section of the 2-cover G -> G/<T>. It confirms that, and the detector can see the effect (oracle and Legendre(x-x0) both saturate in the N=2 mod 4 class).
- Unsurprising: Legendre(x-x0) (the 2-descent character) is homomorphic only where N=2 mod 4, giving just the known DL parity bit. It is zero in N=0 cells, as expected from the group structure there.
- Limits per PROTOCOL s6: it is not the lead's real object (no cubical/biextension structure, no non-Galois cover, no isogeny), only five hand-picked selectors, p about 2^13-2^14.

## Bearing on the sub-sqrt(p) question
closes_or_confirms_obstruction: the only homomorphic section found is the known 2-descent character yielding DL mod 2, consistent with the recorded obstruction; this toy cannot address the real lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
