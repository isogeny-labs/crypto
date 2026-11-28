# Quartic gluing protocol correction before implementation

QUARTIC_GLUING_PROTOCOL.md remains immutable as the initial plan. Its
wrong-sign rejection prediction128 of256 is a counting error. The other
criteria and populations are unchanged. This correction is made before
new native source or execution,not in response to a failed native run.
[pcc:unverified]

On (Z/4)^2 the determinant is odd precisely when the reductions of both
vectors form a basis over F2. There are6 such ordered bases over F2,and
each of their four coordinates has two lifts,so6*2^4=96 ordered pairs.
Inverting only the F pairing in an anti-isometric graph changes the
pairing product to e_E^2,which differs from1 exactly for odd determinant.
The corrected prediction is therefore96 rejections among256 kernel pairs
per model. Retain both a positive inverse-pairing check and this exact
negative control. No mathematical gluing hypothesis is weakened.
[pcc:unverified]
