# Direct elliptic pushforward from opaque KM input

Frozen before implementation/execution, 2026-11-26. PR #5257, Conductor T-4.
The coordinate-screen/matched-rho waiver persists; no new PCC registration,
scalar recovery or speed comparison. Preserve every previous subject.

Input contract: a valid Fp-rational effective-degree12 Riemann--Roch
representation W=L(9H56-D), given only as its 22x85 evaluation coordinates.
Rank/ambient checks do not certify all adversarial spaces as valid divisors.
The evaluator reads no generating scalar or support witness from the input.

Reuse the existing actual GMP KM normalizer, complete adapter and opaque
Riemann--Roch decoder unchanged as build-only guarded headers. After infinity
normalization, require actual KM equality with the input. Decode an effective
degree-three E, check full multiplicity incidence and re-encode E-Eref into
the actual KM arithmetic, requiring equality with the original normalized
class. Retain the complete decoder certificate, charts and rejected choices.

This is a smaller extraction interface than a new degree12 ideal decoder:
the input is still degree12, but standard RR gives the equivalent degree3
representative. The infinity closed point has degree4, so every rational
effective degree3 representative is affine. Existing projective-chart
0..24, shear0..308 and deterministic factor-splitting0..1023 schedules are
retained. Exhaustion rejects; no larger schedule after observing results.

Separately implement B([E-Eref])=sum f(E)-sum f(Eref) on the OpenSSL backend
used in #5255/#5256. Replay the whole separated support and full quartic
incidence modulo it. Factor it exactly, verify every irreducible factor and
the complete product with multiplicities. Evaluate each closed point in its
residue field and sum all Frobenius conjugates, including repeated factors.
Check prime-field descent. Re-evaluate all three actual reference points.

For units of the actual f denominators evaluate the exact rational functions.
At any denominator zero, lift its own smooth affine chart with fixed
N=4*maxdegree+1=45 precision. Evaluate homogeneous ratios as in #5255,
checking finite endpoints or pole orders -2e,-3e and the elliptic equation.
No alternate precision after a failure. Check the full map equation modulo
the quartic before evaluations. This uses f_* directly; do not invoke the
older Gamma-transpose software for the answer.

Prepare the following twelve fixed inputs in actual GMP KM arithmetic,
then invoke the external-input evaluator separately on each file:

1. Four principal inputs with infinity multiplicities m=0,1,2,3, using the
   existing shift of the identity representation. Require output O.
2. Two nonzero point-class inputs with m=1,2, using the first point of the
   second split fiber minus the first reference-fiber point. Require the
   independently evaluated difference of those two f images.
3. The retained public F(G) KM space (complete-km-v3 class index2), and its
   actual KM double. Require fresh OpenSSL [6]G and [12]G, respectively,
   using only the public G coordinates from the frozen transfer record.
4. The three retained opaque-boundary inputs: special h0=2, triple reference,
   double-plus-single reference. Require RR dimensions2,1,1; retain exact
   multiplicities for the repeated inputs and compare with the independent
   f norm of the original complete input cycles minus Eref.
5. Encode E=3R0 for the original retained affine f-pole R0 (point index0).
   Require decoded E to retain that triple support, so the actual pole
   evaluator is reached; compare with its direct threefold image minus Eref.

The fixture generator may read the original cycles/public point only to
construct test inputs and expectations. The external class adapter consumes
only class_space. The OpenSSL checker reads the resulting cycle and test
expectation; it does not read a scalar used to generate a source image.

Controls: a thirteenth off-ambient input changes one evaluation coordinate
and must reject in the external adapter; changed decoded y must fail full
curve incidence; an omitted support factor must fail the degree/product
contract; and adding the nonzero public G to the returned elliptic point
must fail equality with the expectation. Controls are explicitly marked,
never counted as successful ordinary worker exits.

Each strict build <=300s; every native preparation, extraction or separate
verification process <=600s and <=1GiB on one allowed CPU. Run native
processes sequentially. Retain setup/normalization/decoder/map/trace/check
phase clocks, all wrapper/process costs, source/input/binary manifests,
compiler/OpenSSL versions, exits, errors and all failed attempts. Binaries
stay local. No cap increase; no selected replacement points after failure.
Reporting/coordination/publication/peak-RSS and historical synthesis costs
remain separately scoped. Freeze new extensions before any affected call.

Sources reviewed: exact opaque-KM and boundary protocols/sources/results,
normalizer and complete KM adapter, #5255 projective evaluation, #5256 actual
composition proof; Khuri-Makdisi, math/0105182v3, sections2/5 (Lemmas2.1/2.3,
Algorithm5.5), https://arxiv.org/html/math/0105182v3 ; Galbraith corrected
chapter7 uniformizers/morphism extension and chapter10 divisor multiplicities.
Singular's exact-basis manual was reviewed as an alternative; no new
Groebner implementation is needed. KB remains unavailable as previously
recorded. Established methods are used without a novelty claim. GMP RR
extraction and OpenSSL curve/elliptic arithmetic are distinguished; no signed
or independently compiled geometric proof is asserted.
