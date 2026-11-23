#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.0pt)
#set par(justify:true)
#align(center)[#text(size:17pt,weight:"bold")[Cubic inversion inside the closed Richelot evaluator]#linebreak()17 November 2026]
Prospective unit15 qualifies a guarded direct cubic inverse in the successful14 evaluator. It preserves the exact general compositum inverse and zero rejection. No new native test, Lean theorem or timing result has run yet. Protocol and written obligations are frozen before new code; all earlier14 outcomes remain unchanged. [pcc:unverified]

#align(center)[#image("flow.svg",width:90%)]
For a cubic-field input (a,0), the old quadratic norm is a² and its inverse pair is (a·(a²)⁻¹,0)=(a⁻¹,0), for a≠0. The five explicit cubic multiplication calls around the inner inverse include two with nonzero operands. The proposed shortcut removes those boundary calls, but changes the inner Euclidean-inverse argument from a² to a. This predicts counters and supplies no complete cost or runtime ratio. [pcc:unverified]

The fixed native population is64 distinct nonzero cubic encodings,8 noncubic compositum inputs,2 expected zero assertions,8 wrong-sign and3 invalid-constructor controls. Compare17 forward and17 dual closed divisor calls on the existing17 witnesses; preserve exact classes/routes and record the actual multiplication/inverse/shortcut counts. Require the known9 lower-degree/8 quadratic forward routes, with no narrowing after results. [pcc:unverified]

The written subfield argument uses prime inputs and cubic g/h coefficients, with ring operations, nonzero inverses, polynomial division/extended GCD and Cantor reduction closed in the embedded cubic field. Existing nonunit/degree-drop boundaries stay unchanged; pointwise fallback may need the quadratic extension and retains the original inverse. Finite coverage will not establish every boundary case. [pcc:unverified]

The planned Lean4.35 source proves norm-on-(a,0), the inverse-by-square identity and pair equality over the installed standard Lean.Grind.Field. Keep default proof limits, warnings as errors, copied read-only proof input and complete printed axioms; no sorry, custom axiom or native\_decide. This will not construct a Field instance for the Rust representation or complete the Picard bridge. [pcc:unverified]

Scientific execution remains Rust1.99.1/Lean, one worker120s per call,8MiB RAM/3MiB scratch, exact prebuilt dependencies and copied555 qualifier. One new test/14 inherited filtered, direct strict-Clippy controls, format and all source/dependency/tool guards are required. Retain every full successful/failed packet; current artifact policy and S3 publication limits remain explicit. [pcc:unverified]

Sources checked: exact14F nonsplit/map/closed sources; ConsumerE079 CLOSED\_FIBRES.md; Lean4.35 Init.Grind.Ring.Field source and actual inverse-product/cancellation lemmas. KB tools absent/prior wrapper127, no novelty inference. Full protocol: PROTOCOL\_15.md; written derivation: PROOF\_15.md. Original full EKS work, paidSTOP, vault lock, signed replay and complete solver comparisons remain open. [pcc:unverified]
