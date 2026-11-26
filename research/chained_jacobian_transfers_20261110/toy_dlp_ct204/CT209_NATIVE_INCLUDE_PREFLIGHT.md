# CT209 — native Rust literal-include source preflight

Date: 2026-11-18. Existing draft [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. **[pcc:unverified]**

## Exact new failure

[GitHub Actions run 44317792024](https://github.com/isogeny-labs/crypto/actions/runs/44317792024), native job 134848408067, ran the CT208 manifest preflight successfully then compiled the Rust dependencies and the main crate. It **failed before the 43-log test** at 2026-11-18 14:14 UTC:

```
src/bin/volcano_ic_driver.rs:214:
include_str!("../../research/full_volcano_graphwide_ic_20261017/execution/PROTOCOL.md")
error: couldn't read .../execution/PROTOCOL.md: No such file or directory
```

The referenced PROTOCOL.md is present on the exact branch (Git blob `2038eb2be578a6d858d4a412397c06f1de0811e1`). The sparse checkout contained `execution/summarize.rs` alone, not other files in that directory. This is a build-input omission, **not** a Rust group-arithmetic failure. Independent Python field-oracle job passed.

Cargo documents that integration tests may build binary targets [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html), explaining why an unrelated executable's embedded file blocks the requested test. Git's [sparse checkout](https://git-scm.com/docs/git-sparse-checkout) uses directory and file patterns to control checkout scope.

## Implemented

- Changed the native sparse checkout to include the **whole** `research/full_volcano_graphwide_ic_20261017/execution/` directory instead of only `summarize.rs`, covering the exact missing PROTOCOL.md.
- Added `ct209_native_source_preflight.py`, checking the 36 explicitly registered root Cargo binary source paths and literal `include!`, `include_str!`, `include_bytes!` resources in those binary roots and `build.rs` *before* expensive Cargo compilation.
- Added `test_ct209_native_source_preflight.py` with five isolated filesystem controls: missing binary, missing resource, successful resource, commented macro, and `include_bytes!` with `build.rs`. Ran `python3 -m unittest -v test_ct209_native_source_preflight.py` locally: **5/5 passed**. This does not verify native compilation.

CI keeps Rust 1.98, 40-minute cap, two Cargo jobs, separate independent field oracle and exact toy test. No S3 original-model access, EKS/Spot usage, changed fixture or protected merge.

## Reproduction and boundaries

```sh
cd research/chained_jacobian_transfers_20261110/toy_dlp_ct204
python3 -m unittest -v test_ct209_native_source_preflight.py
cd -
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/ct209_native_source_preflight.py .
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
```

The new scanner intentionally does **not** prove Rust compilation: it does not resolve dynamically computed macro arguments, transitive includes from dependent modules, feature-conditioned assets or generated OUT_DIR files, and it is not a full Rust lexer. Real compiler/test output remains the acceptance gate. Research-KB was inaccessible in the connector runtime; sources reviewed were exact CI logs, repository CT204–208 reports, crypto #3877/#4697, cryptanalysis #1766, autoresearcher IDEA-20261109-4668b8 and official Cargo/Git documents. No novelty claim.

The toy 43/43 recovery, physical norm/reuse pricing, original authenticated 695/701 fixture and cold rho/BSGS/IC comparison all remain outstanding. The original CT200 model hydration HTTP 403 is unrelated to this toy build. **[pcc:unverified]**
