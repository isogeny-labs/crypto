# Development checks for the refined oracle

2026-11-24. Local evidence; [pcc:unverified].

- Isolated crate: six release tests passed. The recorded
  [test output](evidence/refinement_v2/unit_tests.txt) includes full tuple-partition
  checks, an expired deadline and failure after a completed branch.
- Five version-1 invocations and two version-2 invocations exited zero; all
  intended query and recovery records are retained in their versioned directories.
- Root library: `cargo +1.99.1 test --release --lib --locked`, two build jobs,
  exited 124 under an initial 180-second and a subsequent 600-second timeout.
  Both stopped while compiling dependencies, before any library-test result.
  [First log](evidence/refinement_v2/root_build_180s.txt) ·
  [Second log](evidence/refinement_v2/root_build_600s.txt).
- Benchmark-record gate: after materializing the tracked records and progress
  files, `python3 tools/benchrec.py gate` stops on
  `benchmarks/records/20261105T0828Z-icb-large-prime-n53-s1-archive.json`:
  command 1 lacks a successful integer exit code and needs historical handling.
  The recorder, all record files and progress files are unchanged from
  `origin/main` at `a6223dac`. The flagged record has Git blob
  `49346ce39eb6ec7d77f3c507eac89ac73cce3bd3` in both trees. No benchmark metadata
  is changed by this lane.
- PCC statements have unverified standing. The ledger has 1,862 entries and
  its hash chain and `origin/main` prefix pass the local check. Report and
  protocol prose lint passes.
- The three-page report PDF was rendered and visually inspected, including
  the graph, tables, source links and command lines. Its initial build used
  the wrong Pandoc math-reader setting; enabling `tex_math_single_backslash`
  corrected it without changing the mathematical text.

The complete root library suite, admitted measurement and independent replay
are not represented as passed. The public PR reports those remaining checks.

## Production comparator continuation

The [new report](PRODUCTION_RESULT.md) and [reproduction instructions](PRODUCTION_REPRODUCE.md)
retain the optional source integration and all thirteen full-base resource
refusals. The final adapter suite passes ten tests; the production closure's
32 library tests, eight public-example tests, and six unchanged default-feature
tests also pass locally. A separate planted four-leaf unit control exercises
successful native output and adapter conversion. [pcc:unverified]

The frozen panel source is 8be878c873; the later source adds that unit test
without changing the panel executable. Evidence and before-call identities
live in `evidence/production_v1`. The earlier root compilation limitation and
unchanged historical benchmark-record gate error remain open. No broad root
test rerun or cost measurement is represented as successful.

## Same-equation external output continuation

The [new output report](COFACTOR_OUTPUT_RESULT.md) retains the initial export
metadata test failure, two full-set mismatches, their source revisions, and
the subsequently frozen RUR guard and fixed-form protocol. Final feature
tests pass 15/15 and default tests pass 6/6; the original library's 32 tests
and 8 public-example tests passed after the export change. All are local,
unsigned evidence. [pcc:unverified]

The frozen source is `3176dae78f`. Three new panels produce 22 records: eight
complete cofactor calls, five cofactor timeouts, and nine complete Refined
regressions. No full-base answer is inferred from a narrower regular chart.
Every accepted cofactor RUR is retained. [pcc:unverified]

The exact source and protocol, raw fixtures, test/build logs and complete
records are under `evidence/separating_v1`; earlier failures remain under
`evidence/cofactor_msolve_v1` and `evidence/cofactor_diagnostic_v1`.
The new manifest is `evidence/COFACTOR_OUTPUT_SHA256SUMS`. Raw solver whitespace
and full test logs are preserved, including Git whitespace warnings on those
data files. The root-library build limitation and unchanged historical
benchmark-record gate failure above are still open. No EKS job was submitted.

## Four-leaf oriented-chain continuation

The [oriented report](ORIENTED_RESULT.md) retains a new composition of the
existing pair identity, using the unchanged production F4/root visitor.
Source `21a7ef9221` passes eighteen feature tests and six default tests, with
the initial converter compilation failure preserved. Both membership models
accept/replay 312 ordered witnesses; a separate fixed-slot native integration
control completes. [pcc:unverified]

All 26 fixed full-base calls are incomplete at matrix admission. Dense asks
for 2,006,513 entries and tower for 2,001,665, against 2,000,000, on every
target. No assignment or forbidden domain evaluation occurs. The tests do
not replace these failed full-base controls. [pcc:unverified]

`evidence/oriented_v1` contains the source/binary identity, complete records,
test/build receipts and original production inventory. The separate manifest
is `evidence/ORIENTED_SHA256SUMS`. The production engine, original comparator
and root source have not changed in this continuation; previous full-library
and historical benchmark-record limitations remain open.

## Cold-accounting continuation

The [new report](COLD_ACCOUNTING_RESULT.md) binds the actual production consumer
to every-draw child records and a full process interval. Frozen source a4cd5ee0b3
passes 23 feature tests and six default tests; its release build succeeds.
Three F127 arms recover scalar 69 with reconciled process receipts: Direct
9 queries, Refined 10, cofactor 12. Five cofactor queries are unsupported because
the target is in the base. Nineteen Direct/Refined histories match the prior
recorded control. Local and unsigned only. [pcc:unverified]

The 22:54 CI inspection of 7d4ecdff41 found one real historical source-pin
mismatch in PCC-20261124-f6947c. The [layout note](native/SOURCE_LAYOUT.md)
describes the repair: restore exact historical bytes at the pinned path and
compile the current byte-identical implementation from production_current.rs.
Claims, ledger history and gate logic are unchanged. Post-repair test and PCC
receipts are retained separately; no control was rerun.

The canonical experiment-export schema/tooling is unreadable through the
available GitHub retrieval paths; development_flow.json is explicitly a local
receipt, not a schema-validated canonical export. Root-library compilation,
historical benchmark-record gate and registered EKS workload dependencies remain
as documented above. No admitted measurement or cloud submission occurred.

After the source-layout repair, all 23 feature tests pass again and all 26
branch-added claims return not_covered with no local verifier failures. These
are not signed verification. Claim PCC-20261124-5d70b3 binds the byte-equality
and test receipts. [pcc:unverified]

The frozen COLD_ACCOUNTING.md protocol is unchanged from its pre-run hash.
The prose linter flags its negated phrase "not a faster relation method" at
line 20, which asserts no advantage. This historical annotation finding is
retained rather than changing the frozen protocol after the run. The current
result/reproduction reports and PR body are linted separately.

## Common signed-base lookup continuation

Frozen source 6a0c732e8e passes 27 feature tests and six default tests, and the
release build succeeds. The [new report](BASE_LOOKUP_RESULT.md) records two
three-arm F127 suites: all six recover with valid process receipts. Original
Direct/Refined/cofactor use 9/10/12 four-leaf calls and match all 31 earlier
mask/row/consumer histories. The shared lookup uses 8/7/8 draws, 4/3/4 singleton
hits and four four-leaf calls per arm. It introduces no new map or admitted cost
comparison. Claims c54b6a, 1a672f and 98f805 remain unverified. [pcc:unverified]

All five original target-in-base failures are retained. The lookup accepts a
different one-term relation; it does not implement that missing four-leaf chart.
Misses still call the old oracle. Every original oracle answer still contributes
only its first sorted relation, so full-output consumption is the next common
policy to qualify before a strongest-reference claim. The prior historical
source snapshot, protocols, controls and manifests are unchanged. No cloud job
or canonical export validation is represented as completed.
