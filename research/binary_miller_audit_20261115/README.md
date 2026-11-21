# Binary Miller state and an intermediate-pole counterexample

Lean proves the actual carried binary Miller state and its guarded mathematical evaluation semantics. An actual curve over F5 shows that a valid final divisor value does not make each intermediate numerical division valid. This concerns item4 of the fourteen-item audit. The whole assignment remains incomplete. [pcc:unverified]

On **E/F5: y²=x³+1**, take **P=(0,1)** of exact order3, **Q=−P=(0,4)**, **T=(4,0)** and **D=(Q)−(T)**. The normalized chosen function f3 is y−1. Its actual guarded divisor value is **2**; D has degree0 and its actual image over AlgebraicClosure(F5) is disjoint from the full geometric principal-divisor support. The first doubling nevertheless uses **(y−1)/x**, whose order at Q is **−1**. The numerical x-factor vanishes at Q. A strict numerical trace rejects; totalized field division gives **0**, which differs from the correct **2**. The final sum-zero addition correctly contributes x, as permitted by the source exception. The counter does not manufacture an unnecessary final0/0. [pcc:unverified]

The positive control replaces Q by **R=(2,2)**: **Dsafe=(R)−(T)**. Every encountered line/vertical factor is an actual local unit on this support; Lean proves the generic recursive factor guard. The strict trace returns **4**, equal to the actual guarded final divisor value. Both examples and the resource budget were fixed before formal code. [pcc:unverified]

| Actual support | y−1 at first point | y−1 at T | x at first point | x at T | Proper final value | Strict trace |
|---|---:|---:|---:|---:|---:|---|
| Q,T |3|4|0|4|2|rejects|
| R,T |1|4|2|4|4|returns4|

Every entry is tied to actual coordinate point evaluation and actual unit-valued divisor evaluation in Lean, rather than a supplied numerical oracle. [pcc:unverified]

## What is proved

[BinaryState.lean](lean/BinaryState.lean) defines a carried pair (point,function). Each recursive bit step doubles the carried point and optionally adds P; it does not recompute a scalar multiple inside each implementation iteration. For every field, actual DecidableEq operation data and actual nonzero discriminant, Lean proves the state is (nP,chosen f_n), including zero/vertical branches and characteristics2/3. The actual resulting function has the proper full geometric Miller divisor and leading residue1. The explicit n0/1 conventions extend the source n≥1 domain. [pcc:unverified]

[BinarySchedule.lean](lean/BinarySchedule.lean) proves exactly Nat.log2(n) doubling rounds and at most that many conditional additions. These are structural iteration/state-update counts. They do not establish native runtime, concrete field-multiplication cost or a DLP speedup. [pcc:unverified]

[GuardedStep.lean](lean/GuardedStep.lean) constructs the actual source numerator and denominator units, retains the actual exceptional branches, and derives the evaluation quotient from the actual divisor-evaluation homomorphism under actual per-factor principal-support guards. [GuardedBinary.lean](lean/GuardedBinary.lean) defines the carried mathematical execution relation GuardedRun. Its constructors require factor guards and the source recurrence, with no correctness or evaluation-commutation premise. Lean proves every run erases to the actual binary state and its numerical recurrence equals actual divisor evaluation. Every recursively factor-admissible input admits a run for every n. This is an all-iteration mathematical execution proof; a concrete numerical backend and its runtime refinement remain separate. [pcc:unverified]

[TraceControls.lean](lean/TraceControls.lean) proves the safe example satisfies the generic trace guard and the counterexample fails it. [GeometricCancellation.lean](lean/GeometricCancellation.lean) constructs the actual algebraic-closure coefficient/point/function images and both actual mapped divisors, proving the full geometric final-domain guards and the actual intermediate geometric pole. It does not assume a general finite-point/divisor basechange theorem. [CancellationValues.lean](lean/CancellationValues.lean) derives proper values2/4 through actual local residues. [NumericalTrace.lean](lean/NumericalTrace.lean) proves strict rejection/raw failure/safe correctness. [BinaryCertificate.lean](lean/BinaryCertificate.lean) assembles the full checked claim. [pcc:unverified]

## Source scope and assurance

Galbraith corrected [chapter26](https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch26.pdf), Algorithm29/Exercise26.3.6: fresh full text and visual PDF6/printed578, partial text PDF5 Definition26.3.4/Exercise26.3.5, within credited cumulative full text/visual1–6; not all16pages. The earlier final evaluation domain is retained. The literal unconditional numerical input domain needs an intermediate-factor qualification. The qualified function-field recursion and guarded numerical algorithm survive. Original Miller paper[427] is unread; no claim about its full qualifications or novelty. See [source review](evidence/source-review.json). Copyright source files remain local. [pcc:unverified]

87modules contain76 byte-identical credited predecessor modules and11 new modules. Lean v4.35.0/mathlib56a005855ea17e0b0958f73ec801ae140b0a1679; default limits, no holes/custom axioms/native proof evaluation/raised limits/global flags/waivers/frozen edits. Final build16 is clean. Six failed states03/05/07/12/14/15 and all intermediate advisories are retained; failed compiler placeholders are not accepted proofs. Whole tracked-source gate and the frozen native full-written-header check are recorded in [preflight](preflight.json), [proof map](proof-map.json) and [source plan](sources.json). The header check is procedural evidence, not compiler/parser/physical-host refinement. Same host/prover; lean4checker absent. Local replay and signed standing are distinguished. [pcc:unverified]

Six correct KB calls and two Conductor calls exited127 before query processing because the executables were absent. Bounded local/official/primary fallback was used before code; this supplies no absence/novelty claim. Actual predecessor local-unit evaluation, geometric classification/basechange and all-n chosen addition, plus official Nat.log2_def/order-of-element APIs, are credited. [pcc:unverified]

The [pre-code protocol](PROTOCOL.md) fixes50 retained states/deadline13:00Z/local$0/cloud0/agent experiment uploads0. All actual attempts are losslessly retained; `python3 restore_attempts.py DEST` restores them without overwriting unequal files. See the [one-page report](reports/report.pdf) and [verification receipt](verification.json). [pcc:unverified]

Full scheme/localRing/Divisor API comparison, general finite-point divisor basechange/tensor/Galois descent, translation/reciprocity/isogeny/Lang/Cartier/Jacobian/canonical Weil–Tate/MOV–FR and the remaining fourteen-item obligations stay open. No canonical corpus promotion, independent machine/backend, original-paper refutation, practical implementation failure or runtime claim follows. The parent immutable-ledger fork needs human handling; this packet does not rechain, force, rebase, retarget or merge it. [pcc:unverified]

Exact local **PCC-20261115-16a863** replay PASS at 2026-11-15T12:31:28Z; signed standing unverified. See [verification receipt](verification.json). [pcc:unverified]

Publication: existing ready [PR3743](https://github.com/isogeny-labs/crypto/pull/3743), remote proof `df82af348f2888257553b8e3ae6e89438cf587a0` and local receipt `0a826971cb761acd51d35a7e7b4ca30f503811be` share tree `8534ed5164f937175b50cf739ad329e46b02ffe7`. Anonymous fetch completed; fresh PR head/body/open/ready check matched. Publication metadata is outside the frozen pinset. [pcc:unverified]
