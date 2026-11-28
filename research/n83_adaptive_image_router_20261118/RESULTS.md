# Projector-only adaptive routing for the n=83 image

## Result

The norm/projector is a correct image-membership mechanism, but by itself it
is not a new relation router.  It is constant on every pushforward-kernel
coset:

```text
K in ker(phi_*)  ==>  p(D + K) = p(D),
p = [3^(-1) mod r] phi^* phi_*.
```

This theorem is Lean-checked without axioms and has claim
`PCC-20261118-81b046` `[pcc:PCC-20261118-81b046]`.  The consequence is scoped: projecting an ambient
class retains its elliptic norm label and discards its Prym/kernel residue.
It does not rule out a future adaptive construction with an additional
source-label, descent, or controlled Prym-cancellation oracle.

## Exact exhaustive control

The native audit reconstructs all 486 classes of the existing `GF(8)`,
`a=1`, `b=3` fixture and pushes each class through the general closed-point
norm.  The six elliptic labels have exactly 81 preimages apiece.  It then
enumerates all `486^2` ordered pairs.

| Exact population | Count |
|---|---:|
| ambient Jacobian classes | 486 |
| elliptic norm labels | 6 |
| classes per norm label / kernel size | 81 |
| ordered pairs | 236,196 |
| arbitrary-class homomorphism pairs / failures | 236,196 / 0 |
| pairs whose elliptic labels sum to zero | 39,366 |
| pairs whose Jacobian classes sum to zero | 486 |
| projected-zero but nonzero Jacobian pairs | 38,880 |
| exact lift fraction among projected-zero pairs | `486/39,366 = 1/81` |

These are deterministic finite-fixture counts from
[`certificate.json`](certificate.json), SHA-256
`cf4643eb1a9798a479ecbcce7bca3829e6a8f83c482799387ef5dbae8ab49b45`.
They have not had an independent replay and are therefore
`[pcc:unverified]`.  They are a correctness/ambiguity control, not an
estimate of n=83 relation probability.

The positive control checks that every exact inverse pair projects to inverse
elliptic labels.  The negative control exhibits a nonzero kernel class whose
pair with the identity is projected-zero but not zero in the Jacobian.

For a surjective homomorphism of finite groups `N : J -> E` with kernel `K`,
the observed counts have the forced form

```text
projected-zero ordered pairs = |J| |K|,
exact-zero ordered pairs     = |J|.
```

Here `|J|=486` and `|K|=81`, giving `39,366` and `486` exactly.

## n=83 replay

On the pinned order-`r` point, with

```text
r = 2417851639230796216685689,
3^(-1) mod r = 1611901092820530811123793,
```

the same native certificate checks:

- `[r]P=O` for the pinned nonzero point;
- the general arbitrary-class norm gives `phi_*phi^*(P)=[3]P`;
- the normalized projector is idempotent;
- it fixes `phi^*(P)`;
- the complementary kernel component is zero.

The abstract Lean proof also checks that an exact Jacobian pair relation
pushes to an elliptic relation, and that pullback is injective whenever the
push-pull composition is injective.  On the order-`r` subgroup the latter
composition is `[3]`, and `gcd(3,r)=1`.

## What this closes

| Proposed use of the projector | Gate result |
|---|---|
| Project before relation collection | The factor base lies in the transported image. Pullback is injective on `r`-torsion, so its relation module is the source elliptic relation module. The projector adds no rank. |
| Collect exact ambient Jacobian relations, then project | The projected relation is valid, but the projector does not reduce the cost of finding the exact ambient relation. The prior full-Jacobian screen still applies. |
| Accept a projected elliptic relation as an ambient relation | Invalid without a kernel/Prym cancellation witness. The exhaustive control has an 81-fold ambiguity and 38,880 false projected cancellations. |
| Increase the degree of a reduced Mumford factor base | A reduced genus-three Mumford representative has support degree at most three. A genuinely growing degree needs non-reduced effective divisors, Riemann--Roch data, or another representation. |

Thus the projector-only adaptive candidate stops at the formula/information
gate.  No timing experiment, matrix construction, or individual-log stage is
admitted.

## What remains open

A broader growing-degree or adaptive target-local design remains unclassified
only if it supplies a mechanism not contained in `p` itself.  At minimum it
must specify:

1. how non-reduced/effective divisors are generated and represented;
2. how projected factor-base labels are obtained without already solving the
   source discrete logarithms;
3. how Prym/kernel residues are made to cancel at useful probability;
4. independently checked rank after projected-label merging;
5. relation collection, sparse matrix, memory, blind individual-log descent,
   and final verification costs in the actual order `r`;
6. a finite crossover against the strong source rho control.

The next useful experiment is therefore not “project more ambient relations.”
It is a concrete oracle proposal satisfying items 1--3, first on
`n=9,13,19,23`, with a closed cost model before any n=83 timing.

## Validation scope

The task-local release test passed (`1` passed, `0` failed), and a second
native run reproduced `certificate.json` byte for byte.  The Lean proof gate
passed all 14 audited theorems with zero trusted declarations.  The PCC claim
and 1,368-entry ledger chain passed their independent checks.

The repository-wide `cargo test --release --lib` target does not compile on
the pinned base: `src/cryptanalysis/ic_framework/plugins.rs` defines
`the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` twice, at lines
5285 and 5322.  That file is unchanged from `origin/main`; this study neither
introduces nor repairs the unrelated baseline defect.

## Sources checked

- C. Diem, [*Index calculus with double large prime variation for curves of
  small genus with cyclic class group*](https://arxiv.org/abs/math/0606607).
  Its factor base and relation construction concern the full class group and
  rational curve points; they do not provide image labels.
- D. Lombardo, E. Lorenzo García, C. Ritzenthaler, and J. Sijsling,
  [*Decomposing Jacobians via Galois covers*](https://arxiv.org/abs/2003.07774).
  It identifies the Prym as the connected component of the pushforward
  kernel, which is precisely the information erased by projection.
- [General norm/projector and Prym audit](../n83_jacobian_projector_20261031/RESULTS.md).
- [Degree-two image-closed gate](../n83_image_closed_degree2_20261101/RESULTS.md).
- [Full destination-calculus screen](../n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md).

The research-KB connector/CLI, the relevant lead-atlas entries, and the P1546
artifact were unavailable as readable text in this environment (`VLT1`
ciphertext for the latter records).  No novelty or absence claim is inferred
from that gap.
