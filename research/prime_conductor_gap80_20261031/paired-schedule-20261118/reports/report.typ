#set page(paper: "a4", margin: 16mm, footer: context align(right)[#counter(page).display()])
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.5em)
#let s = json("../final-summary.json")
= Pairing endpoint order and seeds
Native ecbench planning · 18 November 2026 · draft PR 4723

The versioned `paired_curves_v1` mode groups same-order prime-field
workloads by target index, shares their algorithm seeds, and alternates
endpoint and arm order over measured rounds. The original workloads,
targets, solver implementations and accounting are retained.

*Observed qualification:* #s.focused_tests_passed focused tests pass.
The full panel retains #s.full_workloads workload identities and plans
#s.full_executions individual executions. A separate CLI doc checks
#s.cli_workloads workloads and #s.cli_executions executions; legacy output
is byte-identical to the preserved previous CLI. Four invalid admissions
reject. These are correctness/planning checks, with no performance session.
[pcc:unverified]

#image("endpoint-order.svg", width: 100%)

The panels show the actual source/image/coordinate-control order for
one target index in each field. The image is from the previously evaluated
degree-2 or degree-5 control. Neither is the original 80-bit-degree map.
Warm-up parity is separate from measured parity. Each measured group and
its arms reverse together on odd rounds; an even measured-round count
provides marginal order balance. [pcc:unverified]

== Exact seed contract
Canonical JSON carries domain
`ecbench.paired_curves_v1.algorithm_seed` and integer parts

  [measurement.seed, r, target_seed, target_index, seed_round].

The seed is the first eight SHA-256 bytes, big-endian. Warm-ups use a
separate `ecbench.paired_curves_v1.warmup_seed` domain. The existing
`algorithm_seed_round` option retains absolute indices by default;
the supplied packet explicitly selects `measured_index`.

A native OpenSSL fixture supplies exact reference vectors; the actual
Rust planner checks matching seeds on the transported/shared-scalar
workloads. All arms on the same subgroup-order/index group receive the
same RNG input. Coordinate hashing can still make their walks differ.

#pagebreak()
= Validation and remaining measurements
#table(columns: (1fr, 1.6fr), inset: 5pt, stroke: 0.4pt,
  [*Control*], [*Outcome*],
  [Focused actual-library target], [Eleven tests: six scheduling/admission/tiny-replay controls plus five unchanged arithmetic/workload controls],
  [Full two-field plan], [192 workloads; 32 target indices per model, six models, six measured rounds plus one warm-up, three arms],
  [CLI compatibility], [Legacy doc bytes identical; paired doc and full summary validated against native seed vectors],
  [Negative admissions], [Odd rounds, public target law, unpaired order and unsupported IC reject],
  [Source lint], [Root-library and focused-test checks with warnings denied],
)

The six models are source, small-degree image and coordinate control in
the existing 202-bit and 96-bit prime fields. The full native integration
test checks all planned events, scalar/seed pairing and preserved workload
identities. The codec doc uses two target indices per model; the separate
full plan retains all 32. All workloads remain unregistered. [pcc:unverified]

== Preserved versions and failures
The first successful focused build exposed an unused reexport only when
the old arithmetic tests were nested. A local wrapper attribute fixes that
without editing the original tests. Version 3 passes eleven tests and CLI
checks, but root lint rejected a manual parity expression. The corrected
version uses the standard integer method and has separate manifests and
fresh qualification outputs. Earlier planner/test sources, executables,
raw outcomes and PCC subjects remain preserved.

The root's separately recorded duplicate-test failure remains: this
focused public target does not compile the full legacy cfg(test) suite.
The exact dependency lock is retained, including its original parent
versions. The old CLI source is unchanged; it links the actual new library.

Producer standing remains unsigned/unverified, verdict `not_covered`.
The source/binary manifests and raw stdout bind exact versions; they are
not independent signed execution certificates. No source-oracle, unit-test
duration or build time is an ECDLP timing. [pcc:unverified]

== Original requirement status
The minimum 80-bit ordinary conductor crossing still needs an affordable
complete quotient, non-kernel images and endpoint conductor evidence.
Scalar timing integration, explicit endpoint-statistics joining,
registration, strongest supported wall reference, wide-field IC and live
cloud/image admission remain open. Scheduling establishes none of these
by itself. No cold setup is shared between targets.

The configured S3 store still blocks the earlier archive's verified
publication, so the PR remains draft. No benchmark or target job is active.
[pcc:unverified]

Sources: native ecbench spec/canonical/workload/runner contracts;
the original measurement protocol; prior typed transfer packets;
ecbench-extend and ecbench-measure workflows. README.md has commands,
version details and receipt paths. KB unavailable, wrapper exit 127.
