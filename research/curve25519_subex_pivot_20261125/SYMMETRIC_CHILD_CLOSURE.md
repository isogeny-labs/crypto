# Symmetric packet-child closure probe

This note records exact finite evidence about the unresolved child of the
complete-multigraph root family. It is a topology and packing probe, not a
recursive closure proof or a practical Curve25519 attack. `[pcc:unverified]`

## Native model

The control in
`crates/hashed-ecdl-tensor/src/contraction/symmetric_child_controls.rs` uses
the production conditional-expectation labels, packet construction,
deformation modes, and child constructor. It starts from a complete bipartite
multigraph with radix two, packet exponent `a=1`, and two parallel edges per
vertex pair. It follows coordinate zero and group zero through the schedules

```text
a=1, L=2, D=4
a=2, L=4, D=16
a=3, L=8, D=64.
```

The zero exponent tuple is sufficient for this geometry check: tuple values
change packet tensors, but not child vertices, incidences, degrees, or the next
conditional-label computation. `[pcc:unverified]`

## Below-guard boundary

With four vertices per root side, `M=32`. The first auxiliary child has 32
edges and accepts the `a=2` packing with 10 packets. Its group-zero child again
has 32 edges and maximum degree 8, but the `a=3` packing fails exactly at the
packet-padding bound. The selected labels have maximum class size 2; rounding
that class to length 8 across 64 classes costs 512 slots, exceeding
`3M=96`.

This is not a source-admitted counterexample because the next schedule already
requires

```text
M >= L D = 8 * 64 = 512.
```

The fast default test fixes this rejection and its exact occupancy transcript.
`[pcc:unverified]`

## First guard-boundary orbit sweep

With sixteen vertices per root side, the root has `M=512=L D` at the `a=3`
level. The incidence-Gram implementation in `CONDITIONAL_LABEL_GRAM.md` makes
all four two-group paths cheap enough for the default suite. They measure:

```text
root packets                         256
path               00    01    10    11
a=2 packets       132   132   134   134
a=3 child edges   512   512   516   508
maximum degree     32    32    32    32
maximum class      10    10    10    10
padded class mass 1024  1024  1024  1024
a=3 packets        74    78    77    77
maximum side load 261   261   260   262
```

All four packings pass their production padding, packet-count, and group-load
bounds. Paths 00, 01, and 10 also have at least `LD=512` edges; path 11 has 508
and would take the production fallback before this scale. The former single
path took 107.32 seconds under the edge-pair objective. The exact Gram reduction
cuts that path to 0.02 seconds and the full four-path sweep to 0.06 seconds in
the warm Rust debug harness. This remains finite radix-two evidence, not every
coordinate, depth, or radix-64 closure. `[pcc:unverified]`

## New proof target

For every source-admitted packet child at depth `d`, prove the following
directly from its old-vertex and packet-pattern orbit data:

```text
maximum pair-label class <= 2 M_d / D_d,
maximum coordinate/group endpoint load on either side <= 3 M_d / 64,
maximum degree <= delta.
```

The first inequality and `M_d>=L_d D_d` imply

```text
D_d * L_d * ceil(maximum_class/L_d) <= 3 M_d,
```

which is the padding bound; the packet-count bound then follows from the same
full class array. The second inequality gives the required eightfold child
shrink, and the third preserves the degree hypothesis.

The production labeler now exposes the exact aggregate score as degree-bucket
mass plus sparse incidence Gram forms. The next step is to express those forms
over the equitable partition `(packet pattern, block index, old-side orbit)`
and show that greedy first-minimum labeling preserves the two displayed caps. A
counterexample must satisfy the actual `M>=LD` and `M>=2D^2 delta` guards;
under-scale padding failures do not refute closure. `[pcc:unverified]`
