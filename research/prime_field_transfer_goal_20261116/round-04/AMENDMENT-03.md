# Version04: rational Padé candidates

The three u7 setup timeouts remain preserved; there is no u7 model or
source-point image. The constant-denominator recognizer at u5 accepts1/4
public inputs at every original precision. Do not change those records.

Try a classical rational representation beta=A(W)/B(W), with Eisenstein
coefficient polynomials of degrees at most da=floor((d−1)/2) and
db=d−1−da. This is informed by repository CT84, not a novelty claim.
There are d+1 coefficients over K, hence a nonzero solution to A−beta*B
in the d-dimensional field; this linear-algebra existence observation
does not bound coefficient heights, LLL success or computational cost.

The new integer congruence lattice has2d+2 variables. It encodes
A(w(P))−w(Q)*B(w(P))=0 at the chosen p^N place. Use the same exact LLL,
delta3/4 and10000-step cap, inspecting reduced basis rows only. Reject a
zero denominator or one vanishing at the source residue. Screen the
homogenized C(A,B) over the fixed auxiliary field. Then compute the exact
global beta=A*B⁻¹ modulo C, require C(beta)=0 and source specialization,
and evaluate this same beta at the target as before. All native controls,
Smart decoding and source checks remain binding.

Use the existing u3/u5 exact model files, checking their source hashes;
do not repeat or extend failed u7 setup. Qualify all18 u3 nonidentity
auditor inputs at1/2/4/8/16/32 and the four fixed hash-derived u5 inputs
at4/8/16/32, each call capped60seconds. Retain rejection, incomplete
coverage, LLL cap and timeout; none is a successful map. There is one
original call per bin; any diagnostic or further modification is versioned.

The core still receives coordinates, model and precision only, without
a scalar, point table or orbit label. Converting a candidate into the
dense field basis and exact certification are charged target-dependent
work. This representation still has degree/dimension growing with r;
no cryptographic-size efficiency or arbitrary-curve conclusion follows.
