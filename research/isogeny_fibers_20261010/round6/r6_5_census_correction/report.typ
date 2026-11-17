#set page(margin: 1.8cm)
#set text(size: 10pt)
= R6.5: the A-pair correction against the §15.2 j = 0 census
*Verdict.* Hypothesis refuted on the recorded census; correction confirmed on an ensemble. The recorded ζ-closed/plain m = 2 ratios (0.931, 0.959, 1.021 at p ≈ 2^18, 2^19, 2^20; all reproduced exactly in Rust) are set by each base's cofactor-class composition (derived per-base sd 0.12 at |F| = 72). The residual after the |A| = 6 correction (−0.085, mean) exceeds the residual before (−0.030): the falsifier fires, within noise.

On p = 195709 with 200 random bases per size, the ensemble ratio is 1.130 [1.095, 1.170], 1.059 [1.043, 1.077], 1.033 [1.024, 1.044] at |F| = 36, 72, 144, against 1 + 4/|F| = 1.111, 1.056, 1.028 (|A| = 6: negation counts because the deterministic pairs {ζ#super[a]P, −ζ#super[b]P} need both signs). |A| = 3 is excluded at every size.

#figure(image("excess_vs_inv_size.svg", width: 100%), caption: [Excess of the ζ-closed base over the plain base vs 1/|F|; bars are 95% bootstrap over bases; crosses are the three recorded round-3 censuses.])

Toy curves only; no claim about deployed curves; the correction is a constant and moves no §15.4 row. Details and the proposed erratum: `RESULTS.md`.
