#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Bounded calibration and heldout alignment over F631
Genus 3 · 15 November 2026
#image("flow.svg",width:100%)

Six fixed anchor pairs produce five supported fibers and one retained empty fiber. The first supported case, anchors01, is selected before fitting. A fresh basis change predicts four withheld multiples. This is correspondence-component evidence, not a global map proof or original-input recovery.

== Exact calibration
The source anchor pool is (0,7),(4,45),(5,35),(8,68). Decoded zero counts for01,02,03,12,13,23 are1,0,1,4,1,3. Every restriction has degrees A15/B11, exact degree12 anchor-factor removal and degree18 saturated norm. Each case matches592direct source evaluations and the complete rational zero set. Empty anchors02 is retained with its classified exit101.

Selected variable point(168,162)gives target pair[(491,348,1),(621,330,1)]. Fourfold clearing gives source u=[41,441,426,1],v=[542,232,540]. Multiplier4 clears the actual cofactor; the earlier multiplier2 is not a general cofactor clear here. A separate frozen supplement verifies all ten decoded target pairs as nonzero prime-subgroup points, including nonselected candidates. Fibers are not rerun.

== Correct centering and alignment
The freshly discovered target origin is(0,93). The centering class has a nonzero2-primary component: the old odd-only half fails. Splitting off ell·H, finding a rational half in the complete four-point2-primary group and adding the odd half gives exact2beta=H. Two rational half choices exist; the first lexicographic choice is frozen.

#table(columns:2,inset:5pt,[Alignment check],[Result],
 [Coefficient rank],[63],
 [Basis matrix rank],[8],
 [Training / withheld],[12 / 4 pass],
 [Original / XZ overlaps],[16 agree],
 [Corrupt matrix],[Rejected])

The selected two-hit chord has only one rational pair even with the Hessian erased, so that erasure is not discriminating there. Other retained fibers exercise four-hit chords. No expected matrix, relative calibration scalar or old image/log table is supplied.

== Custody and scope
Separate freezes/checkpoints: fiber13aa3d28806b/020907Z, alignment059208547e82/021254Z, full-pair verifierf9991cebb5a5/021532Z. Positive statuses0/stderr empty; the empty fiber is explicitly unsupported. The supplement closes the initial first-target-pair-only coverage without changing scientific outputs.

Centering.lean covers conditional additive centering and translation clearing, not geometry or implementation. Actual D1/B inverse images, full transfer verification, destination IC and charged comparison remain. No timing, exponent, novelty or tenfold gain; genus two/four and the full objective remain open. Prior failed charged comparison unchanged.
