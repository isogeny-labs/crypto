# Trace-selected C1 double-large-prime comparison

[Implementation report](REPORT.pdf) and [pipeline status diagram](pipeline.svg).

Status: **partial implementation; no measurement run**. This follows
[PR #2686](https://github.com/isogeny-labs/crypto/pull/2686), source head
`76fc6b286358e9c51515cb3e03427d5ccc290e10`, independently of its unmerged branch.
The construction's native findings and Lean receipt remain upstream evidence,
not new results of this change. No end-to-end improvement is established.

## Requirement and evidence

| requirement | status | evidence / remaining requirement |
|---|---|---|
| native standard and trace-selected bases | implemented | `ecbench_large_prime/trace_line.rs`: kernel of Frobenius^d+1; bounded, deterministic u scan; rejects Tr(a)!=0 and exhaustion |
| matched four-summand, 2LP toy reference | implemented | `ic.trace_line_2lp`, exact bounded MITM; explicit nested small base and complete accepted-relation transcript |
| actual algebraic C1 trial oracle on both lines | **not implemented** | MITM is deliberately a correctness reference, not the Gaudry–Diem algebraic trial engine |
| matched targets, controls and rho | prepared | [PROTOCOL.md](PROTOCOL.md), prospective ecbench specs; same public target per independent workload |
| independently verify relations and scalars | implemented | `examples/trace_line_replay.rs`, general Vec-backed BinaryCurve arithmetic; mutation and truncation tests |
| complete priced setup, trials, combination, rank and LA | **partial** | group phases and existing native counters retained; unpriced native terms remain explicit; complete priced cost stays null |
| execution with L2, A/A and independent-host replay | **blocked** | no available Iso Lab connector, AWS identity/CLI, workflow-dispatch tool or S3 checkpoint token; see [problem record](../../problems/trace-c1-2lp-measurement.md) |
| raw records, protocol and executable hashes | prepared | ecbench seals actual sessions; no session has been fabricated; replay emits input and verifier SHA-256 |
| verified findings and PR | implementation only | [validation/README.md](validation/README.md) records actual development checks |

## What is implemented

`ic.trace_line_2lp` takes `line=standard|trace`, `d`, `small_dimension`,
`max_u`, `max_trials`, `max_states`. It requires N=4d<=62, uses exactly four
summands and up to two large columns, and refuses unsupported shapes. The
standard base is the actual embedded subfield, not the low d bits of the
polynomial basis. Both bases use the same RREF subfield basis; the candidate
multiplies it by the first valid u under the declared scan cap. The first
`small_dimension` binary basis vectors define the nested small base. This is a
fixed subset policy, not an optimisation of the 2LP split.

The base keeps both curve lifts, then applies the existing cofactor projection
and negation fold. Construction work has named counters, all labelled
`_uncharged` until a calibrated pricing implementation exists. The old
`ic.large_prime` defaults, method identity, sample limit and serialised fields
are preserved when an explicit basis is absent.

For explicit bases, all accepted raw decompositions and projected equations
are retained, with raw atom and projected column tables. The independent replay
checks each four-point sum against aG+bQ, each linear row against its right
side, and every recovered scalar with the general curve implementation.
It rejects missing records, malformed points, truncation and changed rows.
This is independent arithmetic on the same host; an independent-host ecbench
audit and external review are still owed.

## Interpretation boundary

The 16x statement in #2686 concerns a relation-count model. This reference
returns at most one admissible decomposition per trial, so its success
probability can saturate and **must not be tested against a blanket 16x
success-rate expectation**. Its setup and lookup work also differ with base
size. It neither assumes constant c_trial nor estimates algebraic c_trial.
No model delta is promoted to a measured pipeline improvement.

The C1 implementation must expose the changed x=u*v coefficients to the
algebraic decomposition solver, cross-check all its toy solutions against this
reference, and record its own native work. The existing
`gaudry_quartic_bin` engine is a translated-t-coordinate, torsion-invariant
lane, not an interchangeable implementation of x in u*F_q; substituting that
lane changes the experiment.

Before measurement, development testing exposed a degenerate fixture: the
max_cofactor=1024 constructor could select a tiny subgroup, and the planted
scalar 7 then made a zero target. The tests and prospective reference specs
now use max_cofactor=16. This corrects the fixture and keeps subgroup size
growing with the field; it is not a change chosen from confirmatory data.
The first failed development log is preserved under `validation/`.

No ECC2K-130 transfer is asserted: GF(2^131) has no proper intermediate
subfields over GF(2). The named c2pnb208w1 and c2pnb368w1 applicability remains
the upstream construction result, not a scalar-recovery measurement here.

## Reproduction and admission

Development checks, not benchmark evidence:

```sh
cargo test --release --lib trace_line
cargo test --release --lib
cargo test --release --example trace_line_replay
cargo build --release --bin ecbench --example trace_line_replay
ecbench plan --spec research/trace_c1_2lp_20261027/reference-d3.json
```

Before measurements: resolve the [blockers](../../problems/trace-c1-2lp-measurement.md),
pin the tested source, binary and image digests, validate and freeze
`protocol.json`, register the exact curve representations, and successfully
push the S3 resume checkpoint. Submit through `.agents/skills/isolab-eks`.
Do not run the local reference as a substitute for an authorised L2 experiment.
Retain failures, caps and interruptions. Audit every record, then replay:

```sh
cargo run --release --example trace_line_replay -- SESSION/records.jsonl > replay.json
sha256sum replay.json
```

Actual sessions belong in a new `sessions/` directory; exports and reports sit
beside sealed sessions. No session, export, replay certificate or validation-gate
receipt exists for the prospective comparison yet. No leaderboard or benchmark
row is changed because no performance result was produced. Canonical graphs
remain unchanged for the same reason.

## Sources checked before implementation

- #2686 README, PROTOCOL, review corrections and current head: trace line
  conditions, exceptional cases, native counts, and distinction from C1 costs.
- `src/cryptanalysis/ecbench_large_prime.rs`: exact bounded four-summand
  decomposition, cofactor projection, LP elimination and matrix rank.
- `src/cryptanalysis/gaudry_quartic_bin.rs`: torsion-invariant t-line engine;
  its coordinate and solver assumptions differ from this x-line experiment.
- `docs/ecbench/README.md`, AGENTS.md, report-evidence, controlled-experiment,
  ecbench-extend, ecbench-measure and isolab-eks: matching, accounting,
  checkpoint and replay requirements.
- Canonical experiment-protocol skill and worked example in
  `isogeny-labs/crypto-autoresearcher`: record shape and prospective freeze.
- Primary-source retrieval attempted: Diem, *On the discrete logarithm problem
  in elliptic curves* (2011), author PDF returned 404; Gaudry–Thomé–Thériault–Diem,
  *A double large prime variation for small genus hyperelliptic index calculus*,
  eprint 2004/153 PDF returned 403. The papers were inaccessible in this session;
  no novelty or theorem claim is inferred from snippets. Existing reviewed
  repository implementations supply the implementation contracts.
