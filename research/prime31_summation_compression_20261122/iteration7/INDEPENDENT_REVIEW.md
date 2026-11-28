# Independent review: growing smooth-torus field family

Date: 22 November 2026 America/Los_Angeles / 23 November 2026 UTC.
Reviewer: the higher-arity mathematical worker, independent of the hourly
task that authored the family note. Written review only, **[pcc:unverified]**.
No numerical experiment, prime search, curve generation, point count, native
run, paid compute, remote mutation, or signed/formal proof was performed.

## Reviewed input and disposition

Reviewed `SMOOTH_TORUS_FAMILY.md` at source commit
`5fb95f63bd8a38de306abf14b93ed54f032fbe8a`, Git blob
`d135329cf6490134982ec30da13a8cc187826461`, SHA-256
`d4898626a557ff220ed7ddbb4d75042cd2909bdc654e54d30aa1f129a195fa66`.
The local read-only source is `remote-5fb/SMOOTH_TORUS_FAMILY.md`.

**Accept the core growing-field and split-fibre deductions at their stated
scope, with the bibliographic correction and cost qualifications below.**
The existing note correctly separates torus smoothness, elliptic-group
structure, and the missing relation-solving algorithm. Its generic warning
that fibre splitting alone does not imply elliptic liftability is correct;
the additional fixed-curve averaging theorem supplied with this review now
removes that density gap when the fibre is selected on an already specified
odd-order curve and its size is capped appropriately. It does not construct
a large-prime-order elliptic family. **[pcc:unverified]**

## 1. Least-prime input: correct theorem, incorrect book title

The linked `primes.pdf` is Dimitris Koukoulopoulos, *The Distribution of Prime
Numbers*, not *Multiplicative Number Theory I*. Chapter 27, Theorem 27.1
states Linnik's least-prime theorem with an absolute effectively computable
exponent L. Its statement is sufficient for the note's polynomial bound.
The source's title and theorem were read directly. [S1]

Writing Q=2^k and d=4Q, the congruence class d-1 is coprime to d. If p is its
least prime, then d-1<=p<=C d^L, and therefore

    (p/(C 4^L))^(1/L) <= Q <= (p+1)/4.

All constants are independent of k. The definition may repeat the same prime
for successive k; the lower bound still forces p_k to tend to infinity and
therefore gives infinitely many distinct primes. No Mersenne-prime infinitude
assumption is present. Dirichlet alone gives existence, while Linnik is the
quantitative ingredient. **[pcc:unverified]**

The source gives an effective existence bound, but this review supplies no
numerical L or implementation with a useful uniform bit-cost constant. A
least-prime definition also requires ruling out earlier candidates, not just
finding one prime in the class. Prime search, primality certification, and
any least-prime certification remain family setup. The inequality is a lower
bound on Q as a power of p; it is not Q=Theta(p^(1/L)) or a prescribed
factor-base exponent. **[pcc:unverified]**

## 2. Full rational Dickson fibre: exact and squarefree

For odd p, the cyclic kernel T of the norm from F_(p^2)^* to F_p^* has order
p+1. For u in T, u^p=u^(-1), hence x(u)=u+u^(-1) lies in Fp. The parameter-one
Dickson identity and doubling recurrence agree with the primary paper of
Bluher and also follow directly by induction. [S2]

Suppose q=2^k, 4q divides p+1, z is in T with z^(2q)!=1, and zeta has exact
order q. Then c=z^q+z^(-q) lies in Fp and is not +/-2. Indeed, for w=z^q,
w+w^(-1)=2 or -2 implies (w-1)^2=0 or (w+1)^2=0 respectively, so w=+/-1.
The q points x(z zeta^j) are distinct: equal traces imply equality or inverse
equality of the torus elements; the second case would force z^(2q)=1. They
exhaust the roots of the monic degree-q polynomial D_q(X)-c. This proves
splitting and squarefreeness, including the exact c exclusion. **[pcc:unverified]**

The enumeration statement is correctly output-sensitive: writing all q
coordinates takes q slots and O(q) extension-field operations once z and zeta
are known. In this power-of-two family p=3 mod4, so F_(p^2)=Fp[i], i^2=-1,
is available without a nonsquare search. Frobenius conjugation computes each
inverse in T; a fresh field inversion per enumerated point is unnecessary.
An O(log q) Dickson evaluation is still not q-root extraction.
**[pcc:unverified]**

### A full primitive torus generator is unnecessary

Uniform u in T gives zeta=u^((p+1)/q), uniform in T[q]. Since q is a power
of two, zeta has exact order q exactly when zeta^(q/2)!=1; the success
probability is one half. Exponentiation and that order test use O(log p)
extension-field operations per trial. No factorization of the odd part of
p+1 is needed. An exact uniform norm-one element can be obtained from a
uniform point t on P^1(Fp) by u=(t+i)/(t-i), with t=infinity mapped to 1.
These are algebraic algorithm specifications, not an executed sampler.
**[pcc:unverified]**

For a deterministic value of c, use the same method to obtain z of exact
order 4q, then set zeta=z^4. The element z^q has order four, so c=0. This is
a valid full fibre. It does not guarantee that this particular c=0 fibre has
many elliptic lifts. The stronger density theorem instead varies c.
**[pcc:unverified]**

## 3. Elliptic-group claims and the trace-three example

The order identity gives

    gcd(q,#E(Fp)) = gcd(q,p+1-t) = gcd(q,t).

For odd t and power-of-two q this gcd is one. Every group homomorphism from
E(Fp), or from one of its subgroups, to T[q] is consequently trivial. This is
an obstruction to that homomorphic transfer; it does not exclude all
nonhomomorphic coordinates or a homomorphism to every possible torus group.
In particular it must not be broadened from T[q] to the whole T without
checking its odd factors. **[pcc:unverified]**

Waterhouse's Theorem 4.1 includes every trace t with |t|<=2 sqrt(p) and
gcd(t,p)=1 as an ordinary isogeny class. Thus the trace-three example is valid
for these p>3: its order is p-2 and it is non-anomalous. [S3] Existence of an
isogeny class is not a priced algorithm for finding a curve in it, nor a
guarantee that p-2 has a suitably large prime factor. Trace three must remain
an ordinary/non-anomalous example, not an established cryptographic family.
**[pcc:unverified]**

## 4. Additional fixed-curve density result

The parent proposed, and this reviewer independently checked, a stronger
averaging argument. Its full proof, exact constants, sampler and cost scope
are recorded in [FIXED_CURVE_FIBRE_DENSITY.md](FIXED_CURVE_FIBRE_DENSITY.md).
It uses the classical Hasse-Weil bound only in genera one and two; the
original theorem and a modern author text were checked directly. [S4, S5]

Let E/Fp be any fixed nonsingular short-Weierstrass curve of odd rational
order, p>=4096. Let 4<=q<=sqrt(p)/16 be a power of two with 4q dividing p+1.
For the M=(p+1)/(2q)-1 regular split Dickson fibres, put B(c) equal to the
number of coordinates that lift to E(Fp). Then

    | average_c B(c)/q - 1/2 |
       <= (3 sqrt(p)/2 + q/2 + 1) / ((p+1)/2-q)
       <= 8/sqrt(p) <= 1/8.

At least one sixth of these fibres therefore have B(c)>=q/4. Conditioned
on c=Tr(z^q) not being +/-2, a uniform norm-one seed z samples these fibres
uniformly because each has exactly 2q seeds. Selecting a qualifying fibre by
exact screening has expected O(q polylog p) field work and O(q) storage,
after the field/curve input has been supplied. **[pcc:unverified]**

This removes the need to change a preselected odd-order source curve merely
to obtain a dense liftable fibre. If that source already has prime order,
all its rational points are automatically in the target subgroup. For an
odd composite group with a prescribed proper prime subgroup, rational
liftability alone still does not establish subgroup membership.
**[pcc:unverified]**

For the Linnik family, replace Q by the largest power of two not exceeding
min(Q,sqrt(p)/16). Enlarge the fixed L to at least two if necessary. The
resulting q still satisfies q=Omega(p^(1/L)), and 4q still divides p+1.
Thus the density result retains a polynomially growing full fibre. It does
not select a desired exact asymptotic exponent or construct prime-order
curves over the chosen fields. **[pcc:unverified]**

## 5. Remaining boundaries that must survive integration

* **Source-compatible base selection is setup.** It depends on the fixed
  curve, not on the relation target. Charge it once when applicable, and
  charge target-specific sign recovery separately. The original phrase
  "target-specific factor-base lifting" should not force a fresh full-base
  lifting pass on every target by definition.
* **A prescribed fibre need not be dense.** The new theorem selects c from
  the regular fibres. It does not prove B(c)>=q/4 for c=0, a previously
  frozen c, or every fibre individually.
* **A prime-order source remains an input.** Neither Linnik nor trace-three
  existence proves an unconditionally generated family of prime-order
  non-anomalous elliptic curves with the required cost. The density theorem
  applies if such a source is already specified.
* **Proper subgroup membership is separate.** Clearing a cofactor changes
  point abscissae and generally changes the advertised Dickson predicate;
  it cannot silently preserve this factor base.
* **There is no relation-yield or rank theorem here.** A density B=Theta(q)
  does not prove random-like signed sum distributions or independent rows.
* **The target solver is still missing.** The O(q polylog p) algorithm builds
  a factor base. It is not an empty-target oracle and does not improve the
  maintained linear group-complement query by itself.
* **Exponent optimization needs the actual q-p relation.** A lower bound
  q=Omega(p^(1/L)) does not ensure q can be chosen near p^(1/4), for example.
  The full-rank budget must use the guaranteed or actually proved base
  exponent, together with all setup and subgroup-construction costs.

All these conclusions are written mathematical analysis and remain
**[pcc:unverified]**. This review does not alter iteration-six pinned files,
its frozen native correctness source, or its current CI evidence.

## Sources checked

**S1.** Dimitris Koukoulopoulos, *The Distribution of Prime Numbers*, author
preliminary version, Chapter 27, Theorem 27.1, printed p. 287:
https://dms.umontreal.ca/~koukoulo/documents/publications/primes.pdf .
Corrects the book title in the reviewed note; no numerical L is imported.

**S2.** Antonia W. Bluher, *Permutation properties of Dickson and Chebyshev
polynomials with connections to number theory*, arXiv:1707.06877, introduction:
https://arxiv.org/pdf/1707.06877 .
Primary-paper verification of the parameter-one functional identity.

**S3.** William C. Waterhouse, *Abelian varieties over finite fields*, Annales
scientifiques de l'Ecole Normale Superieure, 4e serie, 2 (1969), 521-560,
Theorem 4.1, printed p. 536:
https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf .

**S4.** Andre Weil, *Sur les courbes algebriques et les varietes qui s'en
deduisent* (1948), Theorem 13, Corollary 3; text inspected through Denise
Vella-Chemla's explicitly labelled 2022 LaTeX transcription, p. 61 of that
PDF: https://denisevellachemla.eu/Weil-courbes-varietes.pdf . The reviewed
statement is the classical point-count bound, not a claimed new improvement.

**S5.** Steven Galbraith, *Mathematics of Public Key Cryptography*, author
extended/corrected Chapter 10, Theorems 10.7.1 and 10.7.5, Exercise 10.7.9:
https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch10.pdf .
Cross-check of the genus and point-count convention used in S4.

The cited Encyclopedia of Mathematics link failed with HTTP 502 in this
runtime; the recurrence was checked by direct induction and S2 instead.
The research-KB interface and local wrapper were unavailable in the bounded
workspace inventory. Review used the immutable lane note and the primary
sources above; it is not a literature-wide novelty audit.
