# Fixed unbound F17 CAS integration check: interpretation and limits

This note records a bounded DEVELOPMENT algebra-library integration check. It
does not report a timing experiment, scaling experiment, new ECDLP algorithm,
sub-exponential bound, or complete CAS solution extraction. No additional CAS
run is requested by this note.

## Fixed input and retained result

The original unbound canonical input is
`/workspace/crypto/research/prime_one_shot_20261029/continuation/canonical/outputs/m3_t4_remainder.ms`.
Its SHA-256 is
`ff99de2267266794b3e72d56f0a53f2846db7bdc187ad01435a8e90ec9571300`.
All 33 variables were unknown; no source summands, scalar, or coefficient
assignments were appended. This is the arity-three remainder construction on
`E/F17: y^2=x^3+2x+2`, target `(0,6)`, membership domain `X^16-1`, with the
specified nonzero, distinct-root, target-disjoint chart. The independently
enumerated rational decomposition census has 28 models: case 73 in
`continuation/solver/checks/checks.tsv`. The census and unknown-coefficient
solver retain their model sets separately and agree on this case.

The backend invocation was msolve 0.7.5-1 with arguments
`-t 1 -g 2 -f INPUT -o OUTPUT`, under the predeclared 20-second wall cap and
2-GiB address-space cap. The retained protocol is
`/workspace/audit/unbound-cas-unit/PROTOCOL.md`. That directory contains the
input hash, version/help receipt, exit status, stdout, stderr, and printed basis
`remainder.gb`. Exit status was zero. Empty stdout/stderr and exit zero alone
are not a mathematical certificate.

The printed basis reports characteristic 17, the original 33-variable order,
graded reverse lexicographic order, and 243 polynomials. The independent native
staircase check found:

| Item | Retained value |
| --- | ---: |
| Printed polynomials | 243 |
| Minimal leading monomial generators | 243 |
| Monic printed polynomials | yes |
| Maximum total degree of a printed polynomial | 3 |
| Standard monomials | 28 |
| Standard monomials of degrees 0, 1, 2 | 1, 21, 6 |

The maximum printed polynomial degree of 3 is **not** the maximum degree
reached during F4 or any other intermediate computation. No intermediate
degree trace was collected. Neither that number nor the staircase degree
histogram establishes a solving-degree, last-fall-degree, regularity, or
asymptotic complexity bound.

## Independent leading-ideal count

The checker source is `/workspace/audit/gb_staircase.cpp`; its retained output
is `/workspace/audit/unbound-cas-unit/staircase.txt`. The exact native build and
check commands used were:

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror /workspace/audit/gb_staircase.cpp -o /tmp/gb-staircase-check
/tmp/gb-staircase-check /workspace/audit/unbound-cas-unit/remainder.gb > /workspace/audit/unbound-cas-unit/staircase.txt
```

The checker parses and collects expanded coefficients modulo the printed
characteristic. It recomputes each leading monomial from every term under the
printed grevlex order; it does not assume that the first printed term is
leading. It removes divisible leading-monomial generators, checks for a pure
power of every variable, and enumerates the finite monomial order ideal by
multiplying standard monomials by variables and rejecting multiples of leading
generators. It prints every retained monomial as well as the count and degree
histogram. The pure-power check certifies that this *printed leading ideal*
has a finite staircase; implementation caps cause an error rather than a
partial count.

For a genuine Groebner basis `G` of the source ideal `I`, the standard
monomials of `LM(G)` form an `F17`-basis of `F17[variables]/I`. Thus the count
28 would be the algebra's vector-space dimension, also its finite scheme
length or degree. Counting that staircase is independent of a separating
linear form and of rational-univariate parameterization.

This checker does **not** establish Buchberger's criterion, independently
reduce all S-polynomials, prove equality of the printed ideal and the source
ideal, or certify the backend's source parsing and provenance. Its result is
therefore conditional on the printed list being the claimed source ideal's
Groebner basis. It is an independent leading-ideal count, not an independent
full Groebner or source-ideal certificate.

## Relation to the 28 rational models

No field equations `z^17-z` were added for every coefficient variable. A
finite quotient degree normally counts algebraic-closure solutions with
multiplicity, rather than merely rational decompositions. The selected
arity-three geometry matters when comparing that degree with the native
rational census.

Here `X^16-1` splits into distinct roots in F17. For a geometric decomposition,
each point over a rational abscissa is either Frobenius-fixed or sent to its
negative. Let `S` be the sum of the nonrational points. Since the total target
and the rational summands are rational, `S` is rational, whereas Frobenius also
sends it to `-S`. Hence `2S=O`. The curve has odd group order 19, so `S=O`.
One nonrational point cannot sum to zero; two would be opposite and violate
the distinct-abscissa chart; three would leave the total target equal to the
identity. The target here is affine and nonidentity. Thus there is no
mixed-twist decomposition in this particular arity-three chart. This argument
does not extend automatically to arity five.

The expected comparison also uses the norm/chart correspondence and unique
auxiliary quotient/Bezout lifts under their specified degree bounds. Those
mathematical conditions must be retained when interpreting the canonical
system; equality of two integers alone does not prove that correspondence.
If 28 distinct full assignments were lifted and checked against every source
equation, and the printed basis were certified for that source ideal, degree
28 would additionally certify that these exhaust a reduced rational scheme
with coordinate algebra `F17^28`. No such all-assignment CAS extraction or
independent source-ideal certificate was performed in this integration check.
The matching native census, scoped geometric reasoning, and staircase support
the fixed integration expectation without replacing those stronger checks.

## Why a default F17 univariate representation is unsuitable

There are 28 rational solutions but only 17 field elements. Any F17-valued
linear form, and indeed any F17-valued function on those rational solutions,
takes at most 17 values. It cannot separate all 28 points. The reduced product
algebra `F17^28` consequently cannot be generated by one element over F17.
A separating-linear-form or shape-position frontend can therefore fail even
when the multivariate system and its Groebner basis are valid. Such a failure
would not prove inconsistency or refute the rational census.

A multivariate extraction procedure or an explicitly accounted extension-field
representation could avoid that particular obstruction. Neither is needed to
count standard monomials. This unit result supplies no evidence about their
cost on larger fields, other targets, other arities, or general prime curves.
