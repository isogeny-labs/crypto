# Phased ecbench protocol

Date: 2026-11-17  
State: preregistration template; no measurements are included.

## 1. Question and evidence boundary

The program asks whether any registered index-calculus candidate remains
correct, relation-productive, rank-productive, and operationally affordable as
the subgroup grows from approximately 20 to 80 bits. The source ladder spans
Koblitz, general-binary, and prime-field elliptic curves; independently gated
lanes cover algebraic decomposition, generic decomposition, direct
hyperelliptic/Jacobian work, and cover/transfer routes.

This protocol has two distinct decisions:

1. **Phase promotion:** the surviving experiment is sound and affordable enough
   to test at the next scale.
2. **Speedup claim:** complete cold end-to-end solve time is lower than the
   strongest applicable rho reference under every required amortization weight.
   [pcc:unverified]

The first decision never implies the second. Relation yield, projections,
operation counts, warm runs, shared preprocessing, and extrapolations cannot by
themselves establish an end-to-end speedup. [pcc:unverified]

## 2. Size, panel, and target model

Let `r` be the prime subgroup order. A phase size is
`b = ceil(log2(r))`. Binary extension degree `m` and prime-field bit length are
reported separately and are not substituted for `b`.

The exact cells and windows are in `panel.json`. Every workload is one public
target on one curve in a cold process. Phase 1–3 require at least 32 targets per
mandatory curve; Phase 4 requires at least 8 because each target is materially
more expensive. Targets, target derivation, and seeds are frozen before a run.
No target may be discarded after seeing a timeout, OOM, solver failure, or
unfavorable ratio.

Tiny curves are structural and engineering analogues, not security parameters.
The binary cells remain useful to exercise field structure and Koblitz
endomorphisms even though NIST SP 800-186 deprecates binary curves for new
applications.

### 2.1 Independently promoted lanes

`panel.json` defines four lane IDs. `elliptic_algebraic` is for
polynomial-system, SAT, Weil-descent, or other algebraic decomposition engines.
`elliptic_generic_decomposition` is for table, meet-in-the-middle, root-finding,
or subtractive decomposition engines. Their evidence is not pooled.
`hyperelliptic_jacobian` requires an exact curve, Jacobian subgroup and
same-Jacobian generic reference. `cover_transfer` is a typed route from a source
cell through a cover or other transfer, a destination solver, and recovery of
the source scalar.

Promotion is per `(phase, lane)`. A blocked higher-genus or transfer lane stays
visible in the dossier but neither blocks an independently ready elliptic lane
nor inherits that lane's result. Conversely, an elliptic result cannot promote
a higher-genus or transfer lane.

The transfer lane additionally requires route existence, explicit construction,
map correctness, subgroup preservation, destination-solver applicability, source
scalar recovery, and complete cost. Its comparison has three arms on the same
order-`r` workload: the direct source reference, a destination generic control,
and the destination candidate including transfer and recovery. Candidate cost is
the sum of the nine exclusive phases `route_search`, `construction`,
`forward_map`, `solver_setup`, `relation_or_walk`, `linear_algebra`,
`individual_log_or_descent`, `source_recovery`, and `verification`, following
[`docs/transfer-assessment/MEASUREMENT.md`](../../docs/transfer-assessment/MEASUREMENT.md).

### 2.2 Phase 1 executable-adapter contract

The first higher-genus implementation slice is frozen before adapter changes.
It does not authorize a benchmark run and does not turn an earlier smoke result
into a completed phase result. [pcc:unverified]

| Requirement | Exact Phase 1 object or route | Implementation evidence required before admission |
|---|---|---|
| Direct hyperelliptic/Jacobian cell | `jcv1-g2-fp10-4c1c114a`, `J: y^2 = 706 + 731x + 331x^2 + 360x^3 + 270x^4 + x^5` over `F_769`, `#J=571017=3*190339` | pinned explicit curve and generator, 32 public targets, `rho.negation` reference, `ic.hyperelliptic_v2` candidate, source-independent answer replay, cold wall and native work retained |
| Typed cover-transfer route | panel source `P1-K0-23`, `r=2095853`, through `C: v^2+u^2v=u^7+u` and the degree-three pullback | exact field/model binding, construction certificate, `gcd(3,r)=1` subgroup check, generic destination rho/BSGS controls, the transfer-aware characteristic-two genus-3 relation candidate, scalar replay on the source, and all nine exclusive cost phases |
| Evidence boundary | both paths | planning and smoke checks are implementation evidence only; Phase 1 remains `planned_unmeasured` until the frozen 32-target specs complete and pass the protocol gates |

The typed transfer assessment is
[`phase1-cover-transfer-assessment.json`](phase1-cover-transfer-assessment.json).
Its measurement and advantage fields remain unmeasured until a separately
authorized run produces replayable records.

Sources checked before selecting these paths: the executed Jacobian/transfer
study in `research/ecbench_jacobians_transfers_20261025/README.md`, the direct
hyperelliptic ecbench study in `research/hyperelliptic_ecbench_20261024/README.md`,
the checked ordinary-binary cover implementation and `docs/curves/COVERS.md`,
Milne's *Jacobian Varieties* notes cited by the cover report, and the frozen
transfer methodology in `docs/transfer-assessment/`. The shared research KB was
unavailable (`scripts/research-kb status`: no executable), so these pinned
repository sources supplied the review.

## 3. Phase admission gate G0

All items must pass before a phase consumes benchmark resources:

- The spec names exactly one lane. That `(phase, lane)` manifest entry is
  `ready_to_preregister`, has no blockers, and uses only objects admitted by its
  selector. Omission of `curve_policy.lane` is retained only for compatibility
  and means `elliptic_algebraic`.

- Every mandatory local curve has one pinned ICV1 slug, EC1 representation,
  curve UID, field polynomial or prime, model, subgroup, cofactor, generator,
  and proved order/subgroup status. [pcc:unverified]
- An external anchor has been imported into the local registry and traits
  catalogue with the same data before it becomes runnable. A public name alone
  is enough to plan a blocked cell, never enough to run it.
- `subgroup_bits` is within the phase window. Field size is reported separately.
- Each candidate and reference method passes ecbench applicability and `plan` on
  the exact curve spec. Plan refusals are evidence and remain in the dossier.
- The reference is the cheapest measured, correctness-verified rho arm available
  on that subgroup: signed Frobenius for Koblitz when applicable; negation plus
  any verified cubic/quartic automorphism option for the prime sentinels; and
  the strongest applicable generic rho for other cells.
- A `j=0`, `j=1728`, or Koblitz label is not treated as a speedup. The actual
  scalar action on the selected prime subgroup must be checked before an orbit
  or folding reference is admitted.
- Candidate IDs, method parameters, FB1/FBM1 and IC1 identities, seeds, `C`
  candidates, `L` looks, `E` reruns, per-probe row cap, target rank, episode
  count, timeouts, isolation level, and resource envelope are frozen.
- The phase owner records the off-panel reason for each nonstandard cell. P2’s
  degree-37 Koblitz curve (`icv1-f2m37-tm534059-32aad96b`) is explicitly a high-cofactor stress cell because no registered
  low-cofactor Koblitz curve lies in that subgroup-bit window.

If G0 fails, the phase is `blocked`; it is not run on a nearby unregistered or
easier substitute under the same cell name.

## 4. Empirical relation-yield and rank algorithm

Use the native relation-yield pruning v1 implementation and its frozen input
contract. For each `(curve, candidate, episode, look)`:

1. Start from the frozen seed and empty relation matrix. A probe is one
   candidate relation attempt, whether it hits or misses.
2. Record `attempted`. If a purported relation is malformed, outside the
   factor-base contract, or fails direct group verification, record the exact
   failure and immediately prune the candidate for correctness.
3. For a verified relation, increment `verified_relations`. Hash its canonical
   sparse row so duplicates remain visible. [pcc:unverified]
4. Reduce the row modulo `r` against the episode’s incremental basis. Increment
   `independent_rows` only when rank grows. Record rank, duplicate count,
   factor-base coverage, row weight, and collection/verification/reduction cost.
5. At every frozen look, emit cumulative and blockwise values for:

   - verified relation yield `verified_relations / attempted`; [pcc:unverified]
   - independent-rank yield `independent_rows / attempted`;
   - rank novelty `independent_rows / verified_relations`;
   - full-rank episode success;
   - trial, field-operation, group-operation, wall-time, and peak-memory costs.

6. Stop the episode at full target rank or the frozen probe/resource cap. A
   capped episode is a full-rank failure, not missing data.

Coverage, entropy, row weight, and cost are diagnostics. They can explain a
failure or guide the next separately preregistered candidate, but none is a
standalone promotion metric.

### 4.1 Candidate-level statistical gate G1

- Split the familywise error budget exactly as
  `alpha_cell = alpha / (4 * C * L * (E + 1))`, matching the existing v1
  protocol’s four guarded metric families.
- A Hoeffding episode-futility decision is allowed only when the frozen
  per-probe row cap is valid. Without that cap, the futility test fails open.
- After at least 16 independent terminal episodes, compute an exact one-sided
  Clopper–Pearson lower bound for full-rank success. It must be at least 0.80.
- The one-sided lower bounds for verified-relation and independent-rank yield
  must be nonzero and meet the frozen candidate-specific minimum implied by its
  target rank and trial budget. A default operational guard rejects a yield more
  than 2× below the preregistered model/incumbent, but the exact minimum is frozen
  before probes and may only be changed under a new candidate ID.
  [pcc:unverified]
- Structural impossibility, invalid relation, inconsistent replay, or solver
  wrong answer prunes immediately. Otherwise pruning requires the conservative
  v1 statistical decision. Ambiguous evidence survives rather than being
  rounded into a pass or failure.

Pruned candidates remain in all artifacts with their attempted probes and
failure reason. At least the unmodified incumbent/control continues far enough
to keep each mandatory family cell interpretable.

## 5. Correctness and custody gate G2

For every retained candidate and reference workload:

- ecbench verifies the reported discrete logarithm by `[d]P = Q`;
  [pcc:unverified]
- every relation used by the solver has passed direct group verification;
- all cold setup, factor-base construction, relation collection, filtering,
  matrix build, linear algebra, individual logarithm, verification, teardown,
  timeouts, OOMs, and retries are retained;
- no setup is shared in the headline one-target cost;
- the exact source commit, dirty state, toolchain, host fingerprint, isolation
  receipt, ecbench spec/plan/session IDs, seeds, target IDs, FB1/FBM1, IC1, and
  artifact hashes are recorded; and
- lower-bound or unpriced records may guide pruning but cannot pass a performance
  or promotion gate that requires complete costs.

Any wrong answer, invalid relation, hidden retry, lost failure, or custody break
fails G2.

## 6. Controls and replay gate G3

- An A/A duplicate arm is interleaved with balanced measured order. Its paired
  interval must include 1; otherwise noise or scheduling bias is unresolved.
- A positive control must show a known detectable change, and a negative/null
  control must remain null within its frozen margin.
- Every successful answer is replayed locally from the recorded spec and seeds.
- An independent host replays both candidate and rho arms and validates artifact
  hashes and point equations. Differences are reported, not averaged away.
- The amortization ledger is evaluated at weights `w = 0`, `0.1`, and `1`, plus
  a calibrated deployment weight when one exists. A rank reversal between
  weights is an unresolved outcome, not a single headline.

## 7. Resource and scaling gate G4

The phase preregistration states wall-time, peak-memory, storage, and retry caps.
After the phase, use observed p95 values including failures to form the next
phase envelope. Promotion requires either:

- the next envelope is no more than one quarter of the approved cap, giving 4×
  headroom; or
- the owner explicitly approves a revised cap before the next phase runs.

Model projections are labeled projections. They may size a reservation but are
not benchmark observations and do not fill a missing curve cell.

## 8. Phase/lane promotion gate G5

A lane advances to its next phase only when all mandatory objects for that lane
satisfy G0–G4 and:

- at least one non-pruned candidate per core family reaches the frozen target
  rank and full solver completion on the required public targets;
- there are zero unresolved invalid relations, wrong answers, or custody gaps;
- candidate and rho have complete comparable phase counters and cold times;
- every required local and independent replay passes; and
- the complete phase dossier, including negative results and blockages, is
  reviewed before the next phase candidates are frozen.

For the two elliptic lanes, Phase 1 additionally requires all four sentinels
because their purpose is to detect sign, cofactor, automorphism, and
ambient-field interactions. Later phases keep one mandatory Koblitz,
general-binary, and prime cell. A direct Jacobian lane instead requires its
phase-matched registered Jacobian cells. A transfer lane requires its typed
routes and all three comparison arms; checked map arithmetic alone is not a
phase completion.

If a core family or typed lane has no survivor, the program stops for that
family or lane at the current boundary. Work on a redesigned candidate starts
again at Phase 1 with a new ID; it does not inherit the failed candidate’s
evidence.

## 9. Separate speedup-claim gate G6

Passing G5 says only “continue the scaling study.” A speedup statement requires,
on the same current-head artifacts: [pcc:unverified]

- verified end-to-end solves for candidate and strongest applicable rho;
  [pcc:unverified]
- complete cold one-target costs and all required public targets;
- paired candidate/rho uncertainty whose upper confidence bound is below 1 at
  every required amortization weight;
- passed A/A, positive, negative, custody, and independent replay controls;
- no unpriced phase, shared setup, projection, or unsupported curve standing in
  for the measured quantity; and
- the repository’s current `gate`, `size`, and `artifacts` checks.

Results may be family- and range-specific. A win on a 20-bit analogue does not
establish a 60- or 80-bit asymptotic or practical speedup. [pcc:unverified]

## 10. Phase-specific admission notes

### Phase 1 — nominal 20 bits

Run all seven cells. The core trio supplies Koblitz/general-binary/prime
coverage. K1-23 checks the other Koblitz sign; B24 checks a composite-degree
ambient field and cofactor 8 without claiming the curve descends to a proper
subfield; the `j=0` and `j=1728` cells check automorphism applicability and
large-cofactor handling.

### Phase 2 — nominal 30 bits

Run the degree-37 Koblitz cell, B27, and standard-panel P32a. Treat the Koblitz cell’s cofactor 596 as a stress
trait, not as representative of standardized low-cofactor Koblitz deployment.

### Phase 3 — nominal 60 bits

B59 and P61 remain within current explicit u64 paths. The degree-73 Koblitz curve
`icv1-f2m73-tm184271214331-9d25cc67` is registered, but
its wide implementation presently supports reference methods rather than the
matching index-calculus pipeline. The phase remains partially blocked until
that method gap is implemented and passes G0.

### Phase 4 — nominal 80 bits

The degree-83 Koblitz curve `icv1-f2m83-tm6151469093347-debefd74` is the registered confidence-gate curve, but its index-calculus
method gap remains. ECC2-79 and ECCp-79 are exact historical Certicom challenge
anchors, not local registry records. Before Phase 4:

1. import both anchors with ICV1, EC1, provenance, order/subgroup proof status,
   and traits;
2. add generic wide binary and prime ecbench constructors and candidate/reference
   parity without weakening phase accounting;
3. pass all constructor, applicability, registry, and plan tests; and
4. freeze new phase specs and resources under G0.

The challenge problems’ historical solution status is not an index-calculus
performance result for this program.

## 11. Required phase dossier

Each phase deposits the frozen specs/plans, target list, relation-pruning input
and output, raw ecbench sessions, validation reports, replay receipts, complete
failure ledger, machine-readable summary, and a prose report that distinguishes
observations, bounds, projections, and unsupported claims.
