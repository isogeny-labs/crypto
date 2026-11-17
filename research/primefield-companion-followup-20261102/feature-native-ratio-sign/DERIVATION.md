# Homogeneous sign normalization

Let p be the supported odd prime, let N,D,c_inv be canonical nonzero elements of
F_p, and suppose the caller wants to classify s=(N/D)c_inv as +1 or -1, rejecting
all other values. Because D is nonzero:

    s =  1  iff N*c_inv = D,
    s = -1  iff N*c_inv = -D.

Therefore the predicate needs one field product and two possible equality checks;
the second comparison forms p-D. It performs no inversion. A common nonzero
rescaling (N,D)->(lambda*N,lambda*D) preserves both comparisons. Nonzero and
canonical-value checks precede the product. Zero/infinite group points remain a
separate public API case yielding sign0, not a zero ratio accepted as a sign.

The original finite-point pipeline first formed v=N*inverse(D), then computed
v*c_inv and compared with +1/-1. The new predicate structurally removes that final
inverse(D) and the first product. It retains the calibration product, adds explicit
ratio-domain checks and compares against the denominator (and its negation).
This is a derived local operation statement, not a measured whole-pipeline result.

A private RawParts record contains the already-computed numerator/denominator and
all original raw metadata. The raw-value API uses the same record, performs the
original division and returns the original Raw type. The direct sign, legacy
window and prepared seed-window APIs consume the parts through ratio_sign instead.
Raw calibration setup and other source inversions (seed recurrence, division
values, geometry) are unchanged. The low-product polynomial engine is referenced
unchanged, isolating this source-level construction.

The identity applies to every valid ratio in the supported prime field, independently
of r or the feature width. It changes one final operation sequence per finite sign;
it does not change the square-root-size source partition or decoder program size.
Those scale-dependent terms remain candidates for further construction.

Prior context: homogeneous coordinate representations and nonzero common scaling
are standard; see EFD https://www.hyperelliptic.org/EFD/g1p/index.html and
https://www.ichair.hyperelliptic.org/EFD/oldefd/projective.html. This use of cross
multiplication is not claimed as a new identity. The specific integration is into
our normalized elliptic source's sign-only API.

Evidence: exhaustive1944 nonzero triples over F7 and F13,3888 common-rescaling
checks,6 invalid triples; direct-vs-raw sign agreement on three saved curve fixtures;
full106-method native invocation including old raw metadata, window/seed controls
and all8 complete recovery receipts. Source and data locks bind the checked files.
