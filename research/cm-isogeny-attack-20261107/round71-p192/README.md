# Round71: Kummer-surface product response

The explicit marked response has a four-bivariate-polynomial product representation. surface-product-proof.md derives an O(r^(2/3+epsilon) polylog(p)) bit algorithm for every fixed epsilon>0 using published fast multipoint evaluation, with point generation, denominator evaluation and clipped-tail work charged. [pcc:unverified]

The Rust prototype checks that representation with classical sparse multiplication/direct evaluation. All381decoder rows/2694queries and24prescribed P192responses preserve the previous values; nine singular/signed algebra controls pass. It does not implement or benchmark the asymptotic backend. See report.md and execution-record.json for complete qualifications and actual costs.

Independent scientific standing remains unverified. The active theoretical ECDLP objective continues; the proof appendix supplies a direct one-construction scalar extractor from the retained zero indices, still pending native qualification and fast-backend implementation.

Build offline with one job: CARGO_BUILD_JOBS=1 cargo build --release --locked --offline --manifest-path native/Cargo.toml. Each command has timeout120. Run signed-baby-response --surface-controls; signed-baby-response; cached-p192 PATH_TO_THIS_DIRECTORY. Source manifests and publication receipts identify exact revisions. No cloud job or generator is part of this round.
