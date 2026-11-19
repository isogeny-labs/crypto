# Workstream 4: source-bound GHS rho composition protocol

**State:** protocol and deterministic gate frozen before any admitted timing.
The executable source revision, binary digest, EKS request, and receipt paths
are deliberately unset until the production parallel-rho integration is
committed and pushed. A missing execution binding blocks launch; it is not a
wildcard. No result or speedup is claimed `[pcc:unverified]`.

## Question and evidence boundary

This workstream asks which already implemented composition has the lowest
complete cost on the same restricted GHS-weak rho problem:

1. the unchanged six-product baseline;
2. R17 five-evaluation cubic multiplication;
3. R36 setup joint-table reuse; and
4. R17+R36 together, with R64's counted, native-diagnostic, and fully bare
   authority modes used for the corresponding measurement roles.

The deterministic gate establishes only source/binary identity, arm closure,
known-answer recovery, trajectory parity, counter authority, and composition
identities. Wall time becomes evidence only in an admitted IsoLab L2 run with
the frozen interleaving and A/A control below. Incidental local `wall_s` and
calibration fields are retained but excluded.

## Sources checked

The shared research-KB wrapper returned `no executable found`. The documented
fallback review used:

- `research/quartic_ghs_r0_r99_20261119/REPORT.md` and `PROTOCOL.md`;
- `R17_PRODUCTION.md`, `R36_PRODUCTION.md`, and
  `ranges/R64_PRODUCTION.md` in that study;
- `receipts/commit_bound_manifest.json` and its strict native validator;
- C0, M1, and M2 of
  `research/quartic_ghs_strong_rho_parallel_20261118/IMPLEMENTATION_SPEC.md`;
- `research/ecbench_e2e_frontier_20261031/LITERATURE.md` for the matched-rho,
  symmetry, and complete-accounting requirements.

The review fixes composition order as R17 arithmetic, then R36 setup reuse,
then R64 counter removal. Standalone deltas are never added to predict the
composed delta.

## Exact arm closure

[`arms.json`](arms.json) is the machine-readable closure. At primary width 8
the complete deterministic panel contains fourteen unique arms:

| mechanism | counted authority | native-only diagnostic | bare wall arm |
|:--|:--|:--|:--|
| baseline | `strong-b8` (plus alias control `strong-b8-mul6`) | `fast-b8` | `bare-b8` |
| R17 | `strong-b8-mul5` | `fast-b8-mul5` | `bare-b8-mul5` |
| R36 | `strong-b8-walker-table` (plus alias control `strong-b8-mul6-walker-table`) | `fast-b8-walker-table` | `bare-b8-walker-table` |
| R17+R36 | `strong-b8-mul5-walker-table` | `fast-b8-mul5-walker-table` | `bare-b8-mul5-walker-table` |

The same four mechanisms and three authority modes run at widths 1, 4, 8,
16, and 32 in the pre-timing correctness screen. The explicit `mul6` aliases
are counted controls only. They must be identical to their unsuffixed
counterparts apart from arm spelling and excluded clocks.

## Deterministic admission gate

For every q, instance seed, target, and width, the native validator requires:

- exact 40-hex source revision and the SHA-256 of the binary bytes supplied to
  the validator;
- one instance record, one calibration record, and exactly the expected arm
  census with no duplicate JSON keys or duplicate arms;
- `ok=true`, `k_found=k`, a common subgroup order, and equal
  `(steps,dps,restarts,lookahead,cycles,cycle_adds)` across all arms;
- `strong-*` field/native authority true, `fast-*` field authority false and
  native authority true, and `bare-*` both false;
- all field counters zero for `fast-*` and `bare-*`, all native counters zero
  for `bare-*`, and positive counted/native controls;
- `evaluation5` only for mul5 on the admitted q domain and `karatsuba6` for
  the baseline and explicit mul6 controls;
- the exact setup-mode label implied by the arm;
- for each phase, mul5 replaces one `fq_mul` per `f3_mul` and emits four
  `fq_small_div` events per `f3_mul`;
- R36 leaves walk and verification counters unchanged relative to the same
  field kernel; and
- `fast-*` native counts match the corresponding counted mechanism.

The tool's corruption matrix must reject a wrong source, wrong binary digest,
unknown arm, changed trajectory, and changed authority bit before any timing
request can be generated.

## Development and holdout identities

The three fixed sizes are:

| region | q | development seeds | holdout seeds | development policy | holdout policy |
|:--|--:|:--|:--|:--|:--|
| low | 4,051 | 101, 102, 103 | 1, 2, 3 | `sequence`, index 0 | `ic-repeat`, index 0 |
| middle | 16,363 | 101, 102, 103 | 1, 2, 3 | `sequence`, index 0 | `ic-repeat`, index 0 |
| high | 65,479 | 101, 102, 103 | 1, 2, 3 | `sequence`, index 0 | `ic-repeat`, index 0 |

The reserved matched-grid holdout uses `ic-repeat` so the candidate is tested
on the exact frozen IC target law. Development and holdout instances are
disjoint. Width 8 and 32 partitions are
fixed for this composition question; retuning width or partition count belongs
to workstream 3 and cannot be selected from these results.

Each cell first runs the six counted authority arms in one correctness/count
invocation. It then runs the four `fast-*` arms once for native-diagnostic
parity. Neither invocation supplies a wall sample.

## Frozen wall schedule

Each candidate is paired separately with `bare-b8`. `A0` and `A1` are
byte-identical baseline invocations; `B0` and `B1` are byte-identical candidate
invocations. Five blocks are fixed before data:

```text
block 0: A0 A1 B0 B1
block 1: B1 B0 A1 A0
block 2: A1 B1 A0 B0
block 3: B0 A0 B1 A1
block 4: A0 B0 A1 B1
```

The three candidate pairs are R17, R36, and R17+R36. This is 60 wall
invocations per `(q,seed)` and 540 in development; holdout repeats exactly the
same schedule only after one candidate is prospectively selected from all
development cells. No post-result sample extension, arm deletion, or
candidate-specific seed choice is allowed.

The A/A admission interval is the totals ratio over all paired A0/A1 records,
with a 95% paired interval containing 1 and wholly inside `[0.9,1.1]`. Each
wall record must be L2, include one warmup outside the measured five blocks,
and retain failures, timeouts, CPU time, peak RSS, affinity/NUMA readback, and
the complete phase interval. A failed A/A gate leaves wall ratios descriptive.

## Reported quantities and decision rule

Per `(q,seed,arm)` report the raw setup/walk/verify field vector, native vector,
steps, `steps/sqrt(ell)`, DPs, retries, cycles, wall and CPU intervals, peak
RSS, and correctness. For counted arms report complete M-eq at the frozen
calibration and sensitivity weights for exact small divisions and native
operations; do not silently price `fq_small_div` at zero. For wall pairs report
the totals ratio `sum(wall_B)/sum(wall_A)` with a paired 95% interval and the
A/A noise interval.

A candidate is eligible for holdout only if every development record verifies,
all required counters are present, counted ordering is reported across the
complete sensitivity band, and admitted wall B/A is below one with its 95%
interval below one at all three sizes. The holdout promotes it only if the same
conditions hold on all nine disjoint holdout instances. A component win is not
an ECDLP-frontier move; matched GHS IC/rho and publication gates are separate
workstreams `[pcc:unverified]`.

## Execution binding and stop rules

Before launch, create a reviewed binding with the pushed 40-hex commit, source
closure hashes, compiler, 64-hex binary SHA-256, OCI digest, EKS request hash,
finite cost ceiling, and receipt URI. The native validator receives the actual
binary path and hashes its bytes. Any mismatch, dirty/unpushed input,
checkpoint/budget/pool refusal, missing arm/schema field, invalid answer,
counter identity failure, replay mismatch, or L2/A/A failure stops promotion
and preserves the evidence. It never authorizes a smaller substitute.

