# Projective atlas amendment to retained calibration failure

Originalfit6d012eba2099 ran once, exit101: rank63/rank8,12training/4scalar
holdouts andfirst3mixed holdouts pass; B+2D1targetchart unsupported. Keep all
original outputs. Diagnosticd3cb538e6232/checkpoint203329 ran once, exit0,
and showed no original affine graph. Its row2-39row1 is evaluation at the
infinity point[1:13:0]. This amendment repairs coordinates, not test inputs.

Copy pinned original fit source/data into a separate atlasworker with separate
binary name. Preserve old workers/outputs. Two charts: originalZ=1 and the
projective swap(X,Y,Z)<->(Z,Y,X), dehomogenizing by oldX. Packed ideal columns
are pulled back homogeneously. All reconstructed original cubic columns and
class roundtrips must still pass. CurveF transforms as degree4 and each cubic
numerator N_i as degree3; tensor coefficients and group reference stay fixed.
Reuse generalized Hensel/common-base cancellation with transformed F,N_i.
A mere affine x/z substitution without homogeneous numerator transformation
would not give the same section basis.

For every target coordinate request, evaluate both supported charts; when both
are available require projective equality and identical factorization type.
Prefer original coordinates, otherwise use swapped. Log both acceptances.
Require at least1crosschart agreement and1necessary swapped evaluation. Both
unsupported is still a failure/retained exclusion per original gate; no full
atlas claim. Repeated support and nonunit transverse derivative unsupported.

Same original calibration cohort must result, exactly
n=2,3,4,5,6,7,9,10,11,12,13,14,15,16,17,18; original source exclusions1,8
remain. First12fit/last4holdout, all4mixedholdouts unchanged. Rank63/nullity1,
invertibleM, corruption rejection as before. Require M exactly equals the
previous independently fitted canonical matrix in EXPECTED-MATRIX.txt, read
ONLY as a post-fit regression assertion, not used in coordinate calculation
or solving for M. No new calibration or mixed input may be substituted.

Mathematical covariance: the original numerator basis is a homogeneous cubic
linear system with the fixed residual base divisor removed. Pullback by a
projective coordinate isomorphism preserves its section values up to a common
nonzero local frame. Symmetric triple tensors multiply by the product of those
three common scalars, independent of tensor index. Hence all8theta coordinates
change by one projective scalar; group centering beta does not change. Exact
ideal reconstruction and crosschart comparisons check implementation of this
identity. This is standard projective covariance, not a novelty claim.

Use original geometric D1/Bimage certificates as explicit calibration and
verification setup, not a cold map or target secret. Same600second guard.
Freeze source/lock/protocol/binary and checkpoint before one run to RUN.txt/
RUN.stderr/EXIT.txt. Retain failure; no rerun without amendment. No timing,
complete mapper, independent replay or10xclaim. Inverse solving, broader charts,
sparse destination algorithm/full cost and genus2/4 remain open.

Build errors before freeze: renamed crate required offline lock regeneration;
copy extraction accidentally duplicated point_section and was corrected before
successful locked build. No scientific execution occurred during those fixes.
