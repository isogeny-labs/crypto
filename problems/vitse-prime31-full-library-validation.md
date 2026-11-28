# Vitse Prime31 full-library validation

- **Status:** blocked on an existing duplicate production test module
- **Kind:** current-main compilation failure; integration-validation gap
- **Affected work:** [PR #5016](https://github.com/isogeny-labs/crypto/pull/5016), [study](../research/vitse_prime31_compatibility_20261123/REPORT.md)
- **Next action owner:** production root-finding module owner, then lane/vitse-prime31-compatibility for replay
- **Last updated:** 2026-11-23

## Problem

At exact main 71408278696c957d4b5983d4049bdac3e1bc98ba, the full root-library
suite fails with Rust E0428 because src/cryptanalysis/prime_radical_root.rs
defines the test module round73_mersenne_single_fold twice, at lines 5097 and
5409. No root tests execute. The study does not change that source, root
Cargo configuration or defaults. [pcc:unverified]

The isolated compatibility and identity checks do not substitute for the
full suite. Earlier EFD syntax errors were repaired by the newer main and
are retained below as historical evidence, not the current diagnosis.

## Evidence and diagnosis

| Attempt | Exact outcome | Interpretation |
|---|---|---|
| 01: baseline, 600-second cap | Timeout while compiling isogeny_algos; no tests | Cold build window exhausted. |
| 02: baseline, 1,200-second cap | Exit 101 after 764.78 seconds: missing tracked module | Sparse checkout missed transitive relative includes. |
| 03: complete baseline closure, 1,200-second cap | Exit 101 after 4.785 seconds: four EFD delimiter errors; no tests | Existing baseline source could not parse. |
| 04: main 71408278, 1,200-second cap | Exit 101 after 20.599 seconds: duplicate test module E0428; no tests | EFD repair passed parsing; a separate committed test-module collision remains. |

These are unsigned same-team observations. Attempt 01 is registered as
PCC-20261123-1c4d29; attempts 02/03 as PCC-20261123-59776e; attempt 04 as
PCC-20261123-fb8892. The exact command throughout is
cargo test --release --lib --locked, with unchanged defaults and one job.
[pcc:unverified]

Attempt 04 binds all 2,737 materialized tracked files before and after the
call. The failing source matches Git blob
fad8c9f6a1e49621c89ec2772c73e57914dcf02c, SHA-256
0c7e2e8ead138d085adcccbaab612e1804560d2e428758e677c2cf4b69b7de11.
The manifest, lock, toolchain and earlier attempt records remain unchanged.
[pcc:unverified]

The [gate record](../research/vitse_prime31_compatibility_20261123/ROOT_GATE.md)
links each frozen protocol, original log, outcome and source receipt.
No tests were excluded, features pruned, assertions removed or production
source files patched in this lane. The local missing-file problem was
resolved before classifying the committed source failures.

## What would unblock it

- [ ] The production owner resolves the duplicate test-module declaration
  while preserving the intended test assertions and concurrent work.
- [ ] Complete the original unfiltered root-library suite on the review-head
  source and dependency closure, retaining exact source identity and outputs.
- [ ] Check applicable current-head CI and required review before merge.

This closes integration validation only. It does not supply a signed
independent scientific verdict, a concrete Lean bridge, a new solver or a
performance comparison.

## Next discriminating step

Inspect prime_radical_root.rs at the next maintained main revision. If its
blob changes with a duplicate-module repair, freeze a new source-bound gate
and use the existing dependency caches. Repeating the unchanged failing
source would not test a new hypothesis. Keep all four original attempts.

## Resolution and history

2026-11-23: preserved the cold timeout, completed local source hydration,
reproduced the baseline EFD failure, then reran the full gate when main
changed that source. The later revision resolves EFD parsing but fails on
the duplicate root-finding test module. The selected PKM, identity, SHA and
selector blobs remain unchanged. The broader root gate remains open.
