# Results: what moved, what did not, and what to do next

## Result

The reduction is now executable and auditable. It preserves the pinned n=83
prime-order subgroup, handles the exceptional fibers, and supports scalar
recovery. Two named solvers reach the actual transported Jacobian image. This
closes the mathematical-transfer and solver-applicability gates.

It does not close the advantage gate. The current destination algorithms are
generic BSGS and rho. Native Jacobian operations are not source GAE, and the
host earned only L0 wall evidence. No claim that the destination is weaker or
that the complete route is faster is supported.

## Four-size successful panel plus retained failure

Every value below is derived from sealed measured records. Wall medians are
descriptive only.

| n | r | source rho wall (ms) | destination rho3 native ops / wall (ms) | destination BSGS native ops / wall (ms) | BSGS/rho3 native ops | BSGS/source wall |
|---:|---:|---:|---:|---:|---:|---:|
| 7 | 29 | exhausted (40/40) | 35.475 / 2.288 | 16.250 / 2.179 | 0.458 | not comparable |
| 9 | 127 | 1.857 | 71.675 / 2.681 | 32.375 / 2.238 | 0.452 | 1.205 |
| 13 | 2,003 | 1.915 | 215.150 / 4.369 | 90.375 / 3.122 | 0.420 | 1.631 |
| 19 | 130,873 | 1.931 | 1,535.725 / 19.187 | 592.000 / 9.863 | 0.385 | 5.107 |
| 23 | 2,095,853 | 2.092 | 6,194.475 / 77.399 | 2,113.250 / 28.421 | 0.341 | 13.586 |

The source A/A wall ratios were close to one in the successful panel, but that
does not repair the failed isolation tier. Saved comparison JSON correctly
reports the operation comparison as empty/bounded because destination work has
no GAE conversion.

## Destination-only interpretation

BSGS uses fewer native Jacobian group operations and less wall time than the
three-partition rho control at every tested size. Across the four successful
sizes, the descriptive log-log slopes against image order are 0.433 for BSGS
and 0.461 for rho3, close to generic square-root scaling at this small range.
The difference is a standard time/memory trade: BSGS inserts approximately
`sqrt(r)` table entries. It is not evidence of special weakness in the
Jacobian image.

The [editable figure](figures/native-cost-and-wall.svg), [PDF](figures/native-cost-and-wall.pdf),
and [150-dpi PNG](figures/native-cost-and-wall.png) show the native and wall
axes without merging their units.

## Net-route interpretation

The descriptive BSGS/source wall ratio worsens from 1.205 at `n=9` to 13.586
at `n=23`. On this implementation and host, that is evidence against a
practical net win over the selected strong source rho. The limiting factors are:

- wall evidence is L0 and has no confidence interval eligible for promotion;
- source rho has binary-curve structure that the destination generic methods
  do not exploit;
- source rows also retain unpriced native canonicalisation/table work;
- four toy sizes do not justify extrapolating a full n=83 cost; and
- another complete destination algorithm could have different scaling.

Accordingly the correct verdict is “valid and solver-reachable, no demonstrated
advantage,” not “impossible” and not “faster.”

## Measuring whether a destination is actually weaker

Use two comparisons, in this order:

1. **Within destination:** candidate versus destination generic control on the
   identical image subgroup. Require a lower exponent or a complete finite-size
   win after relation collection, linear algebra, individual log, and memory.
2. **End to end:** complete candidate route versus the best applicable source
   reference, including construction, map, recovery, and verification in one
   unit or admitted paired wall trials.

The full decision and promotion rubric is in [GUIDANCE.md](GUIDANCE.md).

## Recommended next workstream

Proceed in this order:

1. Add a named small-field genus-three index-calculus adapter. It must include
   a factor base, relation generation, sparse matrix stage, individual log,
   restriction to the transported image, and source scalar replay.
2. Run it first at `n=9,13,19,23` against `jacobian_image.bsgs`, with frozen
   smoothness bounds and relation budgets. Reject the candidate early if
   relation yield or matrix growth misses the preregistered model.
3. Extend counters to field multiplications/inversions, divisor reductions,
   table bytes/probes, relation bytes, and linear-algebra operations. Publish a
   versioned conversion or keep the units separate.
4. Repeat the three-arm cold panel at L2 or stronger on an independent host.
   Report cold and amortized regimes separately and compute the precomputation
   break-even target count.
5. Only if the candidate passes both destination and net gates should it reach
   an exact n=83 resource model. A full-order generic BSGS/rho attempt is not a
   useful next gate.

These exit conditions are tracked in the open
[problem record](../../problems/genus3-image-destination-calculus-and-common-cost-missing.md).
