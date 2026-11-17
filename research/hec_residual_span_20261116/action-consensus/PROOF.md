# Surviving true action and target consensus

Conditional correctness argument, not a new attack or measured gain. Standard
finite-group and principal-divisor facts; written proof, not formally verified.

## Certified input contract

C/F_q:y²=x5+c, c nonzero, characteristic not2or5; z is a primitive fifth
root in F_q. The entire rational Jacobian has certified prime order ell!=5,
and D!=0. Thus every rational point class is a multiple of D. T, public core
C0 and symbolic seed P are in this group, with verified D=[sm]C0. No seed log
is supplied. The prior strong-solves whole-order receipts give N=ell for
(p,c,ell)=(101,2,7481),(251,18,67931),(601,7,411491), conditional on their
stated Frobenius/order proofs. Point order alone would not establish this
whole-group premise. No extrapolation to arbitrary models or cofactors.

## A true primitive action exists

sigma(x,y)=(zx,y) preserves rational J. Every endomorphism of the cyclic
prime-order group J(F_q)=<D> is scalar multiplication, hence sigma=[lambda0].
For finite P, the zeros of y-y(P) are the five sigma translates of P,
with multiplicities, and its pole divisor is5infinity. Even the fixed x=0
case gives the same identity with multiplicity5. Their principal divisor
therefore gives (1+sigma+...+sigma4)[P-infinity]=0. These classes generate J,
so Phi5(sigma)=0. Thus Phi5(lambda0)=0 in F_ell; lambda0!=1 because ell!=5.
In particular lambda0 has order5 and Phi5 splits into four distinct field
roots (5 divides ell-1). The true component is evaluation Z->lambda0.

## Induction through producer operations

Let k0,s0 be the actual logs of T,P to D, whose existence follows from the
whole-group premise. A ring label h=(a(Z),b(Z),c(Z)) means the conditional
identity Q=a(lambda0)T+b(lambda0)P+c(lambda0)D in the true component.
Initial labels C0=(0,0,sm^-1) and P=(0,1,0) satisfy it.
Signed orbit translation multiplies labels by +/-Z^e, valid on evaluation.
A principal relation from anchor aD+T and three residual points is exact in
J. When two labels are known and the third root becomes a new representative,
negation/sign gives its correct label. No inversion of a potential ring zero
divisor occurs in this step. Induction preserves every geometric label.

When all labels are known, the resulting linear constraint in(S,k) is
satisfied by (s0,k0) at lambda0. Correct CRT elimination may narrow components
only when their augmented systems are inconsistent; it cannot reject this
solution. Consistent zero-pivot components must stay alive. This is exactly
the condition enforced by the separately qualified CRT implementation.

## Acceptance consequences

1. If only one action root remains, it must be lambda0. An additional
   sigma(D)=[lambda]D multiplication adds a diagnostic check but is not
   logically required under the stated certified-input/solver assumptions.
2. If every surviving component determines k, and all values agree on k*,
   then k*=k0 even if several action roots remain. Components with free k
   block acceptance. Empty survivors also block acceptance.
3. Keep independent [k*]D=T in the proposed runtime contract. This is a
   practical output check even though the algebraic argument proves the
   value under its hypotheses. It is not removed by this note.

This supplies a prospective contract change, not permission to erase old
charged action checks or relabel prior counter data. The scalar/group/order
and CRT hypotheses are essential; absent certificates, noncyclic full groups,
unprojected cofactors, invalid geometric rows or unsound nonunit handling
invalidate the argument. Synthetic CRT tests qualify an accessor, not this
entire geometric theorem or a full producer speedup.

## Ring accessor criterion

Each live CRT branch has squarefree modulus m dividing Phi5 and a normalized
two-variable basis ordered(S,k). If k is determined its k-pivot gives k=r(Z)
mod m. It is constant over every root of that branch exactly when the reduced
r has degree<=0. Require this in EVERY branch and identical constants across
branches. This yields consensus without enumerating action roots. A varying
r, missing pivot or no live branches returns no consensus.

Background: original principal-divisor/Jacobian formalism in Stoll,
Arithmetic of Hyperelliptic Curves, section4:
https://www.mathe2.uni-bayreuth.de/stoll/teaching/ArithHypKurven-SS2019/Skript-ArithHypCurves-pub-screen.pdf
Current-input order evidence: ../strong-solves/RESULTS.md and CONTROL.stderr.
The induction and acceptance deductions above are explicit here, not an
attribution of a new theorem or performance result to that source.
