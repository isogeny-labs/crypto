#set page(paper: "a4", margin: (x: 15mm, y: 14mm))
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: false, leading: 0.6em)
#set heading(numbering: none)
= Symmetric PKM: G1–G6 execution prerequisites
2026-10-30 · Native software and structural proof development

*Goal:* determine whether symmetric PKM produces useful elliptic-curve relations cheaply enough to improve a complete cold, single-target DLP computation. The user's full six-stage objective remains open. This deliverable supplies implementation prerequisites; it supplies no measured gain or asymptotic theorem.

#table(columns: (0.55fr, 2.15fr, 2.3fr), inset: 6pt,
  [*Goal*], [*Implemented contribution*], [*Still required*],
  [G1], [Complete signed witness sets; independent tiny affine reference; request-bound replay.], [Frozen multi-curve correctness panel, positive/miss/cap controls and supported-domain coverage.],
  [G2], [Exclusive diagnostic phase ledger; native counters; cold toy rank/LA path.], [Full count coverage, priced residual work, admitted clocks and durable failed-process records.],
  [G3], [Chain/plain-full/symmetric adapters; canonical JSON and msolve input export.], [Equal-domain m=4 panel, qualified pinned external solver/extractor, A/A and null controls.],
  [G4], [No optimization promoted.], [Freeze an equal-output baseline; test the proposed 20% cost or 2x-memory objective with held-outs.],
  [G5], [No scaling panel launched.], [Predeclared depth/field-size cells, censored boundaries and held-out checks within existing caps.],
  [G6], [Tiny cold relation/LA/recovery control; Mathlib affine chain-order dependencies.], [Growing-family cold IC/rho/BSGS comparison, independent rank/cost admission, ideal/curve/cost proof bridges.],
)

== Supported computation
The oracle accepts prime 11 < p < 2^32, m=2..4, depth1..8, an exact-order Kummer tower and an actual finite public point. It canonicalizes all verified distinct-abscissa signed decompositions. Repeated abscissae and identity targets remain excluded. Timeouts, root interruption and output/assignment limits remain incomplete. Full-field visitor branches are visible.

The independent reference scans the field and uses separate ordinary affine arithmetic, without F4, the constructors, Montgomery arithmetic or the factoring API. It is bounded to p<=257 and one million signed tuples. It detects missing/invalid/duplicate outputs and validates the exclusive phase sum. It never supplies oracle answers.

The cold toy control independently enumerates the full curve and requires prime order. It builds the factor base, forms public aP+bQ queries, collects scalar relations, tracks useful rank, performs dense elimination and checks kP=Q independently. Identity queries are charged and skipped. No scalar or base-log labels enter the algorithm. It is limited to p<=257; it is not the qualified G6 panel.

#pagebreak()
#image("architecture.svg", width: 100%)

== What the accounting permits
Comparison admission is always false. Constructor helper arithmetic, emitted visitor/root multiplications, sign/addition calls, independent verifier arithmetic and scalar-field elimination are recorded. F4 symbolic/substitution work, root additions/inversions, native Montgomery work and logical/allocation bookkeeping still lack complete counts. The required pricing band cannot be replaced by zero charges for these gaps.

Oracle phase intervals are exclusive and reconcile to the diagnostic total. Cold toy setup, full query intervals, scalar verification and unattributed overhead reconcile separately. Elimination time is a subset of query time. Process startup, input loading, binding and serialization are outside those process-core clocks. Reported RSS is a whole-process lifetime high-water mark. External process/memory caps and supported journaling/resume remain necessary.

== Proof and validation scope
The abstract theorem assumes an additive commutative group, a+b != 0 and c != d. Two new lemmas check the distinct-x pair-sum implication and finite-chain order on actual nonsingular Mathlib affine Weierstrass points, over a field with decidable equality. Rust/polynomial-to-point and complete-cost bridges remain open. Fresh scoped audit: 13 declarations, zero trusted exceptions. The final module builds and kernel-replays on Lean4.34.1 / pinned Mathlib. Native validation outcomes, source/binary hashes and every retained failure are recorded in validation/receipt.json. Historical receipt directories remain unchanged. The broader 5553-test serial library run was capped at40minutes: 2173passed,0failed,47ignored,one unchanged JV case active and3332not reached. No full-suite pass is claimed; its large raw log is NOT_UPLOADED.

== Execution blockers and sources
The unchanged checkpoint gate fails because no upload token/AWS profile is configured. The managed runtime reports no configured identities/secrets/runtime variables. The available GitHub Isolab path is ecbench-spec-specific; this native example has no qualified source-bound runtime image/route yet. No scientific run, lease, funding, quota or IAM change occurred.

Checked sources: PKM2016 sections1,3.1–3.2,5; the retained PKM lead-atlas entry; tower note sections2.1–2.2/3.3–3.7; PR2632/2650/2868; the separate mixed-fiber solver problem; current F4/root APIs and gb_cas_export.rs. Primary: people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf. No global novelty or complexity claim is made. Canonical IC/curve/performance graphs remain unchanged because no admitted value or new edge was added.
