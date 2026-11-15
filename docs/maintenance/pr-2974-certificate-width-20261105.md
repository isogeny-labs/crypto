# PR #2974: certificate integer width

Review thread PRRT_kwDOSMdswM6yDJCT identified a valid 127-bit automatically generated proof rejected with Limits.bits=128: decimal_digits*4 incorrectly stood in for the integer's width. The new public-path regression reproduces Limit("certificate decimal width") on the original parser. [pcc:unverified]

The parser now uses a separate conservative pre-parse digit allocation bound (bits/3+1, since 2^3<10), parses the canonical decimal, and checks its actual bit width with checked usize conversion. Canonical syntax, proof-node/factor/witness checks and all resource/error handling remain enforced. The regression also rejects an actual 129-bit value under the 128-bit budget and an excessively long decimal. [pcc:unverified]

All 78 standalone hashed-ecdl-tensor library tests pass under the standard Release profile (275.83 seconds), and strict all-target Clippy passes. The initial unoptimized full suite was stopped by this session after more than 18 minutes with one existing tensor test still running; it has no full-pass footer. The unchanged test assertions were rerun in Release. [pcc:unverified]

This is crate-local software validation. It does not supply a full parent/root-library suite pass, an independent scientific replay, a new benchmark, a fastest-prover claim or a signed verdict. Historical sources, measurements and the previously documented root-library validation gap remain preserved. Current-head CI and all other blocking review/validation obligations remain required. [pcc:unverified]
