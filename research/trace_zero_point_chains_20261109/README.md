# Algebraic trace-zero relation models and reversible tau transport

2026-11-09 · isogeny-labs/crypto · PR3744 · unsigned local evidence

Implemented a basis-only trace-zero decomposition oracle, a complete
finite-base full-point chain, an inverse/half-trace membership model, opt-in
constant specialization, and reversible tau-minus-one transport. Twenty
focused native checks pass. The two wide inverse-membership solver controls
return UNKNOWN under their caps; no relation-finding speed improvement is
established [pcc:unverified].

The [diagram](construction.svg), [recorded circuit graph](counts.svg),
[PDF](report.pdf), [raw checks](checks/native-tests.txt),
[recorded counts](counts.json) and [source manifest](SOURCES.sha256)
describe this construction round. No comparative experiment panel ran.

## Requirement and evidence

| User requirement | Working result | Remaining dependency |
|---|---|---|
| Integrate the full trace-zero object | Existing full group-trace rational-point model plus new exact finite x-image and implicit subspace solver | A scheme-level chart over arbitrary extension algebras is not supplied |
| Test whether relation systems shrink | Actual source circuit constructions, constant specialization and exact model counts | Matched successful relation-finding work, not counts alone |
| Check Z[tau] beyond orbit compression | Invertible tau-minus-one transport, complete small relation-existence correspondence, public n131 replay | Simpler equations for the transported base and charged solver comparison |
| Devise and experiment with new leads | Five implementations;20 focused checks,10 original and7 prior-lead regressions pass | Prepared264-row panel still requires the pending execution exception or checkpoint/cloud access |
| Make a PR | Extend existing PR3744 | Existing review/CI/benchrec gates remain |

These are research correctness checks and static constructions. They do not
replace the requested ECC2K-130 relation-finding or full-DLP evaluation.

## Exact object and fixtures

Let E0: y^2+xy=x^3+1 over F2 and n be odd. The group-trace kernel
T_n=ker(1+tau+...+tau^(n-1)) has algebraic dimension n-1 in the Weil
restriction. Its F2-rational points equal4E0(F_(2^n)); the identity is separate
from every affine model below. Existing code validates both trace bits and
replays actual group trace. It does not identify a single field-trace bit
with the full kernel.

Native fixtures are n3/5/7/9 with polynomials z^3+z+1, z^5+z^2+1,
z^7+z+1, z^9+z^4+1 and kernel orders1/11/29/127; n83 uses
z^83+z^45+z^2+z+1 and order2417851639230796216685689; n131 is the
repository's exact Certicom ECC2K-130 fixture, polynomial
z^131+z^13+z^2+z+1, cofactor4, original G and public Q. Wide n83 targets
in this study are stated multiples of its public G, not a public challenge Q.
Fixture source and Cargo lock are pinned by SOURCES.sha256.

## 1. An unenumerated algebraic factor base

`decompose_trace_zero_subspace(variety, basis, target, m, limits)` takes
only a linear basis and target. It defines

    B = { affine P in T_n : x(P) in span_F2(basis) }.

It never constructs a list of B's points. Each summand receives free basis
coefficients, inverse witnesses and the exact membership equations below.
Linear rows tie those coefficients to the existing S3 x wires. Coordinates
known zero from the basis and the fixed target specialize multiplication.
All intermediate-identity masks are retained; intermediate points are not
required to lie in the original subspace.

SAT x-tuples are independently checked, lifted to both original point signs,
and tested in the original group. Only after all signs fail is that tuple
blocked. Complete exhaustion means Refuted; caps, unsupported inputs and
invalid models mean Inconclusive. Returned relations contain actual points,
not invented indices into an unenumerated table. This x-only base is sign
symmetric by definition; use the finite-base oracle for a sign-restricted
supplied base.

The exact small check covers240 oracle calls: every target of the n5/7
kernels (11+29), three subspace dimensions (0/2/3), two arities (2/3).
Independent exhaustive point enumeration constructs truth outside the solver;
it is not supplied as the candidate's input. All decisions and returned point
relations agree [pcc:unverified].

For d16,m3,all-affine intermediates, the actual basis-only constructor builds:

| n | SAT variables | S3 AND gates | Constructed CNF clauses | XOR rows |
|---|---:|---:|---:|---:|
|83|10794|5485|28666|1331|
|131|16890|8509|44938|2099|

The S3 AND column excludes membership gates; total variables/clauses include
them. These are static counts from one exact basis and target per row, not
solver outcomes, runtime samples or an asymptotic relation-size theorem.

## 2. Exact finite x-image with a quadratic inverse witness

For x!=0 put v=1/x and H(a)=sum a^(2^(2i)), i=0,...,(n-1)/2. The model is

    x*v = 1,
    Tr(x) = 0,
    Tr(v) = 0,
    Tr(x*H(v^2)) = 0.

This reuses known cofactor-four validation/halving algebra. US7257709,
condition iii, already gives the final trace test for b=1 on partially valid
keys. Here rationality is made explicit and the inverse is an auxiliary
field variable, yielding quadratic Boolean equations. No new theorem or
novel mathematical test is claimed.

Why it describes this x-image: rational curve lift for x!=0 requires
Tr(x+v^2)=Tr(x)+Tr(v)=0. With Tr(v)=0, set s=sqrt(x), u=s*H(v).
Then u^2+s*u+1=0 and u^2+u^(-2)=x, so u is a half-point abscissa.
Its trace equals Tr(x*H(v^2)) by squaring. Requiring that trace to vanish
is the second halvability condition, giving4E0. The two half-abscissae
have the same trace since Tr(s)=0. Original point lifts and full group trace
are checked independently by the native tests.

The integral expansion (u^2+s*u+1)(u^2-s*u+1)
=u^4+(2-s^2)u^2+1 has evaluated certificate
IDC1h2a629cbcbb26bb0c: degree4,32 issued plus32 local replay points
over2^61-1. It certifies that expansion only, not the characteristic-two
halving argument or subgroup theorem. Expanded coefficients have absolute
value at most2. No formal proof or independent signature is attached.

Raw uses2n variables and n+3 equations. Compact eliminates the two linear
trace coordinates, giving2n-2 variables and n+1 equations, still degree2.
Over a supplied d-dimensional trace-zero subspace the compact model uses
d+n-1 variables. Inversion is unique for each admitted x; actual point
lifts have two signs. This is the finite image of T_n(F2) under x, not all
F2-rational points of a geometric Kummer quotient: twist-side points must
be excluded by Tr(v).

The counterexample n3,x=z,v=z^2+1 passes Tr(x) and the last trace bit but
fails Tr(v). Deleting rationality admits a false x; the independent decoder
rejects it. Exhaustive all-x checks at n3/5/7/9 give image sizes0/5/14/63,
matching full point-trace enumeration. Every raw/compact assignment is
checked at n3/5/7 [pcc:unverified].

| n131 presentation | Variables | Equations | Degree | ANF terms | AND auxiliaries |
|---|---:|---:|---:|---:|---:|
|Inverse compact, unrestricted x|260|132|2|51071|16900|
|Full-point compact, unrestricted x (historical)|260|131|3|117552|43517|
|Inverse compact, d16|146|132|2|3627|2080|
|Full-point compact, d16 (historical)|146|131|2|3076|2200|

Rows with matching x-domain have the same projected x set but different
witnesses. The full-point model retains the two ordinate signs, while the
inverse model has one witness per x. Historical point counts come from the
previous full-variety/lead studies and keep their original source identity.
Inverse compact uses an additional row and can use more terms on d16;
smaller global degree/AND allocation does not determine solving cost.

Actual n83 and n131 inverse-membership S3 singleton controls both return
UNKNOWN at their2-second/2000-conflict cap (21331 and52531 variables).
These outcomes are preserved in checks/native-tests.txt and the initial
control log. They are not successful natural relations.

## 3. Compile constants before multiplying

`build_with_target_constants` and `build_for_domain` enable a separate
constant-aware circuit builder. Constant field multiplication is F2-linear;
zero/one products and duplicate XOR inputs fold exactly. Target-only mode
fixes the target; domain mode additionally fixes bits shared by every
admitted finite code. It retains the same complete finite domain. Default
`build` stays the old circuit, and its conservative admission bound remains.

Every projected small model at n3, arities2/3, finite/identity targets and
every admitted mask is compared with direct S3 field truth in all three
builder modes. No model changes [pcc:unverified]. Static singleton counts:

| n, m3 all-affine | Default variables | Target-only variables | Fully fixed domain variables | Domain AND gates |
|---|---:|---:|---:|---:|
|83|42413|28635|1079|0|
|131|104669|70347|1703|0|

Only the specialized fully fixed domain is solved in this correctness test,
and both rows return SAT with field replay. Baseline/target-only counts are
constructed before solving. This is an x-only S3 control; it has no inverse
membership witness. It does not resolve or replace the UNKNOWN controls in
section2, which are different systems. No timing ratio is claimed.

## 4. Branch-complete finite-base full-point chains

`FullPointChainEncoding` retains correlated (x,y) domain codes and quadratic
addition equations, reusing the native S3 field-product circuit. Generic
addition includes an inverse guard for x1+x2!=0. Doubling, opposite points
and identity-plus-point use distinct branches. Identity wires use a fixed
sentinel. Every accepted model replays each branch and original group sum.

All100 ordered affine pairs in the n5 kernel are checked with their exact
branch and a wrong-output UNSAT control; all1000 triples have an admitted
branch path. The complete oracle agrees with enumeration for every small
target at arities2/3/4 and sign-restricted/multiple-x bases. Branch and resource
caps stay Inconclusive. Pinned n83/131 identity paths use830/1310 variables
and replay G+(-G)+G=G [pcc:unverified].

This completes the finite-base branch core of the previous TZL-03 lead.
It does not yet attach unenumerated full-point membership models to that
core. The basis-only oracle in section1 instead uses x-only S3 and inverse
membership. Neither system is a claim about solving degree from input degree.

## 5. Z[tau] transport as a presentation candidate

E0/F2 has trace -1, so tau^2+tau+2=0. The already implemented map
delta=tau-1 has norm4. Since

    (t-1)*(-t-2) + (t^2+t+2) = 4,

its inverse on the odd-order kernel is [4^-1]*(-tau-2), with the scalar
inverse taken modulo |T_n|. New `inverse_cofactor_image` implements this
inverse, rejects nonkernel points and preserves the identity.
Evaluated integral certificate IDC1he63d40398a440139 records the displayed
degree2 expansion (32+32 points, modulus2^61-1); coefficients are at most4
in absolute value. It does not prove the Frobenius equation or a cost claim.

Complete n5/7 kernels verify bijectivity, both-direction replay and the exact
equivalence of every target's three-term relation existence on a fixed
three-point signed base and its transported image. Wide replay checks actual
n131 G/publicQ and n83 G/17G, including torsion-coset rejection. These tests
confirm coordinate transport rather than an enlarged relation population
[pcc:unverified].

The deciding next candidate is to keep |B| and all targets fixed up to this
bijection and compare direct B equations with a delta-preimage presentation.
A lifted delta map uses Frobenius-linear coordinates plus guarded group
addition. It may expose useful elimination structure, or its extra variables
may cost more. No assertion that delta(B) is a linear x-subspace is made.
This is distinct from Frobenius orbit deduplication, but relation counts alone
cannot improve because both base and target are transported bijectively.

## Reproduce and evidence flow

~~~sh
cargo test --locked --manifest-path research/trace_zero_point_chains_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo run --locked --manifest-path research/trace_zero_point_chains_20261109/native/Cargo.toml --bin certify_inverse_lift
cargo run --locked --manifest-path research/trace_zero_point_chains_20261109/native/Cargo.toml --bin certify_tau_norm
cargo test --locked --manifest-path research/full_trace_zero_variety_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo test --locked --manifest-path research/trace_zero_leads_20261109/native/Cargo.toml -- --nocapture --test-threads=1
cargo check --lib --no-default-features
~~~

Production algorithms and tests are Rust. The small dependency facade imports
actual production sources; no replacement arithmetic is used. `plot_counts.py`
only renders recorded counts; Typst renders the PDF. Exact commands, environment,
failed compile attempts and all capped outcomes are retained in checks/.
The first certificate build used a nonexistent Expr constructor and the first
transport test build used a nonexistent fixture field; both were corrected
before final passing checks. No failed result is omitted.

Flow:20 focused tests completed/passed,10 original regressions passed,7
prior-lead regressions passed, production library type-check passed. No
matched benchmark rows attempted. The prepared264-row protocol remains at
source3ae8b21241256517a9c867b4e78fb25e02ff502e and binary hash15c4d1b193999153c6ddc6ec94959167c8f4d016be197b55b26d6993001b1126;
later source here must not replace that executable. Its local EKS/S3 exception
is pending, and no cloud job launched. See the
[open access record](../../problems/trace-homomorphism-experiment-access.md).
Whole-root unit-suite and historical benchrec gaps on PR3744 remain open;
focused checks do not clear them. All attestations here are unsigned local
evidence, not independent signed verification.

Existing scoreboard, progress timeline and performance-gains graphs were
checked and retain their values: this round supplies no new speed measurement.
Previous study diagrams/PDFs remain historical. New candidates need a separate
pre-data protocol; they are not inserted into the prepared panel after freezing.

## Sources and next discriminating checks

- Kosters–Yeo, [arXiv1503.08001](https://arxiv.org/abs/1503.08001), trace
  homomorphism and summation-polynomial equations; existing local source copy.
- [US7257709](https://patents.justia.com/patent/7257709), condition iii and
  partially valid key/cofactor-four assumptions. Primary text opened in this
  round; the rationality assumption matters. No novelty claim for the criterion.
- Production BinaryCurve halving/addition and earlier full-trace/lead reports.
- [Frobenius characteristic equation](https://math.mit.edu/classes/18.783/2023/LectureNotes13.pdf)
  (standard primary lecture reference, not newly fetched in this round).
  Four F2-rational E0 points give trace -1; the native transport tests verify
  its actual action on the named fixtures.
- Repository KB still unavailable (prior receipt: wrapper exit127). Existing
  mechanism-atlas review maps these to decomposition encoding, factor-base
  subspace choice and cofactor transport. No global originality assertion.

Next comparisons: (a) constant-aware explicit versus implicit algebraic
membership on identical unenumerated B, (b) full-point versus x-only chains
with all identity branches and charged lifting, (c) direct versus reversible
tau-transported presentation with equal relation sets. Preserve all UNKNOWN,
construction, admission and replay costs; successful complete work decides
utility, rather than degree or variable counts alone.
