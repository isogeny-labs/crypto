# Adaptive affine-label peeling: one immediate stopping set

Pushed freeze d6b9b8ad004, single invocation exits0. Frozen hashes verified.
All3781admitted prefixes agree for Sparse/nativeFast target result; all
derived affine point labels and returned scalars verify in original group.
No source/target/subgroup fixture substituted. All5785retained rows examined.

|p|source multiple|input rows|terminal known orbit columns|admitted rows|first k pass:row|
|---|---|---|---|---|---|
|101|1|106|7/7|106|1:19|
|101|17|112|7/7|112|1:9|
|251|1|759|27/27|759|1:202|
|251|17|750|27/27|750|1:499|
|601|1|2004|1/68|0|undetermined|
|601|17|2054|68/68|2054|1:766|

Rows are indexed in complete immutable workload stream, not sparse producer
visits, useful ranks or cost. All positive cases have another full pass to
certify closure. The stalled case stops after first pass, with only initial
source-normalized core0 label. It remains incomplete and has no speed ratio.

## Finite stopping-set certificate

Let K contain initially known orbit column0. The gate aggregates exact signed
weights in each retained row before testing unknown support. In stalled
p601/source1case every one of2004rows has at least2nonzero columns outside K:
otherwise the frozen loop would admit it (including a zero-unknown target
constraint). Hence no row can enlarge K or directly constrain k.
By induction any ordering/revisiting of these same rows under the same
one-unknown rule leaves K unchanged. This is a finite-panel obstruction even
with free perfect split-support discovery, not a statistical failure or a
runtime assertion. It does not cover new rows or multi-unknown elimination.

## Consequence for the proposed producer

Two-known-point collisions can leave a linear residual and provide a correct
new affine label, but cannot bootstrap from these initial labels on all six
panels. Granting the entire16anchor input first was already more generous
than an actual sparse scan. Do not build an unchanged peeling-only producer
on an assumption of automatic label growth. Five successes do not establish
cheap support discovery, runtime saving or a broad result.

A next adaptive mechanism must add a justified multi-unknown initialization
(e.g. weighted LPcomponents) or new sections. The existing affine LPsolver
already qualifies such cycles, so its algebra is not new/free; integration
must charge geometry discovery, matrix/component work and all failure cases.
Prior bootstrap-floor applies if the exact old bootstrap is retained. No
novelty or general impossibility claim; genus3/4 and original10xobjective open.
