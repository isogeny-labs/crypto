# A constant-size, phase-free description of the projected source

T-1786, 2026-11-07. Producer derivation and software evidence, not independent
verification. [pcc:unverified]

## Domain and output

Let E/F_p be nonsingular short Weierstrass, p>3 prime; Q nonzero of odd prime
order r>=7, r!=p, in the original source's coprime domain gcd(r,p-1)=1.
Write p-1=q*m, m odd, q=2 or4, pi(z)=z^m. Let W_n=psi_n(Q) with native
normalization psi_1=1,psi_2=2y. All quantities here lie in F_p.
Define

 beta = W_(r+1)^2 W_2 / W_(r+2),
 h=(r-1)/2, S=h(h+1)(2h+1)/6, b=-S mod q.

Then the projected raw source (before calibration at P) equals

    pi(beta^b * product_{j=2}^h W_j).                  (1)

The constructor stores only r,q,start=2,end=h,b. It performs no geometry,
exponent-map construction or subgroup enumeration. The reference evaluator
still requests every W_j; this is a compact description, not a cheap interval
product evaluator. The existing original half-product construction is prior
work: the contribution here is the phase-free correction and checked bridge
from the actual current signed source expression to this direct descriptor.

## Ward reduction supplies the correction without jets

Theorem4 of Lauter--Stange (2008), citing Ward, gives, at a rank-r zero,

 W_(kr+v)=W_v alpha^(v*k) beta^(k*k),
 alpha=W_(r+2)/(W_(r+1)W_2), beta=W_(r+1)^2 W_2/W_(r+2),
 alpha^r=beta².

Use signed remainder -h<=v<=h, v!=0, j=abs(v). Negative v contributes
W_v=-W_j. Put A=pi(alpha), B=pi(beta). Because r is odd, it is invertible
mod q; A=B^(2*r^-1 mod q). Thus a signed monomial product W_n^e_n folds to
half-range indices with a beta exponent

 b_fold = sum_n e_n*(k_n²+2*r^-1*v_n*k_n) mod q.        (2)

Each negative signed remainder flips the outer sign when e_n is odd.
fold.rs retains both corrections and rejects a multiple of r. No derivative
normalization, radical or unknown scalar label is needed. The point-dependent
correction uses three nonzero division values; the r-torsion check uses W_r.
When b_fold=0 the evaluator skips all beta-specific requests.

The formula works for indices divisible by p too; it is not derived by dividing
by the differential of [n]. This replaces the initial derivative-based plan
and its unnecessary separability restriction. The existing phase implementation
is used only by the original-source reference, not the new evaluator.

## Why the complete source folds to one interval

For q2/4, r²=1 modulo q and r^-1=r modulo q. Define image-valued terms

 U_n=pi(W_n)*B^(-n²).

The Ward formula and n=kr+v give U_n=U_v and U_-n=-U_n. Multiplicativity
and the division identity give

 U_(a+b) U_(a-b) = pi(x([b]Q)-x([a]Q)) U_a² U_b².

This passes through no addition: each coordinate difference is still computed
in F_p. U_1=B^-1 acts as its normalization seed. In the actual source program,
the combined seed exponent is divisible by4, as established in the inherited
feature-native-seed-cancellation/DERIVATION.md. Therefore changing the projected
seed to B^-1 while evaluating its normalized indices leaves the final projected
raw expression unchanged. We can now apply the block-pairing identity to U.

Put h=2wk+2s+e, N=h+1. Babies have a=N+j for0<=j<w; centers have
b=N+w+2wi for0<=i<k. The main pair products use b-a=w+2wi-j and
b+a=r+1+w+2wi+j. Reducing the latter modulo r, these pairs cover each integer
1..2wk exactly once. The partial center is b=-(s+e); using j=0..s-1,
b+a covers2wk+s+1..2wk+2s, while b-a modulo r covers2wk+1..2wk+s.
If e=1 the singleton is U_h=-U_N. The explicit signs in the source are exactly
the signs of these pair products and singleton. Hence the total is
product_{j=1}^h U_j. Substitution of U_j and W_1=1 proves (1).

The baby/center recurrence realizes these U values: initial previous=-U_N
is U_(N-1)=U_h, current=U_N; the boundary negation is the corresponding
negative-index value for the 2w stride. Thus the pairing argument applies to
the actual recurrence, not merely a separately specified product.

This is equality of values on the finite subgroup under the source domain.
It does not equate the ambient rational divisors from the previous audit.
The order-nine support witness remains valid and unchanged.

## Implementation and actual cost

Fold::new implements (2) for an explicit signed exponent program; Fold::compile
uses an arithmetic-preserving generated copy of the previous audit compiler.
Compact::new bypasses that compiler and produces the interval descriptor from r,q.
Compact::value evaluates the interval literally, uses a single ValueDivision
cache, verifies W_r=0, and conditionally computes beta. Raw projected value
still requires public-base calibration to become a source sign.

The current constructor domain is odd7<=r<=1000001 and q2/4; this finite limit
is inherited for audit comparisons, not a mathematical requirement of the formula.
Backend prime3<p<=u64::MAX/2 and prime r are caller-certified. Canonical curve/point,
nonsingularity and nonzero y are checked by ValueDivision. Torsion, projection
order and nonzero factors are checked by the evaluator. No scalar input occurs.

The descriptor has five u64 fields and no term vector. Its construction uses a
constant number of bounded integer operations. Evaluating its reference still
has h-1 factor requests, plus torsion and possible beta requests, and a growing
division cache. No total-runtime improvement is claimed. A large r by itself
must not be treated as permission for an unbounded interval evaluation.

## Recorded native validation

validation-02: four selected methods pass;61 imported methods filtered. Four
bounded commands (test build90s,test30s,audit build90s,audit30s) exit0. Fourteen
transitive Rust/fixture source hashes are checked before and after; source,
compiler,binaries,commands and log hashes are retained. Generated prior.rs only
renames the old main and appends an accessor; generation.json pins both texts.

* 26 comparisons of both folded and compact values against the old projected
  source: all nonzero points for F5/r7 and F7/r13, and8 selected F509/r479 points.
* Sign-flip mutations fail all26 checks; omitting nontrivial beta correction
  fails at least one. These are software mutation controls, not statistics.
* 1001 single-term Ward checks across the three public bases, indices1..2r+3
  excluding multiples of r. This includes indices divisible by characteristic.
* Compact descriptor agrees with the expanded/folded program for all48 odd
  r from7 through101 at both q2 andq4 (96 symbolic cases, primality not needed
  for this combinatorial check). Bad projection, zero-index residue and
  non-r-torsion controls reject.

The audit output for the original three fixtures is:

| r | q | interval | beta exponent | outer negative |
|---|---|---|---|---|
|7|4|2..3|2|false|
|13|2|2..6|1|false|
|479|4|2..239|0|false|

validation-01 is the earlier passing three-method check before adding Compact;
its exact fold.rs snapshot is retained. Unused imported/research-code warnings
remain in logs. No native test failed. Shared division/group primitives mean
this is not independent verification. Original4563-case cohort unchanged;
no experiment panel, timing comparison, cloud job or recovery ran.

## Sources, scope and next target

KB unavailable: no loaded tools in previous discovery; wrapper status again
reports no executable. Reviewed actual source-divisor bridge, original phase,
normalized-transport and seed-cancellation derivations. Read Lauter--Stange,
The elliptic curve discrete logarithm problem and equivalent hard problems for
elliptic divisibility sequences, Theorem4 and section3:
https://arxiv.org/pdf/0803.0728 . The eprint mirror returned403; arXiv primary
paper accessible. Ward quasiperiodicity is reused, not claimed novel. Its
normalization formulas must retain signs; no pairing or extension-field
transfer is invoked. Earlier Miller component remains separate.

Next target is now explicit: evaluate this signed half-interval product more
cheaply than visiting every division value, or find a compact recurrence for
interval products with all boundary information included. Merely squaring the
interval or taking the complete symmetric product removes the sign needed by
the source. The compact constructor alone does not solve that evaluation task.
