# Complete public-target CPU validation through 40-bit fields

The existing native CPU pipeline completed **24 of 24 calls on 12 distinct
public targets**, with four successful calls at each field size. Every original
external certificate replay passed. The two identical arms produced byte-identical
certificates for each target. These are A/A checks, with two distinct targets
per curve. The unsigned outcome record is `PCC-20261123-e8d860`. [pcc:unverified]

| Prime-field bits | Prime subgroup-order bits | Distinct targets | Completed calls | Retained relations per target | Relation attempts per target |
|---:|---:|---:|---:|---:|---:|
| 20 | 20 | 2 | 4/4 | 49–57 | 57–60 |
| 24 | 25 | 2 | 4/4 | 71–79 | 80–86 |
| 28 | 28 | 2 | 4/4 | 105–109 | 106–111 |
| 32 | 32 | 2 | 4/4 | 128–136 | 185–215 |
| 36 | 36 | 2 | 4/4 | 178–183 | 300–380 |
| 40 | 41 | 2 | 4/4 | 267–280 | 332–380 |

Counts and ranges above are read from the recorded rows, including the repeated
identical-arm observations. They describe this fixed cohort. The CPU method is
the existing generic-decomposition `CachedMitm` pipeline, with eight slots and
batch size 64. No candidate modification was measured. [pcc:unverified]

## Prospective design and actual flow

The [frozen protocol](protocol.json) was published at
`c691d21db7a03726e06c6ae1c0b542cdc0ce38c3` before the first cohort call;
canonical protocol SHA256 is
`f4154a35224588ece8fd2b6a6c6d089a95907c9f1c5e468322c2098e88350bad`.
The unsigned prospective record is `PCC-20261123-c84309`.
The [README](README.md) remains the immutable preparation snapshot.

The run used all six fixed curves and seeds 101/202. Public targets are chosen
by setting x to `seed*1009 mod p` and taking the first decodable even-y point.
No target scalar is constructed or passed to the solver. The exact tuples,
targets and recovered scalars are in the local certificates, export and
review 05. Native admission checks primality, nonsingularity, generator order
and the Hasse-interval uniqueness witness. [pcc:unverified]

Original limits stayed at 120 seconds per solver call, 60 seconds per external
replay and 1500 seconds total, with one attempt and stopping before the next
level after any failed cell. All scheduled calls ran; none failed, timed out,
was excluded or resumed. The unchanged primary analyzer reports
`planned=24 attempted=24 verified=24 failed=0 not_attempted=0`. [pcc:unverified]

The local run lasted 11:55:37–11:56:53 UTC on 23 November 2026. It used one
worker under the shared execution lock. No build/test ran during those calls.
The process limit was 4 GiB of virtual address space. The sampled whole-cgroup
peak was 13,337,882,624 bytes against a 15,032,385,536-byte guard; this includes
unrelated retained research and is not per-solver RSS. No physical stop occurred.
[pcc:unverified]

## Controls and secondary audits

Before cohort execution, unchanged Rust 1.99.1 builds and 47 native tests passed
with zero failed or ignored; one other harness was empty. All 108 frozen
source/input hashes and seven frozen executable identities still match after
the cohort. The build records name source commit `cc893a73`; the clean run
checkout is `c691d21d`. The crypto source is identical across those commits.
The build checkout had a separate journal change; `source.dirty=false` in the
export describes the pinned program/input comparison, not the entire compiler
checkout's journal state. [pcc:unverified]

Post-run audits added 24 further Native Archive replays and an OpenSSL 3.5.7
implementation of public-point checks. All 24 OpenSSL positives passed; all
24 wrong-scalar controls rejected at `SCALAR kP=Q`; all 24 consistent negations
`(r-k,-Q)` rejected at `BINDING public target`. The last control tests binding
to the prescribed input even when an alternative point/scalar pair is valid.
These secondary checks were introduced after the cohort, separately from the
frozen primary analysis. They add no solver targets. [pcc:unverified]

The C checker reconstructs each public target and uses explicit point double/add
loops without installing an asserted order in OpenSSL. It checks nonidentity,
`rP=O` and `kP=Q`; it does not independently prove primality, Hasse cardinality,
full relation correctness, timing or execution provenance. Full certificate
replay remains Native and shares the solver's arithmetic library. Read-only
team reviews inspect retained artifacts; they are distinct from a signed
independent execution or validator attestation. [pcc:unverified]

## Export correction and evidence access

The original export failed the canonical cell-key check because both seeds used
the legacy CSV's replicate 0. Preserved v2 fixes those keys but rounds five
timing fields. The lossless successor uses arbitrary-precision JSON numbers and
checks the entire recursive delta before and after serialization. Seven metadata
tests and strict all-target Clippy pass. Final **export-v3b.json** passes the
captured canonical checker with zero findings and preserves all original
measurements exactly. See [the full errata](EXPORT-IDENTITY-ERRATA.md).
[pcc:unverified]

Read-only reviews [05](reviews/COHORT-REVIEW-05.json),
[06](reviews/COHORT-EXPORT-REVIEW-06.json) and
[07](reviews/COHORT-LOSSLESS-REVIEW-07.json) respectively cover original artifact
consistency, the rejected v2 delta, and the final lossless successor. They retain
distinct scopes and immutable evidence manifests. [pcc:unverified]

Compact rows, statuses, analysis, phase receipts, controls and review records are
published beside this outcome. [DATA-RECEIPT.json](DATA-RECEIPT.json) pins the
full local exports and raw certificates. Large JSON data, binaries and build
outputs remain local under the user's no-S3 instruction. Remote publication
alone therefore does not provide the full replay payload. The final handoff
records a separately verified local capsule. No object-store upload occurred.
[pcc:unverified]

The [run report](reports/run-20261123T1155Z/report.pdf) embeds the original primary
plot. Its clocks are L0 descriptive observations, with no inferential interval,
calibrated common-operation accounting or matched candidate/reference. The plot's
word "replayed" refers to separate producer processes. Only u-arm bars appear;
all 24 rows remain in the records. No exponent or comparative speed is derived.

## Requirement status and next experiment

| Requirement | Status and next dependency |
|---|---|
| Local larger complete-recovery cohort | Complete for the 12 prescribed public targets; 24/24 calls and original replays pass. |
| Separate arithmetic controls | Complete at the public-point/binding scope above; 24 positives and 48 negative controls. |
| Prospective protocol and immutable failures | Complete for this cohort; original and rejected export versions retained. |
| Full remote replay payload / signed independent execution | Partial: local capsule and source-bound records; oversized data excluded from Git and no signed execution standing. |
| Original frozen 84-case tensor comparison | Open, with its original protocol and limits unchanged. |
| Improvement over an applicable baseline | Open: reconcile distinct candidate sources, freeze a matched comparison and account for full cold recovery costs. |
| Non-generic mechanism / source-history and bit-work proof | Open: this CPU cohort does not exercise the separate tensor recursion proof. |

Next, choose one distinct reachable CPU candidate from the existing prepared
proposals after source reconciliation, freeze its comparator and acceptance
criterion, and run matched controls on these public-target fixtures. A component
or complete-pipeline improvement will be reported at its measured scope. Keep
the separate mathematical source/history and total-cost bridges active.
[pcc:unverified]

Sources checked: existing CPU `lib.rs`, `checked_sum.rs`, `certificate.rs` and
`examples/scale.rs`; shared `src/cryptanalysis/hashed_ecdl.rs`; original
`research/hashed_ecdl_cpu_20261030/scale/{run.sh,analyze.rs}`; CPU rounds 24/25
proposal records; exact captured canonical protocol skill/schema; official
OpenSSL EC_POINT/BN interfaces used in the checker. Current sibling text reads
failed UTF8 decoding. KB MCP was unavailable and the wrapper exited 127; these
source checks do not constitute a novelty search. Relevant canonical graphs
(`docs/ic-leaderboard.html`, `docs/ic/progress-timeline.json` and benchmark ranking)
are left unchanged because no comparative ranking or gate-qualified performance
entry is supplied. Prior reports, claims and the old 84-case records are unchanged.
