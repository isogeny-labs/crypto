# One-prime CM carrier construction

The new lead replaces twisting an existing conductor-7 character by a ray
character with constructing the infinity-type-one character directly. The
level drops from 49p to 7p. Both levels refer to modularity constructions;
neither alone supplies a map on the target subgroup.

Let K=Q(sqrt(-7)), O_K=Z[alpha], alpha²-alpha+2=0. Its class number is one
and its units are ±1. Choose a split prime p=2nk+1 with k odd, a prime
l above p, and a primitive character eta:(O_K/l)^×→mu_(2n). It is odd.
For an ideal (gamma) prime to l define
phi((gamma))=gamma*eta(gamma mod l). Replacing gamma by -gamma changes both
factors' signs, so the definition is independent of generator. This
algebraic Hecke character has infinity type one and exact conductor l:
eta is nontrivial modulo the prime l, while gamma≡1 mod l gives phi=gamma.

[Molin–Page, Theorem37](https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf)
therefore gives a weight2 CM newform of level |D_K| Norm(l)=7p and
nebentypus epsilon_K*eta restricted to rational integers. That character
is even and has order 2n: at7 its nontrivial quadratic part is independent
of the primitive part atp. The corresponding curve is the smooth compact
modular curve for Gamma_H, H=ker(epsilon_K*eta)⊂(Z/7p)^×. It is a degree2n
character cover of X0(7p), dominated by X1(7p). Its Jacobian contains the
newform abelian factor. The modular abelian variety construction and good
reduction at2 are used here in their usual prime-to-level form; these
geometric assertions are written arguments, not Lean theorems in this study.

## Exact coefficient field and local factor

The coefficient field lies in K Q(zeta_n). These fields are disjoint for
our prime n≠7, so its degree is at most2(n-1). At2 the two split primes
have generators alpha and beta=1-alpha. Let i,j be the eta exponents of
their residues, using the two roots of X²-X+2 modulo p. Then

 a2=alpha*zeta_(2n)^i+beta*zeta_(2n)^j.

CM15 enumerates the full automorphism group of K Q(zeta_n) on this element.
Writing a2=zeta^j+alpha*(zeta^i-zeta^j) shows that two conjugates agree
exactly when their two signed roots agree, since 1,alpha are independent
over Q(zeta_n). Every winner has orbit size2(n-1). Thus the coefficient
field is exactly K Q(zeta_n), and the modular abelian variety has
dimension2(n-1), not merely a predicted dimension.

CM15 multiplies every conjugate Euler polynomial in the exact ring
Z[alpha,zeta_n], reduces alpha²-alpha+2 and Phi_n, and requires its result
to be rational integral and equal to the product of actual trace-zero
Weil polynomials. Even i or j supplies the trace+1 Koblitz factor E1;
odd supplies trace-1 E0. The full exponent orbit is checked; a lone a2
match is never substituted for an Euler factor.

| n | first qualifying p | level | a2 orbit | genus | untwisted factors |
|---:|---:|---:|---:|---:|---|
|5|11|77|8|61|A5(E1) × A5(E0)|
|11|23|161|20|309|A11(E0)²|
|83|499|3493|164|54781|A83(E1) × A83(E0)|
|131|263|1841|260|45589|A131(E1)²|

These are first qualifying primes in the protocol's odd-k≤99 scan,
not a minimality theorem over all modular curves or all characters. Failed
primality, inert primes, nonprimitive residues and insufficient a2 orbits
stay in cm15-one-prime-carriers.json. For n=83 the smaller167 is inert.

## Quadratic diamond twist supplies the other sign

The diamond group of the character cover is C_(2n). Its unique order2
element delta is an F2-defined curve automorphism. All primitive Galois
conjugates of the nebentypus have odd exponent, so delta acts as -1 on
both holomorphic and antiholomorphic parts of the newform factor. The
quadratic F4/F2 twist of the curve by delta consequently replaces Frob by
-Frob on this factor. This exchanges E0 and E1 and preserves genus. In
particular the twist of the n=131 carrier has A131(E0)² in its Jacobian.
The characteristic is2, but the descent extension F4/F2 is separable;
twisting by an order2 automorphism does not require tame ramification of
a cover in this argument. No conductor3 auxiliary character is required.
Native checks verify delta²=1 and its sign on every primitive character;
the curve/Jacobian descent and action on the newform factor remain written.
Honda–Tate/Tate's finite-field isogeny theorem identifies these reductions
up to F2 isogeny, not by an explicit subgroup injection.

## Genus: two independent computations

For N=7p, the X0 index is mu=8(p+1), there are four cusps and no order2
elliptic points. The number e3 of order3 elliptic points is4 if p≡1 mod3,
otherwise0. Hence g0=(mu-12-4e3)/12. The character cover is unramified at
cusps (N is squarefree). The elliptic stabilizers act trivially because
3 is coprime to2n for our n. Thus g=2n(g0-1)+1.

Independently CM15 enumerates every coset in
P1(F7)×P1(Fp)×C_(2n), constructs the actual right S,T,ST permutations,
checks S²=1 and (ST)³=1, and computes
2g=2+index-cycles(S)-cycles(ST)-cycles(T).
All four winners are below the frozen ten-million-coset cap and pass.
This check does not use the Riemann–Hurwitz genus as an input.

## External control and remaining construction

The retained LMFDB database queries confirm
[77.2.l.a](https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/77/2/l/a/):
weight2, CM discriminant-7, dimension8, character order10. Its complete
Euler polynomial at2 is exactly A5(E1)*A5(E0). The old even ray twist
[539.2.f.c](https://www.lmfdb.org/ModularForm/GL2/Q/holomorphic/539/2/f/c/)
has dimension8, character order5, and polynomial A5(E1)². CM15 parses
and independently verifies both retained database rows. No matching131
row returned in the bounded query; this is missing database coverage,
not evidence against the construction.

Next acceptance test: construct an explicit Gamma_H level77 model over F2,
its newform idempotent/correspondence and a verified map on the n=5 subgroup.
For n=131, first construct the diamond twist and its correspondences with
exact denominators; local-polynomial equality does not bound a subgroup
kernel. Equations, map evaluation, subgroup preservation and divisor
arithmetic are unimplemented. No IC efficiency conclusion is drawn.
