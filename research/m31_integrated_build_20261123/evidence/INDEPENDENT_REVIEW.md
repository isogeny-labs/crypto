# Independent integration review — PR #5120

Review snapshot: 2026-11-24 03:28 UTC. Reviewer: delegated
`/root/integration_review`, separate from the three implementation owners.
Standing: unsigned development review, **[pcc:unverified]**. This is not an
independent signed project verdict or a performance qualification.

## Scope and conclusion

No remaining source correctness blocker was found in the reviewed versions.
The review concentrated on arithmetic domains, signed replay, completeness and
stop semantics, source identity, resource refusals, and truthful evidence scope.
It does not establish correctness for every permitted input.

Read scope, relative to the repository root:

- `AGENTS.md`, the latest relevant `docs/session-logs/2026-11-24.md` material,
  and the integrated task journal.
- All authored graph Rust modules, `Cargo.toml`, source pins, graph integration
  tests, and graph README; relevant original filter, witness, projected-rank,
  and curve-arithmetic contracts.
- Structured `lib.rs`, `main.rs`, unit tests, manifest and README; production
  F4 visitor/trace stop contracts and the reused prime-field/point operations.
- RR5 `extractor.rs` diff, complete `query_pipeline.rs`, `streaming_tests.rs`,
  existing `relation_adapter.rs`, native module registration, and the F4 source
  build-script pin/inventory diff.
- Integrated/RR5 scope documentation and the producer test receipts listed below.

Graph relation arithmetic uses ell=2147446831 while curve coordinates use
p=2147483647. The fixed graph identity is the complete existing PRIME31 tuple,
not just its coordinate prime. The structured component performs coordinate
solving on the supplied nonsingular M31 curve and does not substitute a subgroup
order. RR5 public-query labels and adapter elimination use the admitted curve's
order, with literal relation and recovered-scalar point replay.

## Findings corrected during review

| Finding | Correction observed in source |
| --- | --- |
| Structured visitor initially reversed the native boolean contract, stopping after a retained root and continuing at the cap. | False now continues after storing a solution; true stops at the assignment cap. Multiple-solution and cap controls were added. |
| Optional selector preprocessing could consume the shared symbolic-product budget before the raw cancellation chart existed. | All raw charts are built first; failed optional preprocessing retains each original chart. A zero-remaining-product-budget fallback control covers all formulations. |
| Graph pivot output could be marked accepted after the original filter rejected fill. | Retained output is `partial-output`; atomic refusal remains `refused`. Neither is accepted as a complete comparison arm. |
| Prefix identity was described as a raw prefix digest although it hashes reserialized rows. | The field/documentation now identify a canonical-prefix digest separately from the complete raw input digest. Source and limit identities are checked across children. |
| Four individually accepted graph runs could be reported successful without comparing their outcomes. | The coordinator requires per-prefix/round rank and target-outcome equality and exact eager/lazy output hashes. |
| Publicly mutable tape dimensions could make `apply` accept a changed dimension and then panic on private indices. | Columns and statistics are private and exposed through immutable accessors. |
| RR5 rejected signed assignments overwrote specific cap/deadline statuses. | Only Complete/Empty is demoted to Inconclusive; other termination statuses survive with additional replay-failure context. |
| One-row RR5 fixtures did not test continuation to a second callback. | A disclosed B5 fixture supplies at least two signed rows; batch/stream equality and the assignment cap are tested. |

The structured owner also identified and corrected an exact-coefficient issue:
calling production `normalise` independently on certificate cofactors would make
each polynomial monic and break the cofactor identity. The reviewed conversion
sorts monomials while preserving coefficients. The parent corrected the RR5
empty-query control from G, which is representable by the disclosed signed
leaves, to 100G before the passing test run.

## Independent execution

This reviewer directly executed the already-built structured unit-test binary,
from `/workspace/scratch/5e8abb017b93/crypto-build`:

```sh
research/m31_integrated_build_20261123/structured/target/debug/build/m31-structured-f4/979c017d939e16e6/out/m31_structured_f4-979c017d939e16e6 --test-threads=1
```

Observed terminal result: **exit 0; 10 passed, 0 failed, 0 ignored**. These include
independent finite-base signed-set equality, an independent Sylvester determinant,
cofactor/source/selector corruption, infinity and two-torsion cases, resource and
output stops, and exact fallback preservation. This was an independent execution
of producer-authored tests, not an independently rebuilt test suite.

| Object at independent execution | SHA-256 |
| --- | --- |
| Structured test binary | `0cbb03ba316f5668d808d379f6a7629c9a14006de61fa392a2ad6189b84987db` |
| Structured `lib.rs` | `40caca23c14e4424385e919ec11c37207e411eb82a3ed7217e9308678ca8f64e` |
| Structured `tests.rs` | `7e7ba9f7db695a6711075c11089026aa6040bcf995743aaf0785cafb55434e6e` |

## Producer evidence read, not independently rerun

- `graph/evidence/05-tests.{command.txt,exit,stdout}` records a release
  `graph_suite` run: exit 0, 7 tests passed, including 128 dense block-projection
  controls and four fresh CLI children. The before/after source manifests are
  byte-equal (`cmp` exit 0). `RECEIPT.json` and the retained CLI records describe
  planted group-valid controls, not natural collector data.
- `structured/evidence/release.all-targets.{exit,log}` records exit 0, 10 library
  tests and 3 CLI tests passed. The earlier debug and failed dependency/source
  hydration attempts remain distinguishable in their separate logs.
- `rr5/evidence/native-tests-initial.{exit,log}` records the parent's release
  library run: exit 0, 42 tests passed, including all ten new M31 streaming
  controls. The RR5 all-targets gate was still in progress at this snapshot.

Through this initial review snapshot, this reviewer ran no scientific or
performance experiment, natural capture, paid workload, external communication,
or commit. The handoff was then the reviewer's only authored repository file.
The separately authorized auxiliary addition below changes that authorship
scope without changing the solver arithmetic or existing tests.

## Residual limits and publication follow-through

- Graph cross-arm rank/target agreement is not a general proof that four
  arbitrary output row spaces are identical. Exact dense row-space comparison
  belongs to the bounded tape controls. Documentation should preserve that
  distinction. The matrix-free component remains an operator, not a completed
  Krylov solver or a target-recovery result.
- Graph complete-cold-cost fields remain null. Group admission establishes the
  supplied equations, not collector provenance or collection cost. Process
  limits are applied by the coordinator; direct worker/library calls need their
  own external process envelope where required.
- Structured completeness is relative to all declared charts, unchanged leaf
  selectors, rational signed lifting, and successful bounded traversal. Caps,
  rejected fallback branches and unfinished charts remain incomplete.
- RR5 exhaustion refers to its supported distinct-four chart. Identity and
  target-in-base charts remain unsupported. A successful consumer stop is not
  exhaustive chart completion. The public-query wrapper uses the cofactor
  presentation; the hybrid streaming entry point is separately exposed.
- RR5 deadlines are cooperative. Arbitrary caller callbacks cannot be preempted
  by the library, and setup/query-point arithmetic is outside the extractor's
  construction/root deadline. No complete accounting or speedup follows from
  the callback API.
- Source digests establish byte identities, not signed validation. The graph's
  relative source symlinks must remain Git mode 120000 when publishing. The
  parent acknowledged this requirement and will preserve the native tree modes.
- The initial documentation follow-ups were subsequently corrected: the
  integrated README distinguishes cross-arm outcomes from dense tape row-space
  controls, and the RR5 README describes implemented APIs and terminal evidence.
  RR5 unsupported-chart terminology concerns the query point R; the pipeline's
  public target T=identity is already recovered as scalar zero.

## Authorized follow-up — 2026-11-24 03:43 UTC

The parent requested the repository-required algebraic identity record after
the read-only review. This reviewer added only the new structured
`identity_certificate.rs` binary, its Cargo target, auxiliary documentation,
and the corresponding evidence. Existing production arithmetic and tests were
not edited. Structured `lib.rs` and `tests.rs` still have the hashes above.

Certificate **`IDC1hd6568d78855aef4c`** compares
`(af-cd)^2-(ae-bd)(bf-ce)` with the independently constructed full 24-term
Leibniz determinant of the quadratic 4x4 Sylvester matrix. It uses the unchanged
`crypto_f4::cryptanalysis::identity_certificate` module. The serialized record
was rechecked with 32 certificate points and 16 domain-separated extra points.
Both issuance and checker evaluation rejected the corrupted closed formula
plus one at point zero; the checker control recomputes the statement ID so it
tests value rejection rather than only a stale-ID refusal.

The format is fixed to **M61, 2^61-1**. This is a generic auxiliary
formal-variable algebraic record, not an elliptic-curve fixture, not a
replacement for M31 tests, and not a formal proof. `proof` is null. The
issuance/recheck are producer evidence for code authored by this reviewer;
they are not represented as an independent signed validation.

A separate read-only helper, `/root/integration_review/certificate_review`,
reviewed the new binary against the existing checker API and certificate
documentation. It found no material issue in the matrix, permutation parity,
degree bound, serialized replay or value-based corruption control. It performed
no execution or edits; this narrow static review also remains unsigned.

| Follow-up artifact or gate | Observed result |
| --- | --- |
| `structured/evidence/quadratic-resultant.identity.json` | 4,697-byte certificate; SHA-256 `fb1c8a1de505b63687c00199ae58670fe3670ef179a26b1f6cdcc435ac077716` |
| `structured/evidence/quadratic-resultant.check.json` | Acceptance and both corruption counterexamples; SHA-256 `a33654000ff2d2c90ec18d62cdf5183f5d48937f0f28b34f4a2dbc268b60ceec` |
| `structured/evidence/identity-certificate.command.txt`, `.log`, `.exit` | One cached release invocation under an explicit 180-second cap; exit 0 |
| `structured/evidence/release.final.command.txt`, `.log`, `.exit` | This reviewer ran the requested final cached `--release --all-targets -j1 -- --test-threads=1` gate under an explicit 180-second cap; exit 0, 10 library and 3 CLI tests passed; both binary targets compiled |

The final all-targets gate does not run the certificate binary's `main`; the
separate certificate invocation above did. The earlier release gate's enclosing
timeout is not inferred retroactively from the new, explicitly recorded command.

Additional terminal evidence read after the initial snapshot:

- Graph `06-tests.exit` is 0 with 7 passing integration tests; before/after
  source manifests are byte-equal. The updated coordinator only parses bounded
  rows and identities before child launch; full group admission now runs solely
  in capped children. The producer retained four child refusals for a wrong
  identity control. These are producer receipts, not reviewer reruns.
- RR5 `native-all-targets.exit` is 0; its log reports 42 library and 8 public
  example tests passed. This closes the compatibility gate that was pending
  at the initial snapshot. It was not independently rerun by this reviewer.

No novelty, security result, asymptotic improvement, or end-to-end ECDLP cost
claim is made by this review.
