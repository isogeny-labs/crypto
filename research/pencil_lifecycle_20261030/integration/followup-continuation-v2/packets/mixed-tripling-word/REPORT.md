# Literal UInt64 tripling bridge

Four declarations compile: bounded literal multiplication reduction, literal tree equivalence, canonical payload/decode, and original-input repeated-support preservation. Main attempt2, controls3 and independent axiom readback1 exit0. Independent source review is pending.

The source tree retains raw constants5,8,10. For c<=10 and canonical a<p<2^31, c*a<2^64; therefore literal multiplication reduces identically to multiplication by c%p. Neither8 nor10 is assumed canonical atp5/p7. Ordinary canonical products retain the existing p² bound. `triple_literal_tree` establishes equality to the reduced-constant expression tree before existing expression composition supplies field decode. All six fields match the independently reviewed `MixedTripling.rawOutput`.

Original normalized valid point/divisor inputs, canonical coordinates/usedcoefficients, four nonzero word weights and literal outer zero/cancellationfalse/equalother guards give y!=0, Some, canonical output, nonzero weights and normalized Mumford validity. The constant curve coefficient a0 is used only in validity and has no arithmetic read in triple. The restricted wrapper's None outside this route is a marker, not production behavior for other branches.

Source correspondence explicitly covers retained odd-prime Field.n=1,tables=None. The table backend is excluded because raw8/10 would not be canonical small-prime indices. Counter/compiler behavior and historical Cargo linkage are not established. Fixed controls cover raw8/10 atp5/p7, upper bound representativep2147483647, explicit noncanonical literal facts, full original-input p5 branch, nonunit p7 complete decode, cancellation precedence and distinct-other exclusion.

Controls1 preserves a theorem-argument inference heartbeat failure; controls2 preserves polynomial C-coefficient simplification failures; controls3 passes after explicit arguments/coefficient identities. Main source compiles on both attempted stages. Readback shows only propext/Quot.sound for arithmetic/tree lemmas, and standard Classical.choice additionally for field/validity composition. No scientific trials or measurements were run; cost ratio is null.

Replay privately using check.py WordTriple <fresh-tag>, Controls and Readback (each120seconds, existing pinned dependencies). generate.py is the construction helper for the initial literal definitions only and must not be used as a full proof replay script. PDF1page visually inspected, SVG/status/sources/failures provided.
