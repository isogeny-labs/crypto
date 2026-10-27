# Round10 validation

Raw receipts,including failed attempts,are in runs.tar.xz.

| Check | Receipt | Outcome |
|---|---|---|
| Root library | root-lib-tests-01.log | 4405 passed,0 failed,118 ignored |
| Native study | native-tests-02.log | 106 passed,0 failed,1 ignored |
| New replay | replay-02.log | All13 canonical JSON outputs byte-identical |
| Prior round09 | prior-round09-replay-01.log | All five outputs unchanged |
| F2 templates | q8-F2-polynomial-templates.json | 129600 basis products,360 Frobenius basis checks,64 dual-number controls |
| Four polynomial models | q8-graph-0..3-F2-relations.json | 1679760 total degree≤2 equations;3359520 equation-component and144000 unit checks |
| Parsed models | q8b-parsed-F2-presentation-check.json | All systems recomposed,34560 input coefficient values recovered |
| Literal F2 polynomials | q8c-expanded-F2-equations.json | 72 sparse equations,4608 local gate controls |
| Projective prototype | q8p-projective-saturation-prototype.json | False naive boundary point;all three S-pairs reduce to zero after repair |
| Coupled cusp strata | m13-coupled-cusp-valuations.json | All480 cusps,eight pole divisors,all cusp zeros and local jets |
| Complete product divisor | m13b-complete-product-divisor.json | 22 closed cusps,2640 zero/pole degree,zero residual and projected images |
| Collision screen | m14-Phi11-collision-resultant.json | Exact degree96 resultant and factorization;2046 gcd controls |
| SS diagonal support | m15-supersingular-diagonal-component.json | Actual automorphism binding,3080 classes,400 matching labels;460 multiplicity gap remains |
| Lean build | lean-build-04.log | Pinned project passes,23 new named proofs |
| New transitive audit | lean-axioms-03.log | 23 theorems,0 failures,0 native trusts |
| Full proof gate | lean-full-gate-01.log | 146 project theorems pass,0 native trusts |
| Dashboard/site | dashboard-check-01.log,site-tests-01.log | Current pins and49 site tests pass |
| Workflow parse | workflow-tests-01.log | Four tests pass,focused25-minute cap unchanged |
| IDC issuance | record-01.log | Two Lean-bound identities,64 issuer and128 extra checks each |
| Source/raw hashes | record-source-hash-check-01.log,record-raw-hash-check-01.log | All ledger bindings checked |

Build01 used a nonexistent field hex method;build05 needed a BigUint type
annotation and borrowed polynomial slices. Collision01 passed an untrimmed
specialization to the small-field gcd atj=1;the accepted rerun trims it.
Lean build01 needed an explicit associative cancellation step. The original
q8b counter called8640 coefficient values blocks;its raw JSON/log are
preserved and the canonical metadata corrected without changing arithmetic.
Replay01 covers the original counter label;replay02 and the final archive
replay cover all13 canonical outputs. Warnings were corrected before the
final build. No failed attempt supports an accepted theorem.

Naive homogenization is rejected by an actual false projective point.
The repaired prototype's saturation is complete;the whole-quotient
projective saturation is not computed. The diagonal degree-exhaustion
attempt is incomplete:400 lower multiplicity versus860 required.
All these partial or failed hypotheses remain visible in the ledger.

Exact polynomial checks use split evaluations in K tensor K. Gate witness
values are determined by the parsed program;residuals,units and trace-dual
input recovery are checked. This is same-agent verification with shared
primitives,not an external audit. Finite fixtures alone do not prove the
scheme isomorphism. Its arbitrary-algebra argument,affine descent,Tate
cusps,Hurwitz module identification and degree bound are written in the
proof map. Native finite-field semantics and the global geometric bridges
are not fully Lean. lean4checker is unavailable.

The full proper projective quotient,boundary gluing and polarization on
that model remain open. Seven coupled noncuspidal supports,the460 diagonal
multiplicity,antidiagonal,explicit full torsion coordinates,Jacobian
identification and individual descent remain open. No relation rank or
IC cost is promoted. Canonical narratives and dashboard metadata are
updated;cost/rho graphs remain unchanged because no cost was measured.
The editable study diagram displays the recorded construction and divisor
strata. Remote CI/review is checked on the pushed commit separately.

The final archive passes its digest,all61 member hashes,all13 canonical
fresh outputs and a byte-identical deterministic repack,as recorded in
ARCHIVE_VALIDATION.txt. The ledger binds72 sources and58 ordinary raw
files;the three native assembly/hash receipts are additionally covered by
the archive manifest. Both IDC certificates are issued and checked.
Every page of the five-page PDF was rendered and inspected for clipping,
overlap,math symbols,links and legibility. Editable SVG/PNG/Typst sources
are retained. No earlier evidence archive changed and no background
experiment is scheduled.
