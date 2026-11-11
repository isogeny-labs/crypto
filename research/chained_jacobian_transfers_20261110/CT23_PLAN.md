# CT23 — closed-fibre Richelot evaluation

Prospective correctness and geometric-kernel work, following CT22. No new
evaluator, kernel certificate or arithmetic advantage is accepted by this
plan. Preserve the exact CT22 publication head on its own branch, and work
in a fresh child. [pcc:unverified]

## Sources checked before choosing the approach

KB unavailable: no relevant MCP tools are loaded; `scripts/research-kb status`
again exits127 because the executable is absent. The autoresearcher atlas
remains vault locked at8edec422a; no plaintext or guessed lead IDs are used.
Fallback sources are the existing CT21 global correspondence proof,
consumer src/nonsplit_map.rs and native/src/richelot.rs, the existing
isogeny_algos/src/genus2.rs Richelot implementation, and Kunzweiler,
“Efficient computation of (2^n,2^n)-isogenies”, DCC92 (2024), §3.2,
Propositions7–8 and Algorithm1. The primary publisher PDF and its hash are
in SOURCES_20261111.md. These sources establish the correspondence and
standard composition/reduction; the proposed elimination below requires
its own derivation and controls. No novelty claim is made.

## Question, competing outcomes and exact scope

Can the accepted R0/R1 forward/dual maps evaluate a degree2 Mumford support
as a closed fibre over their existing coefficient field, avoiding the
pointwise route's Fp2 splitting and Fp6 compositum for prime inputs?
The same method, when valid over the existing Fp6 field, may evaluate
R0 on the actual49-point Phi kernel without finding support roots in Fp12.
No new field, polynomial or Cantor engine is allowed.

A successful algebraic chart may cover most inputs with explicit boundary
fallback. A nonunit, degree drop or incomplete balanced reduction rejects
that chart on that input; it does not reject the map. Coverage and every
fallback reason must be counted. Keep the unchanged pointwise evaluator
as the independent route, including all exceptional projective fibres.

## Proposed elimination, to prove and test

For monic u(x)=x²+a*x+b, reduce
g1(x)h1(z)+g2(x)h2(z) modulo u to A(z)*x+B(z). The resultant is

    R(z)=B(z)²−a*A(z)*B(z)+b*A(z)².

Initially accept only degreeR=4, A a unit moduloR, and source ordinate
v(−B/A) a unit moduloR. Compute x(z)=−B/A, then

    Y(z)=g1(x(z))*h1(z)*(x(z)−z)
         /(source_scale*v(x(z))*target_scale) moduloR.

Check u(x(z))=0, the correspondence equation, and
Y²=target_curve moduloR using existing operations. Repeated support must
retain multiplicity; do not replace a nonreduced algebra by its roots.
The resulting four-point divisor represents finite4−2K. Its raw balanced
infinity parameter is−1; reuse the existing degree4 reduction by factoring
that code into a common method, preserving its statements and checks.
Prove the sign/count bookkeeping rather than assuming it from the formula.
[pcc:unverified]
Use the existing reverse scales and final negation for the positive dual.

Degree0/1, branch support, nonunits and degree drops retain the old completed
route on prime inputs. Extension inputs need explicit supported cases;
never claim that unsupported Fp6 support is impossible.

## Acceptance and bounded execution

1. First targeted test: generic closed fibres on deterministic actual R0/R1
   inputs in both directions; all algebra identities, output validity,
   exact comparison and rejection predicates, with frozen source/raw receipts.
2. Compare ALL12317 rational classes on each of the four map domains with
   the unchanged completed pointwise maps. Count chart/fallback categories,
   repeated supports and infinities. Preserve both dual[2], bijections,
   paired addition and all109 original source checks.
3. If the chart covers all nonzero actual Phi-kernel classes, form
   KerPsi=R0(KerPhi):49 distinct7-torsion classes over existing Fp6, all2401
   additions, Frobenius closure/eigenscalars and actual commutative controls.
   Otherwise record the exact unsupported cases and implement only justified
   projective completions; do not weaken the49-point obligation.
4. Add feasible faithful resultant/elimination/cleared ordinate Lean lemmas
   in a NEW module and a written correspondence/reduction bridge. Root97
   existing helper sources and all frozen PCC subjects remain unchanged.
5. Integrate only after comparison, retaining the original evaluation route.
   Run the complete changed-source guards, formatting and strict Clippy once
   the implementation is stable. Preserve failed attempts, append PCC before
   citing results, inspect all report PDF pages, commit and publish ready
   stacked PR against the immediate parent feature branch.
6. Continue CT24_PLAN.md's further nondual7-hop after checkpointing this rung.

Local bounded development tests and proof builds only: no timing claim,
solver, collector or paid allocation. S3 admission and live budget/capacity
are still missing. If they return, freeze a separate matched benchmark
protocol before measuring, with construction, transport, solver, recovery,
verification and direct elliptic/existing ECDLP controls. Whole-fibre
correctness alone does not establish reduced field-operation counts or cost.
Root and consumer ledger forks remain untouched. [pcc:unverified]
