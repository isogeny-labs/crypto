# Native cubic cofactor inverse and exact raw-word bridge

The separately versioned Rust candidate replaces the two-Frobenius cofactor with the established polynomial P=(a²−3bc,3c²−ab,b²−ac) in the exact q223 context. All15 public fixtures pass the three preregistered development tests, with equal original inverse output and both product orders equal to one. The entire original common.rs is compiled unchanged as the reference dependency, SHA832fd288. Protocol/fixtures were normally committed and tagged at5f7c710d before candidate creation, data4dd1ec. All claims remain unsigned. [pcc:unverified]

## Implementation and declared accounting

`native-cubic-cofactor-inverse-06/src/lib.rs` validates the original q223,c3,w183,w2=39,size11089567 context and normalized coefficients. Zero inverse, noncanonical input, malformed parameters and zero norm have distinct explicit rejections. The cofactor uses six squares/cross-products, two constant products and three original Fp.sub calls. The candidate then uses the literal original norm block, original Fp.inv and original mul_fq. Original engine/population/runtime sources are untouched. [pcc:unverified]

| Original declared counters, MUL/INV/ADD | Frozen reference | Candidate |
| --- | --- | --- |
| Cofactor |15/0/8 |8/0/3 |
| Full inverse |23/1/8 |16/1/3 |

All15 rows match these prospective counter predictions, data0ebbb8. The literal original norm block charges5 MUL for four explicit word-product expressions and omits its additions. Both variants preserve that same charge/block. Validation/setup/division/compiler/counter overhead is unpriced. These are declared diagnostic units, with the historical discrepancy retained; no physical instruction count, calibrated cost, runtime ratio or end-to-end improvement is inferred. No benchmark was executed. [pcc:unverified]

The15 fixed public canonical inputs cover axes, mixed coefficients and boundary residues, SHA b574f4c9ebfda9e453241e72f1d509657ffdb436da29b8941348ea3283dea1e2. Every fixture checks the cofactor against the two original Frobenius images multiplied, exact original inverse equality, both inverse product identities, and base-valued nonzero cofactor product. All invalid-domain controls, five malformed parameter controls and the deliberate wrong-sign control pass. The original zero-inverse panic is caught as the expected negative behavior; its full stderr remains retained. This finite unit set is not a universal nonzero-norm proof. [pcc:unverified]

## Raw UInt64 refinement

`lean/CubicCofactorWordBridge22307.lean` preserves the exact old WordBridge01 prefix5743a021 and models the native multiply, canonical subtraction and exact cofactor expression order. Under input word values<223, products are at most49284, the subtraction adjustment at most445, and all outputs remain canonical. Decoding commutes with each multiply/subtract; the cofactor's three raw toNat outputs equal the Fin223 polynomial values exactly, rather than only agreeing after another modular reduction. Both unchanged original Lean4.35/4.34.1 gates audit42/0 failing/0 trusted each, data15a361:27 old,11 new named and4 new generated equations. Four nonfatal deprecated-if warnings per build are retained. No native decision, custom axiom or gate/resource relaxation is used. [pcc:unverified]

The model checks cofactor(1,1,1)=(221,2,0), cofactor(0,1,1)=(220,3,1), and a noncanonical2^63-times2 overflow counterexample to an unconditional decode homomorphism. Full bindings and hypotheses follow. [pcc:unverified]

| New declaration | Complete unsigned binding |
| --- | --- |
| `CubicCofactorWordBridge22307.word_mul_value` | `PCC-20261116-c44007`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_mul_canonical` | `PCC-20261116-dd35b8`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_sub_value` | `PCC-20261116-102d1d`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_sub_canonical` | `PCC-20261116-97daa6`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_mul_decode` | `PCC-20261116-7d4fda`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_sub_decode` | `PCC-20261116-cb3e26`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_cofactor_canonical` | `PCC-20261116-c286e4`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_cofactor_decode` | `PCC-20261116-2b087a`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.word_cofactor_exact_values` | `PCC-20261116-265ccd`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.canonical_word_controls` | `PCC-20261116-839962`. [pcc:unverified] |
| `CubicCofactorWordBridge22307.noncanonical_overflow_control` | `PCC-20261116-e9369d`. [pcc:unverified] |

This correspondence covers normalized cofactor word arithmetic. It does not prove compiler equivalence, the native norm block or Fp.inv loop, universal nonzero norm/field construction, original counters, Jacobian maps or running time. Full04 already supplies the abstract ring product/norm/conditional inverse implication60bdae, and trace05 remains a separate additive descent criterion1421da. Next formal work is to remove the supplied nonzero-norm/base-inverse gap for q223 using the existing no-base-cube-root certificate and finite base-field units. [pcc:unverified]

## Checks, evidence and primary sources

First native release-unit command is retained in the entire run tree: Rust1.99.1, offline, one compiler/test worker, timeout120s, all3 tests pass, no ignored/measured/filtered test. Whole source, hardware/cgroup/tool versions, command, stdout/stderr, original-byte checks and target outputs remain in `evidence/native-cubic-first-unit-and-broad-lint-negative-03/entire-native-unit-and-Clippy-run-tree.tar.xz` (381024B). The archive was assembled after the broad lint attempt and includes those debug outputs; it is not a pre-lint cache snapshot. Build/test elapsed fields are not performance evidence. [pcc:unverified]

Broad cargo fmt1 and strict Clippy101 fail on immutable historical reference formatting/Rng::next/manual_is_multiple_of. These failures remain0ebbb8; the historical reference is not edited or suppressed. Required new-code scope then passes package-only format and no-deps/all-target strict Clippy0, data495eb9. All original test assertions and code bytes remain. New CI replays exactly this new-package scope and all15 unit fixtures, and both original proof gates preserve j1/120s/default proof memory. [pcc:unverified]

Sources checked before code3eafa4: entire original common.rs and full04/old WordBridge, official Arkworks algebra v0.5.0 ff/src/fields/models/{fp3.rs,cubic_extension.rs}, exact Git blobs96007f3fa4f31aaab58d44d6397f34add3c3be7d/1049908bf99900c0e510617f3cc07d9894b84ad6, full Apache/MIT licenses. Arkworks already implements this cofactor method and documents the negative sign in s2; no novelty or fastest claim. Its cited Algorithm17 paper was not itself reviewed. Raw primary sources, responses, licenses and hashes remain in native-cubic-cofactor-primary-review-01. Word refinement preregistrationbcce0b reads the actual UInt64/Fin interfaces. KB status/exact/negative/adjacent/theorem wrappers127 and conductor127; locked atlas unread, no absence inference. [pcc:unverified]

Canonical chain.dot,cubic.dot and F101 psi.dot are unchanged: this implements arithmetic without adding a geometric transfer edge. Current c91 whole ownCI3dd59d succeeds on six proof components but original S3 pulls/migration still fail403 and block normal merge. Original full scientific LP/360/witness/controls/feedback/post, runtime/admission and matched-cost obligations remain open under paide42949STOP. [pcc:unverified]
