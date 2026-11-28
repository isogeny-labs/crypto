# F631 bounded calibration and withheld basis alignment

The six fixed anchor-pair cases produce five supported calibration fibers and one empty rational fiber. The preregistered first-supported rule selects anchors01 before any alignment test. A fresh basis change then fits twelve multiples and predicts four withheld multiples, with exact chart and corruption controls. This advances the larger-field correspondence beyond subgroup matching; it does not verify the global isogeny, original D1/B images or a DLP.

## Frozen anchor cohort

Source13aa3d28806b/checkpoint20261116T020907Z precedes the once-only six-case cohort. The first four admissible source anchors among the fixed prefix are (0,7),(4,45),(5,35),(8,68). All pairs are retained:

| Case | Rational fiber zeros decoded | Outcome |
|---|---:|---|
|01|1|supported and selected|
|02|0|unsupported, empty fiber retained|
|03|1|supported|
|12|4|supported|
|13|1|supported|
|23|3|supported|

Each restriction has A degree15, B degree11, exact degree12 fixed-anchor removal and degree18 saturated norm. Roots are factored, not found by a cyclic generator scan. Every case compares against592direct source values and the complete rational zero set, including the empty case. These exhaustive checks are verification costs, not omitted timing work.

The selected variable point is (168,162); its decoded ordered target pair is [(491,348,1),(621,330,1)]. The four-cleared source calibration class has u=[41,441,426,1],v=[542,232,540]. The source cofactor is4, so both source class and target difference use multiplier4. No relative calibration DLP or expected image matrix is supplied. Negation reverses the decoded pair. The selected chord has only two rational hits; Hessian erasure leaves one pair and is therefore not a discriminating negative control there. Other retained fibers include four-hit chords with six erased candidates.

All ten decoded source classes were checked in the fiber loop. The initial outer target check covered only each case's first returned pair. The separately frozen all-pair supplement (f9991cebb5a5/checkpoint021532Z) closes this coverage gap without rerunning fibers: all ten target pairs are on the actual quartic, four-cleared nonzero, killed byell59296481 and not killed byell−1. All ten pass. No unverified nonselected pair is hidden by the chosen first case.

## Actual F631 centering correction

The target chart is freshly discovered: O=(0,93), tangent y=442x+93, residual x²+286x+270 and a computed split canonical line. Its section construction passes exact rank/coefficient, determinant, tensor and base-cancellation controls. Graph-chart arithmetic is ported to631, with exact class round trips and original/XZ covariance.

H=2(Dinf−3O)−(K−4O) has a NONZERO rational2-primary component. The old formula beta=((ell+1)/2)H fails the checked identity2beta=H. The new calculation generates the complete four-point target2-primary group, splits H2=ell*H (ell≡1 mod4) and Hodd=H−H2, halves Hodd in the odd subgroup, and adds the first lexicographic rational half of H2. Two rational half choices exist; the selected beta satisfies2beta=H exactly. This is a necessary cofactor-aware correction, not a performance gain.

## Alignment and independent coverage

Alignment source059208547e82/checkpoint20261116T021254Z reads only the selected case. It does not rerun the fiber or choose another anchor after fitting. The first16multiples are all supported:12train,4withheld. The projective coefficient system has rank63 and its unique8x8matrix has rank8. All16comparisons pass, with16original/XZchart overlaps. A deliberately corrupted matrix fails a withheld comparison. MATRIX.txt and CENTERING.txt are retained in alignment/; no old matrix is an input.

The matrix links the implemented source quotient sections to independently constructed target sections on the calibration sequence. It is not a full proof of the global isogeny, and no original public D1/B target classes were recovered here.

## Formal and claim boundary

Centering.lean formalizes two conditional additive identities: halves of the odd and2-primary parts combine to a half of H; fourfold clearing removes a2-torsion translation. It does not prove the geometric translation premise, completeness of the group computations, correspondence or implementation. Proof output/axioms are in LEAN.txt/EXIT-LEAN.txt, with scope in PROOF-MAP.md.

All positive stages exit0/stderr empty. anchors02exit101is the explicitly classified unsupported empty fiber. Every once-only scientific output and amendment is retained. No timing, exponent, novelty, full DLP or tenfold improvement is claimed. Original-input inverse lifting, transfer verification, full destination IC and charged comparison remain, alongside genus2/4. Earlier failed cold-cost comparison unchanged.
