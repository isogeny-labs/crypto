# Implicit intervals: a uniform-list closure check

Prospective scope/proof plan38ae306f73; pre-edit assembly probe4cab4d5c88.
This checks one concrete statistical assumption behind a proposed coordinate
filter: independent uniform points in a large abscissa interval do not retain
the old interval constraint with near certainty after addition. It does not
rule out every interval join or an algorithm using correlated/adaptive lists.
The finite Fourier endpoint is generic; the elliptic analytic application
remains a hand argument using explicitly cited external results.
PCC-20261115-730397 is unverified. [pcc:unverified]

## 1. Actual subset count and its Fourier bound

Let G be a finite additive commutative group of order r>=1, let S be an actual
subset of size s, and define

    F(chi) = sum_(P in S) chi(P),
    T(S) = |{(P,Q,R) in S^3 : P+Q=R}|.

Each ordered input pair P,Q has at most one R; when s>0, the probability that
independent uniform P,Q in S have P+Q in S is T(S)/s^2. Suppose M>=0 and
|F(chi)|<=M for every nontrivial complex character of G. Then

    |T(S) - s^3/r| <= M*s,
    |Pr[P+Q in S] - s/r| <= M/s.                         (1)

Proof. Existing finite character orthogonality gives the exact identities

    sum_chi |F(chi)|^2 = r*s,
    r*T(S) = sum_chi F(chi)^2*conjugate(F(chi)).

The trivial character contributes s^3 to the second identity. Triangle
inequality, the explicit flatness assumption and the first identity give

    |r*T(S)-s^3|
      <= sum_(chi nontrivial) |F(chi)|^3
      <= M*sum_chi |F(chi)|^2
       = M*r*s.

Cancel r>0 and divide by s^2 for the probability form. No curve-specific
group representation or assumed sampling equation is inserted into this
counting argument. [pcc:unverified]

## 2. An abscissa interval on an actual prime-field elliptic curve

For this application take a nonsingular short-Weierstrass E/F_p, p>3, with
full group G=E(F_p) of prime order r. This full-group restriction keeps the
character application explicit; no result for every cofactor/subgroup model
is asserted from it. Let I be a cyclic interval in F_p and

    S = {P in E(F_p)\{O} : x(P) in I}.

Write psi for an additive field character and chi for a group character.
For nontrivial psi, Kohel--Shparlinski's character estimate for f=x gives

    |sum_(P != O) chi(P)*psi(x(P))| <= 4*sqrt(p).

Their interval Fourier L1 bound is

    sum_psi |sum_(t in I) psi(t)| <= p*(1+ln p).

Only f=x is used: its pole divisor is 2O and p>3, so the pole order is
coprime to p. Do not apply this step to an arbitrary Artin--Schreier-
exceptional rational function or include the both-trivial character term
inside the square-root bound. [pcc:unverified]

Expand the interval indicator by additive character orthogonality. For a
nontrivial chi, the trivial psi contribution is -|I|/p, because the full
group character sum is zero and the removed point O contributes one.
All other psi terms are bounded by the two displayed estimates. Thus an
explicit safe choice for the hypothesis in (1) is

    M = 1 + 4*sqrt(p)*(1+ln p).                           (2)

The same expansion with trivial chi gives the cardinality estimate

    |s - (r-1)*|I|/p| <= 4*sqrt(p)*(1+ln p).              (3)

These are hand applications of external analytic theorems, not formal curve
character-sum laws in crypto-lean. Kim--Tibouchi's conference manuscript
section2/Lemma3.1 supplies a second source for this Fourier/interval setup.
No coset-equidistribution theorem is misread as saying that S itself is
uniform on the entire elliptic group. [pcc:unverified]

## 3. Balanced parameters and the failed retention assumption

Put n=ceil(log2 r), choose b=2^(O(sqrt(n))) with b growing, and take an
interval of length floor(p/b). Hasse's bound gives r=p+O(sqrt(p)), so (3)
implies s~r/b. Combining (1) and (2) yields

    Pr[P+Q in S] = s/r + O(M/s)
                 = (1+o(1))/b,

since b^2*log(p)/sqrt(p) tends to zero. The old interval constraint is
therefore retained with probability near its ambient density, not near one.
This refutes the proposed near-certain closure for independent uniform
subset inputs at these parameters. It does not by itself establish a time
lower bound for all coordinate-aware merge algorithms. [pcc:unverified]

In an XOR tree, two points already satisfying old zero-bit constraints
automatically retain those constraints; only the new bits are filtered at
the next level. Equation(1) shows why this property cannot simply be assumed
for uniform abscissa-interval points. A small description of I supplies a
membership predicate. It supplies neither an efficient pair enumerator nor
the required multilevel output distributions.

## 4. Controls, exceptions and scope

- Empty S is covered by the count theorem; the probability expression is
  used only when s>0. Taking S=G gives exact retention one and zero
  nontrivial Fourier coefficients, consistent with(1).
- The interval set S is invariant under point negation. Thus the early
  identity probability is exactly Pr[P+Q=O]=1/s. Counting this as a useful
  early relation does not restore near-certain retention; at the balanced
  parameters it is 2^(-n+o(n)).
- Scalar intervals in the additive group Z/r have a different addition
  geometry: sums of short integer representatives remain in an interval of
  comparable length. Wagner's scalar-interval join uses this representation.
  Access to x(P) is not access to a scalar interval coordinate for P.
- A real solver's selected outputs need not be independent uniform on S.
  Correlated lists, selective/adaptive output rules, other coordinate maps,
  and different filters are outside this inference. Any proposed use of(1)
  must establish its input-distribution hypotheses; matching support alone
  does not establish them. Global clean-base rejection also needs a separate
  conditioning argument; independent uniform subset points are the model here.
- This analysis supplies no fast selector. A successor needs a constructive
  join, its actual output law at every level, and a capped cost/useful-yield
  bound charged through scalar recovery.

## Evidence and sources

Build004 at shared5b7a9d2a9a2638a1251a805d20d31349c35fa904 checks the
finite-subset energy, Parseval and actual triple identities and the scaled/
unscaled flatness-conditioned deviation. The canonical all-source/axiom
gate now audits149theorems,0failures,0trusted native exceptions. This module
contributes8audited declarations, including2definition equations. Complete
four-build/gate logs and source hashes are in checks/interval-laws/.
PCC-20261115-ab6d22 remains unverified; no signed verdict is claimed.
[pcc:unverified]

The final assembly probe applies the endpoint to both generic subsets and
Mathlib's existing Weierstrass affine point group, with finiteness and
flatness explicitly supplied. All six named laws' axiom prints list only
propext, Classical.choice and Quot.sound. The probe does not prove the
elliptic character bound, subgroup parameters or a solver's sampling law.
[pcc:unverified]

Shared PR15 merged as56a7aec51b80be1f8f42eb3f5c2cd1979744367a.
Hosted43746326008executed zero steps with the explicit account billing-lock
annotation. No hosted pass or separate --fresh checker pass is asserted;
lean4checker is not available in this runtime. [pcc:unverified]

PROOF-MAP.md lists inherited structures/laws and the explicit missing
formal analytic/sampling bridges. Unlike the retrospective A1/A2 review,
this A3 six-field plan and floor-law probe precede the first generic edit.
No numerical case, fitted exponent, timing or paid job.

Primary sources read: Kohel--Shparlinski ANTS-IV2000 sections3--4,
Theorem1/Corollary1/Lemma3; Kim--Tibouchi's NutMiC2019 conference manuscript,
section2/Lemma3.1; Wagner2002 full author version section2. Exact links and
PDF hashes are in checks/interval-primary-sources.txt. KB remains unavailable;
no novelty or exhaustive literature-search claim is made.
