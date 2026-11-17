# Actual F4 and F9 supersingular model checks

Lean proves point-count classifications and modular embedding-degree
bounds for **every nonsingular general Weierstrass model** over explicit
actual fields of sizes 4 and 9. The source supersingularity criterion is
exactly gcd(t,q)≠1, with q the proved field cardinality and
t=q+1−#E(Fq) the actual point-group trace. [pcc:unverified]

| Actual field | General coefficient tuples covered | Source-supersingular point counts |
|:--|--:|:--|
| F4 = QuadraticAlgebra (ZMod 2) 1 1 | 1,024 | 1, 3, 5, 7, 9 |
| F9 = QuadraticAlgebra (ZMod 3) (−1) 0 | 59,049 | 4, 7, 10, 13, 16 |

For each prime r dividing the actual point count, Lean proves
ordᵣ(q)≤6, coprimality with q and existence of an actual point of exact
order r. Explicit nonsingular source-supersingular controls have orders
7 over F4 and 13 over F9; both have exact modular degree 3.
[pcc:unverified]

## Actual fields and complete model coverage

The fields use Mathlib's explicit two-coordinate quadratic algebra. The
base polynomial has no root, checked in the actual prime field; the
library's field axioms and computable arithmetic apply. Cardinalities
4 and 9, characteristics 2 and 3 and both coefficient-population counts
are proved. No composite `ZMod 4` or `ZMod 9` is used as a field.
[pcc:unverified]

For F4, an arbitrary Weierstrass equation has at most two y-values for
each x: the monic quadratic fibre has at most two roots. A proved
sigma-fibre equivalence yields counted(E)≤2q+1. Positivity and the exact
trace-gcd condition then give the five counts above. This covers every
coefficient tuple by a structural proof; it is not a claimed native
1,024-row enumeration. [pcc:unverified]

For F9, **every general model** is treated by completing the square in
characteristic three:

```text
y' = y − a1*x − a3
a1' = a3' = 0
a2' = a2 + a1²
a4' = a4 − a1*a3
a6' = a6 + a3²
```

Lean proves the equation equivalence, explicit coordinate bijection and
inverse, and preservation of the counted point set. The 729 normalized
models have kernel-checked count classifications, split into nine
81-model obligations. Explicit exhaustion of all nine a2 values joins
them; the coordinate bijection transfers the result to all 59,049
general coefficient tuples. No assumption that every characteristic-3
model is short Weierstrass is made. [pcc:unverified]

The counted equation set is bound to **actual Mathlib curve points**
under the nonsingularity hypothesis. Source trace, integer trace-square
classification and modular-order lemmas then prove the bound and
coprimality. The Cauchy application keeps the equality decision explicit
so its point group matches the actual concrete caller's group.
[pcc:unverified]

## Proof map

| Module / namespace | Checked obligation |
|:--|:--|
| `QuadraticModels`, `QuadraticFieldModelsAudit` | Actual field/cardinality/characteristic and population sizes |
| `SourceTrace`, `FiniteFieldSourceTraceAudit` | Exact source criterion using actual field and point cardinalities |
| `FibreBound`, `WeierstrassFibreBoundAudit` | Monic quadratic root bound, explicit fibre equivalence, counted(E)≤2q+1 |
| `CompleteSquare`, `CharacteristicThreeNormalizationAudit` | Generic char3 equation equivalence, inverse coordinate map and count preservation |
| `NormalizedPredicate`, `NormalizedCase00`–`22`, `NormalizedAll` | All 729 normalized F9 models via nine exact kernel decisions and explicit a2 exhaustion |
| `QuaternaryClassification`, `NonaryClassification` | All general models; actual point counts, trace cases, prime-divisor modular bounds/coprimality/actual prime-order points |
| `ExtensionControls`, `QuadraticExtensionControlsAudit` | Actual nonsingular curves of orders 7/13, source trace criterion and exact degree 3 |
| `QuadraticCertificate`, `QuadraticFieldCertificateAudit` | `quadratic_field_model_audit_certificate` combines both complete model statements and controls |

## Source scope and assurance

FST 2010, printed pages 230–231, supplies the exact trace and embedding
definitions; its trace list on page 237 and Waterhouse 1969 Theorem 4.1,
printed pages 536–537, were reread. The exact primary hashes/read scopes
are retained. The Lean classification over these two fields is proved
directly, not assumed from Waterhouse. No claim of a full-paper read is
added. KB MCP tools were absent and a fresh wrapper status returned 127;
bounded primary/repository/Mathlib reads supplied the fallback. Conductor
unavailable. No novelty claim. [pcc:unverified]

This extends the earlier complete F2/F3 checks to the quadratic fields.
Higher extensions remain open. The geometric equivalence of the source
trace criterion with endomorphism or p-rank definitions and the full
torsion-field/roots-of-unity interpretation of modular order remain
separate formal obligations. The two degree-3 controls support the
qualified small-degree bound at their actual scope. [pcc:unverified]

Lean 4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are
pinned. Only propext, Classical.choice and Quot.sound are allowed.
No holes, custom axioms, native_decide, linter waivers or increased
elaboration limits. The first combined census reduction was interrupted
with SIGTERM/143 for resource use; its source, diagnostic and resource
snapshot are retained. Failed predicate unfolding, prime-field-only API,
group-instance and tactic attempts remain archived. [pcc:unverified]

The normalized-case import chain serializes large kernel checks during
ordinary Lake builds. Each case has its own exact decision proof. No
native experiment, new sampled certificate, timing claim or paid compute.
Issued parent sources remain unchanged. [pcc:unverified]

```sh
cd research/supersingular_quadratic_fields_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/supersingular_quadratic_fields_audit_20261110/lean
```

Source pins, full gate, preissue source commit, packet and actual local
verdict appear in sources.json and verification.json. Signed standing
remains unverified. See the [report](reports/proof-20261110/report.pdf)
and [editable proof diagram](reports/proof-20261110/proof.dot). Existing
performance, benchmark and corpus records retain their prior scope.
[pcc:unverified]

The full project gate passes 42 declarations, zero failures and zero
trusted-native exceptions. All 57 attempt/source/resource files and five
gate files have byte-equal archive roundtrip checks. [pcc:unverified]

Source commit `711ed6cc` precedes `PCC-20261110-520ce8`. Full local
verification passes at 13:57:53Z; signed standing remains unverified.
The inspected final report is two pages, 51365 bytes. [pcc:unverified]
