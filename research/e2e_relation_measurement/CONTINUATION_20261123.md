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
