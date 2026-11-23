# Sparse H2, fair plain controls, and staged recovery

Follow-up to merged [PR #4923](https://github.com/isogeny-labs/crypto/pull/4923),
recorded in [PR #4945](https://github.com/isogeny-labs/crypto/pull/4945).
Initial tracking PR #4937 merged while implementation was being completed;
it contained only the task entry. The code and evidence are in the continuation.

**Implementation and development qualification are complete; campaign execution
is blocked before launch.** The retained optimized native suite reports 76
passing tests, zero failures and zero ignored tests across 17 executables.
These are unsigned development checks, covered by current claim
`PCC-20261123-f873fd` with standing **unverified**. No Prime31 campaign, recovery
comparison or performance measurement was executed. [pcc:unverified]

## Read first

- [Visual tutorial and follow-up PDF](reports/Prime31_Sparse_H2_Followup.pdf):
  isogenies, the sparse route, cost boundaries, matched controls, twists, and a
  step-by-step checklist on the last page.
- [Mathematics](MATHEMATICS.md): exact kernels, maps, equality conditions and
  the twist distinction.
- [Protocol](PROTOCOL.md): frozen prospective input pools, selection rules,
  physical caps and acceptance conditions.
- [Accounting](ACCOUNTING.md) and [baseline review](BASELINE_REVIEW.md): what
  each control measures and which costs remain unavailable.
- [Development receipt v2](DEVELOPMENT_RECEIPT_v2.json),
  [claim status](CLAIMS.md), and [execution status](RUN_STATUS.json).

## What changed

For H2 on `y²=x³−x`, quotient the full rational two-torsion subgroup first.
The first degree-four map has x-coordinate

\[
\delta_x(x)=\frac{(x^2+1)^2}{x(x^2-1)}.
\]

Its finite-fiber equation is `x^4 − z*x^3 + 2*x^2 + z*x + 1 = 0`.
Eight more degree-two maps finish the degree-1024 quotient. Native contracts
compare the complete kernel, fiber, numerator, denominator, endpoint and final
curve with the retained standard route. A sparse input equation alone makes
no solving-degree or speed claim. [pcc:unverified]

The v2 plain control reads a frozen, hash-bound reference column count without
constructing an auxiliary curve inside its algorithm. Independent audit
reconstructs the reference separately. Historical reference-preparation cost
is unavailable and remains null. The outer process includes parsing and lookup;
the internal algorithm clock begins afterward. [pcc:unverified]

The additional pair-table control uses exact source-pinned production table
construction/probing and batched group arithmetic through a new adapter.
Its cancellation-first policy, table setup and scoped native work are explicit.
It shares the public target, factor base, query stream and recovery criterion
with the polynomial controls. [pcc:unverified]

## Prepared does not mean executed

| Stage | Prepared inputs | Query ceiling per cell | Selected / attempted / completed |
|---|---:|---:|---:|
| Screening | 77 | 64 | 0 / 0 / 0 |
| Recovery candidate pool | 385 | 65536 | 0 / 0 / 0 |

The recovery pool consists of five paired search streams on one fixed public
target. It is not five independent targets, a timing-repeat panel, or an
admitted workload. Every cell retains one computational thread, 4096 MiB,
120 seconds outer wall time, at most 110 seconds collecting, and zero retries.
The inherited aggregate USD5 ceiling includes prior expenditure; unknown
remaining headroom blocks spending. [pcc:unverified]

The earlier 55-cell and 230-cell protocols remain open and unchanged. Their
requested work has not been replaced by these prospective pools.

## Evidence and limits

The retained raw archive contains both build attempts, immutable test/CLI
binary hashes, raw test logs and the failed root integration attempt. The
first native build failed on a missing optional-field test initializer; the
second build and focused suite completed. A separately bounded root-library
check stopped before running any tests on an unchanged duplicate key in the
root `Cargo.toml` at line132. Its failure is not replaced by the focused result.
[pcc:unverified]

The native identity certificate `IDC1hf4aada9d1a00f5d4` is evaluated evidence
with 64 issuer points and 32 additional checker points. Its `proof` is null.
The exact field is the decimal integer `2305843009213693951`.
[ERRATA.md](ERRATA.md) preserves the original metadata receipt and explains
the corrected decimal-string field in v2. [pcc:unverified]

Current gaps are the failed checkpoint upload, unavailable authenticated
runner submission, missing live budget/capacity receipts, and an unfrozen
canonical execution protocol/selection/image binding. No comparison of solving cost,
useful-rank yield, complete scalar recovery or global best-method conclusion follows from
the development checks. The pending execution is recorded in
[`XP-20261122-prime31-sparse-h2-staged`](../../../backlog/XP-20261122-prime31-sparse-h2-staged/README.md).

## Published source

The immutable native source check returned exit0 against published commit
`44e05d7b3cb7ece0d1e761222f8df4e64b2197a5`, matching all84 inventoried
inputs and retaining the same binary hash. [POST_PUBLICATION.json](POST_PUBLICATION.json)
records the scope under claim `PCC-20261123-a63eb4`, standing unverified.
This is unsigned source-custody evidence, not execution admission.
[pcc:unverified]

## Reproduction and next action

Build the isolated `../native/Cargo.toml` workspace in release mode using its
locked dependencies; the original qualification used Rust1.100.0, offline
Cargo and two build jobs. The existing `../isogeny-choice/qualify-development.sh`
copies test binaries before executing fixed contracts. Preserve separate
build, pre-call and post-call hashes, including failed attempts.

The native `staged-panel` command creates inputs only; `source-check` checks
the published source against the binary's embedded inventory. The identity
issuer emits the evaluated certificate and independent arithmetic check.
These commands are not experiment receipts.

To resume execution, first satisfy the actual checkpoint/runner/budget/protocol
gates, freeze and publish a concrete stage selection, and run the existing
`run-staged.sh` wrapper. Inspect and replay every pilot outcome before selecting
recovery candidates. Preserve failed, unknown, killed and capped outcomes;
report timing ratios only after the required external timing admission.
