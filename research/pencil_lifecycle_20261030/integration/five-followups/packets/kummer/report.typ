#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#image("closure.svg", width: 100%)

= Kummer transition closure and a branch-directed candidate

#text("Status: source derivation and private proposal; no new cohort, compilation, arithmetic trial or timing run. The research-visuals skill was used for this report. Date: 2026-10-30. Parent integration must review this candidate before implementation.")


== Sources checked
Historical N19 src/main_labelled.rs (step 41-56, lift 138-171, fallback 184-212, advance 215-280, collector 390-540), src/walk_impl_uncached.rs (counters 5-41, repaired arithmetic 63-93, branch 166-177), and n19-labelled-fallback/VALIDATION.md. This report reads historical source directly; it does not claim these files equal the current main build.
Primary literature: Chung, Costello and Smith, Fast, uniform, and compact scalar multiplication for elliptic curves and genus 2 Jacobians, sections 2, 3 and 6.4, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf . The paper specifies differential addition from a known difference, adjacent-point recovery, and exceptional-input restrictions. It motivates the abstract interfaces here, but does not certify this Rust implementation. No novelty claim is made.

== Derived closure and labels
Let J be an abelian group, kappa its quotient by sign, and R=aG+bQ. Retain (kappa(R),kappa(R+G)), fixed kappa(G), identity data, and labels (a,b). Assuming correct doubling and differential primitives, calculate D0=kappa(2R), M=kappa(2R+G), D1=kappa(2R+2G). For input-derived c in {0,1}, select (D0,M) if c=0 and (M,D1) if c=1. This is precisely the adjacent pair for R'=2R+cG. Its labels are a'=2a+c and b'=2b modulo r. The difference of the two signed entries stays G. This proves operation-set closure conditionally on the arithmetic interfaces, and matches the existing N19 algorithm rather than proposing a new random-addition walk.

#text("Signed recovery uses the fixed signed anchor G and adjacent quotient pair. The source computes the opposite neighbour with repaired differential addition before conversion and recovery. Exceptional lifting is attempted on D_zero; unresolved conversion and recovery failures remain explicit. Recovering a signed divisor supplies the signs of y0,y1 used by graph.edge with the maintained a,b. A quotient point alone is insufficient: in Z/11, quotient classes of R=2 and -R=9 coincide, but with G=1 the successor classes are [2+1]=[3] and [-2+1]=[-1]=[1], which differ. This is an information obstruction to addition from quotient inputs alone, not an impossibility result with labels or extra state.")


== Source-derived pruning candidate
branch(k,p) runs before all three calls and depends only on k and p. It normalizes one coordinate then takes its low bit. Existing step eagerly evaluates both doubles and the mixed term. The selected output uses only two of those three results. The private patch evaluates only those two, in their original relative order. The exact source call-count difference is three versus two repaired-arithmetic invocations per ordinary step; there is no timing or overall cost claim.

#text("This changes observations intentionally: field counters omit the unused call, and the unused repair-root slot is None. The caller sums repair-root presence into a diagnostic count. No caller uses that count or root identity to choose a transition or fallback. Pair Option and selected coordinates are unchanged under the preconditions below. Fallback runs precisely when the selected pair is None; its signed reconstruction and labels remain unchanged. A skipped None never caused fallback in the eager version. The source has no RNG or other mutable data inside repaired arithmetic except thread-local operation counters.")


#text("Equivalence domain: all operands and action matrices canonical modulo p, prime 2<p<=2^32; nonzero t entries; sufficient u64 counter headroom for the eager evaluation; valid indexing; no external interruption. These ensure products fit u64, x+y and x+p-y fit, and inv's assertion holds. Without headroom, eager execution can panic when the candidate does not; under deadlines, removal of work may also change termination. Thus this is state/label/fallback equivalence on normal execution, not identical diagnostic counts, panic behavior on arbitrary inputs, or deadline outcomes. The proposal has not been compiled or executed and must receive a new method identity before any measurement.")


== Symbolic full accounting
Charge initialization signed construction plus two projections and fixed model setup. Each ordinary step pays branch selection, selected double, mixed differential operation, repairs actually attempted, labels, conversion/filter, and bookkeeping. Each split candidate additionally pays opposite-neighbour construction, conversions, signed recovery or exceptional lifting, split checks and graph processing. Each restart pays two signed scalar multiplications, addition, neighbour addition, validity checks where executed, and two projections. Each failed differential step pays its failed attempts PLUS the signed fallback reconstruction, checks and projections. Final recovery and independent verification remain charged. Precomputing inverse constants is a separate possible change; it is not included in this proposal.

== Next concrete checks
Independent source review of conditional equivalence; identical-output paired transition checks including forced repair/None, nonselected repair, and fallback; explicit counter-delta accounting; runtime qualification and prospective protocol before any fresh 1,260-target cohort. Preserve all existing N19 data. Coordinate arithmetic and exceptional recovery still require their separate fidelity proof. No new cost ratio is reported.

