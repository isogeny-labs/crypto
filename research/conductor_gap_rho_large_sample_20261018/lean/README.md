# Formal companion: representation-hashing rho is conductor-blind in distribution

`RepresentationHashInvariance.lean`, Lean 4.34.1 (`lean-toolchain`), no Mathlib,
no project file: check with

    ~/.elan/bin/lean RepresentationHashInvariance.lean

`build.log` is the output of that command on 2026-10-18: the three `#print axioms`
lines report `[propext, Quot.sound]` and nothing else (no `sorryAx`, no custom
axiom; `Classical.choice` is not used).

## What it proves

Setting: two types `G`, `H` with binary operations and an `Iso` (a bijection
compatible with the operations; group axioms are never used, so the groups of
points of two isogenous curves, or a curve and an isomorphic model, are special
cases). An `r`-adding walk is `(hash : G → Fin r, mult : Fin r → G)` with
`step x = x * mult (hash x)`; its cost `rhoCost w x bound` is the first `n ≤ bound`
whose trajectory `x, step x, …, stepⁿ x` has a repeated element (the ρ-length),
else `bound + 1`.

| theorem | statement |
|---|---|
| `iter_transport`, `traj_transport` | the transported walk `(hash ∘ φ⁻¹, φ ∘ mult)` on `H`, started at `φ x`, has trajectory `φ` of the original one |
| `rhoCost_transport` | it has the same ρ-length at every budget |
| `transport_left_inv`, `transport_right_inv` | transport is a bijection `Walk G r ≃ Walk H r` |
| `costPreservingBijection` | the three together |

## What it means for the conductor gap

`research/conductor_gap_cm_pairs_20261010/lean/GenericInvariance.lean` covers
algorithms whose control flow depends only on equality answers (BSGS type). The
sheriff correction SHF-20261016-3caa6f pointed out that ecbench's rho and kangaroo
are not of that kind: their partition hashes the point representation, which an
isomorphism changes, so for a *fixed* hash the two sides of a gap can have
different costs (its toy example: 197 of 200 instances).

This file states the invariance that does hold for such walks. The transport is
a bijection of the whole space of (partition, multiplier) choices that preserves
cost from every start point. Read outside Lean: the push-forward of the uniform
distribution on walks under a bijection is uniform, so over a uniformly random
partition the distribution of rho's cost is the same on the crater and on the
floor (and on every curve of the isogeny class, as abstract groups of equal prime
order are isomorphic). The endomorphism conductor, the j-invariant or any other
invariant that is not an invariant of the group of points can enter only through
the one fixed partition an implementation uses. Whether ecbench's fixed partition
(`mix(key)`) does so is the empirical question this study measures; it is not a
theorem and cannot be made one without fixing the hash inside the formal model.

## What it does not prove

Nothing about elliptic curves, isogenies, endomorphism rings, or conductors is
formalized (today's Mathlib has none of the needed objects). The result is about
walks on isomorphic structures; the mathematical bridge "isogenous curves of the
same prime order have isomorphic groups of points" is an elementary fact used
outside the file. It says nothing about non-generic algorithms (index calculus,
transfers) and nothing about the cost of computing the connecting isogeny, which
`TransferBound.lean` in the 20261010 study handles as a black-box cost.
