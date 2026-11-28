# Written cubic-inverse and closed-coefficient obligations

Let K be a field of odd characteristic and L=K[t]/(t²-nu), with nu a nonsquare.
Write an element as (a,b)=a+b*t. The retained inverse for a nonzero input is

    N=a²-nu*b²,
    (a,b)^(-1)=(a*N^(-1),-b*N^(-1)).

For b=0, nonzeroness means a!=0. Then N=a² is nonzero and
a*(a²)^(-1)=a^(-1); the second component is0. Thus the candidate returns the
same element, in the same reduced cubic coefficient representation. At (0,0)
both code paths retain the original assertion before cubic inversion. At b!=0
the candidate literally invokes the unchanged old inverse. This is a written
field argument; a standard-Field Lean lemma is planned, not yet executed.
[pcc:unverified]

The equality follows from associativity and inverse uniqueness: multiplying
a*(a²)^(-1) by a gives1. Equivalently (a²)^(-1)=a^(-1)*a^(-1), and cancellation
leaves a^(-1). It introduces no curve-dependent root or sign. The valid cubic
constructor checks prime characteristic and a cubic without a base-field root;
degree3 makes that irreducibility. The quadratic tower constructor checks the
chosen nonsquare. A Lean proof over the standard Field interface will not by
itself construct the Rust field model or prove its Euclidean implementation.
Those bridges remain distinct from finite checks. [pcc:unverified]

For closed prime-input evaluation, u,v lift from Fp into K, and every selected
factor/bracket coefficient lifts from K into L with b=0. Addition, negation,
multiplication, nonzero inversion and exact polynomial quotient/remainder and
extended-GCD operations preserve the embedded subfield. Hence the resultant,
coordinate inverse, ordinate inverse, balanced correction and Cantor reduction
stay in K while the declared closed chart applies. Nonunit and degree-drop
conditions retain the exact old boundary route; the complete pointwise fallback
can use b!=0, where the old inverse is unchanged. The written induction explains
the optimization's scope without claiming that every pointwise operand lies in
K or that every boundary route is covered by17 witnesses. [pcc:unverified]

There are exactly five explicit CubicField.mul calls around one inner inverse
in the retained Field inverse: a²,b²,nu*b²,a*N^(-1),(-b)*N^(-1). For a!=0,b=0,
only the first and fourth have both operands nonzero. Direct cubic inversion
removes these five boundary calls, including two nonzero-operand calls. The
inner inverse remains one call but receives a instead of a²; its polynomial
Euclidean work can differ. This source-level calculation predicts counters,
not a runtime ratio or complete operation count. The paired native unit must
test it, and later timing must retain setup/online/verification accounting.
[pcc:unverified]

Once field operations agree, every unchanged polynomial, group and map expression
has the same value and branch conditions. The native comparisons exercise this
claim on the fixed operational maps, not on an enlarged class population.
Richelot degree, kernel, positive dual and DLP-preserving subgroup statements
remain those of14; this optimization supplies no new cross-prime transport or
destination-solver advantage. No novelty claim is made. [pcc:unverified]
