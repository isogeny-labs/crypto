# Exact point counts and embedding orders for a cubic family

Lean proves that, for every finite field K of cardinality q with q mod3=2,
characteristic different from2 and3, and every b≠0, the actual curve
E_b: y²=x³+b is nonsingular and has exactly q+1 points. The point-count
trace is0. Every divisor r>2 of its cardinality has modular embedding order2,
including composite divisors; the theorem also derives this for every
actual point of additive order>2. [pcc:unverified]

For every prime p≥5 with p mod3=2, the prime-field specialization derives
the characteristic guards. There is no upper p bound. This proves the
family in Galbraith Exercise9.10.4 and the trace/order consequences relevant
to original item3. It does **not** close the general geometric
supersingularity/Hasse/full-torsion-field proof. [pcc:unverified]

The construction is classical and credited to the source. Derive
3*((2q−1)/3)=2(q−1)+1; Fermat gives the two-sided cube inverse
x↦x^((2q−1)/3), including zero. Each y has a unique x, the cube root of
y²−b. The proof constructs an equivalence between actual Mathlib
nonsingular curve points and Option K, rather than assuming a count or
point correspondence. The discriminant is explicitly proved nonzero.
Lagrange supplies the actual point-order divisor, and q≡−1 gives exact
modular order2 when that order exceeds2. [pcc:unverified]

| Checked conclusion | Lean declaration in CubicPermutationCountAudit |
|:--|:--|
| Explicit cube inverse and permutation | cubeRoot_cube, cube_cubeRoot, cube_bijective |
| Actual curve discriminant and count | curve_nonsingular, actualPointEquiv, actual_point_count |
| Actual point-count trace0 | actual_trace_zero |
| Divisors and actual point orders | actual_divisor_order, actual_point_order_embedding |
| Prime-field specialization, characteristic guards derived | prime_field_count |
| Coefficient/characteristic/congruence controls | coefficient_zero_count, characteristic_two_count, congruence_one_count and their deleting_* negations |
| Actual r2 exception | r_two_control |
| Combined explicit binding | cubic_permutation_count_certificate |

All rows retain the exact hypotheses in the [proof map](proof-map.json),
[CubePermutation.lean](lean/CubePermutation.lean),
[CubicCurveCount.lean](lean/CubicCurveCount.lean) and
[CubicCountCertificate.lean](lean/CubicCountCertificate.lean).
[pcc:unverified]

The controls use actual nonsingular-point sets, even when the equation is
singular. On F5,b=0 the actual count is5 rather than6; on F2,b=1 it is2
rather than3; on F7,b=1 it is12 rather than8. Lean directly negates the
corresponding omitted-guard count claims. The actual F5,b=1 curve has6points
and r=2 divides its count, but ord₂(5)=1, directly negating exact2. Existing
F5 cusp/r2 controls and the textbook F7 count are credited. These do not
refute the qualified family theorem. [pcc:unverified]

The whole repository proof gate passes26audited declarations/0failures/
0trusted native. Main and all printed results depend only on propext,
Classical.choice and Quot.sound. The independent native official Lean
parser resolves1complete/0prefix/0unresolved and matches the full explicit
1230-character written certificate:
[pcc:unverified]

```
bb894ca30f297e8548c0a85fdbce66f660ae55191557c09b50f5f846b9c3dcaa
```

This native parser/hash observation is procedural evidence, separate from
the mathematical theorem and signed execution assurance. It is not a
kernel refinement of the shared verifier or parser. [pcc:unverified]

Six complete source/config build states and all logs/exits are preserved,
including exponent-orientation, predicate/private-instance and finite
WithZero/subtype failures:69files/32objects, lossless10268-byte XZ archive.
All successful and failed attempts remain distinct; no failed proof is
accepted. Five final gate files and native input/protocol/output are retained.
Exact local **PCC-20261114-2d6d52** replay PASS at 2026-11-14T15:39:33Z; signed standing remains unverified. See [verification receipt](verification.json).
See [sources.json](sources.json), [source review](evidence/source-review.json),
[binding observation](evidence/binding-check.json) and the fully inspected
one-page [report](reports/report.pdf). [pcc:unverified]

```sh
cd research/cubic_permutation_count_audit_20261114/lean
lake update
lake build
cd ../../..
tools/lean/proof-gate.sh research/cubic_permutation_count_audit_20261114/lean
python3 research/cubic_permutation_count_audit_20261114/export_binding_input.py --out /tmp/cubic-family-binding
bash research/statement_binding_audit_20261114/run_audit.sh /tmp/cubic-family-binding/manifest.json
python3 research/cubic_permutation_count_audit_20261114/restore_attempts.py /tmp/cubic-family-attempts
```

The metadata exporter only copies the frozen extractor's source/header
input; native Lean performs the independent comparison. Use a new export
directory and resolve the pinned audit dependencies first. [pcc:unverified]

Sources checked before code: Galbraith author chapter9 Exercise9.10.4p184
and Theorem9.11.2/Definition9.11.3/Example9.11.4p189; both selected full pages
read and visually inspected, PDF SHA f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495.
No full chapter/book read or novelty claim. Prior finite-field count and
CompositeTorsionAudit minus-one proof credited. Official Mathlib finite-field
Fermat, actual affine-point equivalences and ZMod APIs reviewed. KB/Conductor
unavailable; primary/prior/official fallback recorded. Washington mirror404
remains unread and is not used. [pcc:unverified]

Lean4.35.0/Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679, default settings.
No holes/custom axioms/native proof evaluation/global flags/raised limits/
waivers/new cryptographic experiment/paid compute/agent upload. General
geometric supersingularity, p-rank/endomorphism/Hasse equivalences and full
torsion-field identification remain open. A modular order statement is not
an unproved full geometric torsion-inclusion statement. The original
fourteen-item request remains partial. [pcc:unverified]

Public replay snapshot: commit`1657db79a690ea94a2ae72868cb86b8b657b82ac` has tree`95da42198e748e655d6c275ce943ebc82d70260f`, identical to local receipt00e01a094c8c88386b860b9315b1f1ee187999f2. Completed anonymous fetch/tree comparison and fresh ready PR/body confirmation. See [publication metadata](publication.json). Signed standing remains unverified. [pcc:unverified]
