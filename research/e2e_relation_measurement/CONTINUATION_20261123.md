# Continuation — 2026-11-23

## Observation and decision

PR #5105 is open at eba88cc0f884 and is marked ready. The panel emits a receipt for an externally timed-out process, but a timeout during the algebra step discards the generated points, system and completed phase measurements. This makes it impossible to connect the interrupted solver call to the seed and inputs. The current Sage backend still does not expose the solver's internal degree or arithmetic counters. `src/cryptanalysis/f4_fp.rs` has distinct bounded F4 counters; integrating them requires a separately validated backend adapter and cannot make Sage's counters appear retroactively.

## Work in progress

Add atomic phase checkpoints in the existing measurement driver and have the panel preserve and hash them on successful and unsuccessful child exits. A bounded correctness control will exercise a completed run and a forced interruption after the system has been built. Keep incomplete phases explicitly marked, and do not infer solver statistics.

## Coordination

Conductor T-10240 owns `research/e2e_relation_measurement`. The shared 2026-11-23 session log was held by T-10406 when checked; this scoped continuation note records the work until it can be appended safely.

## Checkpoint implementation

The driver writes an atomic JSON snapshot on entry to each phase and after completing generation and the run. The panel supplies a per-attempt checkpoint path and records its SHA-256. A running checkpoint names the phase and includes only completed phase timings; no operation count or algebraic degree is invented. Tests inject a deterministic solver failure after constructing the input system, and check that the last snapshot retains the system and generated candidate trace.

Next: run the injected failure test and the complete correctness suite. These are functional checks on the local test group, not a performance panel.

The focused checks `test_checkpoint_survives_solver_failure` and `test_panel_pairs_targets` passed (2 tests, 10.791 seconds). The injected failure snapshot contained all candidate records, the three input polynomials and the completed generation/construction timings, with `phase=groebner_basis` and no completed solver timing.

The complete local suite passed (7 tests, 26.236 seconds), including the new injected interruption control. The panel hashes each snapshot but did not bind the source digest until the end of a successful driver run. Add a start-of-run source digest to the checkpoint and compare the driver source digest before and after the panel.

The source-binding follow-up tests passed (2 tests, 8.489 seconds); the snapshot digest matches the panel's initial source digest, and the before/after file digests agree. The previous seven-test full suite passed before this narrow digest change. The shared session log remained held at the follow-up check (T-10413), so this scoped note retains the evidence.

Remaining work includes an instrumented backend with definitions and coverage for solving degree and arithmetic work, durable outer-process controls for resource measurements, a matched complete single-target comparison, and a frozen admitted run. The checkpoints do not make missing metrics available. PR #5105 was observed open, ready, and merge-conflicted; its review state was not changed.

## 2026-11-23 — relation outcome accounting review

The panel currently counts a run as useful only when its recovered scalar verifies, but does not separately expose the requested count of signed relations with nonzero B, or distinguish a failed scalar check from an ordinary run without a useful relation. The planned correction records those counts separately, treats any failed nonzero-B verification as an invalid result, and retains failed/incomplete attempts in the panel totals. The same active Conductor task T-10240 continues to own the experiment directory.

Implemented explicit `zero_b`, `nonzero_b`, `nonzero_b_verified` and `nonzero_b_verification_failed` counts. A failed nonzero-B scalar check changes the run status to `verification_failed`. Panel summaries now retain nonzero-B run counts, unknown counts for runs without witnesses, verifier-failure counts and candidate counts. The new accounting module is included in startup and panel source hashes. Next run the correctness tests and inspect the outcomes; no comparative timing claim is intended.

The full local correctness suite passed: 8 tests in 50.984 seconds. It includes a non-vacuous successful relation control, independent affine replay, paired-target checks, timeout and solver-failure checkpoints, and an explicit count-classification test covering a zero-B witness, a verified nonzero-B witness, and a failed nonzero-B verification. These checks establish accounting behavior, not a comparative benchmark. The objective remains open: solver-native degree/work metrics and a matched end-to-end reference remain absent.

## 2026-11-23 — coverage-aware panel accounting

The panel records whole-process wall cost across attempts, but phase costs and acquisition counts are scattered in result files. Add per-mode sums of observed generation, construction, solver, root and verification times and candidate/rejection counts, including snapshots from interrupted attempts when valid. Each aggregate must state its observed and unknown attempt counts; partial sums are not complete costs. Validate the summary independently with fixed records and rerun functional checks.

The focused coverage, paired-target and timeout tests passed (3 tests, 10.264 seconds), followed by the strengthened timeout test (1 test, 0.092 seconds). The synthetic control confirms that two observed generation times and one unknown produce an observed sum of 8 and unknown count 1, rather than an invented third timing. The panel uses completed results or valid last checkpoints and records the observation source; whole-process wall remains separately charged for every attempt. These are accounting checks, not performance results.

## 2026-11-23 — structured polynomial input

The Sage output currently serializes the exact polynomials only as human-readable strings. Add an ordered sparse coefficient/exponent representation and exact term counts, bound to the same system and checkpoint. This makes independent reconstruction and eventual instrumentation of a backend possible; it does not supply missing solving-degree, regularity, or field-operation observations. Validate the representation by evaluating it independently against the curve's S3 formula and block vanishing products.

The structured-polynomial and injected-failure checks passed (2 tests, 11.371 seconds). The positive control independently evaluated the S3 equation and both vanishing products at three input pairs using the structured coefficients, and matched each value modulo the field prime. A remaining correctness guard is to reject non-prime characteristics and primes at most 3, outside the short-Weierstrass S3 formula's stated domain.

The prime-characteristic guard and solver-failure checkpoint checks passed (2 tests, 8.723 seconds). An invalid characteristic exits before curve setup, with a setup checkpoint and no result file. The checkpoint also retains the new structured terms after polynomial construction. Run the full test suite next.

The full local correctness suite passed: 10 tests in 40.507 seconds. The structured polynomial values were independently checked at three points, the positive signed-relation control remains valid, and an unsupported field was rejected before system construction. This validates representation and domain guards; it is not backend telemetry or comparative performance evidence.

## 2026-11-23 — explicit backend and raw solver protocol

Sage's `groebner_basis` documentation lists `libsingular:std` and a `prot=True` option that prints Singular's computation protocol. A bounded test of a two-variable polynomial ideal confirmed that this local Sage installation emits a raw protocol. Add an explicitly identified Singular backend and an opt-in protocol flag, preserving the protocol in the panel's raw stdout and recording stdout/stderr hashes. Do not parse protocol digits into solving degree or regularity. The instrumentation affects overhead and the selected backend changes the workload compared with earlier default-autoselection runs.

The focused protocol, checkpoint and positive-relation checks passed (3 tests, 5.695 seconds). The protocol flag emitted Singular's product-criterion line into stdout, the recorded backend was `sage/libsingular:std`, and the run retained verified useful witnesses. Solving degree remains null. Run the full suite before publication.

The complete local correctness suite passed (11 tests, 29.715 seconds) with the explicit Singular backend. Protocol output remains a raw backend observation, with no automatic degree or operation-count claim. Sage's previous autoselection was not pinned; earlier timing data must not be combined as if it had the same identified backend. Solver-native degree/operation telemetry and the requested matched end-to-end comparison remain open.

## 2026-11-23 — trace attribution

Reviewed the Singular 4.4.1 option manual, https://www.singular.uni-kl.de/Manual/latest/sing_324.htm, and the Sage libSingular option documentation, https://sagemath.gitlab.io/documentation/html/en/reference/libs/sage/libs/singular/option.html. The `std` protocol's `[m:n]` is a representation change, while a separate bare integer indicates a current leading-term degree; the meaning of degree depends on command and ordering. Neither is automatically a certified solving degree or regularity. Existing stdout can also include root-extraction output. Add flushed begin/end markers around the actual basis call, retaining raw stdout and treating missing end markers as incomplete; validate marker placement on a fixed correctness control.

Focused protocol and checkpoint checks passed (2 tests, 6.952 seconds): the raw Singular product-criterion text was between exactly one begin and one end marker. Extended the injected-failure check to assert markers are also closed on a Python exception; a killed process may still lack the end marker.

The strengthened protocol and injected-exception checks passed (2 tests, 3.881 seconds); the end marker denotes that the call returned or raised, not that it succeeded. Begin the complete functional suite.

The complete correctness suite passed (11 tests, 29.193 seconds). Documentation now identifies the `std` protocol symbols and ordering caveat using the Singular manual and Sage libSingular documentation. This work establishes trace attribution, not a solving-degree, regularity, or operation-count measurement; those and the matched cost comparison remain open.

## 2026-11-23 — point-generation operation counts

Add explicit counters around point-addition and scalar-multiplication API calls made in the generation phase, including repeated target draws, short-walk setup, and two-group precomputation. Counters describe high-level calls only; they cannot be interpreted as field operations, scalar-multiplication internals, or work outside generation. Test each strategy against independent count formulas and retain incomplete coverage in panel summaries.

The three focused tests passed (20.405 seconds): all three generation strategies matched independently derived API-call count formulas, the paired panel retained the counters, and the one-candidate cap was charged. The counters include target draws and generation setup, and exclude subgroup validation, scalar-multiplication internals, polynomial solving and relation verification. Run the full suite next.

The complete local correctness suite passed (11 tests, 32.786 seconds). The new counters provide exact instrumented API-call counts for the generation phase, not field-level operations or whole-solver work. Full solver telemetry and the matched complete-cost comparison are still outstanding.

## 2026-11-23 — saved-panel comparison

One panel accepts only one filter configuration. Add a read-only comparison of saved panel summaries that verifies plans, source identity, seeds, modes and target pairing before displaying paired outcome and whole-process wall data. Equal timeout budgets are not equal realized costs; platform strings cannot establish identical hardware or load. Missing attempts and failed verification must remain explicit. This comparison does not run a solver or establish a rho result.

The saved-panel comparator and tests are in place. Four synthetic tests passed (0.015 seconds), covering paired targets, missing/incomplete outcomes, changed artifacts, duplicate arguments, and a receipt relabelled inconsistently with its hashed result. A real local correctness panel passed comparison with itself (1 test, 6.558 seconds). This confirms the comparator reads produced artifacts; it does not establish a filtered performance advantage or equal realized budgets. Resource envelopes and the single-target reference remain absent.

An additional adversarial fixture showed that matching targets across two panels is insufficient if their modes disagree within each panel. The comparator now rejects that case; all five comparator tests passed (0.020 seconds).

## 2026-11-23 — distinct-filter integration control

The saved-panel comparator has been tested with synthetic receipts and with one real panel compared to itself, but not yet against two actual distinct filter configurations. On the fixed public 101-element validation group, run one seed per generator with the same target and block size under an unrestricted x range and a narrower interval. This is a functionality control, not a performance campaign; do not interpret the observed timings or success frequency as a comparison result. The repository's EKS skill file remains absent at both configured host skill locations, so a benchmark campaign is not launched here.

The distinct-filter integration control passed (1 test, 14.500 seconds): both real panels completed on the fixed validation group; the comparator confirmed target pairing across all three generation methods, and the narrower interval rejected at least one candidate. Its shared-host timings are not performance evidence. Add per-mode outcome and observed cost summaries to the comparison so the acquisition work and unknown coverage are visible without manual receipt aggregation.

After adding coverage-aware per-method outcome/cost summaries and checking candidate counts against their hashed result or checkpoint, the distinct-filter integration control passed again (1 test, 18.026 seconds). Six comparator tests then passed (0.046 seconds), including a checkpoint-count mismatch. No timing comparison is asserted from these functionality runs. Backend solving degree/field operations, matched resource conditions and the single-target reference remain open.

## 2026-11-23 — resource and memory observations

The panel's platform string does not identify CPU model, affinity, inherited limits or thread settings; successful driver outputs contain CPU time and process-lifetime peak RSS but the panel summary omits them. Record bounded host/parent metadata and aggregate those existing child observations with coverage. Do not equate matching metadata with the same machine, equal load, isolation or resource envelopes. Interrupted child CPU/RSS remain unknown unless actually recorded.

The focused comparator suite passed (8 tests, 0.032 seconds); resource metadata is now classified as unavailable, incomplete, different, or metadata-match-only, and a metadata match is not a hardware match. Paired panel tests passed (3 tests, 19.433 seconds) before tightening the incomplete-metadata check. Re-run the paired tests under the tightened check and verify CPU/RSS coverage and timeout behavior.

The tightened resource-metadata check passed the three real-panel controls (3 tests, 16.517 seconds), including a same-panel comparison, a distinct-filter comparison, and forced timeouts. Completed children exposed CPU and process-lifetime RSS observations; timed-out children correctly remained unknown. Resource metadata equality is labeled metadata-match-only and does not establish matching hardware or workload isolation. The full single-target reference and solver-native operation/degree metrics remain open.

## 2026-11-23 — bind results to the frozen plan

A read-only merge preview of PR #5105 against the current local origin/main found a conflict in `docs/ACTIVE.md`. Separately, the panel comparator checks cross-panel plan equality and receipt/result fields, but it does not verify that the result's curve, target point, block sizes, generation controls and filter match its own frozen plan. A mislabeled result could therefore pass pairwise checks. Add a strict reconstruction of expected driver parameters and reject mismatched result/checkpoint configurations, with a regression that changes a recorded parameter while recomputing the artifact and receipt hashes. Equal resource metadata still does not prove equal isolation or total cost.

The comparator's expanded 10-test suite passed (0.048 seconds), including rehashed result and checkpoint configuration tampering, a timed-out receipt falsely claiming a relation, and source-digest consistency checks. Run real-panel and timeout integration controls against the stricter plan binding before publishing.

The stricter comparator passed real distinct-filter, same-panel and forced-timeout controls (3 tests, 19.004 seconds). The tests verify data binding and error handling, not an equal-cost or single-target speed claim. A local merge preview still identified the shared `docs/ACTIVE.md` conflict; it was not resolved during this reserved experiment-directory task.

## 2026-11-23 — independent saved-record audit

After the user reactivated the goal, reserved Conductor T-10522 for a distinct independent replay task. Add a standard-library verifier of saved point and relation records, including full affine arithmetic, sign-aware witness completeness for the reported roots, exact coefficient identities and exact structured input polynomials. For a bounded independent primality check, initially accept field and subgroup primes below 2^32. The audit must state that it does not establish that the solver returned every root, regenerate the seed stream, authenticate the record, or verify timing and cost observations. Preserve the existing measurement and failed-task history.

The focused independent replay controls passed twice (3 tests, 14.282 and 14.381 seconds). The audit verified the positive signed-relation control and all three generators, rejected modified candidate coefficients, polynomial coefficients, missing witnesses and scalar values, and recognized capped acquisition as a partial audit. The CLI additionally binds its receipt to the input SHA-256 and reports its own explicitly scoped wall/CPU interval. It does not search for a scalar or certify root completeness. Next run the full measurement and comparator suites.

The full local suites passed with the independent auditor (12 measurement tests in 55.401 seconds, 10 comparison tests in 0.051 seconds). Address the previously identified supervisor-memory risk next: stream child stdout/stderr directly to per-attempt files and hash incrementally rather than accumulating logs with `communicate()` and `read_bytes()`. Retain timeout and launch-failure receipts and do not claim that child peak RSS includes supervisor memory or output-storage cost.

The log-streaming changes passed three focused checks (9.071 seconds), including a test child producing 2 MiB on each stream per attempt, forced timeouts, and a completed paired panel. The comparator suite passed (10 tests, 0.046 seconds) after switching its artifact hash to bounded streaming. Raw protocol disk usage remains unbounded and is not part of child peak RSS; this is documented. Run final suites on the combined source.

Combined-source validation passed: 13 measurement tests in 44.519 seconds and 10 comparator tests in 0.056 seconds. Two additional focused tests passed in 5.004 seconds for explicit launch-failure receipts and invalid audit CLI exit/receipt behavior. The independent auditor is limited to field and subgroup primes below 2^32 and does not establish root completeness or validate cost; its measured interval excludes startup and receipt writing. Log streaming removes supervisor in-memory pipe capture, while disk growth and supervisor cost remain outside child RSS. Solver-native degree/work metrics and the matched full-cost reference remain open.

## 2026-11-23 — bind independent audits to panel runs

The standalone auditor is not yet connected to panel receipts; a driver-verified scalar could be counted without the independent check, and independent-audit time is outside the panel. For completed bounded-field records, run the verifier as a separately timed subprocess and preserve its result, stdout/stderr hashes, exit/timeout status, and source digest. Keep the audit cost separate from solver time. When field or subgroup bounds exceed the verifier's 32-bit support, report unsupported instead of treating it as a verified result. Tests will distinguish independently verified, skipped/unsupported, and interrupted attempts. This does not make root completeness or equal-resource timing established.

The integrated audit path passed the comparator suite (11 tests, 0.127 seconds) and focused invalid/unsupported, completed-pair and timeout controls (3 tests, 15.997 seconds). A completed bounded-field run is counted as independently verified only when its separately timed audit returns a matching verified receipt and source hash. Comparison refuses matching checks when any independent audit is not verified. Audit time is separate from solver time; child-plus-audit wall excludes supervisor work and is not a full single-target cost. Run the combined suite next.

The combined measurement and comparator suites passed (15 tests in 73.702 seconds; 11 tests in 0.088 seconds). A further receipt-integrity check now rejects negative audit durations, missing audit log hashes, impossible audit states and fabricated audit execution; its expanded comparator suite passed (12 tests in 0.103 seconds). Recheck real completed and timed-out panel paths after that validation tightening.

Final focused integration controls passed (4 tests, 20.178 seconds) after receipt-state validation, covering real matching panels, distinct filters, driver timeouts and invalid/unsupported audit outcomes. The combined source previously passed 15 measurement and 11 comparator tests; the tightened comparator then passed 12 tests. The auditor remains limited to p,n<2^32 and verifies reported witnesses rather than root completeness. The measured child-plus-audit sum excludes supervisor work, other target-dependent phases and any retries outside the panel; it is not a full single-target comparison.

A final comparator test confirms that an unsupported audit remains unverified, makes matching checks fail, and records audit time as unknown; all 13 comparator tests passed (0.112 seconds). This is in addition to the 15-test measurement suite and the four focused integration controls recorded above.

## 2026-11-23 — top-form degree of regularity

Sources checked: KB revision 752f945c662745959e4516205d027b5e, source:295bb95fecaf5781cb631fd98be85a43, `crypto:research/notes/ecc2k130/RESEARCH_ISOGENY_DEGREE_SEARCH.md` at d390e84; and Bigdeli et al., https://eprint.iacr.org/2020/1375.pdf, Definition 2.1. These distinguish the invariant of the generators' highest homogeneous forms from a solver's solving degree. For this two-variable presentation, the candidate top ideal is (x^2 y^2, x^b, y^b), where b is the block size. Its quotient has first zero degree 1 for b=1 and b+1 for b>=2. Validate the three top forms before reporting it, label the definition, and independently check the formula by exact homogeneous matrix ranks on fixed public systems before treating the diagnostic as implemented. This quantity is not a solver operation count or a timing prediction.

The focused diagnostic, independent matrix-rank and arithmetic replay checks passed (3 tests, 17.870 seconds). Homogeneous matrices over F_101 agreed with the first-zero formula for block sizes 1 through 8; a deliberately altered leading form returned unsupported instead of a value. The standalone independent auditor checks the reported value by a separate standard-monomial enumeration and rejects a modified regularity metric. Run integrated panel and complete correctness checks before publication.

The complete combined suite passed before exposing the metric through panel receipts (16 measurement tests in 43.122 seconds and 13 comparison tests in 0.114 seconds). Focused checks after adding receipt binding and independent evidence validation passed (3 tests in 12.453 seconds, then 2 tests in 11.075 seconds). The model reports the top-form first-zero degree separately from solver-native quantities; no solver operation count or comparative performance claim is inferred.

The distinct-filter and timeout integration checks passed (2 tests, 12.110 seconds), and the comparator suite passed again (13 tests, 0.104 seconds). Both interval settings produced top-form degree 5 for block size 4 in this correctness control, as predicted by the input-form calculation; this does not measure an affine solving degree. The cited paper's author list was checked directly before finalizing the note.

## 2026-11-23 — bounded Singular protocol summary

The Singular `std` progress stream includes a line labeled `product criterion` and `chain criterion` on this installation. Add strict extraction of those two explicitly labeled backend-reported counters only when the requested basis trace is bounded by exactly one begin/end marker and contains exactly one matching summary line. Do not interpret protocol integers as solving degree, field-operation counts, or a proof of completion. Preserve unsupported, missing, ambiguous and interrupted trace states. The Singular manual documents the protocol as progress information: https://www.singular.uni-kl.de/Manual/latest/sing_324.htm.

Focused protocol controls passed: 3 tests in 11.279 seconds covered an actual paired panel with protocol enabled, a direct Singular protocol, bounded long-line handling and incomplete/ambiguous traces; 14 comparator tests passed in 0.117 seconds, including a forged protocol observation rejected against raw stdout. Criterion values are backend-reported labels, not field arithmetic counts. Run the complete suite next.

The complete local suite passed (17 measurement tests in 41.620 seconds and 14 comparator tests in 0.107 seconds). To inspect the semantics of the new counters on an actual solver trace, rerun the already fixed positive control (p=101, seed 42, block size 10, independent generator) once with protocol enabled. Record its criterion labels and regularity value as a correctness observation only; do not use the shared-host timings as benchmark evidence.

The fixed positive control emitted product-criterion count 17 and chain-criterion count 82, with top-form degree 11 and three nonzero-B witnesses. The independent audit returned `verified`; driver/accounting/auditor source hashes matched this tree. The raw result, audit and streams are retained under `evidence/protocol_control_20261123/` with result SHA-256 `d89a203542e81e099d19233ee246e90787133d031eee1557d767fe09387ae243`. This is a correctness observation, not benchmark evidence or a field-operation count.

## 2026-11-23 — benchmark admission recheck

Correction to the earlier missing-skill note: the `isolab-eks` skill has moved to the workspace trunk and is available at `/Volumes/SSD990-2/workspace/.agents/skills/isolab-eks/SKILL.md`. Read its submission and recovery references and the crypto EKS harness/cost controls. The current AWS caller is not in the skill's specified account; three configured profiles were checked, none returned that account, and one profile check failed. The configured Kubernetes context could not list nodes. No credential or account identifier was copied into an artifact. The current GitHub workflow supports a source-pinned ecbench spec and selected frozen command cases, but inspection found no general Sage-panel command input; its OIDC route has not been validated for this workload. The available budget remainder, admitted image, matched rho arm and L2/A-A evidence are not established. No cloud job, build, or benchmark was launched. A successful STS call alone is not submission authorization.

Current upstream main was fetched and a read-only merge preview at `9ef955b17dc3b0eb1d1ac49985cf1cdb0dbf4598` still showed a conflict in `docs/ACTIVE.md`. That tracking conflict was not modified by this preflight. The next admitted measurement needs the correct account/authorized route and capacity/budget readback, an image containing the exact tested workload, and a matched reference; an unrelated default fixture would not satisfy this request.
