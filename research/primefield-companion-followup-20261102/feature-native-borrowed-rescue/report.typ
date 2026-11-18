#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= Borrowed interval ownership
T-1881 · Prime-field coefficient infrastructure · 2026-11-07

The original coefficient context owns its interval engine inline. A doubled context borrows that same cell through an explicit lifetime. The ownership constructor no longer allocates an Rc or performs reference-count operations; arithmetic vectors retain their allocations.

#image("flow.svg", width: 100%)

*Lifetime checks.* Three nested borrowing panels retain one cell address and leave the owner usable. The actual for_doubled API rejects an attempted return of a local owner's borrow with E0515 under the escape_probe cfg. The ordinary native binary compiles and runs the same source without that deliberately invalid fixture.

#image("layout.svg", width: 100%)

*Layout tradeoff.* Engine metadata is 224 bytes, the enum 240 bytes and the full context 264 bytes on the checked target. Borrowed variants reserve space for the owned variant. These are structure sizes, not peak-stack or elapsed-time measurements. No total-memory reduction is claimed.

*Retained arithmetic.* 760 coefficient equalities over 20 panels, 120 key/borrow rejections and 20 interleaved pairs pass. Shared polynomial/progression setups stay 16/16 versus 32/32 separately. Batches stay 10,147 at equal widths and 9,953 with separate native widths. All 1,686 public target/cap comparisons preserve results, routes and source calls. Cap 8 retains ten unresolved targets; other panels recover 281/281.

*Attempts.* validation01 fails compilation with E0106 in an inherited control field; its changed source is retained. validation02 passes after explicitly annotating that owned field. lifetime01 rejects the escape with the expected E0515. The successful dependency hashes are checked before the negative compiler probe.

*Evidence.* PCC-20261107-2a715d is a producer attestation; independent signed verdict pending. Original cohort unchanged. No cloud or timed workload. DERIVATION.md, RESULTS.md and README.md provide scope, failures and replay.

*Sources and next.* Exact shared-engine predecessor and official Rust lifetime documentation (doc.rust-lang.org/book/ch10-03-lifetime-syntax.html); KB unavailable. Next choose a declared possible-doubling width before first use, accounting upfront work and no-rescue cases against saved later batches.
