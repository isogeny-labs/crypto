# Frozen follow-up: staged affine-leaf contraction

Date: 2026-11-26
Lane: draft PR #5254
Status: frozen after the disclosed first m=7 run and before implementation or
execution of this follow-up arm

## Motivation from the first frozen run

The first single-target run matched every preregistered combinatorial and group-
addition count. Its non-isolated local medians were 88.562 ms for the ordinary
retained-4 hash baseline, 77.862 ms for retained-4 with the exact prefix
directory, and 64.202 ms for target-centered retained-3. The retained-3 arm won
the cold single-target comparison because the 486,115-state retained-4 setup
cannot be amortized across targets.

The retained-3 query still spent 13,029,908 field multiplications and 5,086,791
squarings. Most of that work came from adding the fourth streamed leaf in
Jacobian coordinates and then normalizing the result. This follow-up attacks
that exact terminal contraction. The existing result informed this design; it
is not represented as a prediction made before the first run.

## Fixed target and accounting

Use the exact same public seed, Prime31 curve, 68 signed Dickson points, one
target, 2,048-element query blocks, and nine cold rounds. Reusing the same
target is a paired single-target comparison, not cross-target amortization.
Every arm rebuilds and discards its own retained three-sum directory.

## New exact arm

`retain3-prefix16-affine-leaf` has three stages:

1. Build the canonical 27,370-state three-sum quotient and the exact 16-bit
   prefix directory exactly as in `retain3-prefix16-centered`.
2. Enumerate all 54,740 target-centered three-prefix complements
   `Q-P_i-P_j-P_k` in Jacobian coordinates and batch-normalize that frontier
   once.
3. For each admissible fourth index, compute the terminal affine sum with the
   corresponding signed factor-base point. Within each fixed block, collect
   all nonzero affine denominators, invert them with one Montgomery batch
   inversion, finish the affine additions, and probe the exact directory.

Identity, inverse, and doubling branches are handled explicitly before batch
inversion. Every output point is checked with full coordinates; neither the
prefix directory nor the affine batch uses a probabilistic fingerprint.

The group-level work remains exactly 1,028,789 additions and 971,635 terminal
candidates. Query inversions are predicted to be 476: one for the finite
three-prefix frontier and 475 for `ceil(971635/2048)` terminal blocks. The
generic affine path uses roughly five multiplications and one squaring per
terminal candidate, versus the mixed-Jacobian-plus-normalization path. Exact
field counts may differ because exceptional branches are target-dependent and
must be reported rather than assumed away.

## Endpoints and decision rule

Run both the existing `retain3-prefix16-centered` arm and the new affine-leaf
arm in the same cyclic cold panel; retain the other preregistered arms as
context. Primary endpoints are exact field/group operation vectors, candidate
and key-comparison counts, public witness validity, and independent affine
full-scan replay. Secondary local timing remains non-isolated producer evidence.

The new arm succeeds structurally only if it:

- searches all 971,635 terminal candidates and returns a valid witness;
- keeps the same 27,370 retained states and 26,248 exact keys;
- strictly reduces both query multiplications and squarings without reducing
  the searched set or hiding setup; and
- passes the independent verifier and artifact hashes.

The result will report any exceptional branches and all attempts. No wall-time
or algorithmic-novelty claim is admitted solely from this local run.

## Proof boundary

The existing transparent seed commitment, public witness, source hashes, and
independent affine replay remain the proof mechanism. They are not zero
knowledge or producer-host attestation. A ZK proof would hide the witness; it
would not certify timing or unbiased seed choice, so none is claimed here.

