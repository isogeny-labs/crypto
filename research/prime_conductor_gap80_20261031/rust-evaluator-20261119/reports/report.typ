#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10.5pt)
#set par(leading: 0.6em)
= Rust evaluator: complete retained conductor crossing
19 November 2026. User requested Rust; all isogeny scheduling, jet/log-norm identities, coverage checks and twist transport are now Rust. Field/Jacobian arithmetic reuses the unchanged repository source. The small C ABI exposes existing FLINT/GMP primitives and allocations; it does not implement or call the C++ constructor. [pcc:unverified]

#image("architecture.svg", width: 100%)

Final source is src/bin/ecbench/isogeny_eval.rs with its private flint.rs ownership wrapper. Opaque contexts and polynomials retain Rc ownership of their library/context. Dynamic library SHA256 and ABI are checked. The supported ABI is Linux with 64-bit long; field coordinates stay four limbs and arbitrary public isogeny scalars remain BigUint. Source a,b accept signed residue representatives, matching the original input format; point/field/degree encodings stay strict. [pcc:unverified]

The final build passes seven arithmetic/coverage/parser tests,18 complete small-map comparisons against retained independent PARI references (470 point inputs; two fields; square/nonsquare twists; full/blocked rectangles), and eight rejection controls. Port code passes scoped clippy with warnings denied. Two original indexed-loop lints are excluded only on the unchanged included Montgomery dependency; original diagnostics remain. No full-repository test or formal-proof claim is made. [pcc:unverified]

The complete retained ordinary map has prime degree 33555419 and actual End-conductors 1→33555419, a26-bit prime ratio. The exact source/quotient equations and original prime subgroup 5629830654216971 are unchanged. Final Rust main/probe/dual runs independently pass 33 point/order checks,32 scalar identities and 33 normalized dual compositions. Their numeric output agrees with the retained reference. The standard CM conductor argument applies to these exact equations; see the adjacent CM_ARGUMENT.md and its fresh PARI arithmetic certificates. [pcc:unverified]

Source equation: a=5620419248410339, b=675862186879016. Quotient: a=2539833775737119, b=1737207970637906. Field prime 5629830687772387. Transported original G=(2975092980436849,5333622322807887). This evaluated milestone remains below the original 80-bit minimum. [pcc:unverified]

#pagebreak()
= Actual local calls and remaining goal
#table(columns: (2.7fr,1fr,0.6fr), inset: 4pt,
 [Whole process / actual outcome], [Elapsed seconds], [Exit],
 [Rust first build: main +33 images], [8.719], [0],
[Rust first build: dual +33 images], [8.743], [0],
[Rust final build: main +33 images], [8.336], [0],
[Rust final build: auxiliary probe], [0.609], [0],
[Rust final build: dual +33 images], [8.776], [0],
[Original 80bit: first codec refusal], [0.032], [2],
[Original 80bit: final Rust attempt], [60.008], [124],

)

All elapsed values are individual Bash whole-process L0 observations. They include executable/library hashing and loading, input/primality checks, quotient construction, supplied point evaluations and output. The300second cap for retained cases and 60second cap for the original 80-bit target are budgets; 12 GiB is an address-space limit, not observed RSS. No build/audit/benchmark overlapped a timed call. No separated phase cost, speedup or high-isolation admission is inferred. [pcc:unverified]

The first 80-bit call refused its−140 coefficient before construction. That exact refusal is retained separately. The final signed-input build then executed the unchanged original33-point input and reached its 60second watchdog (124), without quotient, images or durable partial-coverage journal. This is a censored attempt, not an impossibility or completion certificate. The original 80-bit crossing remains open. [pcc:unverified]

Existing matched Rust solver measurements on these identical retained endpoints are preserved as baseline, not re-run or relabelled by this port: four verified rho recoveries (source/image means 7.637/10.318seconds, one target/two rounds), four IC exhaustions at 10000 trials with zero relations, and 672 independently checked scalar batches over 32 bases/model and six measured rounds plus warmup. All eight solver attempts replay exactly. The word-field solver and four-limb scalar clocks are separate from constructor clocks. Full 32-target ECDLP and admitted performance remain unfinished. [pcc:unverified]

Exact sources, failed builds/role wiring/smoke/refusal attempts, pre-call hashes, raw output, independent checks and deterministic complete raw archive are preserved. The archive inventory records every extracted hash and byte-identical repack; it does not claim an S3 upload. Numerical libraries and qualified existing release-library build identities remain explicit. Producer evidence is unsigned/not_covered. [pcc:unverified]

References: existing Rust Montgomery/Jacobian source; native complete-evaluator and prepared-norm derivation; FLINT/PARI API contracts; ordinary-order/CM references linked in the retained CM_ARGUMENT.md. KB unavailable. No novelty, world-record or performance-leaderboard claim. [pcc:unverified]
