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
