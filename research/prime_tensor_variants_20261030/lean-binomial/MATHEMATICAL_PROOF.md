# Readable proof of the checked core

Put `N=2^i`. Lucas's theorem modulo two says

\[
\binom nN \equiv
\binom{n\bmod2}{N\bmod2}
\binom{\lfloor n/2\rfloor}{\lfloor N/2\rfloor}\pmod2.
\]

For `i>0`, the first factor is one because `N mod 2=0`. Inductively stripping
`i` binary digits leaves `choose (n/2^i) 1 = n/2^i`. This proves, for every
natural `n` and `i`, that the parity of `choose n (2^i)` is the parity of
`floor(n/2^i)`. This argument is checked directly against Mathlib's Lucas
theorem; it does not test a bounded collection of integers.

For a negative signed integer, write `t=-(n+1)` with `n≥0`. The actual
generalized binomial satisfies

\[
\binom{-(n+1)}N=(-1)^N\binom{n+N}N.
\]

The sign is one modulo two. The natural formula gives parity
`floor((n+N)/N)=floor(n/N)+1`. The exact Euclidean division formula gives
`floor(-(n+1)/N)=-(floor(n/N)+1)`. Negation is the identity in `ZMod 2`,
so the same digit formula holds for every signed integer.

Now take a nonzero signed `t` and `i=padicValInt 2 t`. The integer
`q=t/2^i` is odd: divisibility `2|q` would imply `2^(i+1)|t`, contradicting
the maximality of `i`. The signed digit formula therefore makes
`Ring.choose t (2^i)` odd. This is the central signed-node parity lemma in
the written construction.

The rational polynomial is defined using the actual Mathlib generalized
binomial polynomial `Ring.choose X N`. The `Ring.map_choose` theorem under
polynomial evaluation and the integer-to-rational embedding proves that its
integer values are exactly the generalized integer binomials above. Thus
these parity facts concern the construction's binomial polynomial, rather
than a separately defined numerical sequence without a proved bridge.

For `H_(i,p)(T)=(1-binomialPolynomial (2^i)(T))^p`, the binomial value at zero
is zero since `2^i>0`. Hence `H_(i,p)(0)=1`. At a nonzero node of valuation
`i`, oddness makes `1-Ring.choose t (2^i)` divisible by two; raising to the
`p`th power proves divisibility by `2^p`. The rational polynomial's value
is proved to equal the cast of this integer power.

The stronger statement that every coefficient of every scaled node chart
is integral, or divisible by `2^p` at the selected level, needs the
coefficientwise Vandermonde/product argument. That part is not supplied by
these node-value theorems and remains an open formal obligation.
