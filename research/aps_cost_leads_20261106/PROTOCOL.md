# APS cost panel: design continuation, not a run admission

Status: **draft, not frozen, no benchmark launched**. This preserves the
[original four-size request](../aps_measurement_20261029/PROTOCOL_DRAFT.md)
and adds the four implemented lead controls. The original request remains
open until the checkpoint, pilot, frozen protocol and measured comparisons
exist. Development fixtures and test durations are excluded from its data.

## Questions and units

For each fixed public target, does a single change reduce the cost of a
bounded cold solve, including unsuccessful attempts and final scalar
verification? Does the result persist across four supported sizes? These
are comparative questions; this document contains no empirical answer.

The new instruction path measures **Callgrind Ir in the method invocation
plus a separate ordinary-residue scalar-check body**, using the same frames
for every arm. It includes method setup, polynomial construction, allocation,
map/hash work, trace learning, rejected replays, numerical certification,
fallback, roots, sign search, recovery, and method report aggregation.
Workload generation/loading, executable startup, record sealing/serialization,
post-run complete-set audit, and external file I/O are outside these frames.
They need separate reported observations; this is not a whole-process cost.
`method_online_ir` is a nested diagnostic, never the cold denominator.

Ir is a simulated user-space instruction unit, not wall time, a hardware
retired-instruction count, GAE, S, bit complexity or an execution proof.
GAE accounting remains incomplete. Keep every native unpriced counter and
the 0/0.1/1 GAE band on every arm when eligible; missing work prohibits a
complete-S ranking. Do not invent prices from bytes or trace storage words.

Wall time is a separate secondary metric, conditional on L2 isolation,
passing A/A admission, no restore and the prescribed matched execution.
Never time under Valgrind for a wall claim. A separately sealed profiled
record must match the unprofiled record's entire mathematical trajectory
and counters. If slower execution changes a cooperative deadline, retain
both records and mark Ir missing for that pair; never reseal it as a match.

## Populations and pilot

Keep four field-size classes inside the supported `p <= 65521` cap.
The maintained `prime_search` constructor accepts at least eight bits, so
the proposed classes are **8, 10, 12 and 14 bits**. Before main-data freeze,
generate and review exact explicit curve, prime subgroup, generator and
target identities, independently validate them and register the curves.
Draw curves independently of arm; do not select them for observed solver
success. Cubic-orbit comparisons use a separately declared shape stratum,
with the same exact curve and certified beta/lambda in both arms.

Pilot workloads must be disjoint from the held-out main workloads. After
the successful source checkpoint and pilot admission, time one call per
arm per size, unprofiled and profiled, retaining every outcome. The pilot
must establish the executable/image, fixed and per-call cost, memory cap,
oracle-work bound, profile size, trace acceptance/refusal rates and whether
all arms can finish within identical budgets. Choose the finite main budget
from that receipt. No numerical runtime estimate is supplied before a pilot.

The main design retains at least eight independent targets per curve, five
measured rounds and one warmup, interleaved arm order and paired seeds.
These are minimum population requirements, not a power calculation.
Before freezing, justify power for the declared effect threshold and test,
freeze native analysis code and its digest, and mask arm labels until that
analysis has run. A changed sample size or budget is a versioned amendment
filed before the affected data, never a silent replacement.

## Arms and one-factor contrasts

All arms use the same source-built binary, single thread, compiler, allocator,
curve/target population, cooperative and process budgets, root policy,
stopping policy and label law unless the contrast explicitly varies one.

| Contrast | Baseline | Candidate | Fixed requirements |
| --- | --- | --- | --- |
| Registry continuity | `ic.aps_v2` | v3, all leads off | Identical six-encoding parameters and literal equations; classify any transcript-only overhead separately |
| F4 trace family | v3 trace off | v3 trace certified | Same cell order and seeded bases; cold learning and all refusals/fallback charged; no cross-target warm cache |
| Line chart | original three-variable line divisor | two-variable nonhorizontal chart plus horizontal cell | Same full labeled base; same global-sign convention; prune only on the existing certificate |
| Unequal profile | equal partition sizes | unequal sizes of the same total | Same sampling law and total population; freeze each profile; measure relation yield as well as cost |
| Elimination choice | no guessed coordinate | explicit guessed coordinate 0, 1 or 2 | Same unequal profile, coordinate equations and deadline; this is distinct from the profile contrast |
| Retained growth | `growth=retained_full` | `growth=frontier` | Same initial base and nested draw law; retain all old setup and failed work |

The growth implementation exposes `growth=none|retained_full|frontier`; a panel comparing
independent fresh bases against retained nested bases changes the population
law too and must be labeled factorial/confounded, not a one-factor speedup [pcc:unverified].
Both retained modes use the same seeded sampling and ceiling policy. The
independent reader checks the full product in `retained_full` and permits
old-region subtraction only for certified `frontier` attempts.

Also retain the original exhaustive/first-informative, legacy/exact,
unordered/line and certified orbit/pruning comparisons. Use the strongest
applicable matched rho and available same-input external solver as reference.
Name any absent licensed solver rather than substituting an oracle answer.

## Controls and variable ledger

* A/A: identical arms and seeds test deterministic count/profile agreement;
  distinct preregistered nuisance seeds additionally measure noise. Label
  these purposes separately.
* Positive: the public informative horizontal relation must be found only
  when its required cell is retained; an admitted cached specialization must
  agree with a cold basis; deliberately expensive setup must increase the
  inclusive cost. The last control still needs an actual profiled run.
* Negative: independently labeled cubic orbits cannot authorize pruning;
  a poisoned cached trace must fall back; an uninformative B=0 relation cannot
  recover a logarithm; a partial old traversal cannot authorize a frontier.
* Fix source, binary, flags, thread count, allocator and input law. Randomize
  order; cross profiles/elimination choices explicitly. Measure host isolation,
  placement, memory pressure and interruptions without publishing denied host
  capsules. A/A and admission must refer to the actual authorized execution.

Report cold setup and online phases, matrix dimensions, solving degree,
trace learning/replay/refusal/certification, roots and informative relation
yield. Retain pre-transcript sampling failures, capped partial draws, timeouts,
late witnesses, crashes and recordless infrastructure events. A missing count
is missing, not zero. Resume currently makes inclusive profile qualification
ineligible until lost-work accounting is implemented; do not discard the row.

## Analysis and decision

Pair by exact workload and round, not a bit-size alias. Report per-size
totals ratios with workload-cluster uncertainty, success fraction and the
complete flow population. No successful-only mean, pooled exponent,
subexponential claim or extrapolation beyond this four-size toy range.
An incomplete outcome or absent required profile blocks a speed ranking;
it does not disappear from the export. Freeze the effect threshold,
uncertainty method, resampling rule, multiplicity correction and fixed-N stop
rule after the disjoint pilot and before main data. An unpowered null is
inconclusive. A failed positive control stops the affected experiment.

## Admission checklist

1. Successful approved source/worktree checkpoint, under fenced resume
   ownership and a fresh payload/baseline review. Approval already exists;
   secure upload authentication is missing [pcc:unverified]. No credentials in chat.
2. Source-built tested/pushed image; exact registered ladder and independent
   oracle budget; complete protocol-design and metric audit. This native
   exporter's schema/hash checks alone do not admit a design or execution.
3. Reviewed, schema-valid pilot protocol filed before its data; finite
   budget, source checkpoint and maintained EKS admission. No local fallback.
4. Pilot receipt, fixed native analysis, full-run population/power/budget,
   frozen `experiment.protocol/v1`, plan pin and exact seeds before main data.
5. L2/A-A, source/S3, replay/export and required interruption/recovery checks.
   Every denial on private ledger, shared full logs and host/build details
   remains in force. Do not package them in the source checkpoint.

No protocol freeze or experimental admission is claimed here. T9/complete-cost
research and T10/authblocked checkpoint stay open.
