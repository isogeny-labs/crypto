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

## Common full-output continuation

Source 77c465a6d1 passes 30 feature tests and six default tests and builds.
The initial feature run's one false singleton-arrival assertion and exact
source/log are retained; the correction changes only that test expectation.
Claims PCC-20261124-63f792 and b2543e remain unverified. [pcc:unverified]

The [new control](FULL_OUTPUT_RESULT.md) records four suites and twelve valid
recoveries. Both first-row histories match all 54 earlier attempt records.
Full-output Direct/Refined/cofactor use 2/7/5 calls without lookup and 1/4/1
with lookup. Every submitted row is charged and its receipt retained, including
dependent rows. Unused suffixes are retained after recovery, with their prior
generation already charged. No timing ratio or new map is inferred. [pcc:unverified]

Next qualify the larger workload with this strengthened Direct reference, then
registered execution and admitted costs. Existing complete-chart/output and
canonical-export access limitations remain open. At the 23:40 UTC check-in,
the source identity head da258b6de6 had gate jobs in progress and several draft
jobs skipped; that is not a current-head CI pass.

## Larger controls and separate whole-answer replay

Auditor source 15ff500915 passes 33 feature tests and six default tests and
builds; the recovery executable remains byte-equal to source 77c465a6d1's
frozen binary. Six mutations include a lost unused row with reconciled counts,
which the exhaustive set comparison rejects. Both historical F127 audits pass
before either new suite is run. Claims 2b5ba8 and 4c549b remain unsigned.
[pcc:unverified]

The [larger report](LARGER_FULL_OUTPUT_RESULT.md) retains six F8191 endpoints:
only degree-32 Direct recovers (eight calls, 15 rows, scalar 4356). Degree-16
ranks are 1/0/1 after 30 complete queries each; degree-32 Refined has rank nine
after 30 complete queries and cofactor exhausts all 30 query budgets. Every
reported complete set matches its declared reference and all consumed receipts
replay. Successful replay/process exit does not convert an incomplete endpoint
into recovery. [pcc:unverified]

The auditor shares fixture/reference/consumer source; it is separate post-run
execution, not independently implemented or signed validation. Its work lies
outside the recorded recovery intervals. No admitted cost ratio or new cloud
job is represented. Next investigate full-coverage cutoff partitions and their
total branch work, with the registered payload/admission still open.

### Artifact migration pending

The final artifact-policy check flags the two larger raw JSONL files
(`f8191_d4.jsonl`, 157,366 bytes; `f8191_d5.jsonl`, 410,585 bytes) for the
repository store. Local `artifacts.py push` cannot start because the AWS CLI is
absent; it performs no upload and writes no manifest. The repository-wide
`status` command also stops on an unrelated omitted historical manifest's
Git-filter read. No shared policy, gate or data hash is changed.

The documented same-repository `artifact-autofix` workflow is the alternate
publication route: it runs the trusted base tool, uploads changed data, replays
the remote bytes and commits manifests/removals. Keep the PR draft pending that
migration; do not represent remote storage or the artifact gate as passed until
the actual result is read back. This does not change any scientific outcome.

The actual [migration run 45515847117](https://github.com/isogeny-labs/crypto/actions/runs/45515847117)
at head 623ccfa87c failed in job 138874939285's upload/replay step. Both
content-addressed HeadObject requests returned HTTP 403; the manifest/removal
commit step was skipped. This response cannot distinguish denied access from
an absent key without ListBucket permission. No successful upload, remote byte
replay or object absence is asserted. The supported local and workflow routes
have both been tried; store access or an approved replacement is an external
dependency. No manual retry, exception label or policy change was made.

## Complete cutoff catalog qualification

The [new report](FULL_CUTOFF_RESULT.md) retains a target-independent multiset
catalog and three complete finite witness audits. Source 16f5902e2d passes
36 feature tests and nine default tests; both executables build. The frozen
five-command protocol exits zero throughout, with empty stderr and unchanged
before-call hashes. The certificate is evaluated by the existing native
checker; its formal proof field is empty. Claims 3a7978, f9071f, 941283 and
f70c64 remain unsigned. [pcc:unverified]

The catalogs have 36/18/119 charts and partition 70/35/1001 unsigned tuples.
The audit checks all 17,606 admissible signed bindings and records 90 explicit
identity/target-abscissa exclusions. Rank-two bindings occur in two-image
charts; the complete branch count and singularity handling remain costs for
the next extractor comparison. No new solver, recovery, admitted timing or
EKS run is represented. [pcc:unverified]

Evidence is in `evidence/full_cutoff_v1`, with a separate before-call identity,
post-call unchanged-byte check, raw outputs, tests, certificate and exact
command. `evidence/FULL_CUTOFF_SHA256SUMS` covers these files. All new data
files fit the inline policy threshold; the two earlier larger-control files
still have the separately documented artifact-migration dependency.

At the subsequent normal CI read, identity head `5f69c1ea75` has four successful,
21 skipped and one failed check. The failed migration
[run45518137335](https://github.com/isogeny-labs/crypto/actions/runs/45518137335),
job138882992159, again returns HTTP403 on both older content-addressed
HeadObject requests before upload/replay. The final-head checks are not claimed
passed, and no absent-key or specific permission diagnosis is inferred.

## Full-catalog extractor controls

The [paired report](FULL_CUTOFF_EXTRACT_RESULT.md) records source42728dba83,
identityd181477253, 39 feature tests and12 default tests, a successful build,
and all26 planned query records from three exit-zero panels. FullCutoff
completes7/13 answers and the unchanged cofactor comparator8/13; accepted
whole sets match the full Direct reference. All six candidate failures and
five comparator failures remain explicit. Claims PCC-20261125-b0dd5b and
1df51d are unsigned. [pcc:unverified]

Evidence is in `evidence/full_cutoff_extract_v1`. Every pre-call hash stays
unchanged, and no required prefix/RUR capture is omitted. The shared runner
retains failed-child reasons but does not archive separate child stderr or
partial timeout files. No timing ratio, scalar recovery or EKS execution is
represented. Source-head CI has not been re-polled before its normal cadence.

Three more raw files are selected by the same artifact policy:
`full_cutoff_extract_v1/f127_d4.jsonl` (184,826 bytes),
`full_cutoff_extract_v1/f8191_d4.jsonl` (125,408 bytes), and
`full_cutoff_extract_v1/f8191_d5.jsonl` (314,431 bytes), all below `evidence/`.
Together with the prior two larger-control files, five raw files await actual
upload/replay. The earlier supported workflow's HTTP403 remains the last
observed store-access result; no upload of these new files is claimed.

## Factorwise finite witness and Jacobian controls

Source `3747e09eae`, before-call identity `d0e5cc767b`: 42 feature tests,
15 default tests and the release build pass. Three one-shot finite audits
exit zero with empty stderr; all 15 before-call files remain unchanged.
Every one of the 17,606 admissible signed bindings satisfies the new equations
with fiber rank four, RR5 rank three and total rank seven. All chart, exclusion
and old-rank counts match the prior catalog audit. Claims PCC-20261125-d46228,
de166a and 5bc001 remain unsigned. [pcc:unverified]

The [report](FACTORWISE_RESULT.md) distinguishes the local written argument,
actual-equation finite audit and outstanding formalization from solver output.
No algebraic solver, recovery, EKS job or timing comparison was run. The
auditor shares catalog/interpolation/rank source. New raw audit records are
7,328 / 4,057 / 24,213 bytes; the five earlier artifact-selected files remain
the publication dependency. Existing source controls were run once; no earlier
scientific panel was repeated.

At the next normal-cadence read, identity head `d0e5cc767b` has three
successful checks,21 skipped,one failed migration and one still in progress.
[Artifact run45524444152](https://github.com/isogeny-labs/crypto/actions/runs/45524444152),
job138904288926, reports HeadObject HTTP403 for all five pending content
hashes at01:16 UTC and exits2 before upload/replay. This is now observed for
both older larger-control files and the three full-cutoff-extraction files.
It establishes neither absent keys nor a particular permission diagnosis.
No manual retry was issued and final-result-head CI is not claimed passed.

## Factorwise complete-output controls

Source `d54d9d37bd`, before-call identity `8dd11c8e44`: 43 feature tests,
16 default tests and the release build pass. The new dispatch stub covers
factor-variable order, dimensions, every chart in an empty union and expired
budget. Three frozen paired panels run once and emit all 26 records, with
16 exact and 10 undetermined outcomes, no mismatch or capture omission.
All panels exit zero with empty stderr; all 16 pre-call hashes stay unchanged.
Claims PCC-20261125-1afa6b and 29f2a4 are unsigned. [pcc:unverified]

The [report](FACTORWISE_EXTRACT_RESULT.md) records 8/13 complete answers for
each arm. The factorwise constructor resolves the former F127 chart refusal;
all eight degree-16 answers match Direct. Both arms remain incomplete on
all five degree-32 targets. Factorwise finishes 22/24/24/24/23 of 119 charts
before timeout. No recovery, timing comparison or EKS execution is included.
Historical pushforward records and all acceptance guards remain unchanged.
[pcc:unverified]

Three more raw files require artifact migration: `factorwise_extract_v1/`
`f127_d4.jsonl` (149,177 bytes), `f8191_d4.jsonl` (79,879 bytes), and
`f8191_d5.jsonl` (171,132 bytes). Together with the five preceding files,
eight await required upload/replay. The last observed HTTP403 job above
concerns the earlier five; no successful upload or current-head CI pass is
claimed. The live hook has 682 queued records and no receiving URL configured.
[pcc:unverified]

## Liftable-fiber qualification and retained mismatch

Source `3b7cd7422d`, identity `c2d6a4c318`: 47 feature tests and20 default
tests pass, with a successful release build. The public-base grouping and
polynomial construction are inside each shared query deadline. Three finite
audits pass, retaining all17,606 full-rank bindings and exact prior per-chart
counts. All18 pre-call files remain unchanged. Claims PCC-20261125-6cb640,
8eaff2 and4219a4 are unsigned. [pcc:unverified]

The [report](LIFTABLE_FIBERS_RESULT.md) preserves24 of26 planned query
records:17 exact,6 undetermined,1 mismatch, plus2 unattempted queries.
LiftableCutoff completes11 of12 attempted queries, including four of five
degree32 targets. F127 cofactor returns an incorrect empty answer on target
(41,95), stopping that panel. Its prepared equations, prefix and form match
the previous successful record, but RUR bytes differ. No semantic diagnosis
or retry is represented. Other panels finish; no capture is omitted. The
next step is offline native RUR/equation validation. [pcc:unverified]

The new panel files have111,438 /79,847 /572,672 bytes. They join eight prior
files requiring artifact upload/replay. No new upload, admitted cost result or
current-head CI pass is claimed. The private stream now holds709 records with
no receiver configured.
[pcc:unverified]

At the next normal-cadence CI read, identity head `c2d6a4c318` has four
successful checks,21 skipped and one failed migration. Actual
[run45529687536](https://github.com/isogeny-labs/crypto/actions/runs/45529687536),
job138921738326, logs five HeadObject HTTP403 responses at02:00 UTC and
exits2 before upload/replay. The five logged hashes correspond to all three
factorwise-extraction files and the F127/F8191-depth4 full-cutoff-extraction
files. The job does not supply access outcomes for every pending file; it
also predates the three newest liftable panel files. No absent-key or specific
permission diagnosis is inferred. No manual retry was issued. [pcc:unverified]
