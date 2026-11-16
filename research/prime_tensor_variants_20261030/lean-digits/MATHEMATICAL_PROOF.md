# Readable proof of the integer-slot bridge

Let `x,y` be integers and put `t=y-x≠0`. Let
`i=padicValInt 3 t`, `m=3^i`, and let `c` be the class of `t/m` in `ZMod 3`.
For any integer `z`, define its selected digit as the class of `z/m` in
`ZMod 3`, using Euclidean integer division. These definitions cover negative
integers as well as ordinary nonnegative packet slots.

The maximal-power divisibility property gives `m∣t`. Integer Euclidean
division therefore satisfies

\[
\frac{x+t}{m}=\frac{x}{m}+\frac{t}{m}.
\]

Here each division sign denotes integer Euclidean division. This identity
uses divisibility of the added summand; no identity about unrestricted
floor-division is assumed. Casting it into `ZMod 3` gives

\[
\operatorname{digit}_i(y)=\operatorname{digit}_i(x)+c.
\]

The class `c` is nonzero. If it were zero, then `3∣t/m` by the exact
integer-cast criterion for `ZMod 3`. Multiplying by `m`, and cancelling the
exact division permitted by `m∣t`, would give `3^(i+1)∣t`. Maximality of
`i` then gives `i+1≤i`, a contradiction. The checked quotient lemma proves
this reasoning for every natural base other than one; the ternary
specialization needs no separately assumed primality instance.

Now suppose additionally that `0≤x,y<3^a` and `x≠y`. Then
`0<|y-x|<3^a`. If `i≥a`, divisibility by `3^i` would imply divisibility by
`3^a`. A nonzero multiple of `3^a` cannot have absolute value below `3^a`.
Thus `i<a`, which constructs the required coordinate as an actual element
of `Fin a`.

For arbitrary packet labels `g : Fin a → Tag A`, with `A` any additive
commutative group, let the integer slot map extract these digits pointwise.
The proved digit equality and the proved condition `c≠0` are exactly the
two premises of `TernaryPacket.slot_locality`. Applying that theorem yields
equality of the selected row component at slot `x` and the selected column
component at slot `y`. This is
`TernaryDigits.bounded_integer_slot_locality`.

This proof supplies the integer-digit bridge. The Cauchy kernel's support,
graph ports, coefficient-ring precision, mode enumeration and complexity
remain separate formal obligations.
