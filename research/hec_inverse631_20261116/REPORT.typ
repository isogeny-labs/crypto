#set page(paper:"a4",margin:24mm)
#set text(size:11pt)
= Actual F631 inverse equation construction
Genus 3 · 15 November 2026

#table(columns:(1fr,1fr,1fr),[Input],[Equations / variables],[Positive residuals],[D1],[12 / 8],[12 zero],[B],[12 / 8],[12 zero])

== Construction and controls
Actual quartic → discovered target sections → expanded symmetric tensors → projective inverse equations.

The old field constants, fixed quartic, inverse exponent and fixed positive triple have been replaced. Both cases use the first three runtime control points. Independent pointwise tensor evaluation agrees with the polynomial expansion, and a corrupted coordinate is rejected. The target section builder passes coefficient, determinant, split-tensor and fixed-base cancellation controls.

Section polynomial term counts are 253, 309, 408, 358, 779, 901, 956 and 978. Generated positive and public-input systems are retained separately for each input.

== Custody
Source freeze ef8c6be4b905 and checkpoint 20261116T023406Z precede both once-only executions. Both exit zero with empty stderr. The initial compilation failure from missing helper includes is preserved; it was corrected before source freeze and scientific execution.

== Scope and next test
Equation construction is verified on independent positive controls. The systems have not yet been solved. Next, freeze solver dependencies and generated systems, execute the positive and both input systems, and independently substitute the resulting parametrizations into all twelve original equations. Adjacent-coordinate class and sign checks must follow.

The global correspondence and charged speed comparison remain open. No timing or tenfold gain is claimed.
