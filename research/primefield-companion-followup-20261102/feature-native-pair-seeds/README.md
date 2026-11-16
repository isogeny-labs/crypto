# Homogeneous seed propagation

Prepared windows now retain normalization seeds as numerator/denominator pairs and
feed them directly into a homogeneous source consumer. Recurrence steps and source
seed handling no longer convert those pairs to scalars. Raw/direct scalar-source
functions remain as reference paths; recovery still accepts only a public point.

The new PairSeedWindow exposes seed_pairs explicitly. Anchors/direct phases use
[c,1]; the public identity uses [0,1]. Recurrence updates preserve independent
input rescaling. All scaled division values, recurrence squares, partial tails and
singletons propagate their denominators through to the sign predicate.

One optimized build and one bounded invocation pass all148 native methods (none
filtered), including20736 recurrence/rescaling cases,9 scaled-source checks,
2 rejection cases, prepared seed/phase comparisons and all8 complete recoveries.
All27 prebuild source/data hashes pass. The scalar raw-parts implementation and
recovery source are byte-identical to the prior ratio-sign component.

The paired seed update has6 products and no inverse versus the scalar expression's
4 products and1 inverse, a derived local tradeoff. The homogeneous consumer adds
work and each stored seed now has2 residues. Count those costs before evaluating
complete improvement. ValueDivision, setup, direct phase and geometry inversions
remain. No comparative panel or cloud job ran.

See DERIVATION.md, receipt.json, source-lock.json, build/test logs and the inspected
editable report. Previous comparison identities and original4563-case cohort are
unchanged. Next: separate matched instrumentation of the entire homogeneous path.
