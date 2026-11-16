# Degree-two deck quotients of the n=7 W1 Hess cover

This is a derived character count and a **prior-work obstruction**, not a
new attack. Hess, *Generalising the GHS attack* (2004), Section 8.3,
proves that the norm after the GHS transfer is zero for intermediate
fields over the rational x-line. The natural quotient studied here is
one of those intermediate fields. A classical genus-seven GHS map for
the same coefficient is a different presentation and correspondence.

For one n=7 W1 cubic arm, the geometric AS character space M of the
genus-14 Hess cover has F2-dimension four. As an F2[sigma] module it
splits as `M = <e> + U`, where `<e>` is the trivial one-dimensional
module and U is the irreducible cubic module. The character e has
only one simple pole and genus zero. Each of the other 14 nonzero
characters has two simple poles and genus one. The conductor sum
checks `2g(C)-2 = -32 + 2 + 14*4 = 26`, so g(C)=14.

Each deck involution corresponds to a rank-three character hyperplane
H in M. There are 15 such hyperplanes. Seven contain e; their quotient
has one one-pole and six two-pole characters, hence
`2g-2=-16+2+6*4=10`, genus six. Eight exclude e; all seven nonzero
characters have two poles, hence `2g-2=-16+7*4=12`, genus seven.

Only one of these involutions is defined over k. Indeed the deck group
is dual to M and its sigma-fixed part has dimension one: the dual of
U has no invariant vector because p(1)=1, whereas the dual of <e>
does. The unique k-rational involution has character annihilator U,
so its quotient is precisely the natural genus-seven D. All other
degree-two deck quotients appear only after extending constants.

Hess Section 8.3 and Lemma 16 show that the norm composition from the
source E(K) through this intermediate quotient is zero. Thus its
hyperellipticity, even on the classical coefficient locus, does not
itself transfer the source logarithm. This gives an exact exclusion of
all k-rational degree-two **deck** quotients of this Hess cover as
subgroup-preserving attack targets. It does not exclude a different
correspondence, a non-deck automorphism, or the separate classical GHS
presentation.

Primary source: <https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf>,
Section 8.3, especially the zero norm statement following Lemma 16.
