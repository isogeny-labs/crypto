# Curve25519 Autolab parity continuation

User objective: run Autolab until achieving parity with rho for Curve25519.
Status: active; no new scientific trial in this continuation yet.

## Endpoint

The target is the actual RFC 7748 Curve25519 prime-order subgroup, over
`p = 2^255 - 19`, Montgomery coefficient `A = 486662`, cofactor 8 and
`r = 2^252 + 0x14def9dea2f79cd65812631a5cf5d3ed`.
Use public synthetic inputs. The primary comparison is a cold single-target
complete solve, with identical public targets, matched resources and the
strongest applicable rho reference. Charge setup, failures, decomposition,
relation verification, linear algebra, descent and final verification.
Retain the repository's native-operation price band. Miniature measurements,
small-scalar windows, warmed factor-base logs and arithmetic throughput do
not discharge the full-size endpoint. No feasibility or completion date is
asserted here.

## Current source inspection

The authoritative campaign is `research/curve25519_autolab_20261104`,
implemented by `src/bin/ic_autolab.rs`. Its ledger ends at round 35.
Preserve all completed rounds, exclusions, exposures, seed corrections and
the generic/algebraic track separation. The campaign README labels its
reported results unverified pending independent replay and recomputation.
This continuation has inspected the source and retained descriptions, not
replayed those results or promoted their claims.

Two concrete admission limits prevent simply running that loop at 255 bits:

* `src/bin/ic_autolab.rs`, `SizeInfo`: subgroup order `r` is `u64`.
* `src/cryptanalysis/ecbench/workload.rs`, `CurveSpec::PrimeExplicit`:
  all field/group parameters are `u64`; its constructor requires
  `5 <= p < 2^62`.

`src/bin/curve25519_family.rs` provides a big-integer parameter audit and
miniature construction. Its audit is not a full-size IC solve. The existing
wide arithmetic and relation-elimination components must be traced through
an actual measured solver before claiming that widening a JSON field makes
the pipeline support Curve25519.

The reusable actual-curve implementation is already present:
`src/cryptanalysis/curve25519_wide.rs`, `Curve25519Wide`, uses `Fp256`,
BigUint scalars, an explicit Montgomery-to-Weierstrass transformation and
counted group operations. Its bounded `smoke` scan is not a complete IC
pipeline. The remaining shared interfaces are explicit in current source:
`ic_framework/stages.rs::InstanceCtx.r` is `u64`,
`ic_boundary.rs::CountedGroup::mul` accepts `u64`, and
`ecbench/workload.rs::Instance::r` returns `u128`. The existing ecbench
`Instance::Wide` is a binary Koblitz instance, not this prime-field backend.
Reuse `Curve25519Wide`; do not build a second Curve25519 arithmetic engine.

## Next action

Trace and implement the missing full-width path from the Autolab target
manifest through ecbench workload/target construction, solver dispatch,
relation coefficients modulo r, scalar recovery, counted costs and replay.
Start with the existing wide-field and wide-scalar implementations; identify
the narrowest reusable complete pipeline and its unsupported interfaces.
Preserve the existing miniature schema and frozen records. Do not launch a
full-size exhaustive search merely to demonstrate that parsing succeeds.
Use exact RFC parameters and known-answer group/relation tests for adapter
validation before a fresh, bounded comparative protocol is frozen.

After that path is executable, continue source-level candidate development
and fresh-target Autolab comparisons under finite budgets. Passing adapter
tests is infrastructure progress, not rho parity. A completion claim requires
the complete endpoint comparison and independent validation.

## Runtime state

No build, solver or cloud Job was launched by this continuation.
Earlier in this chat, direct Kubernetes readback failed because the worker
could not assume the configured IsoLab launcher role. The MCP overview's
configured caps do not establish live NodePool/quota enforcement. Reconcile
live jobs, authoritative cap readback, reservations and remaining daily and
monthly funds before any cloud submission; publish exact inputs and source
to GitHub and satisfy the checkpoint gate first. Do not repeat completed or
timed-out attempts under renamed registrations.

Conductor reported the new study directory clear. The shared
`docs/session-logs/2026-11-05.md` is held by task T-1590, so this scoped record
serves as the continuation log without writing into that task's reservation.

## Sources consulted

* RFC 7748 section 4.1, read on 2026-11-05:
  https://www.rfc-editor.org/rfc/rfc7748.html#section-4.1
* Current source paths above and the campaign README, ledger and
  `ideas/STRUCTURE-R4.md`.
* `research/curve25519_algebraic_ecbench_20261102/{README,PROTOCOL}.md`
  in the earlier checkout: historical miniature comparison and replay issue.
* `research/curve25519_rho_parity_bound_20261104/README.md`: a model argument,
  not treated here as a general impossibility theorem or a stopping rule.
* Petit, Kosters and Messeng, PKC 2016: primary PDF retrieval returned HTTP
  403. No new mathematical claim relies on the inaccessible paper.

## Session log, 2026-11-05

### What was done

Fetched main, created isolated branch
`codex/curve25519-autolab-parity-20261105`, reconciled the current campaign,
and identified full-size source admission limits. No raw run data changed.

### Decisions made

Keep actual Curve25519 as the endpoint. Continue the existing native Autolab
and ecbench architecture; do not restart its 35 completed rounds or substitute
miniature parity for full-size parity.

### Open threads

Full-width solver integration, finite experiment admission, fresh comparisons,
and independent validation remain. The exact next action is above.

### Errors

The desktop managed-worktree tool could not resolve the remote checkout;
native Git created the isolated worktree successfully. Conductor rejected the
shared daily-log scope; the new study scope passed separately. No permission
or resource limit was broadened.

## Continuation: target-coordinate recovery, 2026-11-05

### What was done

Current source already supplies `WideInstanceCtx`, `WideGauss`, and a
`CountedGroup::mul_biguint` hook overridden by `Curve25519Wide`. Reuse those
interfaces rather than widening the legacy `mul(u64)` entry point itself.
Added `WideGauss::solve_coordinate`: exact unit-row reduction recovers a
coordinate only when it lies in the relation row space, even at deficient
total rank. Added `recover_target`, which additionally checks the recovered
scalar against the supplied Curve25519 target and charges that group check.
Tests cover wide coefficients, two undetermined factor-base logarithms with
a determined target, underdetermined pivot coordinates, inconsistent systems,
out-of-range coordinates and a mismatched target point. These are development
tests, not measurements or sampled ECDLP solves.

### Decisions made

Single-target recovery need not wait for every base logarithm. Keep the old
full-rank `solve` API and add a target-specific API. The matrix API neither
verifies incoming group-relation witnesses nor accounts for its field work;
both must be supplied by the complete pipeline before comparative use.

### Open threads

Focused release tests are compiling under tool session 33564 (two build jobs,
1200-second timeout). Command: `CARGO_TARGET_DIR=/tmp/curve25519-autolab-target
CARGO_BUILD_JOBS=2 timeout 1200 cargo test --release --lib --offline --locked
curve25519_wide::tests -- --test-threads=2`. Output:
`/tmp/curve25519-wide-tests-20261105.log`. Poll that session before any retry.
Source was updated while dependency compilation was in progress; repeat the
focused command after the initial build to bind the final bytes, then run the
required library gate before publishing code. No test pass is recorded yet.
After validation, integrate verified wide relations and charged elimination
with this recovery entry point, then the ecbench/Autolab execution path.

### Errors

Sparse dependency materialization temporarily removed the study directory
because it had been added with `git add --sparse`; adding it to the checkout
definition restored the committed record. No evidence was deleted. Conductor
reported the source and study scopes clear before edits.

## Continuation: checked wide relation admission, 2026-11-05

### What was done

Added `WideRelationSystem` around the existing arithmetic and matrix. Its
constructor requires the canonical group and nonidentity prime-subgroup base
and target points. Admission requires one canonical coefficient per base
point and verifies `sum c_i B_i = a G + b Q` before inserting the row
`[c_0, ..., c_n, -b]` with right-hand side `a`. Invalid relations leave the
matrix unchanged; group verification work remains in the caller's counters.
Recovery uses the target-coordinate API and the final group check. Tests use
public point inputs derived from a greater-than-128-bit known-answer scalar,
with malformed, tampered, duplicate, identity and torsion controls.

### Decisions made

Keep relation search separate from admission and recovery. The code does not
pretend to discover the known-answer test relations. Scalar recovery from
valid relations is the implemented stage; matrix-operation accounting and
full Autolab execution remain subsequent work.

### Open threads

The initial focused test process (session 33564) exited 101. The compiler
reported 54 missing embedded-file errors; 45 distinct tracked fixture paths
are being materialized from HEAD with `git archive`, session 1338. The
retained path list is `/tmp/curve25519-missing-fixtures.txt`; the failed log
is `/tmp/curve25519-wide-tests-20261105.log`. Poll materialization before
retrying the focused build with a fresh log. No test pass is recorded.

### Errors

The initial sparse profile omitted embedded library-test fixtures and curve
registries. All requested paths were checked with `git ls-files
--error-unmatch`; hydration restores exact tracked bytes without editing
frozen evidence or weakening tests. Formatting succeeded; final compilation
and the required library test gate still need to pass before code publication.

Materialization session 1338 completed successfully (45 paths). The final-source
focused retry is now session 81671, with the same two-job/1200-second limits,
and log `/tmp/curve25519-wide-tests-20261105-attempt2.log`. The first failed
log is preserved. Poll session 81671 rather than submitting another retry.

## Continuation: scalar-field accounting, 2026-11-05

### What was done

Added `WideMatrixWork`, exposed by `WideGauss::work` and
`WideRelationSystem::matrix_work`. Counters accumulate field API calls for
row insertion, full solution, and target-coordinate reduction: multiplication,
addition, subtraction, inversion and conversion. Dependent/inconsistent rows
and unsuccessful coordinate queries retain their work. Counter overflow fails
explicitly. Added an exact two-column accounting regression covering a
dependent row, an unsuccessful query, and subsequent recovery.

### Decisions made

These counters are scalar-field native operations, not GAE. Inversion internals,
allocation, copying, comparisons and field construction are not silently
included; whole-process pricing and the remaining setup/admission operations
must still be bound in the complete measurement protocol. No performance
advantage is claimed from the added counters.

### Open threads

Session 81671 is still compiling the pre-accounting source snapshot. Current
`src/cryptanalysis/curve25519_wide.rs` SHA-256 is
`20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a`.
After that process terminates, rerun the focused command with a new log to
test these final bytes, then the required library gate. Do not claim its
earlier compilation validates these later edits. Code remains unpublished.

### Errors

No new compiler error has appeared in attempt 2 as of this entry. The process
is confirmed live; the missing-fixture failure from attempt 1 remains retained.
Formatting and `git diff --check` passed for the current source.

## Continuation: recovery tests passed; final accounting gate running

### What was done

Session 81671 finished with exit 0: 13 focused tests passed, zero failed or
ignored, 6169 filtered out. Release build took 11m05s and test execution
reported 0.01s. This is the pre-accounting version, as the 13-test list shows;
it is not final-source validation. Its retained log SHA-256 is
`2509cca6dbd52a4c773beb903758fa7a941fbb09d60d367fe11c8e3bdc3d0367`.
PR 3466 is now merged. Uncommitted code is on the new branch
`codex/curve25519-wide-relations-20261105` in the same worktree.

### Decisions made

Freeze source SHA-256
`20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a`
for final validation. Publish the implementation in its own PR after the
focused tests and required library gate pass.

### Open threads

Session 13039 runs the final focused command (two build jobs, 1200-second
timeout) into `/tmp/curve25519-wide-tests-20261105-final.log`, followed only
on success by `cargo test --release --lib --offline --locked --
--test-threads=2 --quiet` with a 1800-second timeout into
`/tmp/curve25519-library-tests-20261105-final.log`. Both use
`CARGO_TARGET_DIR=/tmp/curve25519-autolab-target`. Poll that same process.
No final-source test pass or full-library pass is recorded yet.

### Errors

No new compiler errors have appeared in final validation. Preserve the
original missing-fixture build failure and provisional success separately.

### Additional integration interfaces located during final validation

`ecbench/workload.rs::CurveFacts` stores `r`, `group_order`, and `cofactor`
as `u128`; `ecbench/methods.rs::SolveReport.recovered` is `Option<u128>`.
`ecbench/record.rs::Outcome.recovered` is already an optional string. The
wide pipeline must carry exact subgroup orders and recovered scalars through
all intermediate types, not only stringify the final outcome. Preserve the
canonical serialization of historical records. The final accounting source
remains frozen while session 13039 runs.

## Final focused validation passed; library fixtures being reconciled

### What was done

The final-source focused command passed all 14 tests, zero failed or ignored,
6169 filtered out. Build duration 9m55s, test duration 0.01s. The source still
hashes to `20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a`;
the final focused log hashes to
`e772bf513744e56323a2fb61f0215acd6745e4896a59fd99f40f647f56087b18`.
Session 13039 advanced to the full 6183-test library suite.

### Decisions made

Do not publish code until the full gate is resolved. The first library attempt
has fixture-dependent failures and is retained intact; do not reinterpret the
focused pass as a full-suite pass. No source edits during this validation.

### Open threads

The library suite is still running under session 13039. Device tests require
the tracked `gpu/certicom` emulator sources, absent from the sparse checkout;
hydration session 17465 completed. Additional failures name band, bounds,
challenge, claim and Koblitz rule-default tests. Session 48244 materializes
their tracked research fixtures and docs/bounds via `git archive HEAD`.
Poll both handles, collect final diagnostics, and finish any remaining exact
fixture hydration before the next full-library attempt. Preserve all logs.

### Errors

At least 17 failures were reported before the library suite reached its end.
The device test source explicitly requires `gpu/certicom/emulator.cpp`; the
other affected tests read archived research paths outside the sparse profile.
Exact final failure messages remain to be collected. No source defect is
ruled out by this preliminary diagnosis.

## 2026-11-05T16:41:57Z — tracked runtime fixtures restored

### What was done

Reconciled the still-live full-library session 13039 without launching a duplicate. Completed exact HEAD hydration of the r24 planner sessions, ecbench work-v1 test fixtures, all 15 evidence paths declared by the planner, and the archived n37 SOURCE42 input. The earlier Certicom, bounds, claim, challenge and rule-v8 hydration also completed. Each restoration used tracked Git objects; no assertions or historical evidence were edited.

### Decisions made

Retain the first full-suite attempt and its failures. Finish its diagnostics before retrying; restoration during the run does not retroactively change its failed tests. Keep source frozen at SHA-256 20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a.

### Open threads

The full library process remains active; final status and failure details are pending. The final focused 14-test pass remains the verified code result. Publication waits on the library gate; no new scientific or cloud trial was submitted.

### Errors

Two additional fixture-dependent tests failed: planner::every_cited_file_exists and plugins::compact_orbit_scan_matches_archived_n37_source42. Their tracked inputs have now been materialized. Confirm the exact terminal diagnostics before classifying all suite failures as fixture omissions.

## 2026-11-05T16:44:10Z — all 20 fixture rechecks passed

### What was done

Ran the 20 already-failed test names individually against the exact existing release test binary after fixture hydration, with a 120-second per-test and 600-second enclosing bound. All 20 exited zero. The recheck process (session 31688) is terminal. Retained the names, log, three completed focused-build attempt logs, byte counts and SHA-256 hashes in validation-wide-relations/.

### Decisions made

The unchanged binary passes every failure observed so far once its tracked inputs are present. This supports the fixture diagnosis for these 20 tests. It does not replace the full-library gate. The first full-suite attempt is still running under session 13039 and is preserved.

### Open threads

Collect any additional failures and the terminal full-library result, then rerun the required suite after exact fixture reconciliation. Publish source and validation artifacts only after the gate is resolved.

### Errors

No failure in the 20 targeted rechecks. Original full-suite failures remain in the original log and are not rewritten.

## 2026-11-05T16:47:10Z — matched rho width dependency inspected

### What was done

Read ic_wide::rho_negation and koblitz_strong_rho::RhoScalar. The scalar trait requires Copy plus SampleUniform and an unconditional to_u128 conversion; its only implementations are u64 and u128. ic_wide::gen_below invokes to_u128 to choose its one-word compatibility sampling branch. rho_negation also routes its legacy recovered field through to_u128, while returning an Option<S> separately. A nominal new scalar implementation must not silently truncate at either call site.

### Decisions made

Include the matched rho reference in the full-width integration. Preserve exact one-word sampling and existing committed behavior for u64/u128; require checked narrowing and exact wide sampling/serialization for the actual subgroup. BigUint cannot directly implement the current Copy-constrained trait. A fixed-width scalar adapter or an explicit trait refactor must cover these requirements before an actual-curve comparison. No new reference implementation or performance result is claimed.

### Open threads

The original library process remains live with its source frozen. After validation/publication, include full-width reference sampling, collision arithmetic, recovered-result transport and replay in the next implementation scope alongside IC workload dispatch.

### Errors

No new failure observed since the 20 fixture rechecks. The width restriction is a source integration dependency, not a failed experiment.

## 2026-11-05T16:54:43Z — full library attempt 1 terminal

### What was done

Session 13039 exited 101. Full suite: 6026 passed, 27 failed, 130 ignored, 1268.84 seconds. Preserved terminal log and hashed library-attempt1.json. All terminal failure diagnostics report absent tracked inputs. Twenty tests already passed after their fixtures were restored; seven additional failures require the tracked CryptoPro-B frozen chain constants.

### Decisions made

Restore the final tracked input and individually verify the seven remaining failures before one fresh full-suite rerun with unchanged source and the same 1800-second bound. Retain the original manifest as an observation at its creation time; library-attempt1.json supplies terminal status.

### Open threads

Complete final fixture hydration and seven rechecks, then execute the full library gate. Source publication still waits for the gate.

### Errors

The first full-suite attempt failed due to unavailable runtime fixture paths, including seven CryptoPro-B chain-set tests discovered at the end. No assertion was changed.

## 2026-11-05T16:56:05Z — all 27 failed tests rechecked; full gate rerunning

### What was done

Hydration session 13416 completed. All seven CryptoPro-B chain-set tests exited zero in individual rechecks against the unchanged test binary. Retained their exact names/logs and hashes under validation-wide-relations/. Together with the earlier 20 rechecks, all 27 original failures now pass. Reverified the frozen source SHA-256.

### Decisions made

Started one fresh full-library attempt after the previous process was terminal and all known failures had passed. Command: CARGO_TARGET_DIR=/tmp/curve25519-autolab-target CARGO_BUILD_JOBS=2 timeout 1800 cargo test --release --lib --offline --locked -- --test-threads=2 --quiet. No test skips or source changes.

### Open threads

Live session 68390; log /tmp/curve25519-library-tests-20261105-attempt2.log. Poll that handle without duplicating the run. Preserve terminal result and publish only after the required library gate passes.

### Errors

No failure in the final seven rechecks. Attempt 1 stays failed and immutable. Attempt 2 has no result yet.

## 2026-11-05T17:01:10Z — fast-probe coordinate regression reproduced

### What was done

An isolated native Rust diagnostic using the copied current curve implementation and repository Fp256 found a real hit mismatch: base G..64G, Q=-67G gives affine (1,3), fast (0,8), assertion exit101. Subtracting A/3 before the Montgomery recurrence and adding A/3 before the stored Weierstrass-key lookup gives (1,3), exit0. The isolated harness extracts the original ledger/trait definitions and uses a panic-only shim for an unused Fp64 primality path. Preserved logs, minimal regression/fix patches and hashes under validation-wide-relations/probe-regression/.

### Decisions made

Keep the repository source frozen for the ongoing full-suite run; the diagnostic patches are not applied. Before full-width collector integration, reproduce with the actual crate, cover identity crossings and batch boundaries, and integrate the correction with source-level tests. No comparative measurement or speed claim follows from this diagnostic.

### Open threads

Full-suite session68390 remains active. The confirmed coordinate mismatch adds a concrete correctness repair before using probe_hits_fast in the full solver. The checked standalone case does not establish all edge cases.

### Errors

Initial standalone compilation missed the unrelated Fp64 primality module; retained its failed log, then supplied a panic-only unused shim. The original native assertion fails as expected; the corrected copied source passes. Main repository tests for this new regression are not yet run.

## 2026-11-05T17:06:59Z — identity and batch-edge probe prototype checked

### What was done

The coordinate-only diagnostic exposed three further mismatches at Q=O,-G,-2G. A copied-source affine-suffix fallback at exceptional x(G)/identity states passed seven cases and then 18 deterministic identity/doubling/batch-edge/bounded-hit cases. Hit indices and legacy affine-equivalent GroupOps agree with direct enumeration in all 18. Retained original failures, prototype logs, hashes and an unapplied combined fix/regression patch in validation-wide-relations/probe-edges/.

### Decisions made

The combined patch supersedes the coordinate-only proposal. Preserve the frozen source until the active library gate ends. The inherited affine-equivalent ledger excludes actual projective block and exceptional reconstruction overhead, so do not use these counter matches as full-cost measurement.

### Open threads

Poll session68390; then apply the combined repair with actual-crate regressions and the required checks before solver integration. Full-width scalar/reference integration remains separate ongoing work.

### Errors

Coordinate-only seven-case diagnostic exited101 with three mismatches. Extended prototype exited0 for 18 cases. These are standalone native diagnostics, not final repository-test passes.

## 2026-11-05T17:11:06Z — probe correction integrated, validation queued

### What was done

Conductor check reported source/study scopes clear. Saved source-before-probe.patch and its base commit/hash to preserve the running suite snapshot. Integrated Montgomery/Weierstrass conversion, exceptional affine suffix and the 18-case regression in the normal curve25519_wide::tests module. Reused the first affine point, avoided duplicate lookup conversion and saturated the batch bound. Documented the inherited affine-equivalent tally exclusion. Rustfmt and git diff --check passed.

### Decisions made

New source SHA-256: d9671086380789e83ed5cfb518cb054a53d3dfda4d7e2e3d957ad4ac48d6039a. The still-running session68390 validates the prior source, not this repair. Freeze this new source for validation. The prototype patches now describe implemented history; the current source includes minor cleanup and the regression within the test module.

### Open threads

Session51745 waits on confirmed prior PID838376 (maximum1800 seconds), checks the repaired source hash, then runs the focused tests (1200-second limit) and only on success the full library suite (1800-second limit), with two build/test threads. Logs: /tmp/curve25519-wide-tests-20261105-probe-fix.log and /tmp/curve25519-library-tests-20261105-probe-fix.log. Poll those existing handles; do not start another validation chain. Preserve terminal old/new results separately.

### Errors

No new test result yet for the integrated repair. The old focused14 pass and standalone18-case pass remain scoped to their recorded snapshots. Source is not yet published.

## 2026-11-05T17:18:56Z — pre-repair full library gate passed

### What was done

Session68390 exited0: 6053 passed, zero failed, 130 ignored, 1317.01 seconds. Retained the full terminal log and library-attempt2.json hash/size receipt. Separately reconstructed source SHA-256 20ae4700e15f0afca073a83217b3875d28032fab447e27f7cf3d939eb708f28a from the recorded base and source-before-probe.patch; all 23 archived artifact hashes/sizes in the checked manifests matched.

### Decisions made

This is a complete library pass for the relation-recovery/accounting implementation before the later probe repair. Keep that scope explicit. Do not attribute it to the repaired source d9671086380789e83ed5cfb518cb054a53d3dfda4d7e2e3d957ad4ac48d6039a.

### Open threads

Session51745 has moved from waiting to compiling the repaired source. Its focused log now reports Compiling crypto; no focused or full repaired-source result yet. Retain this handle, then capture both results and publish the implementation once its final gate passes.

### Errors

No failures in the pre-repair rerun. The original failed attempt remains preserved with all 27 passing rechecks.

## Reporting clarification and delivery preparation

### What was done

Read the repository's PCC reporting contract and restored its tracked tool/keyring/ledger files from HEAD. Added DELIVERY.md with explicit unsigned producer-observation labels; `pcc lint-prose` returned zero findings. All prior local test, hash, replay and correctness statements in this continuation are producer observations, not signed PCC execution verdicts. [pcc:unverified]

### Decisions made

Preserve historical raw logs and result records. Use explicit `[pcc:unverified]` status in delivery prose until a signed verdict exists. Neither the local artifact manifest nor the successful library process establishes PCC verification. [pcc:unverified]

### Open threads

Conductor denied the claims-directory and ledger scopes because task T-1641 holds them exclusively. Official registration remains pending; do not bypass the reservation. Repaired-source validation continues in session51745 with the source unchanged. [pcc:unverified]

### Errors

The PCC tool was initially absent from the sparse checkout, though tracked; exact hydration resolved that access issue. Official claim creation was not attempted after the ownership conflict. Earlier unqualified uses of verification terminology are superseded by the explicit unsigned scope above. [pcc:unverified]

## 2026-11-05T17:30:23Z — repaired source focused tests passed

### What was done

The focused actual-crate run reports 15 passed, zero failed or ignored, 6169 filtered, 0.13 seconds test runtime. This includes the integrated 18-case identity/batch-edge regression. Source SHA-256 remains d9671086380789e83ed5cfb518cb054a53d3dfda4d7e2e3d957ad4ac48d6039a. Preserved the log and focused-probe-fix.json hash/size receipt and updated DELIVERY.md. [pcc:unverified]

### Decisions made

Keep source frozen. The focused pass is now for the actual repaired repository module; it still does not replace the full-library gate. All results remain unsigned producer observations. [pcc:unverified]

### Open threads

Session51745 proceeds to the full library command with its existing 1800-second timeout. Log /tmp/curve25519-library-tests-20261105-probe-fix.log. Poll that same handle and preserve its terminal result before publication. Official PCC registration still awaits the separate reserved ledger scope. [pcc:unverified]

### Errors

No focused test failures reported. Earlier failed fixture and probe diagnostics remain retained. [pcc:unverified]

## Full-width transport inspection during final validation

### What was done

Inspected the ecbench scalar codec and workload producers/consumers. `compat_u128` already writes values through u64 as JSON numbers and larger values as decimal strings; its parser is limited to u128. `matching_representation` and `parse_hex` narrow both subgroup orders and registry coordinates to u128. `Instance::mul_generator_hex`, `planted_scalar`, the public-target route, and the runner/audit/compare/bounds/work pricing consumers must be included in any widening change. [pcc:unverified]

### Decisions made

Extend the existing number-or-decimal-string serialization convention while preserving historical serialized records and target laws. Use exact internal order/coordinate/scalar representations and checked narrowing into legacy engines. Do not use zero sentinels for orders that exceed u128. Give any new full-width target sampler an explicit law identity rather than changing old seeded inputs. The exact order is greater than 2^252 and below 2^253: its integer bit length is 253, although log2(r) is approximately 252; distinguish this from the 255-bit field. [pcc:unverified]

### Open threads

Implement full-width construction, public target generation, exact registry identity, scalar recovery transport and replay together after this component's delivery. The rho adapter separately needs exact wide sampling and checked result conversion. Final repaired-source library session51745 is still active; no additional run was submitted. [pcc:unverified]

### Errors

No new test failure observed during this inspection. No transport code was changed, and no widened workload is claimed executable yet. [pcc:unverified]

## 2026-11-05T17:40:09Z — exact-scalar codec candidate tested

### What was done

Conductor reported the proposed ecbench/exact_scalar.rs scope clear. Prepared and natively compiled an isolated four-limb unsigned codec, then ran four tests covering exact width boundaries, order/bit comparisons against BigUint, checked narrowing, optional scalars, invalid/oversized JSON, and legacy wire shape. Test output reports four passed and zero failed. Saved source, logs, compiler/dependency hashes and scope in next-integration/exact-scalar/. [pcc:unverified]

### Decisions made

Keep the current library source frozen. This candidate is not registered in ecbench and supplies no arithmetic, target sampler or solver. Next integration must compare against the actual legacy codec and preserve committed record bytes and target laws, then widen consumers rather than introducing truncated/sentinel values. [pcc:unverified]

### Open threads

Final library session51745 continues without a reported failure. Publish the relation/probe implementation after its gate; use this separately tested codec candidate for the subsequent complete width integration. [pcc:unverified]

### Errors

No standalone build or test failure reported. This result is not an integrated full-width workload pass. [pcc:unverified]

## 2026-11-05T17:44:16Z — exact-scalar legacy compatibility checked

### What was done

Compiled a native harness including the actual extracted compat_u128 codec and the unchanged candidate. Test output reports seven passed: four original tests plus three compatibility tests. All 3204 records across four retained fixtures were read; 9612 order/cofactor fields preserved their original JSON values and matched legacy serialization. Constructed boundaries through u128::MAX cross-decoded and compared bytes, including nullable values. Retained sources, logs, fixture hashes and compatibility scope in next-integration/exact-scalar/. [pcc:unverified]

### Decisions made

Historical records checked here contain no above-u64 field. Do not present that cohort as historical u128-record coverage; constructed boundary cases cover the larger old codec domain. This candidate remains unregistered in the library and does not alter the source under the current full suite. [pcc:unverified]

### Open threads

Session51745 remains live with no reported full-suite failure. After its result, complete delivery and integrate exact transport through the workload, identity, target, solver and audit paths. [pcc:unverified]

### Errors

No build or compatibility-test failure reported. No archived fixture was edited. [pcc:unverified]

## 2026-11-05T17:54:19Z — final repaired-source full library pass

### What was done

Session51745 exited0. Final full library output: 6054 passed, zero failed, 130 ignored, 1348.17 seconds. Source SHA-256 remains d9671086380789e83ed5cfb518cb054a53d3dfda4d7e2e3d957ad4ac48d6039a. Retained terminal log and library-probe-fix.json. Updated DELIVERY.md with final results and explicit standalone codec scope. [pcc:unverified]

### Decisions made

Proceed to commit, push and regular PR publication of this validated local component. Keep all scientific results unsigned; current PCC reservation readback names T-1649, replacing the earlier T-1641 observation. No reserved claim/ledger writes are made. [pcc:unverified]

### Open threads

Current-head PR checks and delivery remain, followed by full-width Autolab/ecbench and rho integration. The codec candidate remains outside runtime dispatch. [pcc:unverified]

### Errors

No final focused or library test failures reported. All earlier failed attempts and exact scope boundaries remain preserved. [pcc:unverified]

## 2026-11-05T18:05:31Z — published component and next branch

### What was done

Published commit a9864459c793875edd5ab4f3f3065ebf499f7bcc on codex/curve25519-wide-relations-20261105 and opened regular PR https://github.com/isogeny-labs/crypto/pull/3488. Auto-merge enabled at18:01:37Z. Initial exact-head readback: required gate/size/artifacts running, PCC gate passed, PR open. The desktop attachment call ended cancelled without confirmation. No duplicate attachment or CI run was submitted. Created codex/curve25519-exact-transport-20261105 at the published commit; first PR head remains untouched. [pcc:unverified]

### Decisions made

Continue exact transport integration on the new branch. Conductor reported exact_scalar.rs, ecbench/mod.rs and the study scope clear. No subscription/scheduled check-in tool is exposed in this runtime; check PR3488 at the repository cadence (about50minutes after the initial readback), not on each goal continuation. [pcc:unverified]

### Open threads

First PR checks/merge remain pending. Move the tested codec into ecbench and connect it to full-width workload/result/reference interfaces. Current source tests cover the published component; new changes require their own checks. PCC registration remains reserved by T-1649 as last read. [pcc:unverified]

### Errors

The app attachment request was cancelled before a response. GitHub creation and auto-merge requests succeeded; no app attachment success is claimed. [pcc:unverified]

## 2026-11-05T18:08:59Z — exact-scalar module registered

### What was done

Added ecbench::exact_scalar with the candidate type and seven tests, wired through ecbench/mod.rs. Tests now reference the real canonical codec and use the crate fixture root. Decimal parsing bounds significant digits before BigUint allocation while retaining leading-zero compatibility. A native harness compiled the current module with an exact compat_u128 extraction and reported seven passed, zero failed, 0.46 seconds, including all 3204 retained records/9612 fields. Preserved source hashes and logs under transport-integration/. [pcc:unverified]

### Decisions made

This registers the type without changing any workload order, target or solver-result consumer yet. Continue by widening those actual interfaces, preserving legacy wire bytes and target laws. Do not claim this registration makes full-size workloads executable. [pcc:unverified]

### Open threads

Session43442 runs CARGO_TARGET_DIR=/tmp/curve25519-autolab-target CARGO_BUILD_JOBS=2 timeout1200 cargo check --release --lib --offline --locked. Log /tmp/curve25519-exact-transport-check-20261105.log. It reached Checking crypto; terminal result pending. PR3488 remains on its published head with auto-merge enabled; next normal check-in about18:51Z, not on every continuation. [pcc:unverified]

### Errors

No native module test failures reported; no whole-library type-check result yet. [pcc:unverified]

## Exact-scalar module type-check completed

### What was done

Session43442 exited0; cargo check --release --lib completed in3m10s. Retained terminal log/hash. Native seven-test module check is recorded separately. [pcc:unverified]

### Decisions made

Checkpoint this locally before widening SolveReport.recovered and its solver/runner adapters. Preserve JSON encoding and use explicit checked legacy conversions. The existing full suite validates the prior published component; this stage has a type-check and standalone native tests only. [pcc:unverified]

### Open threads

Widen result transport and then actual workload orders/targets and rho. Full new integration gate required before another push. PR3488 check-in remains at the recorded cadence. [pcc:unverified]

### Errors

No type-check error reported. [pcc:unverified]

## 2026-11-05T18:28Z — recovered scalar transport integration

### What was done

Checkpoint 710a0f9ac14a registered the exact scalar codec locally. Widened SolveReport.recovered to ExactScalar, converted narrow solver adapters explicitly, preserved BigUint results through 256 bits, added exact runner/audit parsing and subgroup-reduced verification. Added report wire-format/full-width and legacy-instance verification regressions; updated existing expected values without weakening assertions. Conductor cleared ecbench and the neighboring ecbench_large_prime/trace_line_tests.rs assertion. [pcc:unverified]

### Decisions made

Preserve existing number-through-u64/string-above encoding and canonical workload target laws. Full-width workload construction and matched rho remain the next integration boundary. Do not push until the complete required library gate passes. [pcc:unverified]

### Open threads

Current focused ecbench build/test command uses CARGO_TARGET_DIR=/tmp/curve25519-autolab-target CARGO_BUILD_JOBS=2 timeout1200 cargo test --release --lib --offline --locked cryptanalysis::ecbench:: -- --test-threads=2. Log /tmp/curve25519-transport-stage2-ecbench-tests.log; terminal result pending. PR3488 next cadence check remains approximately18:51Z. [pcc:unverified]

Read-only cloud reconciliation: IsoLab overview lists no active Curve25519 job; direct NodePool retrieval still fails sts:AssumeRole for the configured launcher role. STS identifies the existing WorkerRole in account104186811093. Budgets API returned monthly USD538.72/1000 updated11:55:08Z and daily USD104.44/200 updated11:47:42Z. These delayed observations exclude a verified reservation balance; remaining usable funds and live cap enforcement remain unconfirmed. No cloud submission. [pcc:unverified]

### Errors

First library type-check found five u128 adapters; corrected. Broad test-target checking first failed on four sparse tracked bin paths, restored exactly from HEAD. Retry exposed expected-value type errors and unrelated missing cover-links.yaml in a bin test. Narrow library test build then exited101 on one incorrect test import; corrected. Retain /tmp/curve25519-transport-stage2-{check,tests-check,tests-check-attempt2,lib-build}.log; the pending focused run is a fresh attempt. [pcc:unverified]

## 2026-11-05T18:29Z — full-width target sampler candidate

### What was done

Developed a bounded deterministic rejection sampler using counter-tagged canonical JSON SHA-256 inputs. The standalone native harness compiled the candidate with the actual hash, canonical codec and exact-scalar modules and reported32 passed/0 failed, including five sampler tests. Retained source/log hashes under next-integration/exact-sampling. [pcc:unverified]

### Decisions made

Keep this candidate outside the currently compiling crate graph. Integration will use a new named law, preserving historical target derivations. Equal-multiplicity checks exhaust one-byte inputs for bounds1..256; no distribution/benchmark claim is made from the seeded checks. [pcc:unverified]

### Open threads

Focused ecbench test session51270 remains compiling at18:28Z; log /tmp/curve25519-transport-stage2-ecbench-tests.log. Integrate sampler with exact workload orders and Curve25519 dispatch after this transport stage. [pcc:unverified]

### Errors

The native harness emitted five unused-code warnings from reused modules, no test failures. [pcc:unverified]

## 2026-11-05T18:35:07.664366+00:00 — sampler replay helper

### What was done

Added replay.py for the standalone sampler: checks pinned source hashes, records dependency hashes/compiler version and build/test exit codes. Its fresh invocation reported32 passed/0 failed; logs and receipt retained. [pcc:unverified]

### Decisions made

Keep observed harness and replay outputs separately. Replay is development evidence only. [pcc:unverified]

### Open threads

Focused ecbench session51270 still compiles; same log and timeout. Workload and rho widening remain next. [pcc:unverified]

### Errors

No replay error reported. [pcc:unverified]

## 2026-11-05T18:39:04.813606+00:00 — recovered-scalar transport tests completed

### What was done

Actual-crate ecbench filtered suite completed:517 passed,0 failed,5 ignored,5671 filtered in106.46s after10m51 compilation. Both new full-width report serialization and subgroup-reduction verification regressions passed. Rechecked all stage2 source hashes against the pending manifest and retained the terminal log and hash manifest. [pcc:unverified]

### Decisions made

Checkpoint completed result transport locally. Do not push before the complete library gate. Continue exact workload-order, target-generation and reference integration. [pcc:unverified]

### Open threads

Session46114 runs cargo check --release --offline --locked --bin ecbench --bin ic_autolab with the same target directory and2 build jobs, timeout900. Log /tmp/curve25519-transport-stage2-entrypoints-check.log; result pending. PR3488 cadence check remains about18:51Z. [pcc:unverified]

### Errors

No test failure in this attempt. Earlier failed attempts remain archived. [pcc:unverified]

## 2026-11-05T18:40:25.294308+00:00 — entrypoints pass; full library gate started

### What was done

Session46114 exited0: cargo check for ecbench and ic_autolab finished in2m04. Retained log/hash. Started full-library gate with timeout1800, two build jobs, two test threads, unchanged source. [pcc:unverified]

### Decisions made

Complete this result-transport delivery before the next workload-order implementation. No push until full gate terminal success. Keep existing PR3488 head unchanged. [pcc:unverified]

### Open threads

Full library test session91773, log /tmp/curve25519-transport-stage2-library-tests.log. Poll original handle, no duplicate run. Local checkpoint commit session11828 remains pending; poll before any next Git mutation. Its command stages result transport and candidate sampler, then commits with message Carry exact recovered scalars through ecbench reports and verification. Entry-point receipt and this final log entry were written after that staging and need a later commit. PR3488 cadence readback around18:51Z. [pcc:unverified]

### Errors

No entrypoint check error. Cloud admission unchanged: launcher AssumeRole denied, cap/reservation readback unconfirmed; no new cloud job. [pcc:unverified]

## 2026-11-05T18:46:41.347870+00:00 — actual Curve25519 workload candidate

### What was done

Commit e843634c832b completed. Conductor cleared ecbench/study scopes. Added a separate candidate for exact Curve25519 coordinate validation, full-width generator multiplication, and new named planted/public target construction. Five native tests passed using actual curve/Fp256 source with extracted ledger/trait and unused Fp64 shim; retained sources, dependency hashes and both build attempts. [pcc:unverified]

### Decisions made

Keep candidate outside the current crate graph while full-library gate91773 runs. Reject mutated parameters and generator, preserve historical target laws. Wire into the actual workload/dispatch after the current result-transport delivery. [pcc:unverified]

### Open threads

Full-library91773 remains live; latest progress above2175/6193. PR3488 readback due about18:51Z. New adapter is not actual-crate dispatch yet. [pcc:unverified]

### Errors

Native first build selected num-bigint without rand and failed on imported field-test APIs. Corrected selection by Cargo fingerprint features; attempt2 exited0 with5 passed/0 failed. No production source change was needed for that harness failure. [pcc:unverified]

## 2026-11-05T18:49:21.531479+00:00 — workload replay and checkpoint

### What was done

Local checkpoint59939389294e records the workload candidate and earlier entrypoint receipt. Added a portable hash-checking native replay helper; its fresh invocation exited0 and reported5 passed/0 failed. Retained replay dependency hashes, commands, logs and receipt. [pcc:unverified]

### Decisions made

Replay imports actual arithmetic with the same documented ledger extraction/shim scope. Keep this separate from actual-crate and cloud results. [pcc:unverified]

### Open threads

Full library91773 remains live in existing long-running Semaev tests; no failure seen. PR3488 check due around18:51Z. Current source gate still covers result transport, not the unregistered candidate. [pcc:unverified]

### Errors

No replay error reported. [pcc:unverified]

## 2026-11-05T18:50:40.471844+00:00 — relation/probe PR merged

### What was done

GitHub fetch_pr3488 returned merged=true, closed, original heada9864459c793875edd5ab4f3f3065ebf499f7bcc, merge commitfab26948518b438673d3b7d43b546e2e23895906. Saved reduced readback. Combined legacy-status API returned an empty list; this is not a check-run inventory. [pcc:unverified]

### Decisions made

No further polling of merged PR3488. Publish the exact-transport branch only after its active full-library gate succeeds. Current local HEAD59939389294e contains result transport and the unregistered workload candidate; replay helper/logs are the remaining local checkpoint. [pcc:unverified]

### Open threads

Full-library session91773 is still live with original timeout1800, log /tmp/curve25519-transport-stage2-library-tests.log. At18:50Z its latest numbered progress was2175/6193, followed by ongoing dots in the long Semaev section and no failure text. No duplicate test run. Next implementation is the actual full-width workload/Instance registration after current source delivery. [pcc:unverified]

### Errors

No PR-status error. No signed PCC standing inferred from merge or native tests. [pcc:unverified]

## 2026-11-05T18:59:19.841704+00:00 — separate workload integration checkout

### What was done

Created /home/ec2-user/crypto-curve25519-workload-20261105 on codex/curve25519-workload-20261105 at51ae0683ef662077b5dd5f186f77dc3703074f97. Native sparse read-tree exited0. App attachment listing did not return and its read-only call was terminated. Conductor cleared ecbench/study scopes. Original result-transport checkout remains unchanged for live full-library91773. [pcc:unverified]

### Decisions made

Integrate exact workload orders and scalar transport here while retaining old source for its gate/publication. Preserve old target laws through u128, add new bounded rejection law above it. [pcc:unverified]

### Open threads

Apply the prepared exact-order patch, compile and resolve all consumers, then register the actual Curve25519 instance/target adapter. Original full-library91773 still live, no duplicate run. [pcc:unverified]

### Errors

Sparse setup required read-tree after --no-checkout; successful terminal result confirmed before source edits. [pcc:unverified]

## 2026-11-05T19:08:45.396216+00:00 — actual workload/order integration type-check

### What was done

Widened CurveFacts orders, planted scalar transport and comparison/price consumers; registered ExactScalar sampling and actual Curve25519 workload modules. Added CurveSpec/Instance constructor, exact target construction, identity facts and verification. Library check attempt3 exited0 after resolving narrow conversions, new exhaustive matches and five sparse include inputs. Added a construction/replay regression; test execution remains pending. [pcc:unverified]

### Decisions made

Preserve legacy target laws and record encodings. Actual full-width solver adapters are next; until connected, applicability and direct solver entry explicitly return unsupported rather than narrow coordinates. Removed unused narrow bit helper. Restore missing tracked test inputs from the unchanged gate checkout before compiling tests. [pcc:unverified]

### Open threads

Compile/run actual-crate workload tests and adapt legacy test boundaries. Original transport full gate completed6063/0/130; publication proceeds in original checkout. [pcc:unverified]

### Errors

First order check failed on missing sparse inputs and narrow consumers; second found three exhaustive-match cases and an ambiguous double conversion. Both retained under /tmp/curve25519-workload-orders-check*.log. [pcc:unverified]

## 2026-11-05T19:15:44.301077+00:00 — workload test compile adapters corrected

### What was done

Order library check attempt3 passed. First test compilation exited101 on six missing concat-include diagnostics and four legacy test adapters. Restored40 direct includes/record inputs plus7 concat inputs from tracked unchanged files in the gate checkout; retained input hashes. Corrected legacy narrow test conversions, expected-order comparison and a SizeRow constructor. Added exact float/checked-conversion regression. [pcc:unverified]

### Decisions made

Rerun the actual Curve25519 workload construction test against these corrected sources. Existing 64/128-bit fixture expectations remain unchanged as values; conversions are explicit. [pcc:unverified]

### Open threads

Original result-transport source is published as PR3495, head722cf1bab65a6413558c00bb8654687b346da2ac, auto-merge enabled19:10:59Z, initial checks running. Next normal check about20:01Z; no repeated CI polling. App attachment call did not return and was terminated without confirmed attachment. Actual full-width solver adapters remain next after workload tests. [pcc:unverified]

### Errors

Retained all failed checks and first test compilation under workload-integration/. No cloud trial launched. [pcc:unverified]

## 2026-11-05T19:30Z — actual Curve25519 workload test passed

### What was done

Actual-crate construction/replay regression passed1/0,6204 filtered after10m17 compilation; terminal exit0 recorded with source hashes and log under workload-integration. Prepared an unregistered exact rho scalar arithmetic/sampling module. Conductor scopes for ic_wide.rs and koblitz_strong_rho.rs were clear at19:19. [pcc:unverified]

### Decisions made

Generalize shared rho scalar conversions with checked narrowing; retain a separate narrow-only bound for legacy binary IC and claw routines. Keep native u64/u128 random streams unchanged. Exact scalar modular API counts remain unpriced. [pcc:unverified]

### Open threads

Integrate and validate exact rho arithmetic, then connect a bounded full-width solver adapter. PR3495 next scheduled check20:01Z. [pcc:unverified]

### Errors

No new test failure. Earlier compile failures retained. Cloud admission remains unavailable as recorded previously; no experiment submitted. [pcc:unverified]

## 2026-11-05T19:31Z — rho conversion boundary checked

### What was done

Registered exact scalar arithmetic and generalized shared rho exports. First library check found an additional legacy narrowing consumer in Wiedemann diagonal sampling. Conductor cleared linalg.rs before editing. [pcc:unverified]

### Decisions made

Constrain legacy Wiedemann to NarrowRhoScalar, matching its existing sampling domain. Add a distinct explicitly capped Curve25519 reference adapter with no inherited square-root-sized default. Extend seeded trajectory regression across the exact scalar backend. [pcc:unverified]

### Open threads

Type-check the correction and adapter, then run native regression tests. [pcc:unverified]

### Errors

First check exited101; retained exact-rho-check-attempt1.log. [pcc:unverified]

## 2026-11-05T19:33:38.824003+00:00 — bounded Curve25519 reference adapter type-check

### What was done

Library check attempt2 exited0 after constraining legacy Wiedemann. Added rho.curve25519_bounded with required max_steps, exact recovered values, field counters and unpriced scalar API counters. Added actual adapter replay/exhaustion/input-rejection tests and exact-versus-u64 trajectory checks. [pcc:unverified]

### Decisions made

Expose existing walk cap as an iteration stop threshold: at most17 counted walk operations can cross it; setup and verification are additional and counted. Restrict threshold to1000000. Input validation occurs before solver timing. [pcc:unverified]

### Open threads

Compile and execute focused tests; new tests are pending. This is a bounded reference adapter, not an index-calculus trial. [pcc:unverified]

### Errors

No additional library error. First failed check preserved. [pcc:unverified]

## 2026-11-05T19:36:33.927832+00:00 — Autolab manifest boundary

### What was done

Verified native test session43449 remains live compiling. Inspected Autolab: SizeInfo.r remainsu64 and arm method generation fixes ic.pipeline. Conductor cleared src/bin/ic_autolab.rs. [pcc:unverified]

### Decisions made

Widen manifest order with ExactScalar and preserve existing small JSON encodings. Prepare a bounded full-width pair-decomposition collector feeding WideRelationSystem; no trial or comparative claim is made by implementation tests. [pcc:unverified]

### Open threads

Complete native reference validation, collector construction and Autolab routing. [pcc:unverified]

### Errors

No new failures observed. [pcc:unverified]

## 2026-11-05T19:39:57.997222+00:00 — full-width relation collector component checks

### What was done

Standalone native pair collector passed3/0 tests: signed repeated-summand public-input recovery including r-2 above128bits, deterministic bounded exhaustion with exact random coefficients, and malformed inputs/resource limits. Snapshot and native build/replay evidence retained in next-integration/pair-collector/attempt1. [pcc:unverified]

### Decisions made

Add bounded factor-base construction from lifted x coordinates with cofactor clearing, retaining G as the one publicly known base logarithm. Keep this module unregistered while the actual-crate rho test compiles; test it independently against actual curve and matrix source. [pcc:unverified]

### Open threads

Run factor-base checks and integrate the resulting complete bounded collector into ecbench and Autolab. [pcc:unverified]

### Errors

No new native failures. Standalone harness uses extracted GroupOps/CountedGroup declarations and an unused Fp64 primality shim, as recorded by replay.py; it is not an actual-crate dispatch test. [pcc:unverified]

## 2026-11-05T19:42:07.493005+00:00 — bounded public factor-base construction checked

### What was done

Pair collector attempt2 passed4/0 native tests, adding reproducible cofactor-cleared public lifts, subgroup checks and explicit factor-base exhaustion. README describes algorithm, fixture scope, accounting gaps and portable replay. Attempt1 source and both attempts' evidence preserved. [pcc:unverified]

### Decisions made

Retain this as the generic two-summand pipeline component. Integrate actual-crate dispatch after the current rho compilation finishes, and carry explicit bounded reference configuration into Autolab without rewriting miniature history. [pcc:unverified]

### Open threads

Native actual-crate test session43449 remains live; Autolab exact-order serde test is written but pending. [pcc:unverified]

### Errors

No new failures. No cloud experiment submitted. [pcc:unverified]

## 2026-11-05T19:43:33.559532+00:00 — validation processes reconciled

### What was done

Native library test session43449 remains live compiling, log /tmp/curve25519-exact-rho-tests-attempt1.log. Autolab test session3990 is live waiting on that target-directory build lock, log /tmp/curve25519-autolab-exact-manifest-tests.log; it is queued, not a duplicate library test. Recorded the exact Autolab source hash and command. Corrected all four legacy SizeInfo test constructor conversions. [pcc:unverified]

### Decisions made

Do not change registered library source until the current build and queued binary build have consumed their snapshots. Candidate collector remains outside the registered module graph; its four native checks passed. [pcc:unverified]

### Open threads

Read terminal outcomes, run exact-rho scalar and one-word trajectory tests against the resulting library test binary, then register the collector, connect explicit Autolab methods/reference selection, and complete the required full-library gate before publication. PR3495 followup remains due20:01Z. [pcc:unverified]

### Errors

No new failures observed; queued build lock is expected serialization. [pcc:unverified]

## 2026-11-05T19:47:06.785618+00:00 — actual reference and exact scalar checks passed

### What was done

Actual-crate Curve25519 tests completed28/0/1ignored after11m10 compilation. Explicit exact scalar tests passed3/0; legacy trajectory equivalence passed1/0, covering nine seeded curves/traces with identical group work and counters. Rechecked every frozen library source hash; all match. Logs and receipt retained under workload-integration. [pcc:unverified]

### Decisions made

Add explicit collector stage observations for the measured adapter, preserving the standalone no-op path. Keep the queued Autolab build's registered source unchanged until it completes. [pcc:unverified]

### Open threads

Autolab session3990 now compiling, rather than waiting on the lock. Next connect collector dispatch and reference configuration. [pcc:unverified]

### Errors

No new failures. [pcc:unverified]

## 2026-11-05T19:47:58.048298+00:00 — fixed-source checkpoint for measured integration

### What was done

Collector stage-observation regression passed; attempt3 native suite5/0. Library actual-curve28/0/1ignored, scalar3/0 and trajectory1/0 results retained. Autolab session3990 still compiling the widened manifest source. [pcc:unverified]

### Decisions made

Checkpoint current source locally before further registered edits. Continue collector dispatch in a separate checkout from this checkpoint so the Autolab build and subsequent full-library gate have stable source. No push until the required gate passes. [pcc:unverified]

### Open threads

Autolab terminal result, full library gate and publication; collector dispatch and explicit bounded Autolab reference/candidate configuration. [pcc:unverified]

### Errors

No new failures. [pcc:unverified]
