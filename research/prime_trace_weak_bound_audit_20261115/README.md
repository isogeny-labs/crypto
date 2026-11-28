# Prime-field source theorem with actual cyclotomic degree

For every prime p≥5 and every nonsingular general Weierstrass model over Fp, Lean proves that FST2010's trace/gcd definition of supersingularity forces actual trace zero and actual point count p+1. Every integer r>2 dividing that count, including composite r, has exact modular order 2 and **actual cyclotomic extension degree 2**. No Hasse estimate, point count, trace-zero conclusion or modular-order/field-degree identification is assumed. An actual r=2 curve and field extension refute the literal unrestricted degree-2 statement. Signed standing remains unverified. [pcc:unverified]

The source explicitly defines trace as `q+1−#E(Fq)` and supersingularity by its gcd with q being different from 1. It defines embedding degree using the extension obtained by adjoining all rth roots of unity. This packet uses those definitions. It does not claim a new equivalence with alternate geometric p-rank, Hasse-invariant or endomorphism definitions. [pcc:unverified]

The point-count proof uses a weaker estimate than Hasse. Over any odd finite field of size q>3, a degree-3 polynomial f cannot have constant Euler-power function `f(x)^h`, where `h=(q−1)/2`. If constant ε, the actual polynomial `L=X^q−X` divides `f^h−ε`. Writing this as Lg, formal differentiation gives

```text
L · (f·g′ − h·f′·g) = f·g + h·ε·f′.
```

The right-hand side has degree less than q, so is zero. If g is nonzero, the degree of fg is at least 3, contradicting the derivative term's degree at most 2. If g is zero, the original polynomial equation contradicts the positive degree of f^h. Lean checks the root-factorization, derivative identity and degree argument. The original protocol's general-ε sign error was corrected in a separate pre-code log; the original draft is retained. No novelty claim. [pcc:unverified]

An explicit completed-square equivalence connects every actual nonsingular general Weierstrass model to the actual square-root fibers of a monic cubic. The actual Mathlib point count is one plus the sum of their cardinalities. Nonconstancy at ε=−1 and ε=1 rules out both extreme counts and gives

```text
1 < #E(K) < 2q+1,     −q < actualTrace(E) < q.
```

For prime p≥5, the source gcd condition implies p divides the actual trace. The strict interval then forces trace zero and point count p+1. This removes the explicit Hasse premise from the earlier conditional prime-field argument; it does not prove Hasse's theorem. [pcc:unverified]

The field in the degree theorem is literally

```text
IntermediateField.adjoin (ZMod p)
  {z : AlgebraicClosure (ZMod p) | z^r = 1}.
```

For r>2 dividing p+1, characteristic p does not divide r. An actual primitive rth root exists, every rth root is its power, and the entire root-adjoin is a simple algebraic extension. Finite dimension and its finite carrier are derived. Actual p-Frobenius squared fixes all root generators. Frobenius is nonidentity, because a fixed primitive root would have square 1 and force r to divide 2. Its order is 2; the actual finite-field Frobenius-order theorem identifies that order with `Module.finrank`. Thus the result concerns the actual extension field. [pcc:unverified]

The controls use actual curves and group operations. Over F3, the nonsingular curves `y²=x³−x±1` have counts 7 and 1 and traces −3 and 3; both satisfy the source supersingularity definition. They refute deleting the small-field guards. Over F5, `y²=x³+1` has count 6, trace zero and an actual point of order 2. Its square-root-of-unity field is the base field and has degree 1, directly refuting literal degree 2 at r=2. Over F7, `y²=x³+x` has count 8 and an actual point of order 8; its actual eighth-root field has degree 2. This is a positive composite-order control. These known examples are credited to the earlier cohorts and boundary packets. [pcc:unverified]

The closed main is `PrimeTraceWeakBoundAudit.prime_trace_weak_bound_certificate` in [PrimeTraceCertificate.lean](lean/PrimeTraceCertificate.lean). See the [proof map](proof-map.json), [source plan](sources.json), [source review](evidence/source-review.json), [one-page report](reports/report.pdf) and [diagram](reports/prime-trace.svg). Exact local **PCC-20261115-b69cc7** replay PASS at 2026-11-15T01:57:20Z; signed standing remains unverified. See [verification receipt](verification.json). [pcc:unverified]

Sources checked before approach: Freeman–Scott–Teske, [*A Taxonomy of Pairing-Friendly Elliptic Curves*](https://doi.org/10.1007/s00145-009-9048-z), primary723726bytes/57PDFpages, SHA2acd937be8567abfb26bb3861e3aaf34312270c36df9a86f91cf9676c3caf785. Printed230/PDF7 and237/PDF14 were fully text-read and visually inspected, covering the exact definitions, Remark2.2 and the prime-field claim. This does not mark all57pages read. Prior EquationCount, SquareFibers, TorsionBoundaries, CompositeTorsion, SourceFSTBoundary and procyclic proofs are credited; their original bytes are retained in the provenance archive and remain unchanged. Pinned official polynomial, affine-point, primitive-root, intermediate-field and Frobenius APIs were read. KB MCP was absent; fresh mechanism/negative/theorem wrapper and Conductor returned127. Prior/primary/official fallback was used; no failed-search absence claim. Copyright primary PDF/text/images remain local. [pcc:unverified]

The repaired prime-field source clause is proved. Alternate geometric supersingularity/p-rank/Hasse-invariant comparisons, the general nonprime-field trace classification and at-most-6 theorem, general Hasse bound and full geometric torsion-field bridge remain separate. No canonical corpus promotion or completion of all fourteen items is claimed. [pcc:unverified]

Reproduce with Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679:

```sh
cd research/prime_trace_weak_bound_audit_20261115/lean
lake build
```

From repository root run `tools/lean/proof-gate.sh research/prime_trace_weak_bound_audit_20261115/lean`. The whole gate passes56/0failures/0trusted native, with only propext/Classical.choice/Quot.sound. The complete1233-character written declaration has SHAd190809525ceaf4787000e559cc18b79c39b86f3c811bddb94579290d7c2c638; independent native processing reports1complete/0prefix/0unresolved. Source processing is procedural, without kernel parser/hash/physical-host execution refinement. No fresh independent lean4checker is available. [pcc:unverified]

Restore all140 retained files/56unique objects using `python restore_attempts.py /tmp/prime-trace-attempts`;25832-byte losslessXZ roundtrips every byte, plus five gate files. Thirteen snapshots retain eight failed states, including dependent-Sigma kernel metavariable errors and noncomputable polynomial-evaluation attempts. The full final13 logs have no warnings or errors. The one-page47699-byte PDF was fully visually inspected; all text is within bounds. No holes, custom axioms, native proof evaluation, raised limits, new global flags, linter waivers, frozen edits, cryptographic Python, new native cryptographic experiment, paid compute or agent upload. [pcc:unverified]

Published in existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743) as `30d9b3e98ead8c77fc973aa4442239373f56ae8d`, identical to local receipt tree `07f622bcb47a00018efa95a08059656990bf7039`; all44paths/7large-or-binary blob IDs match, anonymous fetch completed and fresh PRhead/bodyv116 confirmed. The nonempty598-row ledger prefix is preserved through599entries. Last01:15UTC CI scan concerned preceding Galois-character headfd8228; no later canonical-H1 or prime-field-head CI pass/signed promotion is asserted. The parent immutable-ledger fork remains needs-human; no sidepick/rechain/rebase/force/retarget. Publication metadata is outside frozen pinsets. [pcc:unverified]
