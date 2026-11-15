# Single-relation alternative: scope and checks

15 November2026. Source review and hand derivation, before any new native
implementation, numerical call or supporting-law build. User criterion remains
a complete classical subexponential ECDLP algorithm; this study does not claim
it. The published random-labelled-base alternative replaces auxiliary setup
and a final relation matrix by one zero-sum search. Its actual search cost must
still be supplied. Prospective PCC-20261115-bd411d is unverified.

## Literature and existing work first

KB unavailable: no loaded connector; scripts/research-kb status127, no executable.
Read the repository state-of-the-art section8.5, the selective-oracle/code-
guessing reviews and the explicit-relations GPU contract (generic meet-in-the-
middle, no subexponential claim). The approach is existing work, not novel.

Read Amadori--Pintore--Sala, On the discrete logarithm problem for prime-field
elliptic curves, FFA51(2018), sections3--3.2, equations7--11:
https://pure.tue.nl/ws/files/91158650/609.pdf . The publisher endpoint returns403;
the university-hosted accepted manuscript is available. The paper builds
random labelled points and solves one partitioned summation-polynomial system,
leaving its solving cost unestimated. Check the exact probability argument
directly rather than treating the paper's heuristic count as a proof.

Read Spaenlehauer, arXiv2610.32508v1(23October2026), Definition1.2 and
Theorem1.3, on deterministic univariate root finding given an elliptic datum:
https://arxiv.org/abs/2610.32508 . The theorem charges the univariate degree and
excludes datum construction/verification from its online bound. It supplies no
multivariate relation selector merely by being called after elimination.

Kudo--Yokota--Takahashi--Yasuda, CANS2018, was located through publisher/
institutional metadata. Full text has not been inspected here; no limitation
claim from that paper is adopted. Primary PDF hashes are retained in checks/.

## Questions to resolve

1. For independent uniform points in m disjoint lists of b records, derive the
   exact first/second moments of the number of positive zero-sum tuples and
   a finite existence lower bound. Retain the distinction between existence
   and finding a tuple.
2. Bound the loss from rejecting infinity and coincident abscissas across all
   mb records. Show that conditioning on these point-only tests leaves the
   private mixed labels uniform. Recover from one checked signed relation,
   with every failed base/oracle call charged.
3. Specify an explicit finite schedule and its complete conditional cost.
   Do not replace the missing capped selector by an existence statement.
4. Inspect the actual all-monomial index in critical_pair_norm/produce.cpp.
   For direct degree-b membership equations, derive the cost of this named
   architecture. Do not extrapolate it to all sparse/circuit/GB algorithms.

## Supporting formal checks

Before compiling, put missing reusable laws in crypto-lean over existing
semantic fields/finite types: the mixed-label fibre equivalence/cardinality,
recovery from a weighted relation, and the factorial/binomial index inequality.
Use the pinned Lean4.35/mathlib environment and canonical all-source/axiom
gate, standard foundations only, no native_decide or raised proof limits.
Retain first-build diagnostics and exact sources. Second-moment and elliptic
chart arguments remain hand derivations unless separately formalized; these
supporting laws must not be presented as a complete reduction/program proof.

No numerical qualification is planned or run in this source-only study. The
older multiprecision qualification remains frozen and pending its required
S3 upload access. No old source, raw archive, cap or seed is changed here.

## A1: full finite-group coverage check

After the three supporting laws pass, check the actual coverage statement
over an existing finite additive commutative group G, without a new toy curve
or numerical case. For assignments on the m-by-b record grid, prove tuple-sum
surjectivity and joint-sum surjectivity for distinct tuples. Reuse subgroup
kernel/range cardinality and finite Cauchy--Schwarz laws to derive the exact
event counts and support bound L/(r+L-1). Preserve failed builds and statement
pins, and use the same canonical proof/axiom gate. This target formalizes
unconditioned coverage; elliptic clean-base loss, label conditioning, native
execution and a fast selector remain distinct. No cap/field/seed or numerical
qualification is introduced by this proof continuation.

## A2: fixed-slope elliptic partial-join assessment

After A1, check a concrete non-tensor selector component: for a fixed set of s
field slopes, hash y-lambda*x on two independent random point lists and retain
distinct-abscissa collisions. Competing hypotheses are that this can populate
the first merge at balanced b, or that the cubic line-intersection bound makes
the useful yield too small. Derive a finite upper bound before implementing.
Use the actual short-Weierstrass line equation and the uniform subgroup point
law; charge clean-base conditioning separately. Acceptance is an effective
population/yield bound at growing parameters, not an empty tiny fixture.

Literature first (KB remains unavailable): read Wagner2002 full author version
sections2--3 and the blind-signature subsection of4, from
https://people.eecs.berkeley.edu/~daw/papers/genbday.html (long-version link).
It permits scalar-interval joins and suitable group-quotient chains and gives
the cyclic k-sum/DLP reduction. Its blind-signature example concerns scalar
ROS arithmetic, not an ECDLP solver. Read Sutherland MIT18.783 Spring2013
Lecture2, section2.1.2, for the line/cubic substitution and slope formula:
https://math.mit.edu/classes/18.783/2013/LectureNotes2.pdf . The partial mirror
Wagner PDF timed out; the complete author PostScript and converted PDF are
available. Source hashes/links will be retained with the assessment.

This target concerns a priori point-independent slope sets. Adaptive slopes,
large implicitly described slope sets, other coordinates and all partial-join
algorithms are outside its proposed upper bound. Reuse existing polynomial
root counts; if a missing support law is needed, add it in crypto-lean on the
existing polynomial/curve semantics before making a formal claim. No native
numeric case, paid job or change to the frozen multiprecision panel.
