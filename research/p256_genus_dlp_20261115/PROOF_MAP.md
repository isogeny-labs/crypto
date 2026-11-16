# Proof map and standing

All new mathematical and execution records in this study are producer
assertions without an independently signed PCC verdict. [pcc:unverified]

Cover identity receipt IDs, in genera 2, 3, 4, 5 order:
`IDC1haf18ea018b3a563a`, `IDC1h3de0f175f52fb2b3`,
`IDC1h9ba55381e8877b58`, `IDC1h77461c8427f278a1`.

| Statement | Evidence / formal source | Exact boundary |
|---|---|---|
| Cleared cover equations, all four genera | `results/identities.json`, native exact polynomial checks, `lean/Identities.lean` | Identity receipts are evaluated over 2^61-1, not formal proofs. Native checks use actual P-256 p/b. Lean source is not locally compiled. |
| Smoothness and genera | Squarefreeness checks and written argument in REPORT | Concrete algebraic curves and their smooth projective models are not formalized in Lean. |
| Coprime-degree subgroup injectivity | Existing `research/cover_transfer_assessment_20261013/lean/CoverTransfer.lean`, declaration `CoverTransfer.pull_injective_on_subgroup` | Abstract AddGroup homomorphisms, exact finite subgroup cardinality, coprimality and composition identity are hypotheses. No concrete Picard/norm bridge is supplied by that theorem. |
| Actual P-256 image order and sample scalar relations, genera 3/4 | `results/native-p256.json` | Producer native arithmetic checks on the chosen generator; no independent signed execution verifier. |
| E x F2, E^2 x F3, E x A3, and J(C2) x J(H5) isogenies | Written quotient maps and differential independence argument | Jacobian functoriality, differential independence and the isogeny consequence are not formalized in this import. |
| Toy Frobenius factorization | `results/native-zeta.json`, frozen historical fixture | Exact q=7,b=1 member only; no actual-P-256 complementary-factor order. |
| Toy DLP recovery | `results/native-toy.json` | Known-answer tiny subgroups, not timing data or a P-256 solve. |

The four Lean declarations `P256Genus.genus2_identity`,
`P256Genus.genus3_cleared_identity`, `P256Genus.genus4_identity`, and
`P256Genus.genus5_identity` are faithful polynomial statements over a commutative
ring with `by ring` proofs, but remain **source-only**: this runtime has neither
Lean nor Lake and no local Mathlib environment. No compiled proof or axioms
receipt is claimed. They do not contain `sorry`, custom axioms, or
`native_decide`; absence of those strings is not itself proof checking.

The existing conditional subgroup theorem is read from base commit
`f47a44756c94f2a70eef0fa4155650e36c8ca2fb`; it was not rebuilt here. An actual
formal bridge needs definitions and checked constructions of the four smooth
projective curves, their Picard groups, induced pullbacks and norms, and the
degree composition theorem specialized to these maps.
