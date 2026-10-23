# CLI capability audit — 2026-10-17

Source reviewed: `crypto` upstream main at `c6d64436a33359320ad9d5b42c4acebee2968b9a` and C `cryptanalysis` upstream main at `5a52a13877271144d88e62a82031826759696fc3`. This inventory covers runnable commands rather than inferring CLI support from library modules. A status of **verified** below applies only to the named local check, not arbitrary curves, algorithms, devices, or a live S3 account.

## Current-head hash-file addition — 2026-10-23

`crypto hash --file` now processes SHA-256, SHA-3-224/256/384/512,
SHAKE128/256, and BLAKE3 incrementally using a 64 KiB read buffer. SHA-3
and SHAKE retain one rate-sized pending block. Checked `store:NAME` and
`sha256:DIGEST` references use the same file path after retrieval. Other
hash algorithms in this command still read their complete file input into
memory. The [streaming acceptance receipt](../store/CRYPTO_STREAM_HASH_ACCEPTANCE_20261023.json)
records independent boundary and large-file digest comparisons, option
rejections, and a live Setec S3 upload, fresh-cache reads, and cleanup.
The inventory below is the frozen 2026-10-17 starting point; its historical
statuses should not be read as current-head claims.

## Requirement-to-evidence inventory

| Requested surface | Executable path and evidence | Status and remaining gap |
| --- | --- | --- |
| Binary elliptic-curve IC | `ic` exposes `run`, `logs`, `solve`, `workflow`, `fixed`, `bench`, `boundary`, and `rho` (`src/bin/ic.rs`). `icx run` executes a bounded binary or Koblitz analogue with a generated known-answer target (`src/cryptanalysis/ic_run.rs`). | **Partial.** These commands have bounded input and solver envelopes. `icx run` always selects an analogue, even when its requested bit envelope is raised; its run command does not accept an externally supplied target point. |
| Prime-field elliptic-curve IC | `ec_index_calculus` has relation, factor-base, linear-algebra, and staged DLP code. `icx run` executes a small prime-field analogue with a generated known-answer target; `icx validate` checks named parameters. | **Partial.** `icx --require-named-curve` rejects exact named-curve execution, `icx run` does not accept an external target point, and imported prime curves in `ic` are inspection-only. |
| Cubic extension-field elliptic-curve IC | `gaudry_cubic` implements `Fp3`, relation collection, elimination, and `run_gaudry_opts`. A separate example harness calls it. | **Library and example only.** `icx run` rejects extension families. A command accepting a specified cubic curve, generator, target, budgets, and replay receipt remains open. |
| Other ECDLP | `ecdlp-nist` exposes `list`, `audit`, bounded `solve`, `smart`, and `bench` for its named catalog (`src/bin/ecdlp_nist.rs`). | **CLI implemented; list smoke passed.** This is a separate generic/structural DLP front end, not elliptic-curve IC. The full catalog was not independently exercised in this audit. |
| Isogeny graph enumeration | `crypto isogeny graph` enumerates a bounded small-field graph. Current main also exposes `crypto isogeny class` with a resumable ordinary prime-field graph backend (`3 < p < 2^61`, constrained discriminants/degrees), explicit component/full-class status, replay, and one-vertex BSGS solve. `isogeny_walk` supports prime-field `class`, `walk`, `verify`, `traits`, `collect`, and `plan`. | **Separate commands implemented; new class path not replayed here.** The class command has explicit backend and completeness limits. These commands do not connect a verified route and transported generator/target to an IC run. |
| S3 upload, retrieval, and existence | `crypto store push/pull/verify --remote` uses indexed content hashes and remote head checks. `ic --factor-base store:NAME` resolves on demand. `isogeny_walk` publishes and fetches walk artifacts. | **Implemented for these paths; live S3 unverified.** Other `ic` workflow outputs need explicit publication. A named test bucket/prefix is needed for an operational write/head/pull receipt. |
| AES differential and linear analysis | `crypto cryptanalysis sbox` reports DDT/LAT metrics; `auto`, `boomerang`, and `rectangle` accept registered AES-128 variants at 1–4 rounds. `aes-related-key` and `aes-visual-demo` expose narrower reports. `src/cryptanalysis/aes/` also has differential, linear, impossible, zero-correlation, integral, DFA, DPA, cache-timing, and other modules. | **Partial CLI coverage.** The 1–4-round registry and S-box metrics do not dispatch every AES module. There is no uniform command accepting arbitrary AES rounds/keys and invoking every technique with a common result schema or general full-round validation. |
| SHA-2, SHA-3, and SHAKE | This change dispatches SHA-224/256/384/512, SHA-512/224 and /256, SHA-3-224/256/384/512, and bounded SHAKE128/256 from `crypto hash`. `sha3` remains an alias for SHA-3-256. `crypto cryptanalysis hash-auto` now accepts SHA-256 and SHA-3-256 alongside MD4, MD5, and SHA-1; explicit `length-extension` accepts SHA-256. | **Partial cryptanalysis coverage; validation below.** SHA-256 length extension applies to a bare prefix MAC, while SHA-3-256 receives only generic truncated birthday and differential-bias measurements. These do not claim practical collisions or key recovery on the full hash. |
| C `cryptanalysis` CLI | `ca ic` is multiplicative-group index calculus in `(Z/pZ)^*`; `ca solve --group ec` is generic EC DLP. Its help lists number-theory, group, curve, and worker commands. | **Distinct algorithm scope.** The C `ic` command is not elliptic-curve IC, and C `ca` has no AES or SHA command. Malformed IC option validation merged in [cryptanalysis PR #779](https://github.com/aburan28/cryptanalysis/pull/779). |

## Checks

| Check | Result |
| --- | --- |
| C CLI smoke suite on the paired change | 88 checks, 0 failures, recorded when PR #779 was opened; PR merged 2026-10-11 PDT. |
| C focused `indexcalc` ctest on the paired change | 1/1 passed, recorded when PR #779 was opened. |
| Rust formatting and post-rebase build | Formatting passed. On upstream `34c18f7`, `cargo check --locked --offline` and `cargo build --release --locked --offline` passed for `crypto`, `ic`, `icx`, `ecdlp-nist`, and `isogeny_walk`. After rebasing onto `c6d6443`, `cargo check` fails with 25 errors in untouched current-main files, including duplicate `Fp4` variants in `ecbench/workload.rs` and missing quartic elimination functions. The current head cannot yet produce binaries. |
| Rust hash, hash cryptanalysis, and regime tests | On the pre-rebase parent, focused library tests passed (10 hash-attack and 5 regime tests); the two `crypto` hash CLI tests passed in release mode. Direct CLI checks matched SHA-2/SHA-3/SHAKE known answers, verified SHA-256 length extension against Python `hashlib`, rejected a malformed digest, and exercised `hash-auto sha3-256`. The changed code rebased without conflict, but the current head is blocked by the upstream build errors above. |
| `icx` and local artifact store commands | On the pre-rebase binaries, `icx estimate ss-f3-5 --json` returned `estimate`; `icx run ss-f3-5` rejected the unsupported family. A temporary `file://` remote passed push, pull, byte comparison, and `verify --remote`; removing the remote object made verification exit nonzero. |
| Required `cargo test --release --lib` gate | On the pre-rebase parent, the latest complete run with all tracked fixtures and localhost access had **4,287 passed, 1 failed, 117 ignored**. The failing existing `jv_cover::nagao_finds_planted_six_sums_and_agrees_with_the_oracle` exceeded its 120-second solver budget as shared-host load rose; the same test passed in a focused low-concurrency run and in an interrupted serial suite. The full gate is **not green** locally. The rebased head cannot start it until current-main build errors are fixed. |
| Live S3 transfer; specified prime/cubic target IC; full AES/SHA attack coverage | Not verified by the local checks. The temporary directory remote does not establish S3 availability or permissions. |

## Completion gates

1. Define a finite, versioned CLI coverage grid for algorithm, field, curve, technique, input format, output certificate, and resource envelope. Unsupported combinations should fail before work begins.
2. Connect the prime and cubic IC pipelines to commands with specified public curve and target inputs. Verify scalar replay and report every phase alongside a matched rho run on the same point.
3. Connect optional isogeny transport only after verifying the ordered route, map, subgroup action, and transported generator and target.
4. Make artifact publication and retrieval explicit for each pipeline output; check write-once upload, remote existence, retrieval, and hashes against an authorized S3 test prefix.
5. Inventory each AES and hash attack module by accepted inputs and output certificate, then add CLI adapters and acceptance tests for the concrete techniques chosen for support.

This audit is a status record. Its completed local checks do not close the remaining rows.
