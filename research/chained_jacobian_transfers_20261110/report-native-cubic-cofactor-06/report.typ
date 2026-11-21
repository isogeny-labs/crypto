#set page(paper: "a4", margin: 17mm)
#set text(size: 10pt, font: "DejaVu Sans")
#set par(justify: false, leading: 0.6em, spacing: 0.7em)
#set heading(numbering: none)
= Native cubic cofactor inverse
#text("The separately versioned Rust implementation passes all15 frozen public inputs and all3 preregistered development tests against the byte-identical original reference. The established cofactor formula also has an exact normalized UInt64-to-Fin223 proof. Protocol/fixtures were committed/tagged5f7 before code. Claims unsigned. [pcc:unverified]")
== Exact context and calculation
#text("Validate q223,c3,w183,w2=39,size11089567 and a,b,c<223. Compute P=(a²−3bc,3c²−ab,b²−ac), retain the literal original norm block, original Fp.inv and final mul_fq. The reference is the complete unchanged common.rs832fd288 compiled as a path dependency. [pcc:unverified]")
#image("cofactor-word-boundary.svg", width: 100%)
#table(columns: (46%,27%,27%), inset: 6pt,
 [Declared MUL/INV/ADD], [Original], [Candidate],
 [Cofactor], [15/0/8], [8/0/3],
 [Full inverse], [23/1/8], [16/1/3],
)
#text("All15 rows equal the prospective declared-counter predictions0ebbb8. The original norm block charges5 despite four explicit word-product expressions, and omits additions; both variants retain the same block/charge. Setup/validation/division/compiler/counter overhead is unpriced. These diagnostics are not physical instruction counts, calibrated cost, runtime ratios or end-to-end gains. No benchmark ran. [pcc:unverified]")
== All fixtures and controls retained
#text("Every canonical axis/mixed/boundary tuple checks the original Frobenius-product cofactor, exact inverse output, both inverse product orders, and a base-valued nonzero cofactor product. Zero/noncanonical/five malformed parameter controls and a deliberate wrong-sign control pass. The expected original zero-inverse panic is caught; full stderr retained. A finite unit table does not prove universal nonzero norm. [pcc:unverified]")
#text("Rust1.99.1/offline/j1/test-threads1/120s;3 passed,0 failed/ignored/measured/filtered. Whole raw tree/commands/source/host/target preserved in381024B entire archive03. It includes later broad-lint debug outputs and is not a pre-lint snapshot. Build/unit elapsed fields are not performance evidence. [pcc:unverified]")
#pagebreak()
= Exact raw word cofactor bridge
#text("Word07 source22bd9020 preserves the exact Word01 prefix5743a021. Under input word values<223, raw multiplication does not overflow (products≤49284); canonical subtraction adjustment≤445. Every cofactor output is canonical and its raw toNat value equals the Fin223 polynomial value, beyond agreement after an additional modular reduction. [pcc:unverified]")
#table(columns: (36%,64%), inset: 6pt,
 [Original proof gates], [Both Lean4.35/4.34.1 audit42/0 failing/0 trusted;27 old+11 new named+4 new generated equations. Full bindings15a361.],
 [Resources/foundations], [One worker/120s/default proof memory; standard foundations only, no native decision/custom axiom. Four nonfatal deprecated-if warnings per build retained.],
 [Finite controls], [P(1,1,1)=(221,2,0); P(0,1,1)=(220,3,1); noncanonical2^63 times2 wraps and rejects unconditional decode multiplicativity.],
 [Proof boundary], [Normalized cofactor arithmetic only. Compiler/norm block/Fp.inv loop/field/nonzero norm/counters/Jacobian/runtime remain separate.],
)
== Preserved failures and exact CI scope
#text("Broad format1/Clippy101 remain failed on the immutable historical reference (Rng::next and two manual-is-multiple-of lints plus formatting). No original source edit or lint suppression. Required new-package format and no-deps/all-target strict Clippy pass0,495eb9. CI replays those scoped checks, all15 fixtures and both unchanged original proof gates. No source/gate/resource weakening. [pcc:unverified]")
== Primary sources and next dependency
#text("Before code3eafa4: full original common.rs/full04/old WordBridge; official Arkworks algebra v0.5.0 full Fp3/cubic-extension files and Apache/MIT licenses, exact Git blobs retained. Arkworks uses the same established cofactor and documents s2's negative sign; no novelty/fastest claim. Its cited Algorithm17 paper was not itself reviewed. KB wrappers/conductor127, locked atlas unread, no absence inference. Word preregistrationbcce0b reads actual UInt64/Fin interfaces. [pcc:unverified]")
#text("Full04 provides the abstract ring cofactor/norm and conditional inverse implication60bdae; next remove the supplied nonzero-norm gap for q223 using the no-base-cube-root certificate and finite base units. Existing chain/cubic/F101 graphs stay unchanged: no geometric edge added. [pcc:unverified]")
#text("Current c91 ownCI3dd59d retains seven whole ZIPs/413 members and six proof successes; original S3pull/migration403 still blocks normal merge. Original full scientific LP/360/witness/controls/feedback/post/runtime/admission/matched costs remain open under paid STOP. [pcc:unverified]")
