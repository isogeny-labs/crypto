# Frozen follow-up: exact-x-first affine leaf

Date: 2026-11-26
Lane: draft PR #5254
Status: frozen after the disclosed affine-leaf result and before implementation
or execution of this follow-up arm

## Motivation from the affine-leaf result

The first affine-leaf follow-up searched all 971,635 terminal candidates and
reduced the retained-3 query from 13,029,908 to 5,585,265 multiplications and
from 5,086,791 to 1,254,991 squarings. It nevertheless constructed a complete
affine `(x,y)` point for every candidate before asking whether the x coordinate
exists in the retained directory.

The retained directory has only 26,248 exact x keys in a roughly 31-bit field.
The y-coordinate formula costs one multiplication after the x coordinate and
slope are already known. This follow-up defers that multiplication until an
exact full-x match is found. The existing target and result informed the idea;
this is not represented as a held-out discovery or a prediction made before
the earlier runs.

## Fixed input and accounting

Use the same public master seed, target, Prime31 curve, 68 signed Dickson
points, 2,048-element blocks, and nine cold cyclic rounds as the two earlier
protocols. Every arm rebuilds and discards its own retained table. No state,
ordering, target cache, or learned bucket information crosses arms or targets.
Setup and query are both charged to the one target.

The six-arm panel retains the five existing arms and adds
`retain3-prefix16-affine-x-first`.

## New exact arm

The new arm repeats the affine-leaf algorithm through batch inversion. For a
generic or doubling terminal addition it computes the slope and x coordinate,
then probes the same exact 16-bit prefix directory:

1. the prefix selects a bucket but never accepts a point;
2. every entry examined is compared against the full 31-bit x coordinate;
3. only when full x equality holds is
   `y3 = slope * (x1 - x3) - y1` evaluated; and
4. the resulting full point determines witness orientation exactly as before.

Identity and inverse outputs are already resolved and use the ordinary exact
lookup. Doubling participates in the deferred path after its exceptional slope
is formed. Every candidate still contributes one table probe and every exact x
match materializes y, even after the first witness is found.

Add a reported `affine_y_materializations` counter. It must equal the number of
full-x matches on which the deferred formula is evaluated. The independent
verifier still recomputes complete affine points and does not trust this
counter.

## Predictions and decision rule

The retained states (27,370), distinct keys (26,248), candidates (971,635),
table probes (971,635), key comparisons (388,647 on this fixed target), group
additions (1,028,789), squarings (1,254,991), and inversions (476) must match
the affine-leaf arm exactly. The expected multiplication identity is

`x_first_multiplications = 5,585,265 - 971,635 + affine_y_materializations`.

The arm succeeds structurally only if:

- it returns a valid witness after scanning all 971,635 candidates;
- all exceptional branches and exact counters pass unit and independent replay;
- y materializations are at most 97,163 (10% of terminal candidates), removing
  at least 874,472 multiplications from the affine-leaf arm; and
- it does not change setup, the retained representation, or candidate order.

The 10% threshold is deliberately far looser than a uniform-x heuristic and is
fixed before implementation. Local non-isolated timing remains secondary
producer evidence and cannot establish an admitted timing speedup.
[pcc:unverified]

## Proof boundary

The existing public seed, source hashes, witnesses, operation vectors, and
independent full affine scan remain the proof mechanism. They are transparent
and unsigned. They are not a zero-knowledge proof, a signed execution receipt,
or evidence that the producer host ran at a particular speed.
[pcc:unverified]
