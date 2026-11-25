# The derived degree4 cover has no F7 degree2-by-degree2 factorization

Written consequence of GLOBAL_SPLIT_TRANSFER_C and the earlier complete
F7 automorphism/cover result941363, not a nativeB branch. Exact source:
https://github.com/isogeny-labs/crypto/blob/62daac1e0c57cf04009fb57952dd6975339facd5/research/chained_cm19_curve_maps_20261123/CURRENT_RESULT_B.md.
The primary committed result was read again before this derivation.
No extension-field automorphism census or novelty claim is inferred.
[pcc:unverified]

The complete fixed-frame census covers all57120 ordered images in the
17-point set. Any F7 automorphism of the smooth canonical plane quartic
lies in PGL3(F7) and maps a fixed general projective frame to one of
these general frames. Exactly6 maps pass the full quartic identity;
their full group is S3. All3 involutions are conjugate deck involutions
for the actual degree2 maps to E7, hence have genus1 quotients.
Thus no F7 involution has a genus2 quotient. [pcc:unverified]

Suppose the derived f4:C7->E7 factors over F7 into two degree2 maps
C7->D->E7. Since char7 does not divide4, both are separable. The first
map is a degree2 function-field extension and has an F7 deck involution.
The complete F7 automorphism result therefore forces D to have genus1.
It has an F7 point, the image of an existing C7(F7) point, so after
choosing origins the second degree2 map becomes an F7 elliptic isogeny
up to translation. Its dual would be an F7 degree2 isogeny from E7.
[pcc:unverified]

But E7(F7) has odd order11. A degree2 isogeny from E7 in char7 would
have an etale order2 kernel defined over F7, whose unique nonzero
geometric point is Frobenius fixed, contradicting the absence of
rational2-torsion. Hence the proposed factorization cannot exist.
The degree4 cover is therefore primitive with respect to factorizations
over F7. No assertion is made about factorizations over extension fields
or geometric monodromy/Galois closure. Explicit f4 functions and fibre
data remain uncomputed. [pcc:unverified]
