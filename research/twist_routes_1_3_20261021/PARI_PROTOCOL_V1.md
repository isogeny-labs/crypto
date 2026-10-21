# Independent geometry and rational-lift execution, version 1

Use PARI/GP's native C backend in the recorded Debian package, with the same
frozen fixture and selected-cover manifests. Commit the source and this
protocol before execution. Keep every emitted GP script, output and exit status.

For E12, compute the genus-two characteristic polynomial independently for all
ten selected covers. Compare every coefficient with the product of the curve
and twist quadratics and compare its value at one with the recorded Jacobian
order. This checks geometry/order, not the arbitrary-divisor norm implementation.

For E17, take both structural integer models and every d=-64..64 excluding zero
and multiples of p. Construct the characteristic-zero standard twist
Y²=X³+d²*A*X+d³*B. Request PARI's rank bounds at effort zero and enumerate
rational points with standard-model x height at most 16, retaining signs.
This supplements the original integral box, with a separately stated height
convention. It does not claim an equal-original-height comparison or a
certified full Mordell–Weil basis.

Enumerate all nontrivial unordered three-term relations among these points
using PARI elliptic addition. For square d modulo p, reduce via
x=X/d, y=sqrt(d)*Y/d². Independently check the finite-field curve equation and
every projected relation with Rust. Compare its coefficient rows and rank with
the exhaustive ordinary Fp three-term relation census on the identical
projected signed base. Nonsquare d stays a separate component control. Retain
failed/empty searches and unresolved rank intervals. No target-lift, full IC
speedup or NIST-size extrapolation follows from high rank or point counts.

Each GP process has a 15-second hard limit, one thread and a 4-GiB enclosing
process limit; the full panel stops after 90 minutes. This is L0 correctness
and bounded-search evidence. Wall time is unadmitted. Independent CAS arithmetic
is distinct from an independent-host replay and from a faithful Lean proof.

```sh
research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments pari research/twist_routes_1_3_20261021/execution-v1/fixtures.json research/twist_routes_1_3_20261021/execution-v1/selected-covers.json FRESH_PARI_DIRECTORY
```
