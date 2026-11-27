# Frozen native EC filter replay protocol

Prepared 2026-11-24 under draft PR #5165. Conductor tasks T-12175 and T-12180 own the source and this protocol; T-12195 owns the separate run artifacts.

## Dated local admission amendment — 2026-11-24 America/Los_Angeles

The required S3 checkpoint gate returned HTTP 403. The user explicitly authorized **"Run locally with bypass"** after being told of that failure. The parent agent relayed this authorization and directed one local native execution. This amendment changes only gate admission: the frozen curve, source SHA-256, rows, expected output, and exact checks below remain identical. The resulting record is a local bypass run and must not be represented as a normally gated benchmark or used for a performance claim. Preserve the HTTP 403 as the failed gate condition; do not retry or replace it silently.

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

The exact invocation, compiler path/version, current commit, source digest, separate immutable qualifier hash, gate receipt, and artifact hashes belong in `WS68_FILTER_REPLAY.pre-call.json` and `WS68_FILTER_REPLAY.post-call.json`. The pre-call manifest must be written once and never replaced; a later reconstruction must be labeled post-call. The raw stdout must be kept byte-for-byte. The local run is admitted solely by the dated user override above; the S3 gate remains failed.

## Acceptance and separation of claims

Attempt 1 compile stopped before execution because the qualifier filename's dot was inferred as an invalid crate name. `WS68_FILTER_REPLAY.stderr` and the immutable attempt-1 pre/post manifests preserve that failure. Attempt 2 changes only the compile invocation by supplying `--crate-name WS68_FILTER_REPLAY_qualifier`; it uses the identical qualifier bytes and separate `attempt2` artifact names. Conductor task T-12206 owns the attempt-2 artifacts. The user-authorized local bypass remains the admission basis.

The binary must exit zero, report `group_count=19`, `source_rows_valid=3`, source rank 3, closed coefficients `[1,1,1,0,0,0]`, closed RHS 1 modulo 19, small rank 3, rank without `x1` 2, target `x2` log 12, disconnected component rank 2 and combined block rank 5. The complete row must be reconstructed from exactly the three source row coefficients `[1,1,1]`; all point-group identities must pass independently of modular rank. A compiler or run failure is retained as a failure receipt. No timing from this control is compared with Pollard rho or with a full EC relation pipeline.

## Observed local receipts — 2026-11-24 America/Los_Angeles

Attempt 1 stopped at compile exit 1 because Rust inferred a crate name containing a dot from the qualifier filename. `WS68_FILTER_REPLAY.stderr` is the raw 281-byte error (SHA-256 `f58f36ceece1c43b86ff9168b4ec45553e092660ac143be1a30af8b0b7b6e538`); `WS68_FILTER_REPLAY.pre-call.json` and `WS68_FILTER_REPLAY.post-call.json` preserve admission, command, exit and unchanged source hash. There was no binary or run.

Attempt 2 added only `--crate-name WS68_FILTER_REPLAY_qualifier`, with separate immutable pre-call and post-call manifests. Compile and run each exited 0; stderr is empty. Raw `WS68_FILTER_REPLAY.attempt2.stdout` has SHA-256 `876fbc24717c0c1060e3206c5f941f2553292d2fcbdf301caf2d61d82b75f6a6`, and the binary has SHA-256 `ae59a2ae530fa0e3988add43b5ba097b147dcec2a74f2e75772e766d916433d4`. Source and qualifier each remained `db5eced2d18572173ff67f93630e4cf0ef164928f89685a36e157a65e77c1671`. The output records 19 curve points, three valid source identities, source rank 3, closed vector `[1,1,1,0,0,0]` with RHS 1, small rank 3, rank 2 without the `x1` equation, target log 12, and disconnected block ranks 2 and 5. This meets the exactness-control acceptance above within the local bypass scope. It has no timing or naturally collected relation claim.

The compiled attempt-2 executable remains in the local worktree only. The PR artifact-migration job received HTTP 403 while checking its SHA-256 object in the configured store. The committed source, compiler and command receipts, raw stdout, and manifests retain the replay inputs and output; remote reviewers cannot compare the original executable bytes until artifact access is restored. The separate native graph verifier executable is also retained locally (SHA-256 `66a74c9afbe4f706725d9fc17e5890d8b6cee07bbf44893dbf64387932bdeffc`) for the same reason; its source and output remain committed.
