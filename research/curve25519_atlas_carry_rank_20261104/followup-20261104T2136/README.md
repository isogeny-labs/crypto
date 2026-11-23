# Atlas, carries and rank: implementation follow-up

2026-11-04. All check outcomes below are producer observations with **unverified
standing** [pcc:unverified]. The original study, claim, integer failures and
published branch remain unchanged. This follow-up completes the native runner,
bounded solver representation, arithmetic replay and source-bound packaging.

## Requested work and evidence

| Requirement | Implemented deliverable | Remaining acceptance work |
|---|---|---|
| 1. Isogeny representatives | Five matched support arms; RFC-parameter GMP fixture; small-prime homomorphism, dual and inverse checks | Original admitted panel and matched solver costs |
| 2. Sparse addition and carries | Full signed, distinct-column three-sum constraints; bounded bit-vector lowering of mod and carry encodings | Complete fixed-cell comparisons, including censored outcomes |
| 3. Useful relation selection | Training-only coefficient-rank/probe criterion; holdout sign orbits disjoint from training; raw generic rows retained | Held-out usefulness and complete selection/setup accounting |
| Native execution | Write-once receipts, input/binary hashes, explicit failures, independent OpenSSL arithmetic implementation | EKS execution and independent signed verification |
| Reproducible packaging | Published source, locked Rust dependencies, digest-pinned base images, UID 1000 runtime | Registry digest, checkpoint and admitted deployment receipts |

These are compositions of existing leads, not established novel mechanisms.
The [original source review](../README.md#source-review-and-novelty-correction)
records RFC 7748, Galbraith chapter 15, the lab lead atlas, existing Edwards
systems, traits and the tower-oracle gap. Before this extension, the
[SMT-LIB fixed bit-vector theory](https://smt-lib.org/theories-FixedSizeBitVectors.shtml)
and native OpenSSL EC/EVP interfaces were also checked. The bit-vector work
changes the representation of bounded equations; it does not supply compact
factor-base membership or Weil descent.

![Implementation and outstanding admission](workflow.pdf)

Editable diagram: [DOT](workflow.dot); vector: [SVG](workflow.svg). Dashed
stages are outstanding. This is a workflow diagram, not an isogeny graph or
performance plot.

## Bounded solver representation

`bv_lowering.hpp` lowers the bounded integer expression grammar to signed
bit-vectors. The width is `max(20, 2*field_bits+5)` in the supported miniature
domain. Products, linear combinations and radix carry columns have headroom;
canonical field variables remain explicitly bounded. Signed remainder is
corrected by adding p when negative. The retained fixture checks exercise the
actual solver formulas, including deliberately incorrect residues.

The [fixture records](fixtures/) retain 196 product cases and 30 relation
cases: 20 SAT models replayed natively, 10 UNSAT answers matching the finite
reference [pcc:unverified]. Each archive is below 64 KiB; manifest and summary
files remain beside them. Earlier QF_NIA timeouts stay in the original checks.
No speed ratio is inferred between these separate development attempts.

## Runner, replay and records

`control/` is a native Rust controller. Each fresh solver process retains its
arguments, raw stdout/stderr, resource counter, exit status, witness replay and
per-row receipt. It accepts the precise Z3 get-model-after-UNSAT diagnostic;
other solver errors invalidate the row. Timeouts are neither UNSAT nor wins.

`openssl_replay.cpp` uses OpenSSL point arithmetic rather than the producer's
group-law implementation. It converts Montgomery coordinates to a short
Weierstrass model, checks subgroup/base membership and verifies the complete
signed three-point sum. Tiny UNSAT cases use a separate exhaustive OpenSSL
census. The RFC fixture also cross-checks three clamped X25519 outputs using
OpenSSL EVP. These producer-invoked checks do not establish independent
execution or PCC verification [pcc:unverified].

The retained [OpenSSL rejection checks](openssl-replay/) reject a forged
UNSAT result for a known positive query and a duplicate model coordinate
[pcc:unverified]. [Seven export canaries](export-canaries-v2/) reject changed
raw output, mismatched receipts, forged resource counts, wrong source, wrong
codec, changed column count and a changed deadline [pcc:unverified].

| Development record | Planned / attempted | Successful producer checks | Retained limitation |
|---|---:|---:|---|
| Historical `unit-example` | 13 / 13 | 13 | Predates mandatory external replay/source binding |
| `unit-example2` | 13 / 13 | 10 | Three carry queries timed out at 30 seconds |
| `image-smoke` | 13 / 13 | 13 | Local container integration only |
| `final-image-smoke` | 13 / 13 | 13 | Final native/exporter packaging; local integration only |

All table observations are [pcc:unverified]. `metadata-recheck` repeats the
canonical export/file check with the tightened exporter and the same retained
local records; it is not another solver run. Canonical schema acceptance does
not promote the recorded scientific or execution standing. Every export uses
`verification.status=no_claim`; clocks remain null and invalid. Z3 resource
units exclude generation, transport, selection, replay and matrix work and
are not GAE, S, wall time or complete pipeline cost.

## Exact source and packaging

The first full image smoke uses published source
`ecb2e20aac22d99fd628f4c8b4861dcd57645135`. `image-smoke` records the image
configuration, four native binary hashes, clean-source input configuration,
13 row receipts and canonical export. `validation/image.iid` is the local
Docker image identifier, **not** an ECR manifest digest.

The final published build source is
`5a7e0fa433aa01dfefee8ec7fcb7ad2e6765a957`: it includes tightened export identity,
deadline, codec and column bindings and fewer builder layers for VFS storage.
Its container receipt is retained separately in `final-image-smoke`; source
and image identities must not be combined across the two runs. An intermediate
rebuild failed for lack of disk space before any fixture ran; its log and cache
cleanup receipt remain in `validation`.

The image is built from an exact committed study subtree. It contains native
Z3 5.3.3, the C++ study and OpenSSL replay executable, and the locked Rust
controller. A build-only CA secret supplies the execution environment's
network trust; it is not embedded in the image. The runtime runs as UID 1000,
and local smoke execution disables networking.

## Original panel and real blockers

[Twelve prospective protocol drafts](prospective/panel.tsv) preserve fields
2029/8173/32749, seeds 13/29/47/71, N=8 and 32 training plus 32 held-out
targets. Each cell has 416 solver rows; the panel plans 4,992 rows. Schema
checks pass, but draft acceptance is not admission [pcc:unverified]. Runtime
preflight is unmeasured, source/image pins are not frozen for deployment, and
the USD 5 aggregate budget headroom has not been established.

[AWS access](validation/aws-access.txt) has no configured credentials. The
[checkpoint gate](validation/checkpoint-gate.txt) explicitly refuses because
neither an AWS profile nor a checkpoint upload token exists. The GitHub
ecbench build route builds its own instrument rather than this custom study;
dispatching its stock fixture would not execute this request. No skip, IAM
change, quota increase, paid job or substitute scientific run was used.

Next executable step: establish the existing authorized EKS/checkpoint access,
check live capacity and budget, publish this custom image by digest, freeze
the exact source/protocol/input identities, run the unchanged panel and replay
every retained outcome. Any preflight-driven protocol amendment must precede
affected data and remain visible. A complete comparison and signed independent
verdict remain outstanding; no index-calculus improvement is claimed
[pcc:unverified]. The [problem record](../../../problems/curve25519-atlas-panel-admission.md)
and [blocked backlog card](../../../backlog/XP-20261104-curve25519-atlas-carry-rank/README.md)
retain these exact requirements, including the missing queue/checkpoint and
independent-checker integration before a ready-card promotion.

## Repository validation

Native self-tests, three Rust controller tests, strict clippy and the metadata
checks are in [validation](validation/). The PCC compatibility fix is in
[crypto #3434](https://github.com/isogeny-labs/crypto/pull/3434), with matching
[autoresearcher #3023](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3023)
and [cryptanalysis #1401](https://github.com/isogeny-labs/cryptanalysis/pull/1401).
Twelve PCC tests and 59 sabotage classes pass locally [pcc:unverified]. The
immutable original claim remains `not_covered`; no producer signature or new
trust was added.

The broad release-library check finished: **6,006 passed, 12 failed, 130 ignored**
out of 6,148 declared tests, exit 101 [pcc:unverified]. The retained
[receipt](validation/library-receipt.json) reconciles that full attempt. Sparse
device and bounds fixtures were restored, after which six device tests and the
bounds test pass on targeted rerun; this repairs six of the original failure
cases [pcc:unverified]. Diagnostic clock tests require
`/proc/self/ns/time` and `/proc/self/timens_offsets`, absent on this host.
Four untouched assertions also fail: Gaudry four-large-prime coverage, residual
operation accounting, wide/word pipeline count equality, and ML profile-limit
binding. The library sources and root Cargo files are byte-identical to the
compiled base across this follow-up. The full suite is not a passing gate, and
targeted successes do not replace it. No assertion is relaxed or ignored here.
Raw log parts decompress in filename order to the original SHA-256-pinned log.

## Reproduction

From the study directory, install the documented native dependencies, then:

```sh
make test
cargo test --locked --manifest-path control/Cargo.toml
cargo clippy --locked --manifest-path control/Cargo.toml \
  -- -D warnings
cargo build --locked --release --manifest-path control/Cargo.toml
study=/tmp/curve25519-atlas-carry-rank
solver=/path/to/z3
bash test-products.sh "$study" "$solver" /tmp/fresh-products bv
bash test-encodings.sh "$study" "$solver" /tmp/fresh-relations full bv
```

The [container reproduction script](../reproduce-container-unit.sh) binds a
published commit, fresh absolute output directory and canonical checker path.
It retains incomplete outcomes and exports all completed row receipts.
The `--runner-fixture` command generates the fixed local integration fixture;
it is not a replacement for `--run P 8 32 SEED FRESH_OUTPUT bv`. Runner/export
commands, source-bound container construction and complete retained records
provide development reproducibility; independent verification remains open.
