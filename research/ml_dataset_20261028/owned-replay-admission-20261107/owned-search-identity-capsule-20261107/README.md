# Native owned-search identity capsule

This packet supplies a private native derivation helper for the next cell-to-replay bridge. It obtains public/factor identities from the existing `VerifiedWorkerOriginal` token, rather than accepting arbitrary public/factor hash arguments. The typed capture binding must be present on that token. The existing `LaunchContext::validate(run, binding)` checks the historical launch's original binding and limits; raw context bytes must decode to that exact typed launch. The raw run hash must equal the token's capture binding. Required context/public/factor artifacts must occur exactly once in the supplied manifest with matching hashes and known sizes.

The helper hashes typed AttemptBinding and LaunchContext serialization, exact raw run/context/manifest bytes, and the original owner domain recipe. The raw execution-context fingerprint is deliberately distinct from the canonical typed launch identity. It prepays its bounded parse/hash/output work and memory through the caller's existing DecodeBudget, then performs a final original-budget check. It creates no allowance, deadline or retry. Caps: each raw context at most1MiB, typed serialization at most65536bytes, artifact list at most4096.

## Admission boundary

`owned_search_identity_capsule.rs` is a child of the existing central verifier; `derive` is only visible to that parent. Its constructor requires the private original token, actual typed launch, retained raw run/context/manifest bytes, and the existing mutable replay ledger. The helper does not load a cell, authenticate a manifest against external custody, perform original public/factor mathematics, or revalidate file contents. These are required preceding and subsequent central stages. `original_math_replayed_here` and `cell_files_revalidated_here` are explicitly false.

A synthetic token can be constructed inside the standalone test harness's ancestor module because that module reproduces native privacy. Those tests do not establish original admission. In production, the exact proposed call site is after native public/factor mathematics verifies and mints the original token; see WIRING.md. The registration patch declares this module but does not call it or broaden the existing SAT-only profile. The completed central owned-search route and Python `cell` mode remain outstanding.

## Evidence

15 Rust tests pass using actual mechanically extracted AttemptBinding fields/validate/digest, LaunchContext fields/validate, VerifiedWorkerOriginal fields, Stop and DecodeBudget definitions. Real cached serde/serde_json/sha2/hex libraries compile the unchanged capsule source. Fixture-only pieces are the replay Ledger, unused CaseConfig alias and synthetic token/source data. No genuine cell loader, Frame, public algebra, storage verification or full module assembly is executed by these tests.

Controls cover actual binding digest compatibility, pretty/raw versus canonical launch hashes, exact original owner domain, mismatched typed binding/limits, mutated raw run/context, substituted token public hash, duplicate manifest artifact, manifest fingerprint mutation, unbound token, raw-size refusal, stopped budget and final refusal retaining charges. The fifteenth test compares capture/launch/owner digests with independent Python serialization reference values recorded in PYTHON_REFERENCE.json. Initial14-test and final15-test logs are retained.

Replay: `python3 prepare.py`, then `timeout 60 bash test.sh`. `prepare.py` reads the hash-bound source paths recorded in SOURCE_HASHES.json and regenerates the extraction harness plus registration patch/proposed central source. Absolute paths reflect the original checkout and packet location. `test.sh` uses installed rustc+1.98 and already-cached rlibs, with60CPU seconds and1.5GiB virtual-memory limits; it does not invoke Cargo or rebuild dependencies. The test binary is software evidence only.

The capsule retains no raw private scalar or witness. It has not been published as an authenticated dataset receipt, passed through full CLI generation, or incorporated into central export. No cloud job, scientific generation, full Cargo build or Git mutation occurred.
