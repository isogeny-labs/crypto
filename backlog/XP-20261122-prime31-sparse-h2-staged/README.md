# Blocked: sparse H2 and staged Prime31 execution

**Status: blocked. No experiment has been launched.** The card has zero funding
and `run.argv = ["false"]`. That command is an intentional launch sentinel,
not the scientific run command. This entry records pending work; it supplies no
runner admission, spending authority, lease, source/image freeze, or benchmark
result.

Tracking: [PR #4937](https://github.com/isogeny-labs/crypto/pull/4937), branch
`lane/prime31-sparse-h2`, following merged PR #4923. The requested card ID carries
the continuing study's date, 2026-11-22; this backlog record was written on
2026-11-23. The required `created` field matches the date in that ID.

## Question and decision value

On the fixed public Prime31 problem, does factoring the same H2 quotient through
the full rational two-torsion subgroup first improve practical setup,
natural-query collection, or verified complete-recovery cost? The comparison
must preserve the complete fiber and use matched standard presentations,
same-base pair tables, plain bases, and all three registered rho controls.

Actual screening receipts will determine the explicit recovery subset. A
replayed complete recovery with matched, complete accounting could support a
practical implementation choice. Failed, negative, capped and unknown outcomes
must remain in the decision record. An input-degree reduction, a passing
correctness contract, a first-hit row, or a high matrix rank is not itself a
complete-recovery cost result.

## Existing work and frozen public problem

The prospective amendment and implementation are in:

- [PROTOCOL.md](../../research/prime31_aux_matched_20261122/sparse-h2-followup/PROTOCOL.md)
- [ACCOUNTING.md](../../research/prime31_aux_matched_20261122/sparse-h2-followup/ACCOUNTING.md)
- [BASELINE_REVIEW.md](../../research/prime31_aux_matched_20261122/sparse-h2-followup/BASELINE_REVIEW.md)
- [DEVELOPMENT_RECEIPT_v2.json](../../research/prime31_aux_matched_20261122/sparse-h2-followup/DEVELOPMENT_RECEIPT_v2.json)
- [RESUME.md](../../research/prime31_aux_matched_20261122/sparse-h2-followup/RESUME.md)

The public problem remains `p=2147483647`,
`E: y²=x³+847514846x+2087444724`, prime order `n=2147446831`,
`G=(625037715,2100627251)`, and `Q=(984837385,1142774582)`. Arity is three, the
auxiliary coset scalar is one, and no target logarithm is supplied to a
collector. The degree-three destination and native/transported-base definitions
retain their existing exact fixtures.

The development receipt reports 76 passing focused native contracts across 17
copied executables, with no campaign or performance comparison. The full
root-library gate remains open: the unchanged duplicate `name` key in the root
Cargo manifest stopped that command before tests. This backlog task neither
repairs that unrelated manifest nor substitutes focused contracts for that gate.

| Prepared inventory | Inputs | Query ceiling | Current execution state |
| --- | ---: | ---: | --- |
| Screening, seed 4188 | 77 | 64 per IC cell | No admitted selection; zero runs |
| Recovery candidate pool, five fixed seeds | 385 | 65,536 per IC cell | Unselected; zero runs |

The five seeds are 4188, 104729, 130363, 169087 and 224737. They are distinct
search streams for one public target and one fixed coset, not independent
targets or timing repeats. Each stream contains 35 auxiliary polynomial cells,
12 auxiliary pair-table cells, 12 plain polynomial cells, 12 plain pair-table
cells and six rho controls. The inventory is not a commitment to execute every
cell. The card's single placeholder shard likewise does not designate an
executable 462-cell batch; replace it with the exact admitted selection only in
a reviewed readiness amendment.

## Actual blockers and prerequisites

1. **Checkpoint upload:** the retained gate attempt reported neither an upload
   token nor an AWS profile. Preserve the actual attempt logs in the study's
   evidence directory; do not replace this gate with a local benchmark.
2. **Authenticated execution:** no authorized IsoLab EKS/Actions submission
   route or current capacity receipt was available to this session. Source
   publication through the GitHub connector does not establish runner access.
3. **Shared budget:** the inherited aggregate lane ceiling is USD 5, including
   earlier builds, failed/interrupted attempts and idle infrastructure. Remaining
   headroom is unknown and therefore blocks spending. Zero cairn funding here
   is a priority choice, not an assertion that the work costs zero.
4. **Canonical freeze and selection:** the canonical experiment protocol,
   original absolute deadline, merged source commit, execution-image digest and
   exact screening cell/hash selection are not admitted. Recovery selection
   requires actual screening receipts and its own prior freeze.
5. **Backlog readiness bindings:** a resumable shard/checkpoint interface and a
   pinned independent verification command are not bound. Existing row/scalar
   replay is available in source, but no nonexistent resume flag or checker
   path is entered into an executable card. Resume semantics must preserve the
   zero-retry rule and completed/failed cell accounting.
6. **Broader validation:** retain the open root-library gate and the exact
   inherited manifest failure. Resolve it through its owning source workflow;
   do not alter unrelated production files to make this card appear ready.

`protocol.path` and `protocol.blob` intentionally remain null. The study's
prospective `PROTOCOL.md` is a source document, not a fabricated committed
`backlog/<ID>/PROTOCOL.md` freeze. Likewise, `code.branch` identifies the active
lane while `code.commit` is null. The build argv documents the existing isolated
native crate; no build was performed to create this card.

## Resource and cost derivation

The source for every bound below is the staged protocol's unchanged physical
envelope and its prepared-input inventory.

| Quantity | Bound or status |
| --- | --- |
| Concurrent work | One cell at a time |
| CPU | One affinity-bound logical CPU; one computational thread |
| Memory | 4096 MiB, represented as `ram_gb=4` and `memory_mb=4096` |
| Outer time | 120 seconds per cell: TERM at 115 seconds plus 5 seconds grace |
| Collection | At most 110 seconds, shortened by remaining cold time |
| Query | At most 2000 milliseconds; cooperative deadline |
| Pair-table setup | Charged to cold setup and bounded by the outer envelope |
| Scientific retries | Zero |
| GPU | None |
| Disk | No reservation admitted; `disk_gb=0` is a blocked placeholder, not a zero-usage estimate |
| Shared dollar ceiling | Inherited USD 5; remaining headroom unknown |
| Aggregate cost estimate | Unavailable until selection, prices, overhead and remaining budget are bound |

Multiplying inventory size by the per-cell outer cap gives a planning bound,
**not a runtime estimate or a funded campaign**:

- All 77 screening inputs at the full cap: `77 × 120 = 9240` process-seconds,
  or 2 hours 34 minutes of sequential cell envelopes.
- All 385 recovery candidates at the full cap: `385 × 120 = 46200`
  process-seconds, or 12 hours 50 minutes.
- Both complete inventories: 55,440 process-seconds, or 15 hours 24 minutes,
  before separate build, audit, qualification, admission and infrastructure
  overhead. At one occupied CPU per cell, the corresponding cell-envelope
  core-seconds have the same numerical upper bounds.

The actual subset is unknown, so `est_core_hours` and `est_wall_hours` remain
null. There is no host-price receipt with which to translate these bounds into
dollars. They do not establish that the inherited ceiling can cover either
complete inventory. Resource caps are not measurements of work performed.

## Readiness and execution sequence

1. Preserve the current implementation, exact source inventory, native
   development receipt and frozen fixture. Recheck the current baseline review
   and any source changes before pinning the executable revision.
2. Obtain legitimate checkpoint upload, authenticated supported-runner access,
   capacity and a current shared-budget receipt. Keep the original requested
   inventory open if it cannot fit the remaining envelope.
3. Freeze an explicit screening subset with exact ordered cell hashes,
   canonical protocol hash, original absolute deadline and source/image/binary
   identities. Complete required timing qualification separately before using
   any wall-clock ratio.
4. Bind legitimate checkpoint/resume semantics and the pinned native independent
   audit command. Copy and commit the admitted protocol into this card, record
   its Git blob, pin the merged code commit, and replace all blocked placeholders
   in one reviewed amendment. Only then may `backlog validate` and readiness
   review establish a runnable card. Keep the existing `run-staged.sh` admission
   checks; they must not be bypassed or satisfied by changing a status label.
5. Publish/lease the admitted work through the applicable cairn/supported-runner
   workflow and execute the exact selected screening cells once, preserving
   every failure, unknown, timeout and incomplete stream.
6. After the pilot receipts exist, freeze the recovery subset using the stated
   selection order, retaining the required representation pairs and matched
   controls. No recovery shortlist is claimed in this card.

The per-cell resource limits remain unchanged throughout. This entry grants no
permission to increase the budget, extend the deadline, create a new account or
pool, or relabel retries.

## What counts as done

Results land under
`research/prime31_aux_matched_20261122/sparse-h2-followup/results` through a PR.
Retain every selected/unselected input decision, exact source/image/binary
identity, stdout/stderr, exit code, incomplete stream and process observation.
Missing counters remain unavailable rather than zero.

The existing independent auditor's result schema is
`prime31-aux-matched.offline-audit/v1`; raw event streams bind the v2 cell and
must be retained alongside that audit and the outer process receipts. The
empty `outputs.verify` array means its executable path/binding is pending, not
that verification is optional. A readiness amendment must name the actual
pinned independent check.

For a complete recovery, independently replay the natural query coefficients,
signed witnesses and rows, then verify the scalar against the original public
target and the destination when transported. A useful target-bearing dependency
can determine the scalar before full factor-base rank. An accepted row or rank
increase alone is insufficient. Complete accounting includes each algorithm's
own cold setup, optional pair table, collection, matrix work and replay, plus
separately retained study expenditure. The internal clock excludes input parsing
and final summary output; the outer process observation must retain them.

If the admitted attempt is censored or fails, retain that outcome and its cost
at the tested scope. Do not infer success from generated inputs, a valid audit
of partial output, development tests, or an unsigned identity certificate.

## Card metadata validation

The current rules were read from the exact tracked objects because this sparse
checkout omits the backlog documentation from disk:

- `backlog/README.md`, Git blob
  `b6a29d96b0ec8a1c1b035c187d80e6cd9585f11d`.
- `backlog/schema/card.v1.json`, Git blob
  `4d87ef178822276fdc601eb1c971718ffc6b886e`.
- The available source `src/bin/backlog/card.rs`, including required keys,
  ID/date agreement, blocked-link rules, non-executable-state allowances,
  placeholder resolution and lab-task field requirements.

No `backlog` executable was found on PATH or in the workspace. A normal root
build is blocked by the already recorded duplicate manifest key; it was not
retried or repaired. The available JSON Schema CLI is used only to validate
this card's metadata against the exact tracked Draft202012 schema. The retained
`validation.json` records its outcome and the narrower scope of this fallback.
It is not a `backlog validate` pass, experiment verification, runner admission,
or permission to launch.
