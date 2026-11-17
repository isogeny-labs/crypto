# Explicit polarized-kernel gates

Status: proposed algebraic deductions, all [pcc:unverified]. No new
scientific execution or speed measurement. Original models unchanged.

## Genus three: bounded self-isogeny exhaustion

For y²=x^7+c at (p,c)=(113,3),(421,2), retain the existing conditional
Frobenius identity pi=P(sigma), sigma(x,y)=(zeta7*x,y). Let
A=sigma+sigma²+sigma⁴, B=conjugate(A), beta=1+2A.
The cyclotomic identities give A+B=-1, AB=2, conjugate(beta)=-beta,
beta²=-7, so norm(a+b*beta)=a²+7b².

Proposed witnesses and expected distinct kernels:

|l|witnesses|count|
|---|---|---|
|2|A,B|2|
|7|beta|1|
|11|beta-2 and conjugate|2|
|23|beta+4 and conjugate|2|
|29|six Galois conjugates of eta29; 1+2beta and conjugate|8|
|37|3+2beta and conjugate|2|
|43|six Galois conjugates of eta43; 6+beta and conjugate|8|

eta29=1-2sigma²-2sigma⁵+4sigma⁶ has cyclic autocorrelations
(25,-4,-4,-4,-4,-4,-4). eta43=3-2sigma-4sigma²+2sigma³+2sigma⁵
has (37,-6,-6,-6,-6,-6,-6). Thus their proposed norms are 29 and43.
These identities, ranks, and distinct kernel counts need exact checking.

Integral Tate module justification: Q(zeta7) has degree6=2dim(J).
Milne Proposition10.23 gives rational rank1; a stable integral lattice
over the product of DVRs Z[zeta7] tensor Z_l is locally free rank1.
Thus J[l] is the regular cyclotomic module, including ramified l=7.
The frozen rank6 powerbasis check means pi generates that finite algebra.

At2,11,23,37 it is a product of two cubic fields, with two dimension3
submodules. At7 it is F7[epsilon]/epsilon^6, with one dimension3 ideal.
At29,43 six eigenlines pair only with inverse eigenlines under the
canonical polarization, giving eight isotropic choices. Norm witnesses
must cover these choices distinctly; other primes<=47 were excluded by
the frozen degree-gcd screen. Rosati sends sigma to sigma^-1; eta†eta=l
makes the quotient by ker(eta) isomorphic to J as a principally polarized
variety. This is a bounded conditional obstruction, not a general claim
about all primes, extension kernels, other polarizations, or genus4.

## Genus two: p251,l19 admission

For y²=x⁵+18 over F251 the retained polynomial reduces to (T²+4)²
mod19. Unramified semisimplicity and algebra rank2 imply minimal
polynomial T²+4. Put A=pi/2; then A²=-1 and A preserves the alternating
Weil form E since pi has multiplier251=4mod19. The form
H(x,y)=E(x,Ay)+iE(x,y), with i acting as A, is nondegenerate Hermitian
on a two-dimensional F361 space. Stable dimension2 F19 subspaces
are F361 lines; E-isotropy is equivalent to H-isotropy.
Diagonalization reduces their count to Norm(x/y)=-d for nonzero d.
The norm F361*->F19* has kernel size20. Prediction: exactly20 rational
maximal isotropic kernels. This is not a quotient construction or speed
claim; independently enumerate and classify polarized quotients next.

For p601,c7,l5 the polynomial is (T+1)^4 and algebra rank2, but the
algebra is ramified. Do not apply the semisimple Hermitian argument.
The nilpotent action and actual polarization require a separate gate.

## Sources and next execution contract

Primary sources checked: Milne, Abelian Varieties, Proposition10.23 and
polarization/Rosati sections, https://www.jmilne.org/math/CourseNotes/AV.pdf ;
Smith, sections1,3,9, https://arxiv.org/html/0806.2995 . These supply
background, not a proof or novelty claim for these finite deductions.
Sources checked (KB): revision9b2119e8fbac482b98b50521c0d604bb,
refresh2026-11-17T06:15:25Z (older than this work), lexical search;
source:b822311ff001956a5e6f3b2a8da76d65,
crypto-autoresearcher:knowledge/literature/KN-LIT-509.md points to
ordinary polarized isogeny graphs; source:1c8daaebce1785877fafc9ad73855272,
crypto:research/notes/index-calculus/RESEARCH_AUTOLAB_LOG.md contains
unverified proposals, not supporting certificates. Primary background and
the frozen frobenius-primes evidence remain authoritative inputs.

Next: freeze a standalone Rust checker, binary/input/source hashes and
protocol in Git before a single bounded invocation. Check integer norms,
modular ranks and canonical kernel distinctness against the frozen screen.
Do not promote finite algebra self-checks to independent verification.
Retain failures and do not overwrite earlier producers. Git custody while
S3 unavailable; no S3 polling. Full cost qualification follows any useful
quotient, with original targets and all transport/setup/solve costs.
