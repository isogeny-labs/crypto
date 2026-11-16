# Trace translation and implicit incidence: historical evidence import

This directory preserves the implementation and recorded evidence from the
existing workspace study, and adds a native adapter for a future matched rho
comparison. It does not record a new benchmark run. Historical timings and
self-checks are unsigned producer evidence. [pcc:unverified]

## What is here

- `historical/fast_trace/`: the trace kernel, native recurrence helper,
  paper-style dense matrix control, complete-call records and source hashes.
- `historical/trace_audit/`: separate point-set and algebra checks.
- `historical/incidence/`: Horner, blocked and modular-composition variants,
  the original timed source, the uninformative-first recovery correction and
  its regression, records, dimensions and retry accounting.
- `historical/repo_controls/`: pinned source assessments of both repositories,
  quotient/GCD and streamed batched-pair controls and support/memory audits.
- `historical/prime_fixtures/`: public synthetic inputs and separately named
  test-only `*.validation.json` truth files. The latter are not solver inputs.
- `historical/pool_design/`: public mixed-label construction and separate
  structured-base controls. No free labels for a PKM/isogeny base are assumed.
- `figures/`: original descriptive figures, preserved separately from any
  admitted benchmark dashboard.
- `check-native.sh`: a native C++ correctness check; it extracts no timing result.
- `../../examples/trace_incidence_rho.rs`: adapter for the existing Montgomery,
  batched, negation-aware `prime_fast::rho_parallel` engine.

`IMPORT.json` identifies preserved and deferred objects. `SHA256SUMS` checks
the materialized import and new native adapter from the repository root.
The oversized original `repo_controls/pair_benchmark.json` is **not in this
PR**: its exact digest and length are retained pending the approved S3
artifact route. Its derived CSV/summary and frozen producer sources are here;
do not mistake those for custody of the entire original raw object.

The Python sources retain their original bytes, absolute workspace paths,
versions and provenance as historical snapshots under AGENTS.md's preservation
rule. They are not a new Python execution path. Replace the relevant path
with native code before extending it or making further performance comparisons.
CSV CRLF endings are preserved; the local whitespace check explicitly treats
CR as an end-of-line byte instead of rewriting frozen evidence.

## Recorded component scope

The degree-256 trace records report about 10x dense/trace CPU ratio on generic
support, about 4x on collision-heavy support and about 8x with exceptional
points. Complete calls include lifting and exceptional handling; constructing
the input divisor is outside both arms. Trace loses at small degrees and
retains a quadratic recurrence. The paper's section 8.1 already proposes
trace power projection: no priority or improved arithmetic exponent is claimed.
[pcc:unverified]

The natural incidence panel uses B=8,16,32,64,128 with r approximately 8B^3,
three curves and four seeds per size. It retains 28 hits and 32 misses.
At B=128, the saved medians are 0.722 ms for native pairs, 404 ms for the
quotient target scan and 353 ms for blocked incidence; generation plus oracle
medians are 52.7, 456 and 408 ms respectively. These are initialized-runtime
complete calls, with process startup retained separately. CPU affinity on a
shared host is not L2 isolation. [pcc:unverified]

The separate saved checks report 1,444 complete translation checks, equality
of all 180 incidence GCD supports with independent affine enumeration, and
additional Jacobian witness checks. These remain unsigned assertions, not
PCC standing or an independent-host timing replay. [pcc:unverified]

## The requested rho evaluation

`rho-protocol.json` records the pending comparison and its admission conditions.
The native adapter consumes the same public P,Q and order, never validation
secrets. It emits censored outcomes and checks each returned scalar on the
original curve. Local Cargo was unavailable, so its Rust tests remain pending.

Crucially, the current implicit-incidence implementation does not call the
trace translation kernel. A trace/dense component ratio cannot be applied to
that solver's total runtime. An integrated native candidate and matched
complete runs are needed to measure movement toward parity. [pcc:unverified]

No EKS build or timing run was submitted. The latest session-log readback
reports monthly spend above its limit; the remaining live budget is unknown
in this runtime. The isolab-eks skill and COST_CONTROLS.md forbid a new launch
under those conditions. Checkpoint upload and artifact-store credentials are
also absent here. No hosted-CI or local timing substitute was used.

## Development checks

```sh
bash research/trace_incidence_20261116/check-native.sh
cargo test --release --example trace_incidence_rho
```

The C++ development check completed all 90 fixture cases locally. The Cargo
command could not start because Cargo is absent. [pcc:unverified]

After a source-exact build and admission, an adapter invocation has the form:

```sh
TRACE_INCIDENCE_SOURCE_REVISION=$(git rev-parse HEAD) \
  target/release/examples/trace_incidence_rho \
  research/trace_incidence_20261116/historical/prime_fixtures/b008_c0_natural_s0.json \
  20261116 4 8 0 100000
```

That command is illustrative; no new performance result is supplied. It is
not a replacement for L2, A/A, the strongest registered reference, full phase
accounting, source/image/binary manifests or independent replay.

## Sources and publication status

- Supplied Spaenlehauer paper, *Deterministic Root-Finding over Finite Fields
  with Elliptic Curves*, arXiv:2610.32508v1, especially section 8.1. PDF SHA-256:
  `bf01081e0261633421a00577dbf35cf84864161e3758ad3c9abdc55327cc963c`.
- `historical/repo_controls/source_provenance.json` and
  `repository_assessment.json`: exact inspected crypto and cryptanalysis
  commits and source blobs. The original upstream Rust binary was not replayed
  in the historical timings.
- `src/cryptanalysis/prime_fast.rs`: current native rho reference. The adapter
  reuses this source instead of inventing a slower reference.
- Sources checked (KB unavailable): no KB MCP connection; the repository
  wrapper exits127 with no executable. Bounded primary-source fallback above.

There is no new leaderboard, benchmark-index or canonical dashboard promotion.
Those graphs are unchanged because this imports unsigned historical evidence,
with raw-object custody and native matched runs still pending. No claim to
be the fastest external implementation is made; mature external comparisons,
the original paper's implementation and an admitted native replay remain open.
