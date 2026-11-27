# Local H1 comparison: the existing last coordinate remains selected

23 November 2026. User-authorized local component control, one thread,
unisolated. All producer findings are **[pcc:unverified]**. This packet
reports an incomplete multiplication counter, not runtime or complete
index-calculus cost.

[Report PDF](report.pdf) · [Held-out counters](held-out.svg) ·
[Original worker summary](original-summary.json) · [Recomputed values](derived.json)

## Outcome

All32 saved systems completed. All192 candidate **root-set comparisons**
match the recorded roots, with zero dense fallbacks. The32 systems contain
34 roots in total. Default non-wall counters and subsequent RNG state pass
the original worker's incumbent-compatibility checks. The fixed development
rule selects coordinate5, the existing last-coordinate default.
[pcc:unverified]

| Zero-based coordinate | Development16 systems | Held-out16 systems | Held-out / incumbent |
|---:|---:|---:|---:|
| 0 | 8,012,329 | 8,008,581 | 2.513 |
| 1 | 7,511,312 | 7,508,605 | 2.356 |
| 2 | 6,072,607 | 6,069,868 | 1.904 |
| 3 | 5,802,646 | 5,800,408 | 1.820 |
| 4 | 5,927,060 | 5,924,159 | 1.859 |
| **5 — incumbent, selected** | **3,188,499** | **3,187,180** | **1.000** |

Unit: the existing **partial `lin_muls` multiplication counter**, including
attempted fast extraction and any dense fallback. Ratios compare that
counter only. The best alternative, coordinate3, reports1.820times the
incumbent on the held-out half. No coordinate change reduces this metric
on the fixed prefix. This is not a complete-cost or elapsed ranking.
[pcc:unverified]

All six development candidates together consume36,514,453 reported
multiplications. Adding the selected held-out extraction gives39,701,633;
the incumbent over both halves gives6,375,679. These totals expose the
selection search rather than present it as free setup. They still exclude
other operations, shared basis generation and other IC phases.
[pcc:unverified]

## What ran and what did not

The source was published at
[`9fe79b2003e460a9faa734c09ce8696584b2ae3d`](https://github.com/isogeny-labs/crypto/commit/9fe79b2003e460a9faa734c09ce8696584b2ae3d)
before the sole local call; prospective record0edbfa lives in that commit.
Worker bytes match the qualified H1 executable, SHA-256
`836600c5193e8b019f8d06cd495dcac50417b389423d60cd49dc33db8d586fe3`.
It links the retained Root41 library while compiling the changed H1 modules
directly. No source or extraction seed was changed for local execution.

- Original Run51 raw input SHA-256:
  `6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d`.
- Exactly the first32 six-coordinate systems, p1009; no new target or
  relation search. Rows0–15 select the action; rows16–31 are an exposed
  same-fixture holdout, with no fresh-instance claim.
- 32 full F4 calls,32 incumbent linear extractions and192 candidate linear
  extractions. No rejected case, retry, replacement prefix or second trial.
- Degree24 with no early staircase/Hilbert stop; full completion flags
  checked. This is not an independent ideal-equality certificate.
- All extraction seeds389. Single thread; 8GiB address-space cap;
  outer120second timeout and five-second forced cleanup. No elapsed metric
  exported. Internal deadline clocks and custody timestamps remain.
- The original eight-call learning audit and older Runpod trials were not
  rerun. Generated bases are retained for later component work without new
  F4 calls.

## Validation and launcher status

The native process writes exit0, empty stderr and `passed:true`. A separate
Rust analyzer reads every captured case, checks ordered indices and all six
root digests, recomputes both counter arrays and the fixed tie rule with
checked wide sums, and matches every original summary/selection value.
The compact cases preserve all per-case counter and root-binding data.
This is producer validation, not independent signed verification.
[pcc:unverified]

The execution tool nevertheless reported session exit1 after printing
`native_exit=0`. The cause is unresolved. Native status, stdout/stderr,
both successful worker-hash checks and all32 completed records are
preserved; this discrepancy is not erased or treated as a reason to retry.
The numeric analysis checks the saved native artifacts directly.

## Local authorization and custody

The user instructed **Run locally** after the cloud blockers were reported.
The S3 checkpoint gate was attempted and failed for absent credentials.
The local plan explicitly records Git/hash custody and unavailable S3
admission. No successful cloud checkpoint, isolation or full benchmark
admission is asserted.

Original LOCAL root:
`/workspace/ic-local-runs/action-operator-h1-20261123-01/`.
`output/cases.jsonl` is89,168bytes and `output/bases.jsonl` is1,362,010bytes.
They exceed the data-file limit and stay LOCAL with exact hashes in
`validation/local-raw.sha256`; artifact-store publication is pending. The
complete compact counter/binding data, summaries, analysis source and
report are committed here. A LOCAL location is not a durable store claim.

The preregistration remained on the previous lane branch because #4986
had merged since its last read-back. This report is a focused follow-up
PR #5094 on current main, rather than a change to the merged PR. Old
evidence and append-only ledger history are preserved.

## Decision and next step

Keep the current default. This fixed coordinate-only H1 panel gives no
reduction in the reported multiplication metric; other fields, bases and
linear-combination operators are not ruled out. Avoid spending on a larger
unchanged coordinate sweep without a new mechanism. H2's elimination
template remains the next distinct construction to investigate. No
whole-solver or rho-parity conclusion follows. [pcc:unverified]

The source-linked graph shows recorded held-out counters, n=16systems,
without sampling uncertainty or runtime estimates. Canonical scoreboards
are unchanged: this does not meet their full-cost/admission boundary.

Sources checked: the H1 implementation/literature packet in the parent
directory, its exact manifests and frozen native artifacts. KB/Conductor
were previously unavailable; no novelty or absence claim is introduced.

## Reproduce analysis without rerunning the solver

Build `native/Cargo.toml` with a separate target directory, then run its
binary with `ORIGINAL_RUN_ROOT NEW_OUTPUT_DIRECTORY`. It emits the compact
cases, checked arithmetic and SVG from frozen data only. Use a fresh
directory: output files are exclusive. No solver, F4 call, timing or new
randomness is part of the analyzer. Strict analyzer Clippy is checked.
