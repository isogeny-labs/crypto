# P-192 projective curve-map verification

Frozen before implementation/call, 2026-11-26. PR #5255; Conductor task T-2.
The user explicitly waived coordinate-screen and matched-rho requirements
for this map/exceptional-case task, retaining existing limits. No new PCC
registrations or performance comparisons. Earlier evidence remains immutable.

Question: can the retained actual quartic-to-P-192 morphism be evaluated at
every closed point on the quartic's line at infinity, and at the retained
affine denominator-zero base point, using separate exact arithmetic?

Use only the frozen kernel3 quartic and backmap from the actual local run.
Reimplement sparse bivariate reduction, homogeneous evaluation, local jets
and rational-function valuations on the separate OpenSSL arithmetic backend.
Reuse its exact factorization/residue-field code; import no producer GMP
arithmetic. The existing extended OpenSSL source is build-only with main
removed. Source/input/binary digests precede calls.

Before endpoint evaluation, independently check the full rational identity
in Fp[x,y]/(F): C^2 B^3 = D^2(A^3-3 A B^2+b B^3), where X=A/B,Y=C/D.
Require all four function polynomials nonzero modulo F. Recompute Hilbert
dimensions of the homogeneous ideal (F,A6,B6) at degrees 14 and 15; both
must be 12, using the original section degree six even when an affine
polynomial's actual degree is five. Under the retained smooth-quartic
assumption, the prior base-divisor argument gives curve-map degree six.

Form F(1,v,0), require degree four and nonzero Y^4 coefficient, and factor
it exactly over Fp. Replay all factors/multiplicities and irreducibility.
The chart X=1 covers all infinity points because F(0,1,0) is nonzero.
For every distinct factor, work in its actual residue field. At each root,
choose the first unit derivative among F_Y,F_Z, hold X=1 and use the other
coordinate as local parameter. Newton-lift the solved coordinate. Reject
a singular or unsupported local chart. The order of Z must match that
factor's line-intersection multiplicity.

Let M be the maximum actual total degree of A,B,C,D (here frozen input
inspection gives 11); use N=4M+1 coefficients, with M<=32. Each nonzero
polynomial function has local zero order at most four times its degree by
Bezout on the retained smooth irreducible quartic. Thus N suffices to find
every leading coefficient; no data-dependent precision increase is allowed.
Homogenize each function to its actual degree and include Z powers when
forming ratios. Finite valuations produce explicit elliptic coordinates;
pole valuations must have orders -2e,-3e with matching leading equation,
so the image is O. Retain coordinates, valuations, chart and multiplicity.
Recompute at the Frobenius-conjugate root for each nonrational closed factor
and compare with the conjugate endpoint; check full field closure.

Also lift the retained affine base point in Z=1, require its image O with
orders -2,-3, compare all N source-y coefficients with the old GMP series,
and compare 24 coefficients of t^2 X_E with the original scaled-X series.
N and the degree-five denominator bound guarantee this comparison precision.

Five controls must reject: changed X numerator in the global identity;
changed Y numerator; omitted infinity factor; changed highest local-jet
coefficient in the solved coordinate; and a changed base-point pole order.
These are actual polynomial/chart checks, without scalar recovery.

Limits: one allowed CPU, 1GiB address space per native invocation, strict
build <=300 seconds, native verification <=600 seconds. Use fresh output
directories; retain failures, errors, exits, source/input/binary versions and
digests, native subphase clocks and complete build/worker/wrapper costs.
Reporting/metadata costs stay outside the diagnostic wrapper. No cap widening.

Sources read: exact ELLIPTIC_BACKMAP_PROTOCOL.md and elliptic_backmap.cpp,
the separate-divisor and correspondence protocols/sources, and Galbraith,
*Mathematics of Public Key Cryptography*, corrected chapters 7 and 9:
https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch7.pdf (uniformizers,
valuations, Lemma 7.3.6: rational maps from a nonsingular curve to a
projective variety extend to morphisms), chapter 9 section 9.2. The existing
degree proof states its smoothness/Riemann--Roch assumptions explicitly.
KB unavailable: no loaded MCP, CLI or sibling executable; no novelty claim.

Scope: this completes the declared point-evaluation boundary panel for the
actual curve morphism. It does not implement every projective target input
of the full Jacobian transpose, prove its global polarization independently,
provide a signed verdict, or price original map synthesis.
