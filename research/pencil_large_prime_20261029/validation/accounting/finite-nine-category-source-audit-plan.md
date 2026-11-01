# Finite nine-category source coverage audit plan

Status: plan only. This document is not a coverage certificate, protocol approval,
binary binding, experiment result, or cost-ratio claim.

## Audit boundary

The audit applies the finite semantic model in `COST_BOUNDARY.md` and
`kernel-primitive-registry.json` to the candidate pencil collector and matched rho
reference. It asks whether every source operation selected by that model is observed
once in the correct domain and phase. It does not attempt a CPU-instruction census,
allocator-internal model, universal formal proof, or valuation of reporting, harness,
wall-time, RSS, checkpoint, retry, or cloud work as field operations. Those resources
remain separate measurements.

The independent reviewer must use one immutable source manifest. A final approval is
inadmissible until an approved frozen protocol identifies the model and an exact
instrument binary is built from the same source and bound by SHA-256. Current source
and software-test evidence may support review preparation but cannot produce
`reviewed_complete` by itself.

## Required inputs

1. Approved frozen protocol path and SHA-256, including the finite pricing policy,
   calibration treatment, failure/cap rules, A/A controls, and record schema.
2. Exact candidate and reference instrument binary SHA-256 values, build receipt,
   Rust 1.99.1 identity, source commit, dirty-source manifest, and reproducible build
   command.
3. Complete source manifest covering all study sources, root adapters, split engine
   accounting modules, Cargo manifests, and lock file, including untracked sources.
4. Exact primitive registry and analyzer mapping SHA-256 values.
5. Raw targeted tests, strict Clippy, final root release-library test, and unchanged
   end-of-run source hashes.
6. Reviewer identity distinct from the producer, with a write-once review destination.

## Category procedure

For every category, inventory the actual candidate and reference call graph, record
file hash and precise line bounds for each selected primitive, trace short-circuit and
failure paths, and compare the event against the registry. Mark every source site as
`covered`, `uncovered`, `diagnostic_only`, `separate_resource`, or `not_reached`.
Diagnostics must never be added to the primitive they summarize. A category is covered
only when all selected reachable sites have a disposition and the relevant exact tests
exercise success, failure, empty, and cap paths.

| Category | Source questions and required evidence |
|---|---|
| `field_arithmetic` | Inventory every candidate and reference base-field add, multiply, square, inverse, standalone reduction, and selected polynomial collection predicate. Confirm setup and source verification are included once, inverse expansion is not charged twice, raw modular operations are not omitted, and M/S/I normalization uses the matched source-addition unit. |
| `scalar_arithmetic` | Trace candidate scalar helpers and reference `Ring` operations, including add/neg/mul/inv and checked inverse-expansion subtraction. Verify wide-modulus behavior, exact atomic counts, and that scalar constructor M/S/reductions use native scalar weight rather than base-field weights. |
| `outer_and_constructor_validation` | Trace actual p-domain and r-domain primality calls, their original short-circuit order, rejected prefixes, and constructor invocations. Confirm one event per actual call, separate base/scalar domains, product reductions diagnostic only, and no second check introduced by instrumentation. |
| `setup_transfer` | Trace cold curve construction, rational-point inventory, projection checks, transfer through tau, solver construction, and candidate/reference initialization. Confirm phase attribution and that setup field, arithmetic, and semantic snapshots enter totals exactly once. |
| `collection_graph_elimination` | Inventory every selected vector/map/set entry visit, explicit retained-element copy, fresh initialization, key comparison, length/emptiness predicate, graph transition, residual update, and cap branch. Verify iterator visits are not duplicated at consumers, owned moves remain excluded, tuple units are consistent, and diagnostics such as row updates, payload terms, and parent hops are not re-priced. |
| `hash_rng` | Trace candidate walk-label invocations, reference table hashing, collision lookups, random draws, and restart paths. Confirm atomic hash/RNG events follow the declared model, pinned hasher/toolchain assumptions are bound, and internal hash expansion is not inferred. |
| `source_verification` | Trace solver-side recovered-scalar verification, adapter additional verification, and any parent audit. Confirm each intended check is counted once at its own boundary, failed checks retain performed work, and parent or harness audits are not silently folded into method arithmetic. |
| `metadata_clone` | Trace `StudyInstance::fresh` and other algorithmic metadata clones. Count selected serde `Value` nodes once; retain string/key/family bytes as supplemental observations; exclude observer traversal and fixed whole-record `Copy` moves. |
| `calibration` | Verify preparation plus source addition equals calibration total, the addition equals the matched normalization unit, geometry is partitioned without adding total and segments together, and reports expose both method-only and explicitly calibration-inclusive results. |

## Cross-category checks

- Recompute every checked sum, subtraction, and phase partition from raw counters.
- Reject underflow, overflow, missing required fields, duplicated categories, unknown
  primitive names, mismatched source hashes, or line ranges outside bound files.
- Compare candidate and reference semantics before pricing; shared event names must mean
  the same unit in both arms.
- Exercise empty, singleton, duplicate, replacement, miss, success, rejection, working
  cap, retained cap, exhausted, and solved paths where reachable.
- Confirm missing observations stay null or block pricing; they are never zero-filled.
- Confirm output/report serialization, allocator data, RSS, clocks, harness operations,
  retries, and infrastructure remain retained in their native units outside the finite
  algorithm price.

## Review output and decision rule

The independent output must be a write-once structured review containing the protocol,
binary, mapping, registry, and source identities; reviewer and producer identities;
per-category source sites and dispositions; raw test/log hashes; blockers; and explicit
nonclaims. A nonce-sampled record replay and headline recomputation are separate run
review steps after records exist.

Emit `reviewed_complete` only when all nine categories are covered, no uncovered site
remains, source hashes match the exact binary build, the approved protocol and mapping
hashes match, and the reviewer is independent. Otherwise emit a finding report with
`reviewed_complete` absent. Passing source tests or closing the three findings in
`finite-coverage-audit.md` is necessary software evidence but is not sufficient for the
certificate.
