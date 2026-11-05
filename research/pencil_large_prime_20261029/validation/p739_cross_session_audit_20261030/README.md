# p739 retained cross-session audit

This directory contains a native, solver-free comparison of the preserved
computation and fresh p739 sessions in
`/home/ec2-user/pencil-actual-results-20261030-a1`. It records observations
only. It does not approve a protocol, admit a timing result, price a ratio, or
make a scientific-priority judgment.

## Result and scope

The v2 checker maps all 1,080 records bijectively by arm, role, warmup flag,
round, exact unsigned 64-bit algorithm seed, workload ID, and method ID. It
also joins every record back to its resolved plan slot, including the full
workload and method objects. The resulting deterministic solver outcome,
method, phase, and operation-accounting projection is exact for all 1,080
pairs. Its SHA-256 is
`91338d7abcda1144d949d64d827de81d874ca49d07a2b536cd803f42ff78a3e1`.

Before forming that projection, the checker recursively inventories every
differing leaf. The 76 permitted paths are fixed in `src/lib.rs`; the current
records exercise all 76. Any additional path, including an additional trailing
array element, fails the audit. `output/audit.json` records the complete
allowlist and the observed per-path counts. `output/row-mapping.json` records
all 1,080 pair identities and per-row projection hashes without copying either
raw record stream into Git.

This is not equality of all non-wall or all resource content. The projection
excludes wall and CPU timings, maximum RSS, faults, voluntary and involuntary
context switches, scheduler CPU and CPU-identity observations, IRQ
observations, isolation/environment fields, and session and record IDs. Those
observations remain in the immutable raw records and are preserved by their
whole-file hashes.

Both retained `ecbench.audit/v1` receipts bind complete 1,080-record sessions.
The v2 checker additionally requires each receipt to contain 900 distinct
replay sequence and record-ID pairs that exactly cover the 900 measured
records. All are marked reproduced and identical. The 180 warmups are
statically audited and cross-session compared but were not replay candidates.

## Receipt provenance

The computation audit SHA-256 is
`1ca83b0869dff64952b8aa85652c142bf7ab96eacd691d5f8dc191a782e2d0ba`.
The checker byte-matches it and the other five inputs against the retained
controller archive, SHA-256
`6c06a0027abebb6c3e9e15725cda8342e407d4942ebb010b1f230eac975eedec`.
Its receipt says the session finished at Unix millisecond `1793319668179`
and the audit ran 23,869 milliseconds later.

The fresh audit SHA-256 is
`f810db8ad80f1195bbfd965d506bfa4b668505051c6a7f9991904f95414dd2d8`.
It is the exact downloaded content of bucket
`isolab-results-104186811093-us-east-2`, key
`runs/pencil-p739-20261030-a1-epoch-1/1af573c8-a09c-4dcb-89a4-1af63e1d13e0/audit.json`,
version `tVGrPMtyzglfsi9hmkjxJsslb2rYkyXY`. The bound object receipt reports
`2026-10-30T00:26:08+00:00`; the audit's `audited_unix_ms` is
`1793319966863`, 5,730 milliseconds after the session's recorded finish. The
auditor binary and environment identifiers equal the fresh session's. These
facts establish a post-session, versioned remote result artifact. They do not
identify a separate auditor principal or process and do not establish auditor
independence.

## Relevant repository sources checked

- `src/cryptanalysis/ecbench/audit.rs`, SHA-256
  `4bf1d7b984fba92b3ce2feca119baa1f7b30c9f47fd5b70e9b7844836148c83b`:
  existing workload/arm identities, replay selection, and replay comparison.
- `src/cryptanalysis/ecbench/record.rs`, SHA-256
  `5ebb2ecd907d4e388004e5b3db05614bbdd6c0c0e080bba9ccc7d302e621f407`:
  record schema and record-ID sealing semantics.

No external mathematical literature was needed for this bounded software
equivalence check. It introduces no mathematical method or novelty claim.

## Reproduction

The request contains exact paths and hashes for both specifications, plans,
record streams, session and host manifests, audit receipts, archive/object
provenance, and the two reference sources. The checker parses decimal-string
record seeds and numeric plan seeds as exact `u64` values and never converts
them to floating point.

```sh
cargo +1.99.1 test --release --locked --offline
cargo +1.99.1 clippy --release --locked --offline --all-targets -- -D warnings
cargo +1.99.1 run --release --locked --offline -- request.json output
cd output
pdflatex -interaction=nonstopmode -halt-on-error report.tex
```

Thirteen native tests cover exact values above `2^53`, malformed and
overflowing decimal seeds, cost/seed/workload/method mutations, missing and
duplicate records, duplicate replay entries, unexpected future fields, and
trailing array entries. The final test, Clippy, comparator, and PDF-render logs
are retained beside the source.

## Historical v1

`historical-v1/` preserves the first successful comparator source and output.
Independent review found that it did not join records to plan slots, did not
require replay entries to be a distinct exact cover, and allowed an extra
trailing array element to hide behind an allowed length difference. It is
retained for review history and is superseded by v2. Neither version ran a
solver or took a measurement.

## Files

- `output/audit.json`: standardized summary, bindings, explicit omissions,
  observed difference counts, provenance, and limits.
- `output/row-mapping.json`: compact 1,080-row exact mapping.
- `output/equivalence.svg`: editable vector count graph.
- `output/report.tex` and `output/report.pdf`: neutral one-page report.
- `request.json`: exact input and source bindings.
- `src/`: native comparator and mutation tests.
- `historical-v1/`: superseded first successful implementation and output.
- `SHA256SUMS`: final packet hashes.

