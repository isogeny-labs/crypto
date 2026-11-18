# Decode rational pairs on partially split chords

V2source94b992677480/checkpoint224346 ran once. Exact divisions/bounds,
base-field coefficient descent and124direct controls passed. Saturated norm
degree18, rationalxroots54,112, exactzeroes(54,56),(112,65). BOTH were rejected
by the old decoder's four-rational-point chord restriction. Exit101 retained
in RUN-V2.*. No anchor, polynomial, target or root adjustment.

V3keeps the SAME fiber, saturation, root selection and all controls. For each
zero restrict targetquartic to the gradient line, factor its degree<=4
polynomial, include its infinity intersection on degree drop, and use only
these rational hits in the Hessian-pair test. No full targetpoint scan is
needed. A chord need not split in four rational points for its selected image
pair to be rational. Require >=2rationalhits and a UNIQUEHessian-compatible
pair; retain all failures. The negation test uses the same chord and orientation.
The final existing-matrix projective control must validate each doubled
source/image pair. It is not used to select roots or the Hessian pair.

Source_jet.rs copies the pinned sourcejet decoder; only its rational-hit
threshold and loops change. Chord factorization supports affine graph and
vertical lines; a line entirely at infinity is explicitly unsupported.
Existing verifier/setup group costs remain. V3write-once outputs RUN-V3.*;
freeze/checkpoint/gate before one run. No timing or10xclaim.
