# Prime-chain proof dependencies

This is partial formalization work dated 2026-10-30, following the native
42-record certificate delivery at commit 4a7888bed8dc. It does not promote
those conditional FFD records or supply new LFD or regularity measurements.
The original Koblitz, binary and prime fixed factor bases remain unchanged.

## Sources and scope

The written proof is `PRIME_LEFSCHETZ_ARGUMENT.md`. The primary references
were checked again before this change:

- [Lundqvist and Nicklasson, Theorem 2.7 and Section 3](https://arxiv.org/html/1604.06820):
  for square-zero variables the socle degree is ell; characteristic greater
  than ell supplies maximal rank for powers of the sum. This is still an
  external mathematical dependency, not an imported Lean theorem.
- [Caminata and Gorla, Sections 1 and 4](https://arxiv.org/html/2112.05579):
  highest-form FFD, filtered LFD, solving degree and regularity are distinct.
  Formalizing the arithmetic below does not bridge these invariants.

The new file `LefschetzBoundary.lean` imports only Std. Its reproducible
toolchain is `leanprover/lean4:v4.34.1`, commit
`5045d0056413266e57c625dcd7c365b10e377c52`. Std is bundled with that toolchain;
there is no Mathlib or crypto-lean dependency. This file contains arithmetic
prerequisites only; no claim is made that the shared crypto-lean gate ran.

## Declaration map

All declarations have namespace `N83LefschetzBoundary`. All parameters are
natural numbers. Here r is a nonnegative coefficient degree, K a total
primitive degree, k an individual primitive degree, d the relation degree,
ell the block width, and e a monomial exponent. The equality r+4+K=d is
essential: using natural subtraction without it could admit negative
coefficient degrees as zero.

| Declaration | Exact mathematical content |
| --- | --- |
| `exponent_within_string` | r+4+K=d, k<=K and 2d<=ell+6 imply r+2+2k<=ell |
| `exponent_within_cap` | Same hypotheses imply r+2<=ell-2k |
| `monomial_exponent_within_cap` | Additionally e<=r+2 implies e<=ell-2k |
| `n83_through_degree16` | ell=27 and d<=16 imply that exponent bound for every admissible r,K,k,e |
| `below_quartic_no_coefficient` | d<4 excludes r+4+K=d |
| `odd_width_boundary` | ell=2h+1, K=k=h, r=0, d=h+4 gives cap 1, below exponent 2 |
| `n83_degree17_boundary` | h=13 yields d=17 and cap 1 |
| `source_characteristic_above_width` | The saved prime's integer value is greater than 27; primality is not asserted |

The degree-17 result proves failure of this uniform arithmetic bound at
the specified sector. It does not construct a polynomial syzygy. The saved
native upper witness is a separate artifact.

## Remaining obligations for the conditional FFD claim

| Written argument | Missing formal object or bridge |
| --- | --- |
| Section 1, external SLP | Define the graded square-zero algebra over the exact field; prove or import maximal rank of every power of L |
| Section 1, strings | Construct primitive spaces, prove the graded direct sum and each string's annihilator, including boundary degrees |
| Section 1, tensor sectors | Construct the graded S-module isomorphism with the tensor sectors; no ring-product assumption |
| Section 2, exponent bounds | Connect polynomial support to e<=r+2 and primitive indices to k<=K, then prove the quotient map injective on the relevant support |
| Section 3, regular sequence | Formalize the weighted leading monomials, product criterion, regular sequence and homogeneous Koszul generation |
| Section 4, transport | Prove sector maps commute with quartic multiplication and lift quotient syzygies through the selector-square ideal |
| Section 4, FFD convention | Define the finite-characteristic quotient and trivial syzygies, then show it agrees through degree 17 |
| Section 5, upper witness | Formally construct H, its nonzero class, annihilation and nontrivial terminal coordinate |
| Exact saved cases | Bridge file hashes and native source audit to those polynomial objects, nonzero weights and coordinate changes |

None of these missing obligations is inserted as a Lean axiom. Consequently
the full FFD theorem is not a declaration in the new file and remains open
for formalization and independent scientific review.

## Next dataset and claim boundary

For a filtered LFD claim, preserve an explicit lower-degree consequence and
a certificate that it is absent from the earlier filtered closure. The
existing highest-form witness alone supplies neither that nonmembership nor
a complete LFD. For actual PDP decompositions, separately verify rational
curve-point membership and the sum against the target using the immutable
factor-base manifests. Neither step can be replaced by a Semaev x-coordinate
relation alone. A new measurement requires its own frozen contract; this
proof-development addition launches no experiment and changes no raw result.

## File format and verification

`LefschetzBoundary.lean` is UTF-8 Lean source. The sibling directory
`validation/lefschetz-boundary/` retains the compiler output, toolchain
identity and verification metadata. Each `#print axioms` output belongs to
the named declaration above. Hashes bind the source and verification files.
The proof checker validates these arithmetic statements only.
