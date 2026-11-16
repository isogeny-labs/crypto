# Reconstructing the full map from a known rational reference divisor

2026-11-16. The fixed kernel3 differential now produces six rational-function candidates on the actual P-192 genus3 source curve. Every candidate has a one-dimensional training kernel and passes32 unused series coefficients. The complete224-term computation also passes4032 native branch,quartic and trace checks. These are producer results;global pairing and the actual recorded-kernel identification remain acceptance obligations. [pcc:unverified]

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

## Next acceptance step

Recover the pairing of x- and y-support coordinates over $\mathbb F_p(H)$,rather than accepting two independent multisets. Require exact quartic incidence,characteristic-polynomial and differential identities. The frozen [global protocol](GLOBAL_CORRESPONDENCE_PROTOCOL.md) gives these checks. Afterwards the original requirements still include the actual recorded kernel,degree/polarization,dual composition and the bridge to the completed public subgroup transfer. Individual descent remains a separate solver component.

Reproduce into new output directories with [check-formal-series.sh](check-formal-series.sh),passing the completed kernel3 study and retained elliptic-backmap result. The [frozen protocol](FORMAL_SERIES_PROTOCOL.md),source snapshot,pre/post-call hashes and process exit0 are retained. No speed or scalar-recovery claim is made. KB was unavailable;the retained primary Tian text and native source/target receipts supplied the source review. Full formalization is absent;the structural identity is limited to its stated field and formula.
