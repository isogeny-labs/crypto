#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.3pt)
#set par(leading:0.45em)
= H1: explicit action coordinate and dense fallback
23 November 2026 · Rust library integration and finite software qualification

The sparse linear stage now accepts an explicit multiplication coordinate and reads every recovered coordinate back in the original order. The default still chooses the last coordinate. A separate IC adapter retains the existing dense stage on fast-stage refusal and includes both attempts in the existing partial multiplication counter. Invalid coordinates are configuration errors. [pcc:unverified]

#image("action-flow.svg",width:100%,height:78mm,fit:"contain")

== What ran
Fourteen native tests pass, zero fail, and the existing explicit throughput test remains ignored. Six new controls cover every coordinate on two-, three-, six- and eight-variable shape-position bases; default root/counter/RNG compatibility with the pinned incumbent; nonseparating-coordinate refusal and dense recovery; an empty rational root set; exact repeated-root recovery; and invalid-coordinate handling. The other eight passing controls are inherited arithmetic/hash tests. [pcc:unverified]

The nonseparating fixture uses an independent brute-force root set and checks that fast-attempt plus dense-fallback multiplication counts are both retained. This is finite regression evidence, with no general completeness or signed-verdict assertion.

== Retained failures and limits
The first new test incorrectly required every repeated-root ideal to force fallback. For `x-y, y²`, the incumbent and candidate can both recover `(0,0)` exactly. The corrected test requires that exact result and incumbent agreement; the original failure and plan remain preserved. [pcc:unverified]

The initial scoped build encountered private polynomial-ring types in Root41. The scoped helper now copies those original items without algorithm changes and compares the copied sections against source. An initial strict Clippy check finds three inherited diagnostics in the old linear module; two documented lint allowances apply to that included module. New adapter and driver code remain under `-D warnings`. Full-library compilation and CI are separate, previously recorded obligations. [pcc:unverified]

== What remains unmeasured
The prospective first32 saved-system comparison is implemented as a driver but *not run*. It would select on indices0–15 and retain a same-fixture holdout16–31, using only a partial multiplication proxy with development-selection costs exposed. It stays on the requested Runpod backend, pending existing funding/checkpoint and execution prerequisites. No better coordinate, runtime gain or new ECDLP solve is established here.

Sources: `zero_dim_la.rs`, `jv_action_operator.rs`, `jv_cover.rs::linear_stage_from_basis`, the pinned Root41 dependency, and the native test receipts. Faugère–Mou, Sparse FGLM, arXiv:1304.1238, supplies the operator-sparsity/shape-position framework. KB/Conductor are unavailable; the bounded literature review and retrieval limits are in README.md. #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR #4986].
