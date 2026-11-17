# Joint conversion of public shifts and source lanes

## Construction

The new public entry point is Prepared::joint_window(Q,length). It accepts no
shifted-point list or scalar label. Starting with Q, generate Q,Q+P,... in
Jacobian coordinates while updating the original mixed-stride grid lanes.
For each finite shift, place that point and its finite lanes into one flat
batch. Retain infinity in the output positions but omit it from inversion.
One conversion supplies both the affine shifted points needed by the source
and all affine grid slots. The all-identity singleton performs no conversion.

The previous native window production prefix is preserved except cfg(test)
counter hooks in field multiplication and inversion. Those hooks are absent
from production. Initial identity/prepared-P/generated branches, point-bound
prepared strides, mixed updates and lane order remain unchanged. Generated
points replace external-prefix validation. Primality/subgroup premises and
p<=u64::MAX/2 remain explicit; no large-field implementation is inferred.

## Exact arithmetic tradeoff

Compare the target geometry portion of the original source: affine-prefix
products/inversions, projective grid/mixed updates, and finite-lane conversion.
Target-independent preparation is excluded from both target tallies. Original
counts come from sealed r479 candidate records; native counts are exact test
hooks, not clocks. Multiplication includes squares and constant products;
inversion is one algebraic operation regardless of its internal Euclid work.

| Public target | Original products | Joint products | Original inversions | Joint inversions |
|---|---:|---:|---:|---:|
| O | 6390 | 6686 | 15 | 1 |
| P | 6922 | 7243 | 16 | 1 |
| 2P | 7410 | 7740 | 16 | 1 |
| -P | 6855 | 7153 | 14 | 1 |

These counts were asserted against a derivation committed before execution.
Projective prefix generation uses236/253/255/227 products, replacing the
recorded45/48/45/42 affine-prefix products. Co-converting K finite shifts adds
7K products, with K=15/16/16/15. Native initial-point validation adds0/4/8/8
products relative to the old source. Thus product increases are296/321/330/298.
The original separate prefix inversions14/15/15/13 disappear. Nonempty joint
conversion still costs one inversion, now over N+K rather than N points.

The prefix counts include identity, vertical and doubling exceptions: identity
costs no group products; general Jacobian addition uses17, direct doubling15,
and a vertical general-add branch8 in this implementation. The corresponding
old affine addition uses3 products normally,6 on doubling, and no products
for identity/vertical returns. Original control-flow branch tests remain costs.

This is a product/inversion tradeoff, not a complete execution-cost comparison.
Additions, integer reduction, allocation, copies, map-free memory traffic and
actual inversion implementations are not priced by this table. No timing win
or strongest-tool comparison is claimed. Complete snapshots remain retained.

## Checks and provenance

At source20ab3b2a90837f9fd49580e530357fa504d854fa, one standalone rustc1.99.1
build and one native unit invocation both exit0; all3 methods pass. Generated
public shifts match all64 supplied historical entries across O,P,2P,-P. All1426
finite slots match saved offset/lane/label/projective/affine/inverse-Z records.
Snapshots and initializations match the prior native window. Count assertions
above pass. Identity-only, empty, invalid-point and finite singleton cases pass.
Case IDs remain checker-only; production receives Q and length.

baseline-geometry.tsv is the exact jq projection of original group/conversion/
affine-prefix counts. counts.json projects the four native receipt rows; no
trial rerun creates the table. Source/reference/fixture locks and original hashes
are retained. Tests use cfg(test) field counters and no production instrumentation.
Native unit elapsed text is diagnostic only. No sign-resultant computation,
scalar-recovery panel, cloud job or workspace-wide Cargo suite ran.

## Next integration

Connect normalized seed/progression and polynomial resultant evaluation, using
value-only requests when seeds are supplied and jets for normalization. Preserve
product-tree/multipoint structure instead of a quadratic cross-product. Then
connect the sign-mask decoder and admit the unchanged full4563-case cohort with
complete source/runtime and cost accounting. Larger prime fields remain a native
backend task; the four fixed geometry fixtures do not settle that scaling.

## References and coordination

Original ec.py:add, mixed-projective-strides-v2/run.py and sealed r479 geometry
counts; prior native grid/window and EFD Jacobian formulas reviewed earlier.
No new group-formula theorem is claimed. The source change is joint conversion.
The shared daily log was reserved by T-1471; this component's SESSION.md records
work under research-only T-1474 without crossing that reservation.
