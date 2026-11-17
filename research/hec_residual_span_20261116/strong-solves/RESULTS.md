# Prime-order Jacobian controls pass; tenfold remains unsupported

The once-only run at pushed checkpoint `720ef8cc584` exited 0. The integer Frobenius-determinant certificate gave #J=ell, prime, on all five selected models. All 160 fresh calls over 20 public source/target workloads recovered verified scalars; no inputs or workload-arms were missing or incomplete.

Repeated outcomes and operation vectors matched exactly. Reduction identities, batched/scalar Pencil determining prefixes, Folded/Expanded determining prefixes and source-normalization flags all agreed. Every model accepted by the preceding frozen strength screen was included.

| Genus | p | c | Certified #J=ell | Point columns | Orbit columns | Folded/Batch base-field REDC | Folded/Batch inverse calls |
|---|---:|---:|---:|---:|---:|---:|---:|
| 2 | 101 | 2 | 7,481 | 35 | 7 | 1.096756–1.577693 | 1.342593–1.859155 |
| 2 | 251 | 18 | 67,931 | 135 | 27 | 0.992878–1.498566 | 1.834862–2.362069 |
| 2 | 601 | 7 | 411,491 | 340 | 68 | 1.881253–2.166771 | 2.769784–3.796610 |
| 3 | 113 | 3 | 1,247,429 | 49 | 7 | 1.806560–2.395078 | 2.333333–3.294416 |
| 3 | 421 | 2 | 77,152,909 | 217 | 31 | 2.949565–4.268808 | 4.561538–7.680952 |

Each range spans four separate source/target workloads. These are specified primitive counts, not wall-time speedups or a weighted complete-cost model. A ratio below one indicates a Batch regression. All methods retain the same shared setup, public source information and charged normalization. Batching changes denominator arithmetic while preserving the stream.

## Order certificate

Under the established binomial-curve formula pi=P(phi), with automorphism characteristic polynomial S_(2g+1), the whole Jacobian order is det(1-P(phi)). The new order.rs constructs multiplication by P-1 on the integer basis of Z[X]/S_n. It evaluates the determinant with checked i128 Bareiss products and exact divisions. The Tate rank 2g is even, so det(P-1)=det(1-P). All five determinants equal the independently verified source prime order ell.

This does not infer #J from the point-order LCM. The separate Hasse-bound and rational-2-torsion argument in strength/RESULTS.md also forces #J=ell for these models. Both arguments use stated curve identities and standard theory; neither is a mathematical priority claim.

## Interpretation and limits

The source now generates the whole prime-order Jacobian group. Weak-subgroup selection therefore does not explain the remaining shortfall on this panel: the genus-2 reduction advantage is still about 0.99–2.17, and genus 3 remains below 4.27. This is stronger evidence against a tenfold claim for the current method.

The curves remain a structured binomial family. This experiment does not cover unrestricted generic curves or certify a genus-4 replacement. The separate strength/GENUS4-ROUTE.md derives why tame odd-order automorphisms force geometric decomposition in genus four and explains the missing qualifications for an even-order candidate.

Common curve/order qualification and public target construction are excluded equally from every provided-valid-instance interval. Fresh per-call setup, normalization, collection, linear algebra and final verification are counted. Field counters overlap and omit some integer and memory work. No wall-time, strongest-external, primary-instruction admission, novelty or tenfold claim is made. The earlier failed wall-noise qualification remains unchanged.

Targets use the fixed new seed 20261118; walk seeds are 2026111800+case. Solver budgets are unchanged. Expected scalars, cofactors and logarithm tables do not enter runtime APIs. Raw controls, fixtures, outcomes, execution identity and hashes are retained; whole-order receipts are in CONTROL.stderr, and Rust summaries are included. All raw hashes verified under Git custody while S3 is unavailable. No retry, field replacement or gate relaxation occurred. The broader tenfold genus-2+ objective remains open.
