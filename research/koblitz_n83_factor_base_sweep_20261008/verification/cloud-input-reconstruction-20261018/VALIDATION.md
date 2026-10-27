# Recovery validation

Tested source: `8f971585692c702d2804668b322e2440177f80d2` on Linux x86-64.
The [source-equivalence check](source-equivalence.json) confirms the library,
exporter, Cargo manifest/lock and adapter sources are unchanged at recovered
upstream commit `63844b3b2b674d31ef07e8dec8d0f63ffe66bc52`.

- Offline release exporter build: PASS; Cargo reported 220 seconds.
- Initial complete release library run: 4,082 passed, nine failed, 117 ignored.
  Every failure was `Operation not permitted` at socket setup in unchanged
  Cairn/Pollard networking tests. The exporter tests did not execute after
  the failed library target. See [preserved failure log](lib-sandbox.log).
- Full rerun with loopback access: **4,091 passed, zero failed, 117 ignored**;
  148.96 seconds of test-harness execution. Exporter: **19 passed**, 0.58 seconds.
  See [complete passing log](lib-and-example-loopback.log).
- N83 Python discovery: **63 passed**, 5.346 seconds.
- Boundary Python suite: **46 passed**, 1.127 seconds.
- Frozen v4 design check: PASS, SHA-256
  `8e512b8dbc6577093f8e31f4afb625894cbdedd9f9f3e87dc4f36a7df52344c2`.
- Byte-for-byte K2048 comparison, every-point generic replay and forged-label
  rejection: PASS. Recovered upstream manifest/source/public-query checks: PASS.

The Rust command was `cargo test --offline --locked --release --lib --example
koblitz_n83_factor_base_export -j 2 -- --test-threads=4`, using the dedicated
scratch CARGO_TARGET_DIR and a 600-second outer guard. The initial sandboxed
run used two test threads and a 900-second guard. Neither guard expired.

No algorithm or performance improvement is claimed. Natural yield, rank,
column-log precomputation, individual-log extraction, matched references and
complete cold runtime selection remain unresolved. Original static-worker
bytes and live Docker cleanup were not independently rechecked here.
