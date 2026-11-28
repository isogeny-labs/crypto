# CT211 — recover the native toy DLP CI gate by covering Cargo auto-discovered bins

Date: 2026-11-18. Existing draft [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715), stacked on #4697. **[pcc:unverified]**

## New exact-head failure from CT210

[Workflow #44340722313](https://github.com/isogeny-labs/crypto/actions/runs/44340722313), source `5915590b45e32adf6e65be94cddbc199a82a9d6f`: independent F31/F31² field oracle **passed** and existing Python literal-include preflight **passed**. Native release compilation reached the main crypto crate, then exited 101 before running either toy DLP test:

```
error: couldn't read src/bin/../../research/n83_jacobian_projector_20261031/prym-factorization.identity.json
 --> src/bin/n83_jacobian_projector_audit.rs:97:62
```

The exact JSON certificate exists on the branch (content blob SHA `7d0cd99ec04e60cdb0ac37b391b4f300c3a5eec1`). This is **not** a Jacobian arithmetic failure. The previous CT209 scanner covered only the 36 explicit `[[bin]]` roots. Cargo 2021 also **auto-discovers** executable sources in `src/bin/*.rs` and `src/bin/*/main.rs`, including `n83_jacobian_projector_audit.rs`. Cargo's reference explicitly documents the 2018+ default and `[package].autobins = false` exception: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery.

## Bounded repair

- Extend `ct209_native_source_preflight.py` to enumerate explicit and auto-discovered binary roots, deduplicating entries and honoring `autobins = false`. Preserve build.rs include checks.
- Add four regression controls: an implicit auto bin with multiline `include_str!`, nested `src/bin/*/main.rs`, `autobins=false`, and deduplication of an explicitly registered binary.
- Include `/research/n83_jacobian_projector_20261031/` in the GitHub-hosted non-cone sparse checkout; leave Rust 1.98, two build jobs, no external S3, existing 40-minute timeout and actual test command unchanged.

Local independently written equivalent Python fixtures passed **9/9** on this runtime; this is *source-preflight* validation only, not Rust execution. GitHub Actions exact-head [CT211 run #44351488002](https://github.com/isogeny-labs/crypto/actions/runs/44351488002) was observed **in progress** after the patch; the independent field oracle passed. Pending native result must be inspected and any further explicit/implicit include omissions corrected.

## Verification and scientific scope

After successful compilation the Rust test must actually recover 43/43 public target scalars on E/F31 and in Jac(C), reject the incorrect scalar and verify the norm pushforward. This remains a toy BSGS transfer **without** an index-calculus advantage or cold wall-time measurement. Source oracle parity, no user-paid compute, no protected merge, and draft standing remain. Original 695/701 Jacobian custody, measured C169 versus C13a+C13b plus N2_actual/D, matched cold rho/BSGS/IC and independent PCC remain separate unresolved performance gates.

No novelty or speedup claim. Research-KB endpoint unavailable to this runtime; reviewed the existing CT204–210 reports and official Cargo target-discovery documentation instead.