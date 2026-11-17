# Exact-coverage follow-up protocol

Frozen after the sampled follow-up and before exhaustive enumeration.
No native sweep is rerun or rewritten.

Question: do the observed degree13 trace-zero GP yields reflect a larger
set of actually decomposable prime-subgroup targets, and does the proper-
subfield control support any such nonidentity targets?

Inputs: all60 sealed attempts from primary, secondary, confirmation and
representation. The one failed empty inventory stays failed with null
coverage. All complete inventories and recorded targets are checked by a
separate Rust implementation of polynomial-basis GF(2^n) arithmetic and
binary-curve addition, in analysis/src/coverage.rs. Exact models/moduli are
read from the unchanged registered records; every inventory point/target
must lie on that model, the subgroup order must be prime, and rG=O for the
recorded nonidentity generator.

For each completed case, enumerate every ordered pair of signed inventory
points, retain every distinct finite sum, and separately retain those
annihilated by r. Report the latter support cardinality over r-1 possible
nonidentity prime-subgroup targets. This is an exhaustive finite diagnostic;
no scalar logarithm is computed. Compare the independent pair census with
every recorded target truth flag and re-add every recorded point witness.
Do not use the prime-filtered set to reject a positive control that can lie
outside the prime subgroup.

Stop/fail closed on any off-curve point, order failure, arithmetic inverse
failure, census disagreement or witness disagreement. No filtering out failed
checks, hidden replacement model or edited native evidence is permitted.
At most256 signed points per inventory are expected; reject larger inputs
before enumerating. The local audit must finish within120seconds or record
timeout; do not raise the limit after seeing the outcome.

Counts establish support on these exact finite inventories. They do not
establish a cheap large-field decomposition oracle, certified regularity,
scalable shape advantage, pipeline speedup or fastest implementation.
Enumeration/setup, subgroup tests and memory costs are not treated as free.
This is same-host, second-implementation arithmetic checking, not an
independent-host performance/research certificate. No accepted timing
comparison is introduced.

Native command, after building the checked-in analysis crate:

```sh
factor-base-lead-audit --coverage RUNS_ROOT docs/curves/registry.json primary secondary confirmation representation
```

The aggregate native audit and unit tests must pass before this diagnostic
can be cited. Preserve all earlier failed analysis attempts and all failed
sweep cases alongside this additive output.

