# Formal scope

The general contraction and ECDLP claims are open. These projects check the
listed mathematical statements, not a runtime bound or a general native
implementation. Written proofs and independent reviews retain their full
scope even where only some lemmas are formalized.

| Written statement | Lean target / declaration | Status |
| --- | --- | --- |
| Actual two-by-two Boolean-kernel composition entry is4 | `lean-gauges/GaugeCounterexample.lean`, `composed_kernel_entry` | Checked |
| Kernel of the actual matrix product has entry6 | Same, `kernel_product_entry` | Checked |
| These Boolean matrices are unequal | Same, `kernel_not_multiplicative` | Checked |
| General true-gauge identity and determinantal-ideal invariance | No full Lean theorem here | Written proof, independently reviewed |
| Exterior composition, degree and integral-chart transfer | No full Lean theorem here | Written use of standard determinant identities |
| Squarefree update expansion and its counting envelope | No full Lean theorem here | Written proof with explicit scope; finite minors checked natively |
| Generic grouping-sign rank and intermediate-hole witness | No full Lean theorem here | Written proof and exact native controls; no actual-Cauchy impossibility inference |
| Actual global-parity Cauchy normalizer, chart coefficient, code membership and passive product | [lean-exterior/PROOF_MAP.md](lean-exterior/PROOF_MAP.md), seven named `ExteriorWitness` declarations | Checked, including fresh replay |
| General normalized passive rank2(L−1) | No full Lean theorem here | Written and independently reviewed; fixed L checked natively |
| Syndrome projection and weighted-pair diagonalization | [lean-coded/PROOF_MAP.md](lean-coded/PROOF_MAP.md), six named `CodedProjection` declarations | Checked under explicit homomorphism/injectivity hypotheses, including fresh replay |
| Abstract exact-filter assembly is injective and obeys the full dimension bound | [lean-packing/PROOF_MAP.md](lean-packing/PROOF_MAP.md), four named `CodedPacking` declarations | Checked over a field with exact cross-annihilation and injective diagonals, including fresh replay |
| Code cardinality, polynomial selectors, valuations, Smith factors and density costs | No full Lean theorem here | Written analysis; finite specialization checked separately |
| Common sparse factorization, full recursion, sub-exponential ECDLP algorithm | None | Not established |

All projects pin Leanv4.35.0, shared CryptoLean
`148f5dc389b341401e06d76abe7783c4ccf42f2e`, Mathlib
`cfc55fc58714a59dc06e6e79edc6d1d95140d3c2`, and transitive revisions in
their Lake manifests. Only ordinary standard foundations are accepted:
`propext`, `Classical.choice`, `Quot.sound`. No `sorry`, custom research axiom,
`native_decide`, native-trust exception or kernel bypass is used.

The actual copied repository targets are staged before the shared proof
gate discovers their Lean sources. An initial pre-staging gate failed with
`FAIL_no_sources`; that infrastructure result is retained separately and
does not count as a proof failure or a passing check. Fresh import-inclusive
replays and exact axiom inventories are preserved in each frozen project.

There are20 explicitly named research declarations:3 Boolean counterexample,
7 actual finite Cauchy witness,6 general projector lemmas,4 abstract packing
lemmas. The automatic gate audits23 declarations because the projector
source also generates3 congruence lemmas. These counts are not a claim of20 independent novel
mathematical results. The binary code's concrete instantiation and its
Kronecker-product identification remain written bridges, as its proof map
states; the generic projection laws do not silently certify those steps.
The abstract packing theorem is fully proved with its field/vector-space
hypotheses. Its code-space application, capacity interpretation and
asymptotic cost substitution remain written corollaries. It is not a theorem
about arbitrary coefficient rings, singular matched maps or all codes.
