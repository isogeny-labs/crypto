# Fixed-cofactor extension of `ecdl-tensor-ic`: design notes (2026-11-06)

**Status: plan for review. No code has been written.** The owner approved
starting these notes on 2026-11-06. Implementation waits for review of this
plan.

## Source and scope

- **What the paper's setting asks for.** The paper the Go prototype follows
  (Akshima et al., *A Subexponential Time Algorithm for Elliptic Curve
  Discrete Logarithm*, v0.2) states its algorithm for curves whose group
  order is c·r. Here c is a fixed small cofactor and r > c is prime. The
  target and generator lie in the order-r subgroup G = [c]E(F_p).
- **What this note is transcribed from.** The repository already implements
  that setting in `crates/hashed-ecdl-tensor`, following §3 of the pinned
  Bochum revision 1ee44e56:
  - `Curve::with_cofactor`;
  - `Encoder::clear_cofactor`;
  - `reduction::decomposition_network` (see `docs/hashed-ecdl.md`, "Use the
    tensor API").
- **What is not yet checked.** This note follows that transcription. The
  wording of the paper's cofactor paragraph itself is still to be re-read
  against the PDF (open question 1).
- **What the port does today.** `ecdl-tensor-ic` (the Go port, #2895) only
  handles prime-order curves (c = 1). The extension adds c > 1 to:
  - the whole-point relation network;
  - the low-degree gadget network;
  - the solver;
  - the ecbench arm.
- **No speedup.** Every counter stays exponential: the [Ope26] contraction
  algorithm is not implemented. The gadgets remain our own construction. The
  extension changes which curves the prototype accepts, not its cost class.

## The mathematics used

The setting is #E(F_p) = c·r, with r prime, r > c and gcd(c, r) = 1.

- [c] maps E onto G, and its kernel is the c-part of E, which has c points.
  So every A in G has exactly c preimages S with [c]S = A.
- Each slot i takes a free point S_i ∈ E(F_p). The network constrains
  A_i = [c]S_i, F_i membership on A_i (the hash is evaluated on the subgroup
  point), and A_1 + … + A_m = R with R ∈ G.
- **Count identity.** For every target R, the cofactor network's count equals
  c^m times the count of the same network with the A_i as inputs. This is
  exact, and it is the extension's main test.
- **Witnesses.** A witness of the cofactor network projects to a relation
  among the A_i, so relations, the matrix and ξ = −U/V all live mod r, not
  mod #E.
- **Factor-base sizing.** |F_i| ≈ (2r)^(1/m) over G; the hash's bound is
  set from r. The SHA-256 bases enumerate G (toy shortcut) rather than E.

## Design, module by module (Rust items to add or change)

| Paper / reference item | Planned Rust item | Notes |
|---|---|---|
| Curve with certified order c·r, generator in G | `Curve` gains `cofactor: u64` (1 for every existing curve); `random_cofactor_curve(rng, b, c)` finds #E = c·r with r prime, r > c, and takes G = [c]·(random point) ≠ O | Naive counting, toy sizes only, as `random_prime_order_curve`. `order` keeps meaning r; a new `group_order()` gives c·r |
| Subgroup membership of targets | `Curve::in_subgroup(pt)` (`[r]pt = O`) | The solver rejects targets outside G |
| `clear_cofactor` (A = [c]S) | Whole-point network: a `FuncVertex` from S_i's 2b+1 bits to A_i's bits, computing `curve.mul(c, S)` | Degree 2(2b+1) per projection vertex, like the existing add vertices |
| Free source points | `encode_point` over all of E for S_i; `decode_point` unchanged | O stays encodable, since [c]O = O ∉ F_i |
| Membership on A_i | The existing `SupportVertex` (SHA-256) or affine predicate, now on A_i | Factor bases list G, not E |
| Relation network | `relation_network` builds S_i → [c] → A_i → membership → sums when `cofactor > 1`; unchanged when c = 1 | (2m−1)(2b+1) + m(2b+1) edges |
| Gadget [c] | `Builder::scalar_mul_const(ar, c, pt)`: double-and-add over the bits of c with `complete_add` (handles O and doubling) | O(log c) complete additions per slot. Degree stays O(b^(3/4)); edges O(m·b·log c) |
| Gadget relation network | `gadget_relation_network` inserts `scalar_mul_const` per slot; the S_i hint enumerates E | Still no support vertex |
| Solver | Works mod r; collects relations from witnesses projected to A_i; final check `[ξ]G = Q` | `Stats` unchanged |
| `GadgetShapes` | Adds a `cofactor_clear` shape for c ∈ {2, 4, 8} | `ecdlscale` table unchanged for c = 1 (byte-identical CI diff kept) |
| ecbench | `ic.tensor_contraction_v1` keeps refusing c > 1. A new id, `ic.tensor_contraction_cofactor_v1`, is added only if the owner wants it measured | ecbench rule: never change what a registered id counts |
| CLI | `ic tensor demo --cofactor C` | Prime-order default unchanged |

## Tests, in the same parity style

Each test carries a marker naming the identity it checks (there is no Go
original to cite).

1. `random_cofactor_curve` for c = 2, 4, 8 at 8–12 bits: #E = c·r with r prime
   and r > c, [r]G = O, G ≠ O, and every point satisfies [c·r]S = O.
2. `[c]` projection: over every point of a small curve, exactly c preimages
   for each A ∈ G and zero for A ∉ G.
3. Whole-point count identity: cofactor network count = c^m × subgroup-input
   count, for m = 2 and 3. Targets include R = O and targets reachable only
   through doubling or O. Checked with `SparseContractor` and
   `BruteForceCounter`.
4. Exhaustive gadget `scalar_mul_const`: for every input point of a 4-bit
   curve with c = 2 and c = 4, exactly one assignment survives, and it equals
   `Curve::mul`.
5. Gadget count identity: gadget network count = whole-point cofactor
   network count for the same factor bases (the existing
   `gadget_relation_counts` pattern), m = 2.
6. `solve` and `solve_with_gadgets` recover ξ on cofactor curves (c = 2, 4)
   and reject targets outside G.
7. Shapes: edges per bit and max degree / b^(3/4) stay bounded with c fixed;
   the c = 1 `ecdlscale` table stays byte-identical.
8. Regression: every existing test (24 in the crate, 3 in `ic_tensor`,
   2 ecbench arm tests) passes unchanged.

## What does not change

- Prime-order behaviour, Go parity, the `ic.tensor_contraction_v1` charge and
  its committed specs and protocol.
- The framing: toy sizes, our own gadgets, no speedup, no attack on deployed
  curves (P-256 has cofactor 1 and Ed25519 has cofactor 8, but both are far
  beyond toy sizes).

## Open questions for review

1. Re-read the paper's cofactor paragraph against the pinned PDF. In
   particular: does it hash A_i or S_i, and are the factor bases sized from r
   or from c·r? The sibling crate hashes A_i and sizes from r.
2. Should the ecbench arm get a cofactor id and a measurement? That would be
   a separate protocol.
3. Keep `order` meaning r (least churn) or rename it (`subgroup_order`) for
   clarity?

## Estimated size

About 400–600 lines of Rust (curve, relation, pointgadget, solver, CLI flag)
plus about 300 lines of tests. One PR. The crate's CI covers it.
