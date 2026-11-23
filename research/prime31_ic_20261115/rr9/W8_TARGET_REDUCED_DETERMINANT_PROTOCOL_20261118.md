# EXP-PRIME31-W8-DET8 — target-reduced elliptic relation determinant

Date: 2026-11-18 UTC. Continue [PR #4728](https://github.com/isogeny-labs/crypto/pull/4728) and the original ordinary cofactor-one curve, `p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, G=(625037715,2100627251). Reuse the unchanged `prime31_algebraic_ic` Rust library (source `1df714645aa683cf975bc76fcacb9842cb4780fd`), not new elliptic arithmetic.

## Proposed exact identity, not a solver claim

For R=(r,s) != O and eight supplied affine points P_i=(x_i,y_i), set an 8x8 matrix T_R whose ith row is

`[(x_i-r), (x_i-r)x_i, (x_i-r)x_i^2, (x_i-r)x_i^3, (y_i+s), (y_i+s)x_i, (y_i+s)x_i^2, (y_i+s)x_i^3]`.

Let E_9 be the prior classical [9x9 L(9O) evaluation matrix](RR9_DETERMINANT_PROTOCOL_20261116.md), on `P_1,...,P_8,-R`, with columns `[1,x,y,x^2,xy,x^3,x^2y,x^4,x^3y]`. Claim `det(T_R)=det(E_9)` **as field elements**, not just zero equivalence: the column basis transformation to `[1,(x-r)x^0,...,(x-r)x^3,(y+s)x^0,...,(y+s)x^3]` has determinant +1, and the last row is [1,0,...,0].

For nine **pairwise distinct affine points**, the Riemann--Roch divisor test therefore gives `det(T_R)=0 iff sum_i P_i=R`. Both R=O and repeated/overlapping points require a different branch (9x9 confluent jets as in prior research); the raw determinant alone has false-zero degeneracies.

This is algebraic **coefficient elimination** on *supplied* signed leaves. It neither finds their x indices nor solves ECDLP. It is closely related to the pre-existing 9x9 determinant and the 7-variable signed chart; no novelty claim for the Riemann--Roch identity.

## Frozen original-curve fixtures and pass criteria

Use the first 31 canonical rational-x points and positive weights `[1,3,5,7,11,13,17,19]`, both from the established #4728 Rust example; reject if 248 weighted positive abscissae are not distinct. For each t in 0..15 and j in 0..7 choose `i_j=(3t+7j+j*j)%31`, signed point `epsilon_j [w_j]P_(i_j)`, where `epsilon_j=+1 iff ((t+3j)%4)<2`, otherwise -1. Define R as their public sum **outside both predicates**. Three cases per t: (a) eight original points at R, (b) negate first point but hold R, (c) hold points but shift target to `R+G`. Require 16/16 positive determinant identities at zero, 16/16 first-point mutation identities with nonzero values, 16/16 shifted-target identities with nonzero values, and 48/48 exact numeric `det8==det9` comparisons. Reject infinite, repeated or -R points rather than accepting a spurious relation.

Adversarial controls: for one fixture, repeat one supplied point (two identical rows) and, separately, place -R among the supplied rows. Raw 8x8 determinants vanish even when the nine-point sum equation is not valid; the strict wrapper must **reject** these degenerate inputs. Independent Python BigInt arithmetic must reconstruct all targets/rows/determinants, validate each emitted residue and reject a deliberately corrupted record.

A local, independent Python preflight before this registration already obtained 48/48 exact det8=det9 identities; all 16 positives were zero and both negative classes were nonzero. This is preflight only: no new Rust compilation or runtime is claimed.

## Implementation and cost gate

Add one original-crate native Rust example with finite-field Gaussian elimination for 8x8 and 9x9, deterministic input generation from existing `Arithmetic` (only outside the public predicate), source-exact tests and deterministic JSON. Add a separately written Python replay checker, a GitHub Actions workflow using the immutable native Rust source, original library tests, focused tests and artifact. Compare field inversions/multiplications for both determinants; runtime is a diagnostic only, not an ECDLP improvement. No paid compute.

## Next genuinely non-generic extraction test

Exploit the eight **slot-labelled** determinant rows as a polynomial circuit over signed slot selectors rather than enumerate 4+4 group tables. On B8, encode each of 16 signed alternatives per slot with four Boolean bits (32 bits total); multilinear bit interpolation gives each matrix entry degree at most four in its own four bits, and determinant total degree at most 32. Compare the factored determinant circuit vs the seven-coefficient eight-resultant system and chart-plus-incidence F4, with certified no-solution/timeout on the existing 64 natural B8 oracle targets and eight planted positives. Keep the generic oracle accessible only to a grader. Count all F4/SAT work, memory, query failures, and algebraic false positives. Advance to B31 rank31/1,024 blinded unknown-Q logarithms and a same-host shared BSGS/mixed-IC/rho comparison only if an extractor produces unseeded relations without table enumeration.

## Prior art and limitations

Checked original Rust #4466 9x9 determinant, jet/Hermite controls, #4539 signed chart, #4692 unknown-chart extractor, #4717 root multiplicity, #4720 signed pair projector, #4728 weighted disjoint-slot protocol and 64-target truth oracle, #4714 complete M6 and mixed/BSGS negative controls. Published [Amadori--Pintore--Sala (2018)](https://doi.org/10.1016/j.ffa.2018.01.009) and [Semaev complexity bounds](https://www.degruyterbrill.com/document/doi/10.1515/jmc-2019-0029/html) are relevant prime-field index-calculus prior art. The research-KB MCP/wrapper is unavailable in the current connector runtime; this is a bounded audit, not proof of novelty. PCC signed independent validation is pending.
