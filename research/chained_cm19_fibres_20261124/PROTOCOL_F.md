# Prospective polynomial-only local evaluator and coefficient leaves F

Exact fixed correctness construction on the one actual C7/E7 map, not an
experiment, cryptographic search or benchmark. Inherit immutable E source
5cd0a093/dataf062d9/writtenb0036d and all original A/B/C negatives/locks.
The only map-function input is the complete original E output, parsing
q,p4,p6,F_C and Q8 with their degree bounds; no fitted coefficients or new
denominator. A remains an independent oracle after evaluation.
[pcc:unverified]

For supplied C point with X=1, compute exact implicit curve series through
t^24, choosing z-z0 when F_y!=0 and y-y0 otherwise. Each unknown coefficient
is determined by the same nonzero linear derivative; require all25 F
coefficients zero. Evaluate q,p4,p6 as exact truncated polynomial series.
Nonzero sections have vanishing orders bounded8,16,24, so all leading
coefficients are present. If vx>=2vq and vy>=3vq, divide leading coefficients
when equality holds, otherwise constant0. If polar, require both deficits
2e and3e with the same positive integer e and return actual elliptic O.
Require the resulting finite value satisfies the actual E equation.
[pcc:unverified]

The evaluator accepts only the supplied point and parsed polynomial object.
It receives no case index, pole support, oracle image or historical f4 value.
Only afterwards translate by R=(3,5)=4(1,3) and compare all17 independent
values with A. The E conic/value-jet records already show3 rational pole
support points and2 residual rational zeros; require exactly those counts,
not a selected passing cohort. Keep complete25-term q/p4/p6 series at all
exceptional points and every rational result. At the first residual point,
change its actual leading y-numerator coefficient by q_lead^3, require
actual-image equality rejects the changed limit, and restore pristine.
Do not require the mutant to fail the elliptic equation if it happens to
give another valid E point. No control tuning or unchanged failure retry.
[pcc:unverified]

Written completeness: q|X=0=YZ and F_C|X=0=2Y^4+2Y^2Z^2+Z^4 imply every
geometric q=0 point has X!=0. Smooth formal implicit series therefore give
all exceptional evaluation charts. The finite/pole rule follows from the
actual global E morphism and orders of x,y at O (2,3), with ramification e.
The degree bounds justify the truncation. Source F implements F7 arithmetic
and qualifies only the full17 rational population; extension-field code
and global formal geometry are not claimed by that finite qualification.
[pcc:unverified]

Separately, lift the actual original coefficient representatives0..6 to
standard integers and form full unreduced products for
p6^2-p4^3-2p4*q^4-6q^6 and F_C*Q8 using checked i128 arithmetic. Require
support total degree<=12, enumerate ALL91 monomials, retain integer lhs/rhs/
difference and check every difference mod7 zero. Emit standard Int closed
Lean leaves containing precisely those91 lhs/rhs literals plus #print axioms.
These leaves are source-bound partial coefficient arithmetic, not a full
Polynomial/curve/Jacobian theorem or proof of native expansion correctness.
The emitted source must be separately frozen/published/fully read back
with its own formal protocol/lock BEFORE its first Lean invocation.
[pcc:unverified]

Before all strict F calls, publish/read every whole F input plus lock.
Pinned Rust1.99.1, all seven strict phases/version/format/compile/Clippy/
polynomial-evaluation, -Dwarnings, unchanged opt3/overflow/debug/codegen1,
120s2GiB one worker; fresh RAM dirs, complete argv/streams and source/tool/
copied0555 qualifier pre/post guards. Preserve first failures and whole
ELF; no suppressions, resource increases or S3 experimental pass.
Exceptional local theorem and full signed scientific standing remain
unsigned. No ECDLP advantage or proper-subfield prime-field descent.
[pcc:unverified]
