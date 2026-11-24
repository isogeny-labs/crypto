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
