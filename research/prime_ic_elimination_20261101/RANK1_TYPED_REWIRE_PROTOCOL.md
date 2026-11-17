# Frozen rank-1 typed functional rewiring screen

Frozen: 2026-11-01, after the support-matched sparse-null result and before any
held-out typed-rewiring draw

Status: exact topology diagnostic; narrower than the draft E1 double-edge null

## Question

Does the natural equality-quotient rank advantage survive a topology-changing
null that preserves every local typed predicate, vertex degree, edge count,
interface, semantic cut, and parsimonious exact evaluation?

The null exchanges complete active input bundles between definitions having
the same semantic region, predicate tag, typed predicate program, input port
count, and output relation. Every exchange is implemented as degree-preserving
double-edge swaps at consumer endpoints. Output incidences, interface edges,
cut edges, and producer equality-tree endpoints remain fixed. Pairs that could
create a self-loop, cross the cut, touch the interface, or introduce a forward
dependency are rejected before mutation.

This is stricter than the sparsity null but narrower than the draft E1 null,
because it rewires only typed functional definitions and retains an acyclic
program. It cannot by itself decide the full E1 hypothesis.

## Frozen panel

Use the four exact held-out natural fixtures and natural-result SHA-256
`045eb440e4a4348930b07f63418b2c98dfb8264e1771b7206bbfaa8c2bb4b0b6`
from `RANK1_QUOTIENT_PROTOCOL.md`.

For each field, generate draws `0..31`. Derive a `u64` seed from the first
eight little-endian bytes of

```text
SHA256("rank1 typed rewire v1\0" || field_bits_le64 || draw_le64).
```

The first eight draws form the unconditioned panel. Among all 32, the first
draw satisfying

```text
floor(log2(1 + null_support)) = floor(log2(1 + natural_support))
```

is the conditioned draw. If none matches, retain all draws and mark
`conditioned_unmatched`; never choose the closest support.

## Required controls

- canonical validation passes after every draw;
- the ordered local-predicate vector, vertex degrees, edge count, interface,
  pins, side membership, raw cut width and left-internal edge count match the
  natural source exactly;
- at least one definition pair and edge pair changes in every draw;
- exact functional contraction replays every selected rewired graph predicate;
- draw zero reproduced independently has identical receipt, canonical hash,
  quotient tensor and rank;
- the mutation domain `rank1 typed rewire mutation v1` changes draw zero;
- natural canonical hash, quotient tensor and rank reproduce the retained
  source result before any null is accepted;
- the direct fanout quotient and 48 exhaustive raw-rank equivalence controls
  remain green.

## Frozen reading

For each cell sort the first eight unconditioned null ranks and use order
statistics 2, 4, and 6 (one-indexed) as `Q1`, lower median, and `Q3`.

- **advance the topology mechanism** if at least three cells have natural rank
  strictly below all eight unconditioned ranks, have a conditioned draw, and
  have `natural_rank / conditioned_rank <= 0.8`;
- **reject this typed topology mechanism** if natural rank lies in the inclusive
  unconditioned interquartile interval in at least three cells;
- otherwise report **inconclusive**.

Advancing still does not establish the broader E1 null, scaling, runtime,
relation collection, linear algebra, or an ECDLP speedup. Rejection applies
only to this typed acyclic rewiring family.
