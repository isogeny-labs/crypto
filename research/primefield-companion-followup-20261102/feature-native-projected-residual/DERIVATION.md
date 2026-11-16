# Geometry residual in the projected image

The existing geometry residual is a homogeneous representation of

    G = (-1)^(w*k+s+e) R_main R_tail A^(2w) B^(2k) C^2,

where A,B,C are the center, baby and prefix recurrence products. R_tail and C
are1 when s=0. The former (-tail)^(2s) factor is1 because geometry-only tail=1.
Here w,k,s,e are the inherited partition and all recurrence operands are nonzero.

Write p-1=q*m, q=2 or4, m odd, pi(z)=z^m and chi(z)=z^((p-1)/2).
For q=2 all the displayed even-power factors project to1. For q=4,

    pi(G) = (-1)^(w*k+s+e) pi(R_main R_tail) chi(A^w B^k C).

This uses multiplicativity only; the resultant computations, including their
additions, remain in the full prime field. [pcc:unverified]

The geometry-only recurrence has v_next=delta*v_current^2/v_previous.
Modulo nonzero squares, its class is delta*v_previous, since inverses and
original values have the same square class. Start representative previous=-1,
current=1 and retain the inherited sign flip at the baby/center boundary.
Accumulate H using each baby current representative if
(k mod2) XOR (j<s), and each center if w is odd. Then chi(H)=chi(A^w B^k C).
The projected residual is returned as the canonical pair [pi(G),1].
Comparisons with the old homogeneous pair use cross multiplication, not raw
coordinate equality. [pcc:unverified]

## Domain and implementation

Every original coordinate difference is checked nonzero, including the final
baby update that is not itself a baby-product factor. With nonzero initial pairs,
these checks are exactly what prevents zero recurrence coordinates. Both
resultants remain unchanged and a zero resultant product is rejected. Grid
shape and supported projection order are checked. The caller still supplies
valid partition, prime modulus and canonical coordinates through the source.
The public paired source keeps its full-value reference and ratio validator.

Order-two projection performs no recurrence-value arithmetic after difference
checks. Order-four projection keeps two square-class representatives and one
accumulator, instead of full homogeneous recurrence pairs and their products.
One projection of the resultant product and one quadratic character (order4 only)
finish the residual. This is a structural change; no complete-cost gain has been
measured. In particular the full polynomial/resultant engine remains.

## Development evidence

Seven selected native methods pass. A deterministic synthetic residual panel
covers2500 canonical geometry inputs over p=5,7,13,29,509 with partition orders
7,13,19,31,479:1349 valid outputs match the projected old residual,1151 cases
are rejected by both. Two additional malformed shape/order cases are rejected.
These synthetic grids need not be elliptic-curve grids; they exercise the residual
algebra and zero-domain behavior, not a curve cohort. The paired source additionally
retains499 subgroup rows,100 splits,1375 advances,450 skips and all prior selected
predicate/cache/closure checks. Each invocation filters75 methods.
Claim PCC-20261107-51c5f9 is unsigned producer evidence. [pcc:unverified]

No scalar recovery, timing comparison or cloud job ran; original4563-case cohort
unchanged. Regeneration reproduces the tested generated files. The raw original
residual is imported only as a test reference here; both implementations share
the polynomial module, so no independent-review claim is made.

Sources checked: feature-native-multiseed-factor/residual.rs, current projected
consumer and previous sign-projection derivation/primary cyclic-group review.
KB tools unavailable in this runtime. No novelty-priority claim: this is the
specific specialization of multiplicativity and square classes to our residual.
Next: inspect whether the root resultant product can be evaluated directly in the
small image without its current full-value accumulation; preserve additive work
where projection does not commute. Complete-cost comparison remains pending.
