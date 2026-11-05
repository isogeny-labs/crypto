# Centered bitplane decoder

Exact sign-magnitude coefficient planes reuse the one-reduction fold.
See DERIVATION.md, control.json, source-lock.json and raw build/test logs.
Compile lib.rs as a native Rust test target with edition=2021, -O and
-C overflow-checks=yes; receipt.json preserves the actual bounded commands.
The saved packet uses 128 population counts, one remainder and 328 payload
words. These are derived operation/layout counts, not measured timings.
