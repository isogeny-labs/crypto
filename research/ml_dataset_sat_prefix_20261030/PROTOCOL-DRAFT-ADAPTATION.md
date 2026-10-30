# Historical draft adaptation — superseded population

This note preserves the earlier schema-only draft. Its 2-arm × 16-instance population was not compatible with the official export checker and is superseded by `cli-fixture/protocol_population.py`: two explicitly entered software workflows, one instance and one replicate, with all actual commands/cells in hashed subordinate ledgers. No old draft was frozen or executed. Command preflight observations do not by themselves supply the new derived workflow estimate. See `cli-fixture/README.md` for current readiness.

# Official SAT fixture protocol draft adaptation

Only this scratch directory was edited. No frozen SAT worktree source changed;
no solver, dataset generation, protocol freeze, or publication command ran.

`cli-fixture/prepare.py` now emits the actual `experiment.protocol/v1` schema.
The active draft is `cli-fixture/protocol.draft-02.json`. It binds source
`8bddd785ff0d44f50b6710649ed403401075f5cb`, copied plan/scripts, the 16 candidate
cells, at most 24 native pattern solves, four copied-input controls, and the
existing per-command/overall budgets. No final binary was supplied: its digest
and empirical preflight timing fields remain null. This is an intentionally
unready draft, not an executable preregistration.

Authoritative source: isogeny-labs/crypto-autoresearcher commit
`f755085ff951888dbe64682c210c9ef7d167b40c`, pinned locally under
`../protocol-tooling-f755085ff951`; its docs, schema, checker and worked example
were read. The official schema permits null preflight values in a draft. The
checker rejects a frozen record with null estimated_total_seconds. That gate is
preserved; budget ceilings are never presented as measurements.

The schema requires at least two arms. They describe actual planned behavior:
`native-sat` selects a qualifying original pair; `copied-source-controls` runs
four conditional corruption/censor exports on independent copies. The generic
sample_size.instances=16 is a ceiling, not an assertion that both arms run on
16 independent instances; exact unequal/conditional populations are stated in
the extension and power note. Alpha is an unused schema-required field for
exact predicates, not a significance or power claim. Positive detector unit
controls are explicit prerequisites; no absent natural fixture becomes a
scientific null. Performance, general UNSAT proof and whole-solve instruction
claims remain out of scope.

## Driver integration

- Replace every `protocol["fixture"]` access with
  `protocol["extensions"]["ml_dataset_sat_fixture"]` (five accesses in the
  copied original driver). Existing binding keys are unchanged.
- The copied original driver launches children with RAYON_NUM_THREADS=8;
  change to RAYON_NUM_THREADS=1 and OMP_NUM_THREADS=1 to match the declared
  instrument and preflight inputs, or explicitly amend all bindings together.
- Merge the other agent's driver/verify changes first, then rerun prepare against
  final source/binary/scripts. This scratch draft binds the copied versions and
  cannot stand in for the future integrated frozen protocol.
- Use the real checker after the real freeze and Git commit, retaining its
  complete file hash/body hash, actual command, pinned checker/schema and logs.
  `checker-receipt.json` here is deliberately a draft receipt and cannot satisfy
  driver's frozen-protocol admission.
- Record native candidates and conditional copied controls under their distinct
  arm IDs in the future unified export. No export adapter is implemented here.

## Missing runtime evidence

Optional `--runtime-preflight PATH` requires an actual receipt with schema
`ml-dataset.sat-fixture-runtime-preflight/v1`, matching source_commit and
binary_sha256, threads=1, excluded_from_candidate_selection=true, nonempty
measured_on and estimate_method, and finite nonnegative fixed_seconds,
per_sample_seconds and estimated_total_seconds. `arm_measurements` must contain
one observation for each of `native-sat` and `copied-source-controls`, each with:

- arm, observed=true, command argv, measured elapsed_seconds;
- receipt_path (absolute or relative to preflight receipt) and receipt_sha256;
- the retained command JSON at that path must contain matching argv/command and
  elapsed_seconds_diagnostic/elapsed_seconds; its actual bytes are hashed.

Acquire these observations through a separately authorized, bounded preflight
on inputs excluded from the fixed candidate selection. Record exact setup/call
boundaries and derive the estimate transparently, including auxiliary CLI work;
never fill timing fields with the 1800-second limit or hypothetical zeros. This
adaptation neither schedules nor performs that preflight. An operator-provided
receipt remains producer evidence, not an independent timer certification.

## Validation

The actual pinned checker validated draft02 with zero findings. Nine scratch
unit controls check the official schema and semantic linter, null preservation,
refusal of an in-memory frozen candidate missing timings, rejection of the old
root-level fixture key, paired candidate/control bounds, deterministic canonical
roundtrip and immutable write refusal, duplicate/nonfinite JSON, invalid digest
inputs, and missing/mismatched preflight arms. No schema or checker stub is used.

The first unit invocation used the schema filename instead of the checker's
schema identifier and raised two KeyErrors; corrected to experiment.protocol/v1.
The real checker CLI had already passed. Retained test runs02/03 pass all nine;
this was a test-call error, not a hidden protocol-validation failure.
