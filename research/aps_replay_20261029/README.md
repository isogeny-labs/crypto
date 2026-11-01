# APS independent transcript replay

Continuation of [PR #2803](https://github.com/isogeny-labs/crypto/pull/2803),
based on `91902d0ca3ec365cf346e5b36aa0ece894c2af2f`.

Integrated published main `992c2579e2891c1c58ee70183888d3a9ce1d32e4`
cleanly as local merge `c07061958dcac0b0ffbe7228e2bbc69344cc5c1d`.
At that integration the task's four pinned source/lock digests matched. Validation
below distinguishes the initial build from post-integration checks.

This implements a native independent replay path for the existing tiny public
fixture transcripts. Completed checks: 34 APS library tests (14 new replay
controls), 10 fixture-runner tests, and strict final-source Rust 1.98 lint.
The CLI matrix covers all twelve arms in both exhaustive and first-informative
modes. Rust 1.99 strict lint passed before the final fixture-only test addition.
The required full library run on the initial compiled tree passed: 5,220 passed,
0 failed, 123 ignored. After integrating main, APS/F4/example tests passed
34/67/10 and strict Rust 1.98 lint passed. The earlier full suite is not
presented as a new-main full-suite run. See the validation summaries.

| Requirement | Acceptance criterion | Status |
| --- | --- | --- |
| Separate EC arithmetic | Reuse BigUint `ecc::point::Point`/`FieldElement`, with no producer solver, fast arithmetic or recovery calls | Verified on fixed public fixtures |
| Input and fixture binding | Strict schema, duplicate-key rejection, bounded input, exact model/point/identity checks | Positive and rejection controls pass |
| Sampling and labels | Reproduce each seeded attempt, discarded draws, sign conventions and orbit labels independently | Both policies, three fixtures and wrapping seeds pass |
| Relations and recovery | Check every reported coordinate set and signed witness; recompute A/B and dG=Q, rejecting B=0 | Corruption and tautology controls pass |
| Complete outputs | Independently enumerate allowed point tuples and compare complete sets; retain partial enumeration as partial | Omitted-root and partial-output controls pass |
| Native CLI and controls | Standalone replay, producer round trip, corrupted labels/metadata/status/witness and omitted-root controls | 24 arm/mode cases and four shipped pairs pass |
| Complete-cost panel | Preserve accounting/export/protocol/checkpoint gates and existing caps | Still open |

Sources checked before implementation:

- [SEC 1 v2.0](https://www.secg.org/sec1-v2.pdf), sections 2.2.1 and 3.2.2.1:
  group law, canonical coordinates, curve membership and subgroup checks.
  PDF SHA-256 `54687189af3de645756a30a801b596fd14c6566992c13bfd58eede399f324a89`.
- Existing `src/ecc/{point,field}.rs`, the separate BigUint reference backend;
  `aps_single_relation.rs`, `prime_fast.rs` and the current fixture runner:
  actual transcript layout, coefficient labels, seed/retry handling, Montgomery
  sign representative and completion semantics.
- The [earlier source map](../aps_extensions_20261026/SOURCES.md) and
  [four-lead study](../aps_followup_20261028/README.md): APS's zero-sum relation,
  source-linked chord law and the remaining correctness/cost boundaries.
- Read the sibling lead atlas and matched independent replay/accounting terms
  before selecting this implementation prerequisite; no new attack mechanism
  or novelty claim is proposed.

The verifier shares serialization, RNG and curve-identity conventions with
the producer. Its EC/relation checks use separate arithmetic. It attests
mathematical results and recorded sampling, not execution history, timings or
the truth of F4 cost counters. Complete operation accounting, native experiment
export, independent metric validation and a frozen controlled protocol remain
separate work. No performance experiment or EKS job is authorized by this note.

The full-set point oracle is audit work. A future timed driver needs a frozen,
common online verification interval and separately reported post-run
completeness auditing. This correctness CLI's total duration is not a solver
speed measurement.

## Current main integration

Main `8171fb53733b43b3174abf66c839d43037a30fc2` is included. The tracking-note
conflict preserves both replay studies and all remaining cost/scaling gates.
The three APS Rust files are unchanged from the reviewed PR head; main supplied
a newer dependency lock. On the main `78f84234` working merge, APS/F4/example
checks passed **34/67/10**, strict Rust 1.98 lint passed, and workflow checks
passed. The final main refresh changes unrelated examples and study artifacts;
the relevant sources, lock, APS fixtures and workflows match that tested merge.
See [merge validation](validation/merge-validation-summary.txt) and the current
[merge source pin](validation/merge-source-sha256.txt). The earlier
[source pin](validation/source-sha256.txt) retains its historical lock bytes.
No new-head full-library result or performance measurement is claimed.

## Native use and checked records

The correctness `solve` command retains its original JSON and now requires
independent replay before successful completion. To replay an existing file:

```sh
cargo run --release --locked --example aps_single_relation_check -- replay \
  --input research/aps_replay_20261029/fixtures/toy101-complete.json \
  --output /tmp/aps-replay-receipt.json
```

Replay success means that the transcript's supported mathematical claims are
valid. Inspect `outcome` and `verified_logarithm`: a valid incomplete or complete
no-relation transcript is not a recovered answer. The receipt binds the exact
input bytes, including whitespace, through `input_sha256`.

| Fixed development control | Field/model/order (p,a,b,n) | Input / native replay receipt | Replayed outcome |
| --- | --- | --- | --- |
| Complete line selection | (101,59,6,107) | [input](fixtures/toy101-complete.json) / [receipt](fixtures/toy101-complete.replay.json) | d=37, three complete attempts |
| Early-stop line selection | (211,166,158,227) | [input](fixtures/toy211-streaming.json) / [receipt](fixtures/toy211-streaming.replay.json) | d=73, final enumeration partial |
| Certified pruned orbit | (103,0,5,97) | [input](fixtures/toy103-pruned.json) / [receipt](fixtures/toy103-pruned.replay.json) | d=17, complete recorded set |
| Complete no-relation result | (101,59,6,107), seed 717, one attempt | [input](fixtures/toy101-no-relation.json) / [receipt](fixtures/toy101-no-relation.replay.json) | No informative relation for this base |

Full ICV1 identities, G/Q, labels, every attempt and the producer's diagnostics
are in the inputs. These are fixed unit/correctness controls, not held-out
performance data. The no-relation `solve` exits 1 as designed after retaining
its JSON; standalone replay accepts that valid mathematical null outcome.
The artifact test reads all four inputs and reproduces their full receipts.

Accepted inputs are capped at 2 MiB and 2,048 coordinate sets per attempt.
Only the existing fixture/arm names are supported; other curves or enlarged
parameter families require a separately reviewed extension. RNG reproduction
uses the producer's pinned rand 0.8 StdRng convention and Cargo dependency lock.

See [CHECKS.md](CHECKS.md) for the acceptance contract and formalization
boundary, and [the diagram](replay.svg) / [inspected PDF](report.pdf) for the
source-linked report. The IC leaderboard, performance charts and canonical
curve graphs remain unchanged: no measured gain or new curve route was found.

Prior rejected source-checkpoint/shared-file uploads remain on hold. The local
Conductor service was restored without workers/scheduler; merged implementation
tasks T-3/T-4/T-6 were closed, and this work is reserved under T-7.
