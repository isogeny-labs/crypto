# ecdl_tensor

A toy, end-to-end implementation of index calculus for the elliptic-curve
discrete logarithm problem in which relations are found by **Boolean tensor
contraction**. It follows the four-step algorithm of the paper that solves
ECDL in expected time 2^O(b/(log b)^(1/4)), conditional on Theorem 2 of
[Ope26]: index calculus with randomly hashed factor bases.

## What is implemented

| Step | Code | Notes |
|---|---|---|
| Prime-order curves over F_p | `curve.go` | Random b-bit curves found by naive point counting (b ≤ 26) |
| 1. Factor-base selection | `relation.go` `NewFactorBases`, `NewAffineFactorBases` | m random hashed membership predicates, each with \|F_i\| ≈ (2r)^(1/m). There is no notion of smoothness. The default hash is keyed SHA-256. The gadget path uses a random affine hash, A ∈ F_i iff (α_i x + β_i y + γ_i) mod p < U_i, because that is cheap to check with low-degree gadgets. |
| 2. Relation finding | `relation.go` `RelationNetwork`, `tensor.go` | `[u]P+[v]Q = A_1+…+A_m` is encoded as a Boolean tensor network. Edges are the bits of A_i and the partial sums. Vertices are predicates for membership/validity, point addition and the target. The network is a graph with (2m−1)(2b+1) edges. |
| Counting → search | `tensor.go` `FindWitness` | Self-reduction fixes one bit at a time, so any **counting** solver works. |
| 3. Linear algebra | `linalg.go` | Left-kernel vector mod r by Gaussian elimination |
| 4. Extraction | `solver.go` | ξ = −U/V. The answer is always checked against `[ξ]P == Q`, so a wrong value is never returned. |

The contraction solver sits behind the `Counter` interface:

- `SparseContractor` contracts the network vertex by vertex. It sums out each edge once no later vertex uses it. This is exact, but it is an ordinary exponential-time contractor.
- `BruteForceCounter` is a reference counter used in the tests.
- `DFSCounter` enumerates depth-first along a schedule. It is needed for gadget networks (below).

An implementation of the [Ope26] algorithm would plug in as another `Counter`.

## Low-degree arithmetic gadgets

The paper needs two things from its encodings: O(b) edges and maximum degree O(b^(3/4)). The gadgets in `gadgets.go` and `pointgadget.go` meet both.

**This is our own construction.** It follows the paper's one-line description ("a two-stage Fourier-transform decomposition"), but the paper's own gadget is not available to us.

- **Field multiplication** (`MulCheck`) checks a·b − q·p − c = 0 with a witness q and 0 ≤ c < p.
  - Numbers are split into n ≈ √b blocks of t ≈ √b bits.
  - Both convolutions are evaluated with a number-theoretic transform of length L ≥ 2n over a prime Q > 4n·4^t.
  - The transform is factored four-step style, L = L1·L2. Stage-1 vertices each read about √L blocks. One vertex per k1 completes the forward transform, takes the pointwise product and starts the inverse transform. Inverse stage-2 vertices finish it.
  - A chain of signed carries checks that the coefficients sum to zero.
  - Each vertex reads O(√L·t) = O(b^(3/4)) bits, and the gadget uses O(L·log Q) = O(b) edges.
- **Linear relations** (`LinCheck`) and **range checks** (`RangeCheck`) are block-level carry and compare chains of degree O(t).
- **Selection and zero tests** (`Select`, `IsZero`, `ZeroIf`) are block chains that pass a 1-bit flag from vertex to vertex.
- **Complete point addition** (`CompleteAdd`) handles every input: the identity (an infinity flag, encoded canonically as inf = 1, x = y = 0), doubling, P + (−P) and the generic chord.
  - It has no branches. The denominator is dx when x1 ≠ x2. Otherwise it is 2y1 when y1 = y2, and 0 when they differ. The numerator is chosen the same way.
  - A zero denominator means the result is O, and it forces λ = 0.
  - Each sum flows through selections on the input infinity flags.
  - Every intermediate value is range-checked or forced, so each one is unique. `AddGeneric`, the earlier chord-only gadget, is kept and tested.
- **Point validity** (`ValidityCheck`) is y² = x³ + ax + b.
- **Factor-base membership** (`MembershipCheck`) computes h = αx + βy + γ mod p with two constant multiplications and a linear check, then range-checks h < U_i.
- **Fan-out.** A value used k times gets one copy vertex per block, with degree (k+1)t. Every network is checked to be a graph: each edge has at most two endpoints.

Witnesses such as q, λ and the linear multipliers are proposed by **hints** in the schedule. Hints only steer `DFSCounter`. The count is defined by the vertices alone, so hints must be complete.

The gadget tests run in exhaustive mode, where every hint enumerates all bit patterns of what it writes. They check that exactly one assignment survives and that it is the correct one.

- `MulCheck` is exhaustive for every input pair at p = 11 and 13. It is also exhaustive on sampled inputs at p = 101, which covers L1 ≠ L2 and empty stage-1 vertices.
- `LinCheck`/`RangeCheck` and `ValidityCheck` are exhaustive at 4 bits.
- `CompleteAdd` is exhaustive on every pair involving O or equal x, and a sample of the rest, on a 4-bit prime-order curve. Prime-order curves have no y = 0 points, so the y1 = y2 = 0 branch is tested exhaustively on y² = x³ + x over F₁₃, which has three of them. It checks the sum and the infinity flag against `Curve.Add`. `AddGeneric` keeps its earlier test, which checks that same-x pairs have no assignment and that generic pairs give the right sum.
- Validity plus `MembershipCheck` is exhaustive over every 4-bit (x, y) encoding, and accepts exactly F_i.

`GadgetRelationNetwork` builds the relation network from these gadgets alone.
- Each A_i is proposed by a hint. Listing F_i in that hint is a toy shortcut; a hint never constrains anything.
- Each A_i is constrained only by validity and membership, and partial sums use `CompleteAdd`.
- It has no support vertex, so every vertex has degree O(b^(3/4)).
- `TestGadgetRelationCounts` checks that its count equals the whole-point network's count for the same factor bases, for m = 2 and 3. The targets include R = O and targets reachable only through doubling or O.

Shape as b grows, from `go run ./cmd/ecdlscale`. Networks are built, not evaluated, and the modulus is p = 2^(b−1)+1. That p is not prime, which doesn't matter here because only the structure is measured.

"add" means complete addition. "point" is the per-point validity plus membership checks. A relation network with m factor bases has m point blocks and m − 1 additions.

|     b | mul edges/b | add edges/b | point edges/b | max degree | degree / b^(3/4) |
|------:|------------:|------------:|--------------:|-----------:|-----------------:|
|    64 |        35.2 |       218.2 |         217.1 |        352 |             15.6 |
|   256 |        31.4 |       198.4 |         195.9 |       1248 |             19.5 |
|  1024 |        29.1 |       186.8 |         183.1 |       2304 |             12.7 |
|  4096 |        27.7 |       180.2 |         175.6 |       8768 |             17.1 |
| 16384 |        27.0 |       176.5 |         171.3 |      17024 |             11.8 |

The degree ratio wobbles because L is rounded to a power of two.

`go run ./cmd/ecdldemo -bits 8 -gadgets` solves a discrete logarithm with relations found on the gadget network. For example, m = 2 took 7.8 s and m = 3 took 15 s on 8-bit curves. It is slow, because the DFS counter evaluates every gadget for every candidate tuple.

## Run

```bash
go test ./...
```

```bash
go run ./cmd/ecdldemo -bits 14 -m 2
```

In this sandbox, `GOCACHE` and `GOTMPDIR` had to point into `$TMPDIR`.

## What this does *not* show

- **There is no subexponential speedup here.** The paper's 2^O(b/(log b)^(1/4)) bound depends on the [Ope26] contraction algorithm. That algorithm is not implemented here (the manuscript is under external review). The paper also says it only saves time above a network-size threshold far beyond any deployed curve; see the original for the exact figure. This prototype costs about r per relation, which is worse than Pollard rho.
- **The default network still reads whole points.** The default network has point-addition vertices of degree 3(2b+1). The low-degree version is the gadget network (`-gadgets`).
- **The gadgets are our own construction.** That includes the affine factor-base hash. The paper's encodings are not in the excerpt we had.
- **O(b) edges does not give a speedup on its own.** With these gadgets, a relation network has O(mb) edges, which is the shape Theorem 2 needs. The running time still comes entirely from the contraction solver, and the solvers here are exponential.
- **Factor bases are listed by enumerating the group.** This is a toy shortcut. At scale, F_i is only ever touched through its membership predicate.
- It cannot attack deployed curves such as P-256, P-521 or Ed25519, and is not intended to.
