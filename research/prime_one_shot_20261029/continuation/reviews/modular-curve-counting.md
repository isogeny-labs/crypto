# Primary-source check: counting auxiliary curves with rational 2-power torsion

2026-10-30 UTC. Reviewer `/root/modular_curve_sources`. This is a classical
modular-curve literature check and a counting derivation. No solver, attack,
scientific experiment, timing, or novelty claim is supplied.

## Verdict

The proposed counting route is supported for **p>3 prime, N=2^t>=8, and
p>=256N^4**. It proves a conservative lower bound of `1/(24N^2)` for a
uniform short-Weierstrass coefficient pair over Fp to define a nonsingular
curve with an Fp-rational point of exact order N. Sampling only nonsingular
pairs has at least this probability. The argument does not require any
congruence condition p=1 mod N or p=-1 mod N.

The fine-moduli condition is essential. For general coarse modular curves,
an Fp-rational moduli point may only provide the desired level structure on
a twist. Here N>=8 and p prime to N put the entire compactification in the
fine schematic locus, so its noncuspidal Fp points correspond to actual
pairs (E/Fp,P in E(Fp)) up to Fp-isomorphism, with P of exact order N.

## Sources actually read

1. **Brian Conrad, _Arithmetic moduli of generalized elliptic curves_,
   author version dated July18,2006.**
   Author PDF: https://math.stanford.edu/~conrad/papers/kmpaper.pdf .
   Local PDF `/workspace/audit/Conrad-kmpaper.pdf`, extracted text
   `/workspace/audit/Conrad-kmpaper.txt`.
   Read Introduction1.1-1.2, Theorem3.2.7, Theorem4.2.1,
   Remark4.2.2 and Theorem4.3.4 with their surrounding definitions.

   - Theorem3.2.7 states that the moduli stack M_Gamma is proper over Z,
     and M_Gamma1(N;n) is smooth over Z[1/(Nn)]. Take n=1.
   - Theorem4.2.1(1) states that the proper flat map has geometrically
     connected fibers.
   - Theorem4.2.1(2) gives an open subscheme containing exactly the
     trivial-automorphism points. For d|N with d>=5, the entire stack
     over Z[1/d] lies in that subscheme. Take d=N>=8.
   - Remark4.2.2 explicitly identifies these large-level constructions
     with fine moduli schemes for Drinfeld structures on generalized
     elliptic curves. With N invertible, exact-order-N Drinfeld level
     structure on a smooth elliptic curve is the usual rational point
     of exact order N.
   - Theorem4.3.4 describes the cusp strata by cyclotomic integer rings
     or their maximal real subrings (with the stated covers in the
     small-component cases). These are finite etale after inverting N.
     Thus the geometric cusp count is unchanged at p not dividing N.

   This source is a primary research treatment. It gives the good
   reduction/fine-moduli facts directly, rather than inferring them from
   a characteristic-zero analytic description.

2. **Andrew V. Sutherland, MIT18.786 Spring2024, Lecture21,
   sections21.1,21.3,21.4; Lecture22 section22.2.**
   https://math.mit.edu/classes/18.786/2024/LectureNotes21.pdf
   https://math.mit.edu/classes/18.786/2024/LectureNotes22.pdf
   Local PDFs/texts `MIT-modular21.*`, `MIT-modular22.*` under
   `/workspace/audit/`.

   Lecture21 gives the congruence-subgroup genus formula, determinant
   condition for geometric connectedness, cusp-orbit description and
   explicit twist warning for general coarse H-level structures.
   Lecture22 recalls good reduction at p not dividing the level.
   These are official course notes, used for the elementary index/genus
   calculations below; Conrad supplies the stronger fine-moduli facts.

3. **Jeremy Rouse, Andrew V. Sutherland, David Zureick-Brown,
   _ell-adic images of Galois for elliptic curves over Q_,
   Forum of Mathematics, Sigma10(2022), e62, section2.3.**
   https://arxiv.org/pdf/2106.11141 .
   Local `/workspace/audit/RSZB.pdf` and `/workspace/audit/RSZB.txt`.

   Read section2.3, especially Remark2.3.1, Remark2.3.2, the genus
   formula, and Remark2.3.4. This primary source explicitly corrects
   an overly literal rational-point interpretation of general coarse
   modular curves: it holds only up to twist in that generality.
   Its warning is why the fine-moduli verification above is retained.

The existing PKM2016 auxiliary-curve section in
`/workspace/audit/PKM2016.txt` uses a heuristic random-curve success
probability near1/N. The conservative bound here is instead derived from
classical modular-curve point counts. It is weaker in N, but adequate for
an exp(O(sqrt(log p))) construction overhead when N is in that regime.
No novelty of this counting argument is asserted.

## 1. A smooth geometrically integral curve over Fp

Let X_1(N) be Conrad's proper compactified fine moduli scheme over
Z[1/N]. For N>=8 it is a scheme, smooth and proper of relative dimension1,
with geometrically connected fibers. A smooth connected curve over an
algebraically closed field is integral: distinct components cannot meet
without a singularity, and disjoint components would disconnect it.
Consequently its Fp fiber is a smooth projective geometrically integral
curve, to which Hasse-Weil applies.

The noncuspidal open Y_1(N) represents pairs (E,P) over Fp, not just
geometric isomorphism classes invariant under Frobenius. There is no
remaining field-of-moduli descent ambiguity for this fine moduli scheme.

## 2. Explicit sufficient genus and cusp bounds

The reduction of SL2(Z) modulo N acts transitively on primitive vectors
in (Z/N)^2. The stabilizer of (1,0) is the image of Gamma_1(N).
For N=2^t there are N^2-(N/2)^2=3N^2/4 primitive vectors. Since
-I is not in Gamma_1(N) for N>2, the effective projective index is

    mu=[PSL2(Z):image(Gamma_1(N))]=3N^2/8.

The genus formula from Lecture21 section21.4 is

    g=1+mu/12-e2/4-e3/3-c/2,

with all subtracted terms nonnegative. Therefore

    g<=1+mu/12=1+N^2/32<=1+N^2.

Cusps are orbits on the mu right cosets under the parabolic stabilizer
of infinity, so c<=mu<=N^2, in particular c<=2N^2.
The cusp strata are finite etale away from N, so this same geometric
bound applies over Fp. The number of rational cusps is at most c.

For reference the sharper familiar power-of-two formula, not needed in
the inequality, is c=2^(t-3)(t+3); the orbit bound avoids depending on
that formula or any small-level exception. Only N>=8 is used here.

## 3. At least p/2 noncuspidal rational points

Hasse-Weil and the loose bounds above give

    #Y_1(N)(Fp)>=p+1-2(1+N^2)sqrt(p)-2N^2.

If p>=256N^4, then sqrt(p)>=16N^2. For N>=1,
2(1+N^2)sqrt(p)<=4N^2 sqrt(p)<=p/4.
Also 2N^2<=p/4. Hence

    #Y_1(N)(Fp)>=p/2.

The threshold is deliberately loose; no asymptotic error or unproved
distribution of elliptic traces is being substituted for it.

## 4. From pointed moduli points to uniformly sampled curve models

Since p does not divide N, E[N](Fpbar) has exactly N^2 elements.
For a fixed Fp-isomorphism class of E, the number of pointed
Fp-isomorphism classes (E,P) with P of exact order N is at most N^2.
Quotienting by automorphisms can only lower this upper bound.
Thus at least p/(2N^2) Fp-isomorphism classes of curves have the
desired rational point.

For p>3, every elliptic curve has a short Weierstrass model
y^2=x^3+ax+b. The orbit of a fixed nonsingular pair (a,b) under

    (a,b) -> (u^4 a,u^6 b),  u in Fp*,

consists precisely of its short models in the same Fp-isomorphism
class. Its stabilizer is Aut_Fp(E), of size at most6 in characteristic
greater than3 (the geometric possibilities are2,4,6). Hence every
such class contributes at least (p-1)/6 coefficient pairs.

The number of favorable pairs is therefore at least

    [p/(2N^2)]*[(p-1)/6]=p(p-1)/(12N^2).

Sampling all p^2 coefficient pairs gives probability at least

    (p-1)/(12pN^2)>=1/(24N^2).

There are p^2-p nonsingular pairs in characteristic p>3, so sampling
uniformly conditional on nonsingularity gives the slightly stronger
bound1/(12N^2). The parent proposed1/(24N^2) is conservative under
either sampling convention.

This counts rational points of exact order N, rather than merely an
elliptic group whose cardinality is divisible by N. Those properties
must not be conflated for a noncyclic2-primary group.

## Scope and limitations

- The result holds under p>3, N=2^t>=8, and p>=256N^4. It does not
  claim a useful constant at levels comparable to p^(1/4) without
  rechecking the finite inequality.
- A random model can be tested by a separate point-count/torsion
  algorithm. This note does not implement that test, price it or
  assert that cardinality divisibility alone certifies exact-order-N
  rational torsion.
- Moduli point counts prove availability/probability, not an efficient
  relation solver or any discrete-log algorithm.
- Early guessed Conrad URL `papers/ModuliPaper.pdf` and the guessed
  index `papers.html` returned404. The author's homepage links the
  correct `papers/kmpaper.pdf`, retrieved and read. A general Milne
  modular-forms PDF was consulted but its full-level/coarse discussion
  was not used to discharge the exact fine-moduli claim.
- No scientific computation was run. Shell operations only retrieved,
  extracted and searched literature and computed file checksums.

## Retrieved PDF checksums

    f53e4ff819d534926e6ee59874cbce4e7bad4a315cbd6d05984b8405b11f15d8  Conrad-kmpaper.pdf
    436ebe7cf78abf8ec568b9ab0e664dab1f52fad2f49e612967c68c2c62d000e4  MIT-modular21.pdf
    272106e4f6fe05ca023696b2494c6041ec1d8b3f7f8f73c55d87bbb93f52f9c0  MIT-modular22.pdf
    7b1ada8521a3a2d4be3b9ab88bc0badd9f8e5561d114722364f0ae2b8453927a  RSZB.pdf

## 5. Original Schoof point-counting theorem: setup cost is polynomial

Follow-up primary-source verification, 2026-10-30. The exact count of each
sampled auxiliary curve need not be treated as a heuristic or an
exponential brute-force operation.

**Rene Schoof, _Elliptic Curves Over Finite Fields and the Computation of
Square Roots mod p_, Mathematics of Computation44, no.170(April1985),
483-494.** DOI:10.1090/S0025-5718-1985-0777280-6.

Author-hosted original-publication scan, linked on his publication page:

- https://www.mat.uniroma2.it/~schoof/papers.html
- https://www.mat.uniroma2.it/~schoof/ctpts.pdf
- Local `/workspace/audit/Schoof1985.pdf`.

**Original statement read visually:** printed page483, abstract and
Introduction, PDFpage2(the scan has a separate JSTOR cover sheet).
The introduction states that for an elliptic curve over Fq given by a
Weierstrass equation, with characteristic neither2nor3, the point-counting
algorithm is deterministic, depends on no unproved hypotheses, and takes
`O((log q)^9)` elementary operations, explicitly identified as bit operations.
The input curve must be nonsingular, as required throughout for an elliptic
curve. It applies to q=p and arbitrary short-Weierstrass coefficients;
there is no ordinary-curve, generic-j or p-congruence restriction in this
point-counting statement.

**Complexity derivation read visually:** printed page490, PDFpage9, the
paragraphs beginning “Next we estimate the number of elementary operations”.
Schoof chooses primes up to L=O(log q) using an effectively computable
universal constant in the product-of-primes lower bound. He accounts for
division polynomials, modular polynomial arithmetic and the CRT, and
concludes the full second step costs `O((log q)^9)` bit operations, with
the remaining steps dominated by that bound. He assumes an irreducible
polynomial specifying Fq over its prime field is supplied. For q=p this
extension-field representation condition is vacuous. The same page gives
an `O((log q)^5)` memory bound; no memory or practical-speed claim is needed
for the parent construction.

**Important scope limit on the same page:** Schoof explicitly says the
algorithm does not find the group structure of E(Fq). Thus exact cardinality
must not be relabelled as a general group-decomposition oracle. The parent
setup only needs the exact cardinality, its2-adic valuation/odd part, and
separate scalar multiplication plus doubling checks on an actual sampled
point. Those checks can certify its exact2-power order without recovering
the entire group structure. Merely observing N divides #E(Fp) is insufficient.

This source therefore supplies a deterministic polynomial-in-log(p) cost
per exact point count for p>3. Combining it with the preceding probability
bound gives a sub-exponential auxiliary-search overhead when N is
exp(O(sqrt(log p))), provided the separate rational-point/torsion sampling
and acceptance procedure has the stated polynomial cost and a proved
conditional acceptance probability. The point-counting theorem alone
does not supply those other steps or a relation solver.

### Retrieval and errata qualifications

The direct AMS PDF URL returned403. The author's own publication page
links both the original `ctpts.pdf` and `schooferr.pdf`; both were retrieved.
The original is an image scan with an empty text layer. Local OCR was used
to locate passages; the decisive exponent9, characteristic condition,
bit-operation interpretation and page490 complexity derivation were
verified by viewing rendered original pages rather than trusting OCR
superscripts.

The author-linked `schooferr.pdf` is _Corrections to Schoof85_, by Nick
Alexander and Janice Tytaneck, January18,2004. It concerns typographical
formula corrections on printed pages488-489. Its own header disclaims
responsibility for accuracy. It does not change the point-counting
complexity statement verified above; it is retained as an implementation
caution, not presented as an independently certified implementation or a
replacement theorem. No implementation is supplied in this literature check.

An initially guessed CWI URL retrieved an unrelated1983 Helminck paper;
its title was checked, rejected as a source, and the local file renamed
`Helminck1983-unrelated-retrieval.pdf`. It is not cited as Schoof evidence.

Checksums:

    29f3cdc7297a08f094cdbe52f7a7692467475b76c965df30cab0f7afcea11895  Schoof1985.pdf
    11ada4d0a54d6f96f671836510a9c1a131fb948c15b5aeae8cd54949b31d9593  Schoof1985-errata.pdf
