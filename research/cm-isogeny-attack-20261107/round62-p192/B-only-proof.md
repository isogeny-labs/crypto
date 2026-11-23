# A calibrated first-coefficient decoder

Derived algorithmic reduction, 2026-11-08. This theorem replaces the pair (B,C) in the Round60 marked-action interface by a single first spectral coefficient, one reusable calibration, and at most seven queries. It does not construct a fast public evaluator for that coefficient. The written proof and native qualification remain unverified producer evidence. [pcc:unverified]

Input and oracle contract. Let E/F_p:y^2=x^3+ax+b, p>3, E(F_p)=<G> of prime order r>6, r!=p, and Q=[k]G with k not 0,+1,-1. Let P in E(F_(p^2)) have 2P nonrational, and let U be rational. The oracle returns

B_Q(P,R)=[s] product_(j=0..r-1)(x(P+jG)-x(R+jQ)+s)

in K[s]/(s^2), for R=P+U. This is a transverse spectral shift of the factors, not an ordinary derivative in P or R. Its constructor, target-dependent setup, arithmetic, field representation and exactness are all charged. [pcc:unverified]

The relevant differential identity. Put psi=1-pi. There is one vanishing factor at the index j satisfying [j](G-Q)=U. Write A=P+[j]G. The constant coefficient satisfies N_Q(P,R)=c_Q*(x(psi P)-x(psi R)). Differentiate in x(P) with R fixed, and then specialize to R=P+U. Translation invariance of dx/(2y), together with differential(psi)=1, gives

B_Q(P,P+U)*y(A)=c_Q*y(psi P).

Both sides are computed on a denominator-safe affine chart. The hypotheses ensure P+iG, A+iG and psi P are finite with nonzero y: otherwise a rational translate of 2P would be rational. The unique-zero cofactor is also nonzero. This is the same differential relation recorded in Round60, now used to specify a complete decoder. [pcc:unverified]

Calibration eliminates c_Q as an input. At U=O the singular index is j=0, so

Bcal=B_Q(P,P),
Bcal*y(P)=c_Q*y(psi P).

This one value can be reused for all U at this G,Q,P. For i=0,...,6, query

B_i=B_Q(P+[i]G,P+[i]G+U).

The singular index remains j, and psi(P+[i]G)=psi P. Hence

y_i=Bcal*y(P)/B_i = y(A+[i]G).

This supplies seven y values without evaluating C or supplying c_Q. The native consistency reference still directly constructs c_Q and charges that additional work; the candidate formula uses only the calibration. [pcc:unverified]

Decoder. Solve x^3+a*x+b-y_0^2=0 over F_(p^2). There are at most three distinct roots; keep the corresponding points A_candidate=(x,y_0). Remove a candidate if any translate A_candidate+[i]G is infinite or its y-coordinate differs from y_i. Return the unique surviving A_candidate-P. The result is [(1-k)^(-1)]U. The native small-field implementation enumerates all p^2 x values and charges them; the general reduction requires a fixed-degree finite-field root algorithm, whose cost is additional to the oracle calls. No P192 root enumeration or B query is executed. [pcc:unverified]

Uniqueness theorem. Suppose distinct candidates A_1,A_2 survive. On E consider the rational function

g(T)=y(T+A_1)-y(T+A_2).

Each summand has its unique pole of order three, respectively at -A_1 and -A_2. These poles are distinct and cannot cancel, so g is nonzero and has total pole degree six. Therefore its zero divisor has degree six, counting multiplicities. Surviving candidates make g vanish at the seven distinct points O,G,...,6G, none of which is a pole. This contradicts the zero degree bound. Existence is supplied by the actual A=P+[j]G, so exactly one candidate survives. This proves the decoder for every input satisfying the stated hypotheses, conditional on exact oracle values; the proof is written, not a Lean formalization. [pcc:unverified]

Costs and guards. Per changing target, pay one calibration query, generation of seven public shifts, and any oracle constructor. Per U, at most seven B queries, seven divisions after the reusable factor Bcal*y(P), one cubic root extraction, and at most three sets of seven public translations/comparisons. Rational/subgroup checks on the returned point are additional. All seven queries are run in the declared native panel; no adaptive rate claim is made. Public exceptional Q cases are handled by the pre-existing marked-action interface. This reduction has a larger query count than the direct (B,C) interface and a smaller coefficient contract; it is not a measured speedup. [pcc:unverified]

Declared qualification. The amendments precede the B-only data and retain the original transport outputs. The control uses seven Q fixtures and seven U fixtures on E/F211 of order199. There are seven reusable calibrations and343 shifted B queries, for350 contractions and69,650 factors. Root enumeration tests2,181,529 field elements on the49 original transcripts and the same number on49 mutated transcripts. Prefix/suffix B contraction agrees with streaming (A,B). The candidate never computes C. The auxiliary map/retraction prelude and normalization-product consistency check are retained additional work. [pcc:unverified]

Next circuit. It is now enough to construct the normalized spectral response B_Q(P,P+U)/B_Q(P,P), rather than both transverse moments. The compact zero-order norm still does not determine this response: Q and -Q have the same base norm and different hidden translated y values. A CM or determinant compression must preserve this first spectral response, with all target-dependent constructor and quotient-algebra costs. The horizontal transfer theorem supplies the derivative rule for transporting a represented response, but does not initialize it. [pcc:unverified]

Sources. Round60 cross-norm-proof.md establishes the norm/differential identity. The divisor fact used in uniqueness is standard: a nonzero rational function on a smooth projective curve has equal zero and pole degree; the proof here lists the two poles explicitly. Cubic root extraction and rational-function divisor counting are not claimed as novel ingredients. Relevant isogeny-coordinate prior art and the unavailable KB review are recorded in report.md. [pcc:unverified]
