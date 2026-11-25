# Frozen native EC filter replay protocol

Prepared 2026-11-24 under draft PR #5165. **Not executed.** Wait for parent confirmation that the required S3 checkpoint gate passes before compiling or running. Conductor tasks T-12175 and T-12180 own the source and this protocol.

## Immutable source and input

- Source: `WS68_FILTER_REPLAY.rs`, SHA-256 `db5eced2d18572173ff67f93630e4cf0ef164928f89685a36e157a65e77c1671`. Refuse a run if the hash differs; do not edit this source after a pre-call manifest is issued.
- Elliptic curve: `E/F_17: y^2=x^3+2x+2`, `G=(5,1)`, subgroup order 19. Native code checks `#E(F_17)=19`, generator order, every point on curve, and every group identity.
- Six relation columns are actual EC points `[3]G,[5]G,[12]G,[10]G,[9]G,[7]G`. Source rows and right-hand sides are embedded and printed by the source. Three partial rows form a coefficient-aware large-prime cycle; two full rows pin the third small-column log. An independent two-column component checks block-rank addition.
- Workload is a deterministic planted exactness control. It does not measure relation discovery, filtering speed, or scalability. The best native comparison is the repository's `large_prime_filter.rs` tests, which could not build in the sparse checkout because of missing unrelated embedded fixtures.

## Planned command after checkpoint admission

Use a distinct pre-call manifest, build artifact, output, and post-call manifest. Preserve the exact source and compiler hashes before and after the call. The source itself has no file writes.

```sh
rustc -Vv
shasum -a 256 WS68_FILTER_REPLAY.rs
rustc -O --edition 2021 WS68_FILTER_REPLAY.rs -o WS68_FILTER_REPLAY.bin
./WS68_FILTER_REPLAY.bin > WS68_FILTER_REPLAY.stdout
shasum -a 256 WS68_FILTER_REPLAY.rs WS68_FILTER_REPLAY.bin WS68_FILTER_REPLAY.stdout
```

The exact invocation, compiler path/version, current commit, source digest, separate immutable qualifier hash, gate receipt, and artifact hashes belong in `WS68_FILTER_REPLAY.pre-call.json` and `WS68_FILTER_REPLAY.post-call.json`. The pre-call manifest must be written once and never replaced; a later reconstruction must be labeled post-call. The raw stdout must be kept byte-for-byte. This protocol does **not** authorize a run while the S3 gate is 403.

## Acceptance and separation of claims

The binary must exit zero, report `group_count=19`, `source_rows_valid=3`, source rank 3, closed coefficients `[1,1,1,0,0,0]`, closed RHS 1 modulo 19, small rank 3, rank without `x1` 2, target `x2` log 12, disconnected component rank 2 and combined block rank 5. The complete row must be reconstructed from exactly the three source row coefficients `[1,1,1]`; all point-group identities must pass independently of modular rank. A compiler or run failure is retained as a failure receipt. No timing from this control is compared with Pollard rho or with a full EC relation pipeline.
