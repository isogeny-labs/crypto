# Incremental source extension for adaptive recovery

2026-11-05. Native stateful construction and bounded correctness checks. No comparative performance panel.

## Construction and invariants

A reusable Source owns the original prime-field curve, public generator and calibration. Prepared borrows that immutable source and stores public strides, seed square and the four anchors once. Each recovery creates a bounded target-specific cursor; it does not rebuild preparation. The existing adaptive polynomial and its two sign predicates are imported unchanged. [pcc:unverified]

For a prefix ending at T=Q+(q-1)P, retain its Jacobian point and the projective grid lanes [lambda_j]T. Prepared already holds affine [lambda_j]P. Advancing performs T <- T+P and one mixed addition in each lane. Therefore each new lane is [lambda_j](T+P). Batch-normalize the new point and its lanes together. At identity, all lanes must also be identity and there is no conversion batch. Nonidentity with an identity lane is rejected, preserving the old grid domain. [pcc:unverified]

The prefix builder returns both its old complete window and the private final projective state; it does not reconstruct that state from affine output. The cursor is tied by Rust lifetimes to the exact preparation. An advance consumes the previous cursor and returns a replacement only on success. The caller supplies a finite extra-step allowance; exhaustion is checked before advancing any geometry. Empty prefixes and invalid initial points are rejected. Private cursor fields prevent caller-injected seeds or grids.

## Seed extension

Let c(T)=N/D and c(T-P)=A/B. With delta=x(P)-x(T) and S=c(P)^2, the existing homogeneous recurrence is

    c(T+P) = [ delta S N^2 B : D^2 A ].

The numerator and denominator represent the same affine seed as delta S c(T)^2/c(T-P). Independent rescalings of either input pair change only the common output scale. This is the preserved pair-seed formula, not a new phase formula. The sign consumer already accepts homogeneous seeds. [pcc:unverified]

Anchors O,P,-P,2P take precedence over the recurrence. They prevent a zero predecessor around identity from reaching a denominator. If only one prefix seed exists and the next point is not an anchor, a direct phase is still required. With at least two preceding seeds, a non-anchor extension uses the recurrence. Invalid or zero homogeneous operands are rejected. The native counter checks actual phase() calls, not just the method string. [pcc:unverified]

The cursor retains only two seed pairs, the previous affine point, and the final projective point/grid. The API also returns the original full prefix; this is not a claim that prefix allocation was removed. One recovery may request at most one extra sign through the adaptive wrapper. A single Prepared value can serve multiple target points while each cursor remains target-specific. No unknown scalar is supplied to source construction or recovery.

## Recorded validation

The optimized overflow-checked run reports89 tests passing, retaining the adaptive decoder's full-field tests and the original source/geometry tests. Eight public fixtures at r479 exercise cuts1,2,14,15 of length18 windows: all320 appended signs, seed pairs, points and method selections match uninterrupted windows. A test-only phase counter increases exactly when the direct branch is selected. [pcc:unverified]

Two small prime-field sources, E/F5 with (a,b,P,r)=(2,1,(0,1),7) and E/F7 with (a,b,P,r)=(0,3,(1,2),13), exercise44 additional extensions through repeated identity crossings. They record20 anchors and24 recurrence steps and no additional direct phases. Empty prefixes, invalid points, zero allowance and exhausted cursors are rejected. [pcc:unverified]

The reusable adaptive test prepares the F509 source once for all eight saved target points. All recoveries agree with their checker-only scalar labels and the frozen pre-cursor source. Exactly four exceptional recoveries extend; each reports recurrence,23 lane updates,24 converted points and one conversion batch. The nonexceptional path performs no extension. The split-window phase checks cover the same exceptional offsets and observe no direct phase there. [pcc:unverified]

Twenty-three source/data hashes are checked before and after compilation; eleven compiler warnings are retained. origin-lock.json records the preserved implementations from which geometry and source were extended. The output is recovery.tsv; no new decoder coefficient packet is generated.

check.rs imports the frozen adaptive implementation, not this component's cursor, geometry or source. It reexecutes eight baseline recoveries and checks values and query decisions against the saved trace. A changed recovered scalar is rejected. Its method/lane/batch assertions are consistency checks on reported metadata, not proof of how the producer executed. This checker is written in the producing session and has no signed independent-review standing. [pcc:unverified]

## Costs and assumptions

For L lanes, one nonidentity extension performs one prefix-point addition, L mixed lane additions, and a batch conversion of L+1 points. The batch routine uses one field inverse; seed propagation adds no inversion. At identity, normalization is skipped. These are source-derived component counts; direct initial phases, residual value/division computations, polynomial work, calibration, predicate checks and decoding remain separate. No elapsed-time gain or whole-source inversion-free claim is made. [pcc:unverified]

The old caller-certified prime p, prime subgroup order r, nondegenerate curve and source-domain assumptions remain. The first q signs still require their original grid construction and batch conversion. The fixed adaptive decoder still requests15 signs ordinarily and16 on its two exceptional fibers. Its original4563-case cohort is preserved; eight integration fixtures do not replace it. No cloud job or timing panel ran.

## Prior work and next step

Read the saved shared-seed, pair-seed and seed-factor derivations and the actual source/geometry implementations before editing. Public Jacobian and mixed-addition conventions were checked against the [Explicit-Formulas Database](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html). Standard point addition and homogeneous recurrence are reused without a novelty claim.

Next bind the existing complete-cost instrumentation to the reusable source and lazy extension before any admitted timing comparison. For algorithmic query reduction, investigate pair-mean targets on shorter signature prefixes or other public shifts; higher-degree features may help represent a collapsed target, although they cannot distinguish equal signatures without another sign.
