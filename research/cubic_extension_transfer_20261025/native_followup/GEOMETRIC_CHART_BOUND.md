# A bound for geometric chart availability

Status: agent-derived argument, not formally checked or independently reviewed.
This proves availability of representatives under the hypotheses below. It
does not prove totality of the current source formulas, interpolation, or
polynomial-system solver. It does not settle isogeny coverage.

Let C/F_p be a smooth plane quartic, let J=Pic^0(C), and assume
N1=#C(F_p)=p+1 and N2=#C(F_(p^2))=p^2+1. Fix a rational degree-three
reference divisor D0. Suppose #J(F_p)=r is prime. For the present transfer
fixtures the characteristic polynomial supplies

    r >= p^3+1-2*p^(3/2).

A good geometric representative of A in J(F_p) is the unique effective
D in the class A+D0, with three distinct geometric support points, all
outside a fixed line L at infinity. Such a D is also noncollinear.

## Counting the bad classes

Every rational degree-three line-bundle class has an effective rational
representative, by Riemann--Roch. Rational divisor classes are represented
by line bundles over a finite field (its Brauer group is zero).

1. Nonunique representatives: Riemann--Roch gives
   h0(D)=1+h0(K_C-D). A special degree-three class has the form K_C-P
   for P in C(F_p), since K_C-D has degree one. There are at most N1
   such classes. A nonspecial divisor cannot be collinear, since a
   containing line would give an effective K_C-D.
2. Nonreduced representatives: a rational degree-three divisor with
   repeated geometric support has the form 2P+Q with P,Q rational.
   A repeated closed point of degree >=2 would already have degree >=4.
   There are at most N1^2 such divisors and hence classes.
3. Support on L: C intersects L in a scheme of degree four. Each closed
   point of degree d<=3 in that support contributes at most
   #Sym^(3-d)(C)(F_p) possible divisors. There are at most four closed
   support points. Each count is at most
   S2=#Sym^2(C)(F_p)=(N1^2+N2)/2=p^2+p+1.

Thus the number b of bad classes satisfies

    b <= N1 + N1^2 + 4*S2 = 5*p^2+7*p+6.

The cubic local-jet encoding in projective_divisor.cpp independently
addresses how to represent supplied exceptional divisors. It is not used
to assume that an inversion algorithm can find them.

## A common multiplier for three images

Let S,T,S+T be nonzero in the prime-order group J(F_p). For each of
these three elements, m -> m*S, m*T, or m*(S+T) permutes the r-1
nonzero group elements as m ranges over F_r^*. The union bound therefore
excludes at most 3*b multipliers. If r-1 > 3*b, at least one common
multiplier has all three images in the good geometric domain. Scanning
any 3*b+1 distinct nonzero multipliers suffices in principle; this is a
quadratic bound in p and is not itself a faster DLP algorithm.

For p>=19 the stated order bound implies the required inequality.
Since p>=16, 2*sqrt(p)<=p/2. Consequently it suffices that

    2*p^3-31*p^2-42*p-36 > 0.

Writing p=u+19 gives

    2*u^3+83*u^2+946*u+1693 > 0  for u>=0.

The polynomial equality is shipped as an evaluated identity certificate
and is also checked by exact integer coefficient expansion. Those checks
do not formalize the geometric argument above.

If multipliers are independently uniform in F_r^*, success per trial is
at least 1-3*b/(r-1); k failures have probability at most
(3*b/(r-1))^k. This probability statement does NOT apply to deterministic
consecutive multipliers or correlated draws.

## Remaining implementation obligations

* Establish that the complete eight-section system and forward map are
  evaluated on every good representative, with no additional poles or
  unsaturated extraneous components.
* Implement and verify a generic inverter that returns all relevant sign
  choices on that domain, rather than assuming that a zero-dimensional
  polynomial system was solved completely.
* Account separately for extra formula-denominator bad loci. A union of s
  translates of Theta adds at most s*S2, but the actual implementation's
  denominators have not been proved to have exactly this support.
* Implement the controller and full native transfer. The present native
  projective-divisor routine starts from support coordinates, not Kummer
  coordinates. A recovery routine is not an inverse map.
* p=7 and p=13 are outside this bound and require separate coverage checks.

The source identity is preserved only if an already established injective
group homomorphism sends P,Q,P+Q to S,T,S+T. This argument constructs no
homomorphism and supplies no subgroup-preservation certificate by itself.
