# Prime31 B8 RR9 relation-cost gate — frozen protocol (2026-11-17)

## Aim and fixed source
Audit the *fixed eight-x* RR9 signed-support predicate on the actual original ordinary, cofactor-one curve, not its full B31 factor base. Reuse the native Rust `prime31_algebraic_ic::{PRIME31,Arithmetic,check_curve,Point}` crate from immutable commit `1df714645aa683cf975bc76fcacb9842cb4780fd`. Pin Rust 1.100.0. First eight rational abscissae must be [0,1,2,3,5,6,8,9]; enumerate every 8-bit sign choice with the original curve group law, assert 256 distinct outputs, no supplied target secret.

## Exact statistical claim; quantifiers
Let `S` be the finite set of 256 distinct eight-leaf signed sums. For any fixed `Q in E(F_p)`, choose `b` uniformly from *all* `Z/nZ` with `n=2,147,446,831`, and form `R=Q+[b]G`. Since `G` generates this prime-order curve, `R` is exactly uniform and `Pr(R in S)=256/n` irrespective of Q. Fresh independent `b` gives exact expected draws `n/256` for one membership success, `r*n/256` for `r` (not necessarily linearly independent) successful draws. Adaptive or nonuniform residual searches, changing factor bases and B31 S9 supports are **not** covered by this bound.

For budget `T=170.113ms` (producer historical *complete* shared BSGS on 1024 targets in #4532), compute the hypothetical *entire-budget-per-trial* upper allowance `T*10^6*256/(r*n)` in nanoseconds for `r=1,8`. This is a conditional feasibility comparison, not a measured per-trial cost or a universal impossibility theorem. Any real full solver must charge misses, setup, matrix work and all other costs.

## Frozen executable outcomes
- Confirm original prime31 curve, B8 xs and all 256 distinct points in the original native Rust arithmetic. A repeated point, an invalid curve or a different base fails.
- Exact rational numerator/denominator `256/n`, single-success trials `n/256`, eight-success trials `8*n/256`. Use integer arithmetic for these quantities; f64 only for displayed budget conversion.
- Confirm all group scalar/log information stays in the *test harness*. No candidate index-calculus extractor or unknown Q log solver is claimed or executed.
- CI: `cargo +1.100.0 test --locked --release --lib`, test the new example, release-run JSON assertion, archive JSON. Reuse immutable native arithmetic source; no access to paid AWS or manual retuning.
- If CI cannot execute, preserve compile/runtime error and mark test outcomes unverified.

## Nonduplication and next-work gate
Prior: #4532 complete IC/BSGS 1024-Q; #4556/#4558 B8 public sign enumeration; #4575 generic B8 4+4 comparator; #4581 seven-of-eight split quotient; #4550 D9 exact theorem; #4604 active D9 sparse Wiedemann implementation; #4596 active original B31 matched batch frontier; already-completed `prime31_s9_b31_half_sum_oracle.rs` B31 4+4 controls. Cross-repo: crypto-autoresearcher #3301/#3305, cryptanalysis #1536. Published: Petit–Kosters–Messeng PKC 2016 DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala FFA 2018 DOI 10.1016/j.ffa.2018.01.009. Research-KB MCP catalog and the local wrapper are unavailable in this tool runtime; this is a bounded source review, not a novelty claim.

Only after interpreting the bound should anyone propose B8-D9-to-full-solver extrapolations. The actual prime31 non-generic research priority is **public-target-conditioned extraction against the same Dickson B31 base**, with multiplicities, exceptional roots, real relation rank and complete blinded solver measurement. #4596 and #4604 have separate owners; do not run or overwrite their experiments.
