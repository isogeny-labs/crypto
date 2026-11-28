#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Section evaluation: remove redundant lifts
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr),[Correctness check],[Observed],[No-base numerator comparisons],[48 exact matches],[Fixed-base cancellation],[Degree-two factor; exact match],[Recovered input classes],[Both unchanged],[Lean residue identity],[Checked; exit zero])

== Mathematical shortcut
A Hensel lift differs from the original interpolation by a multiple of u. Polynomial evaluation therefore has the same residue modulo u. When the common base factor is one, no cancellation requires higher precision: return the reduced numerators already computed during the gcd check.

Compute residues and common base factor → factor one: return residues → nontrivial factor: retain full lift and cancellation.

Every returned coefficient is compared with the original evaluator. The explicit tangent-line divisor containing the fixed base forces a degree-two common factor and exercises the fallback. Actual direct-output reconstruction and public adjacent-sign/corruption checks pass in both coordinate charts.

== Conditional formal proof
Residue.lean checks the ideal-quotient evaluation identity over any commutative ring with explicit ideal-membership hypothesis. Lean4.19 and the pinned mathlib build pass with standard axioms propext, Classical.choice and Quot.sound.

This proof covers residue invariance. It does not formalize Rust arithmetic, computed gcd/lift hypotheses or the complete geometric map. Those boundaries remain separate from finite exact implementation comparisons.

== Custody and next test
Freeze050bd00ca954 and checkpoint20261116T032138Z precede one correctness execution. Exit zero; stderr empty. Both class files equal the prior direct replay.

The comparison build runs both implementations and cannot measure speedup. Prepare a production variant with comparison-only calls removed, preserve the exceptional branch and recheck classes before a frozen timing run. The measured1.621xinverse result remains unchanged. No tenfold gain, generic support or genus-two/four mechanism is established.
