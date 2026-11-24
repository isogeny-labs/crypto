# ECC2K-130 index calculus: below15M total cold instructions

The unchanged fixed n131 d12 uniform query uses **14,688,697 total simulated instructions** in each of five fresh repetitions, below the strict15,000,000 target. Matched baseline **20,779,444Ir**: **29.31% fewer**,1.414655x. The complete oracle uses10,024,047Ir (25.44% fewer). All six cold/oracle non-regression controls pass against both exact previous-best controls. [PCC-20261124-807b8e](../../pcc/claims/PCC-20261124-807b8e.json). [pcc:unverified]

User confirmed **total instructions per cold query**. This is Callgrind Ir; no instructions-per-second, native-time, full-DLP recovery or fastest-external-solver claim.

[Two-page report](report.pdf), [exact six-cell ranges](results-table.md), [plot](instructions.svg), [report source](report.typ).

| Requirement | Evidence and status |
|---|---|
| Strict below15M, same complete cold query | Met on fixed n131 d12 uniform fixture:14,688,697Ir, five identical counts. [Rust summary](candidate3/summary.json), [audit](candidate3/grid-audit.json). |
| Complete cost boundary | Parsing, context/field setup, admission, dictionary/allocation, solver/root checks, original replay, output and process exit charged. Setup4,578,829 + oracle10,024,047 + post85,821 =14,688,697Ir. |
| Semantic qualification |62 focused actual-source release tests and65 retained cases ×3 executables (195 native rows); malformed input, symbolic paths, exceptional/repeated denominator cases retained. [Tests](candidate3/checks/focused-tests.txt), [native qualification](candidate3/equivalence.jsonl). |
| Full controlled panel |120 planned/attempted/completed, zero exclusions/retries, four rotated arms ×6 cells ×5 repeats;240 fresh phased/unphased measurement children. [Audit](candidate3/grid-audit.json). |
| Separate arithmetic cross-check |80 saved tuples replay using separate schoolbook multiply/reduce and Euclid inverse, plus all retained negative checks. [Replay](candidate3/polynomial-replay.json). Same producer; unsigned. |
| Source and export checks |[Unchanged source receipt](candidate3/checks/source-final.json). All three local strict exports pass with zero findings; [final export check](candidate3/checks/export-check.txt). |
| Earlier misses preserved |First17,287,437Ir plus one n83 oracle guard failure; second16,033,438Ir, all guards pass but above15M. [First audit](grid-audit.json), [second audit](candidate2/grid-audit.json). |
| Raw/executable custody |Complete local archives,4,338 raw members and17 executable members; per-member extraction checks. [Custody](checks/archive-custody.json). Large new objects remain local pending store transfer, explicitly enumerated by [pending-artifacts.json](pending-artifacts.json). They are not marked uploaded or silently committed over the size policy. |
| Current PR and remaining gates |This15M rung is checked; prior50x remains future work at its original <=427,207Ir threshold. Root-library execution, artifact-store custody, missing historical leaderboard source and independent signed admission remain open. No admitted row or gate is changed. |

All numeric assertions and producer checks are unsigned. Exact min/max across the five repetitions are in results-table.md and the Rust summary. They are not population confidence intervals. [pcc:unverified]

## Implementation

Lazy shared symbolic reduction matrices, exact shared trace/half-trace maps, word-local projected-row lookup, validated ASCII hex SIMD decoding with the original extended parser fallback, runtime-guarded canonical131 carryless square/multiply with cached ISA dispatch, and reuse for consecutive equal nonzero denominators inside the existing bounded inversion batches. Every supplied point keeps admission and all ordinate/root/group/replay checks; original key/model/index order is unchanged. The prior inline direct index remains.

This is a bundled implementation/configuration result, without isolated per-change attribution. Final panels compile all arms with equal full LTO and one codegen unit. First panel uses the earlier profile and is compared only with its own matched controls. No ratios are compounded across panels. Measured hardware: x86-64 with pclmulqdq/SSE2; portable paths remain but other hardware classes are unmeasured.

These changes reduce implementation work. They do not establish that the full trace-zero variety reduces the summation-polynomial system dimension, or new Z[tau] leverage beyond orbit folding. The previously retained global S4 relaxation failed to reject the actual targets; its nonlinear constraints remain a future algebraic lead.

## Reproduce

Final numerical source **1861f64e14**, corrected launch binding **781e12c34c**, primary protocol publication **9e2d87e27f**. [Protocol](candidate3/protocol.json) SHA6ffbb8617211d163d4d24f7fd9b2d7af13ca1aa0ad96816287180226e5d00060. [Binary identities](candidate3/checks/binary-identities.json), [full source inventory](candidate3/checks/source-inputs.json), [input index](input-index.json).

Build the locked [native workspace](native/Cargo.toml) at the source revision with default, legacy-halftrace and incumbent features into a fresh Cargo target directory; save each executable before the next feature build. Controls import exact previous-best field/context/word/linearized snapshots. The default imports the production modules. [Build recipe](checks/build-isolated.sh), [final preflight](candidate3/checks/run-preflight.sh), [primary](candidate3/checks/run-primary.sh). Use new output directories, benchmark records and a newly frozen protocol for reproduction; do not overwrite original data or reset original epochs.

Rust1.100.0, Callgrind3.24.0, Intel Xeon Platinum8573C, CPU0,2GiB child address space.600seconds per candidate calibration/pilot/primary envelope, zero paid resources, zero infrastructure retries. Original third epoch1795491725 was preserved through both launch corrections; all120 primary rows completed2026-11-24T03:51:45Z. No native wall/IPS result. [pcc:unverified]

Complete archives are local at the study paths named by pending-artifacts.json, with whole hashes in the matching .sha256 files. Once original-policy upload/readback succeeds, publish generated store manifests and hydrate with the maintained artifact tool. Extract RAW-ALL.tar.xz and FROZEN-EXECUTABLES.tar.xz in separate directories; archive-members.json names every original path/hash/length. Packaging independently extracted and compared all members. Local custody does not satisfy remote storage or independent admission. [pcc:unverified]

## Retained errors and gaps

Two candidate3 launcher failures happened before any measurement child: a stale candidate2 path, then an extra absolute-path prefix. Original erroneous bytes, runners and invalid records remain in launch-failure and launch-failure-v2; prior metadata was restored byte-for-byte from Git. Corrected runner derives its own directory, checks executable paths and refuses overwrites. The original600second epoch was not reset. Export check initially omitted its mandatory --protocol option; corrected checks pass without changing scientific data. Report rendering first used a Typst-only interpreter missing matplotlib, then used the existing plotting interpreter. An initial PCC baseline typo20,779,447 is corrected by PCC-20261124-807b8e to the actual20,779,444; original claim is retained, never used as the baseline. [pcc:unverified]

Sources checked (KB): wrapper unavailable127, no loaded MCP. Reviewed actual production/profile stacks, standard shared lazy initialization, carryless Karatsuba/sparse-tail reduction, linear-map lookups and simultaneous inversion; no novelty claim. Conductor unavailable; scope stays in this PR. Native cryptographic/harness/replay code is Rust; Python only metadata/custody/layout.

Root cargo test --release --lib retains prior600second dependency-compilation timeouts before tests; no full-root pass. Original artifact-store authenticated HeadObject403 and missing historical leaderboard native_statistics.json remain recorded; no identical denied retry or weakened admission gate. Focused actual-source tests and producer replay do not substitute signed independent validation. Earlier50x objective remains open. [pcc:unverified]
