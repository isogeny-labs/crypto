# A global genus3 correspondence from a known rational reference divisor

2026-11-16. The six reconstructed rational functions give a global correspondence from the actual P-192 genus3 source curve to the kernel3 quartic. Exact quartic incidence,the y-characteristic polynomial,and all three differential trace identities pass. Two arithmetic implementations produce identical normalized map coefficients. The later [quotient proof](POLARIZED_QUOTIENT.md) now supplies all26 kernel witnesses and the degree27/polarization3 argument(PCC-20261116-3f440b,PCC-20261116-ce4a7b,PCC-20261116-8559fc). These results have unsigned producer standing;generic transpose-dual execution and individual descent remain separate. [pcc:unverified]

The complete coefficients and local series are in [formal-series-v1/result.json](evidence/formal-series-v1/result.json),with unsigned PCC-20261116-b97918. This stage uses the same original prime field as P-192. It consumes neither a hidden discrete logarithm nor a seven-variable inverse solution.

## Why the starting divisor is known

Let $H:Y^2=h(X)$ be the exact monic degree7 source model and let $C$ be the reconstructed smooth quartic. The candidate derivative is the retained matrix $D=\sqrt{s}M_0$. Its source basis is $X^j dX/(2Y)$ and its target basis is $(x,y,1)dx/F_y$.

Choose three known rational points on $C$ whose x-coordinates are distinct,with $F_y\ne0$ and noncollinear support. The fixed backward scan selects retained indices225,224,223. Their sum is $E_{\rm ref}$. A degree3 divisor represents the zero class relative to itself,so the formal initial condition at source infinity is known:

$$[E(0)-E_{\rm ref}]=0.$$

If the candidate isogeny exists with this derivative,its germ can therefore be represented by $[E(z)-E_{\rm ref}]$. Translating the divisor reference preserves invariant differentials. The noncollinearity condition makes this Abel chart regular. This is the local reconstruction approach of Tian section4.3,with the reference-divisor freedom discussed in Remark4.4;the algebra below is native and explicit.

## Differential recurrence

At source infinity take $X=z^{-2}$ and $Y=z^{-7}w(z)$,with $w(0)=1$ and $w^2=z^{14}h(z^{-2})$. The three source velocities are

$$(-z^4/w,-z^2/w,-1/w).$$

Write the moving divisor as a monic cubic $U(T)$ and a quadratic $V(T)$,so its three support points satisfy $U(x_i)=0,y_i=V(x_i)$. In the cubic algebra modulo $U$,solve

$$S_{ik}=\operatorname{Tr}(c_iT^k),\qquad S H=D(-z^4/w,-z^2/w,-1/w)^t,$$

where $(c_0,c_1,c_2)=(T,V,1)$. Its motions are

$$\dot x=F_yH,\quad\dot y=-F_xH,\quad
\dot U=-(U_T\dot x)\bmod U,\quad
\dot V=(\dot y-V_T\dot x)\bmod U.$$

A fixed1643-node arithmetic graph evaluates these coefficient recurrences over the original field. Unit denominators are checked before inverse-series expansion. Separately tracked rational branches supply the curve and interpolation controls. All4032 checked coefficients vanish as required;the singular-seed and altered-branch controls also pass. [pcc:unverified]

## Rational reconstruction

The reconstructed functions are the three elementary symmetric x-functions and three elementary symmetric y-functions. The frozen search bound is conditional on the proposed polarized3 identification:four translated target theta divisors pull back to polar degree at most36 on the source curve. The ansatz is

$$f(X,Y)=\frac{A(X)+B(X)Y}{C(X)},\qquad
\deg A,\deg C\le36,\quad\deg B\le32.$$

There are107 linear unknowns. For each function,the first192 coefficients give rank106 and nullity1;the remaining32 coefficients pass. The bounds,seed and precision were fixed before the computation. A passing local fit is insufficient to prove a global map. [pcc:unverified]

| Function | Training rank / nullity | Heldout coefficients |
|---|---:|---:|
| $x_1+x_2+x_3$ |106 /1|32 pass|
| $x_1x_2+x_1x_3+x_2x_3$ |106 /1|32 pass|
| $x_1x_2x_3$ |106 /1|32 pass|
| $y_1+y_2+y_3$ |106 /1|32 pass|
| $y_1y_2+y_1y_3+y_2y_3$ |106 /1|32 pass|
| $y_1y_2y_3$ |106 /1|32 pass|

Structural certificate `IDC1h4ad7b19e8ec8a81c` checks the generic third Newton identity at32 derived points with an added-one corruption control in the mandated field $2^{61}-1$. PCC-20261116-3a64de is unsigned. This structural record does not verify P-192 coefficients or the isogeny.

## Executed global pairing and differential checks

Let $K=\mathbb F_p(X)[Y]/(Y^2-h(X))$. The fitted x-functions define a monic cubic $U(T)$ over $K$;the fitted y-functions define a monic cubic $Q(W)$. Compute the polynomial gcd of $F_C(T,W)$ and $Q(W)$ in $(K[T]/U)[W]$. Its monic linear result pairs the two coordinate multisets and supplies a quadratic $V(T)$. The explicit correspondence is

$$U(T)=0,\qquad W=V(T).$$

Every coefficient is retained as $(a(X)+b(X)Y)/c(X)$ in [global-correspondence-v2/result.json](evidence/global-correspondence-v2/result.json),unsigned PCC-20261116-fe2b13. The complete record is about41KiB. The native calculation checks the following exact identities. [pcc:unverified]

$$F_C(T,V)=0,\qquad Q(V)=0\pmod U,$$

as well as the full characteristic polynomial of multiplication by $V$. Differentiating in $K$,with $Y'=h'/(2Y)$,gives

$$T'=-U_X/U_T,\qquad W'=V_X+V_TT'.$$

The quartic derivative equation and each of the three identities

$$\operatorname{Tr}\left((T,V,1)\frac{T'}{F_y(T,V)}\right)
=D\frac{(1,X,X^2)^t}{2Y}$$

pass exactly. The rational limits at source infinity recover the chosen reference divisor. Altering the paired y-coordinate rejects. Thus the calculation establishes global incidence and the declared differential beyond the heldout local-series tests. [pcc:unverified]

The first implementation uses Euclidean cubic inverses;its generator inverse controls pass but it omitted per-call source-field inverse assertions. That limitation is retained in PCC-20261116-9d7084. The second uses adjugate multiplication matrices and checks every source-field/cubic inverse by multiplication,plus two fixed cubic-unit and zero-nonunit controls. Both produce byte-identical normalized $U,V$ and differential traces. This is same-producer agreement,not independent verification. The original successful result is preserved. [pcc:unverified]

Structural records `IDC1hbeea2608f845f93f` and `IDC1h3718f832316891d6` check the generic adjugate-column annihilation formulas at32 derived points,with added-one rejection,in the same limited $2^{61}-1$ field. PCC-20261116-181d2a is unsigned. They do not verify the actual P-192 coefficient arithmetic or the global geometric identification.

## Executed source-class evaluator and public bridge

The evaluator now consumes an actual source Mumford divisor $(u,v)$ of degree $d\le3$. It evaluates the global coefficients in $\mathbb F_p[S]/u$,with $Y=v(S)$,and forms the rank3 algebra defined by $U(T)$. A full power basis in this $3d$-dimensional algebra gives a closed target divisor of degree $3d$. Every source denominator inverse,basis solve and target curve equation is checked. This also handles the nonreduced degree2 divisor in the doubling fixture. [pcc:unverified]

The target class is the resulting effective divisor minus $dE_{\rm ref}$. Adding $(3-d)E_{\rm ref}$ and a common auxiliary degree3 divisor to the positive representative gives effective degree12;subtracting $3E_{\rm ref}$ plus that same auxiliary divisor recovers the desired class. The unchanged large-model KM backend evaluates these classes.

| Executed source-class control | Outcome |
|---|---|
| Actual pullback of public $G$,including branch shift | Agrees with earlier $f^*G$ |
| Actual pullback of public $Q$,including branch shift | Agrees with earlier $f^*Q$ |
| Separate source point $P$ and $-P$ | Images cancel |
| Source doubling $2P$ | Image equals twice image of $P$ |
| Source addition $P+\pi^*G$ | Image equals the target sum |

The six source divisors have degrees3,3,1,1,2,3;their closed images have degrees9,9,3,3,6,9. The test completes19 target addflips. Full source/target divisors and class coordinates are retained in [generic-bridge-v1/result.json](evidence/generic-bridge-v1/result.json),unsigned PCC-20261116-b54698. No hidden scalar is consumed by this evaluator. [pcc:unverified]

## Next acceptance step

The [kernel-check design](ACTUAL_KERNEL_PLAN.md) has now executed,including the nonkernel control,and the [quotient proof](POLARIZED_QUOTIENT.md) gives the exact degree/polarization argument. Next implement the generic transpose dual and exceptional-chart interface,then restricted individual descent. The existing elliptic norm and public transfer are already executed components.

Reproduce into new output directories with [check-formal-series.sh](check-formal-series.sh),passing the completed kernel3 study and retained elliptic-backmap result. Then use [check-global-correspondence-v2.sh](check-global-correspondence-v2.sh) with the fixed formal receipt. The [frozen formal protocol](FORMAL_SERIES_PROTOCOL.md),[global protocol](GLOBAL_CORRESPONDENCE_PROTOCOL.md),[adjugate control protocol](GLOBAL_ADJUGATE_PROTOCOL.md),source snapshots,pre/post-call hashes and process exits0 are retained. No speed or scalar-recovery claim is made. KB was unavailable;the retained primary Tian text and native source/target receipts supplied the source review. Lean and lake are unavailable,so no compiled formalization is supplied;the structural identity is limited to its stated field and formula.
