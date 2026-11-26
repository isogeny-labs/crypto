# Five exact coprime-selector sectors for the M31 RR5 chart

**November 23, 2026 Pacific · PR #5119 · construction qualification**

The additive Rust constructor provides a five-sector alternative to the
existing coupled quartic-selector presentation. On the admitted B677
fixture, these five cases cover the same supported RR5 chart. The largest
individual system has 59 variables and 60 equations, versus 122 and 123 in the
unchanged coupled constructor. Every input equation remains quadratic.
The corrected native qualification passes 35 library tests and 8 public-example
tests. These are unsigned local development observations; extraction and
solver cost have not been measured. [pcc:unverified]

## 1. What is implemented

The new public API is
`selector_sectors::construct_hybrid677_sector(fixture, target, k, limits)`.
It returns the existing F4 `Poly` representation, literal RR5 quartic
coefficients, and construction metadata. It does not call a solver, select
a factor-base leaf, enumerate point pairs, or consume a target scalar.

- [Native constructor and controls](../prime31_four_summand_20261122/native/selector_sectors.rs)
- [Frozen development protocol](CONSTRUCTION_PROTOCOL.md)
- [Native output summary](QUALIFICATION.json)
- [Dependency source review](SOURCE_REVIEW.md)
- [Exact successful library output](evidence/library-tests-02/stdout.txt)
- [Unchanged public-example output](evidence/example-tests-01/stdout.txt)

The existing `selector_circuit.rs`, extractor, B677 fixture, fixed extraction
driver, and original scientific protocol keep their incoming Git blobs.
This change is an additional formulation, not a replacement result for the
registered extractor. PR #5099's payload and protected ledger are untouched.

## 2. Exact instance and scope

| Quantity | Value |
|---|---|
| Coordinate field | p=2147483647=2^31-1 |
| Curve | y^2=x^3+847514846*x+2087444724 |
| Full prime group order | n=2147446831 |
| Generator | (625037715,2100627251) |
| Factor base | Original 509 Dickson points plus 168 frozen Y7 points |
| Selector degrees | Dickson 1024; Y7 union 168; product 1192 |
| Y7 CSV SHA-256 | fc484615394ee423190747d05001cea526580fb0df0279fbcf789ab6cafc0c46 |
| Starting main | ceba74bb64854027c4dd16f00428bf676383bfdb |

All new construction fixtures use M31. Existing historical regression tests
retain their own original domains; their success does not supply the new
M31 claim. The base constructor enforces canonical signs, exact membership,
distinct abscissae, complete split selectors and disjoint Dickson/Y7 roots.

The supported RR5 chart has a finite public target outside the signed base,
four distinct nonzero-y abscissae, and the parent's denominator and norm
conditions. This work covers that chart. It does not establish completeness
for repeated leaves, identity targets, target-in-base cases or every possible
four-summand chart. The API explicitly rejects unsupported target charts.

## 3. The exact factor split

Let f=X^3+A*X+B and let the original admitted selector be

    D = D_1024(X,1) - tau
    Y = product_i (f(X)^7 - c_i^2)
    H = D*Y.

The fixture establishes that D and Y are monic, squarefree, split over the
coordinate field, and coprime. The original RR5 parameters b,c,d define

    V=X+b, L=cX+d, D_R=X^2+rX+r^2+A,
    U=D_R*V^2-(X-r)*L^2-2*s*L*V,

where R=(r,s) is the public target and U is monic quartic. The original
selector condition is U|H.

For each k in 0..4, introduce a monic U_D of degree k and a monic U_Y of
degree 4-k, and impose

    U = U_D*U_Y,       U_D | D,       U_Y | Y.

**Point-set equivalence.** Over any extension field, every monic U dividing
D*Y has a unique pair U_D=gcd(U,D), U_Y=gcd(U,Y), with monic gcd convention.
Coprimality makes U_D*U_Y=U. Conversely, the three sector conditions imply
U|D*Y. Exactly one value of k applies. Consequently the disjoint union of
the five sector solution sets projects bijectively onto the supported
original quartic-selector chart. This is a written mathematical argument,
not a kernel-checked or signed result. [pcc:unverified]

The degree-zero factor is exactly 1. Its divisibility condition is vacuous;
the implementation never calls a modular multiplication routine with a
degree-zero modulus. A solver restricted to k=2 would cover only a subset.

**Auxiliaries and scheme qualification.** Monic polynomial division defines
every gate's quotient and remainder uniquely over a coefficient algebra,
including one containing nilpotents. The gates are therefore graph
extensions, provided every coefficient equation is retained. For the sector
factorization, the tangent relation is

    U_Y*delta(U_D) + U_D*delta(U_Y) = delta(U).

Conditional on the parent chart's written reducedness/zero-tangent argument,
delta(U)=0. Coprimality and the strict degree bounds on variations of monic
factors then force both factor variations to vanish. This supplies the
conditional reduced-scheme argument; it does not upgrade the parent note to
a formal proof. The proof map remains explicit: factor-divisor bijection,
monic graph extension and conditional tangent argument are written only;
native controls check instantiated identities, not these universal claims.

## 4. Quadratic construction and exact inventories

For a monic modulus M of degree d in 1..4, each multiplication gate imposes
all 2d-1 coefficients of

    left*right - M*quotient - remainder = 0.

The residues have degree below d and quotient degree at most d-2. A regular
gate has d remainder variables and d-1 quotient variables. A final gate
fixes its affine remainder and omits those d output variables. For d=1,
there is no quotient; the implementation handles this case explicitly.

The Dickson binary ladder ends with a square whose remainder is tau+2.
The Y7 path evaluates f^7 using its short addition chain and then evaluates
the degree-eight polynomial G by the existing blocked schedule. Its final
product has remainder equal to the negative low block. Both final
constraints retain all coefficients. The cubic f residue needs extra
quadratic lifts for degree-one and degree-two moduli.

The native qualification emits the following construction counts:

| Dickson roots k | Y7 roots | Variables | Equations | Maximum input degree | Terms in the recorded planted fixture |
|---:|---:|---:|---:|---:|---:|
|0|4|59|60|2|333|
|1|3|53|54|2|271|
|2|2|56|57|2|218|
|3|1|58|59|2|233|
|4|0|59|60|2|296|
|Coupled baseline|All compositions|122|123|2|686|

Source: the native output in `evidence/library-tests-02/stdout.txt` and its
lossless JSON extraction under `evidence/native-summary-v1/`. All five
sector variable/equation counts are exact for this schedule. The term counts
refer to their recorded targets; they are not a same-target performance
comparison. Local unsigned observations. [pcc:unverified]

The balanced sector has 56 variables, while the largest sector has 59.
These are reductions in the size of each input system. Complete coverage
requires five systems, totaling 285 variable occurrences and 290 equations
across separate problems. No solving-degree estimate, Macaulay matrix-size
claim, exponent improvement, runtime ratio, or memory reduction follows
from these input counts alone. [pcc:unverified]

The original constructor already exploits a logarithmic-length Dickson
ladder. The new split changes the constants and decomposition of the work;
it does not by itself improve that asymptotic construction order. Its value
depends on how elimination behaves in the smaller coupled systems. [pcc:unverified]

## 5. What the native controls establish

The final corrected source was compiled with Rust 1.100.0, a locked offline
dependency graph and two compiler workers. Separate read-only copies of
the resulting test executables were hashed before execution. Tests used
one Rayon and one test worker. Each invocation has its own command,
start/end, exit, stdout/stderr, source manifest and binary integrity checks.

The 35 library tests include the 32 inherited controls and three new groups:

1. **Twenty arbitrary factor/parameter controls.** All five sector degrees
   are exercised at four fixed coefficient assignments, including 0, 1, p-1
   and mixed values. Independently formed dense selectors and scalar long
   division agree with 32 terminal residuals. Repeated-root test divisors
   such as X^k check monic graph identities outside valid relation sets.
2. **Five planted signed construction fixtures.** Exactly k original
   Dickson points and 4-k original Y7 points, with signs (+,-,+,+), define one
   deterministic fixture per sector. Test-only curve arithmetic constructs
   the public target; four linear interpolation equations recover b,c,d.
   All original quartic coefficients and complete gate equations agree.
   There is no unknown-target extraction or relation-yield experiment.
3. **Corruption and input controls.** Every one of 270 non-b,c,d coordinates
   is independently changed by 1 and rejected. Three mixed-sector root
   swaps preserve U and the target but violate the selector equations.
   Invalid sector indices, identity/base targets, noncanonical coordinates,
   zero/overflowed deadlines and term limits are rejected.

All 35 tests pass, as do all 8 unchanged public-example controls. The
coefficient-preserving conversion is covered by exact scalar-value checks,
not merely agreement of zero/nonzero outcomes. [pcc:unverified]

The evidence is registered as `PCC-20261124-36f87f`. The maintained standing
command returns `unverified`: this is an unsigned producer attestation,
without an independent signed execution verdict. The claim pins the exact
constructor, qualification summary, successful outputs, and source/binary
manifests. [pcc:unverified]

To reproduce the component controls from a complete checkout with the
locked dependencies available, use Rust 1.100.0 and run:

    RAYON_NUM_THREADS=1 cargo +1.100.0 test --locked --offline --release \
      --manifest-path research/prime31_four_summand_20261122/native/Cargo.toml \
      --lib --example prime31_rr5_quartic -j2 -- --nocapture --test-threads=1

The retained qualification separated compilation from direct invocation of
read-only, pre-hashed executable copies. Each attempt's `command.txt` and
manifests record that exact procedure. `EVIDENCE.sha256` names the retained
individual records; they remain loose so source and claim paths resolve.
Run `sha256sum -c EVIDENCE.sha256` from this study directory to check their
published bytes. This byte check does not supply an execution signature.

## 6. Failures and corrections retained

| Attempt | Actual outcome | Diagnosis and response |
|---|---|---|
|build-01|Compile exit 101|Borrowed `collect` inferred an unsized slice; name `Vec` explicitly.|
|build-02|Compile exit 0|Built the initial executable; no success inferred before testing.|
|library-tests-01|33 pass, 2 fail|`normalise` made every polynomial monic, changing residual and quartic coefficient values.|
|build-03|Compile exit 0|Use the existing grevlex comparator to sort without rescaling.|
|library-tests-02|35 pass, 0 fail|Original controls retained; no assertion or fixture relaxed.|
|example-tests-01|8 pass, 0 fail|Existing public-example regression remains intact.|

The first unit failure is substantive: scaling a whole equation preserves
its zeros, but independently scaling quartic coefficient functions changes
the quartic. Independent scalar comparisons caught the mistake before any
scientific execution. Both failures, the failing source copy, and a clearly
labelled reconstruction of the earlier compiler-failure source remain in
`evidence/`. Source review alone had missed the normalization behavior.

## 7. Validation limits and next discriminating action

**The full-repository library gate has not passed.** Its first attempt failed
before compilation because the sparse checkout omitted a tracked path
dependency. After restoring the exact tracked inputs, the second attempt
lost its execution session without a terminal receipt; it remains censored.
The final attempt returned exit 124 after its 300-second cap, with source
integrity check zero and no root tests executed. Root source and Cargo inputs
are unchanged. These observations are retained in `ROOT_VALIDATION.json`
and `evidence/root-library-01..03/`, registered as `PCC-20261124-f5a76c`
with unverified standing. The standalone construction result does not stand
in for this incomplete gate. No further root attempt ran. [pcc:unverified]

The next useful scientific comparison must measure actual elimination on
the original coupled presentation and on the complete five-sector family.
Freeze one shared total time/memory/work budget for the whole sector family;
giving each sector the entire baseline budget would change the comparison.
Use identical public targets, preserve every incomplete/timeout result,
replay all original equations and signed rows, and count independent useful
relations. Record solving degree, admitted pairs, matrix dimensions/nonzeros,
root recovery, sign reconciliation, duplicate removal, peak memory and full
charged work before considering any pipeline comparison.

The deferred comparison is tracked in the blocked card
[XP-20261123-m31-rr5-coprime-sectors](../../backlog/XP-20261123-m31-rr5-coprime-sectors/README.md).
It still needs an integrated sector solver, a frozen comparison protocol,
independent input/relation checks, and execution admission.

The original registered B677 call remains unexecuted. Its checkpoint upload
transport, complete raw-results readback, known budget/live capacity and
reviewed source/image route remain separate prerequisites. The protected
published-ledger divergence in PR #5099 was not changed by this work.

A parallel source review found a separate input-binding gap in that payload:
the generic checker authenticates equations against the supplied 677 points,
but the wrapper does not independently bind their complete order/orientation
or the exact positive target/control to the frozen fixture. A coherent base
permutation with matching index changes preserves replay. This must be
qualified against a trusted immutable input manifest before treating the
payload as the exact registered experiment. This new sector constructor
does not repair or rely on that wrapper. [pcc:unverified]

Lean/lake were not available on PATH or the standard installed-toolchain
path. No uncompiled Lean proof is presented as verified. The repository's
identity.certificate/v1 evaluator is fixed to 2^61-1, so it was not used for
the user's new M31-only work. The retained M31 tests are not relabelled as
that certificate, an exhaustive parameter check, or a formal completeness
proof. Signing and independent execution attestation remain absent. [pcc:unverified]

## 8. Sources, prior work and reporting

- Original [RR5 mathematics](../prime31_four_summand_20261122/MATHEMATICS.md).
- [Structured selector derivation](../prime31_four_summand_20261122/scaling-b677/MATHEMATICS.md), especially its exact gate and front-end elimination obligations.
- [Frozen B677 protocol](../prime31_four_summand_20261122/scaling-b677/PROTOCOL.md).
- [Faugere and Mou, Sparse FGLM algorithms](https://arxiv.org/pdf/1304.1238), sections 1-2. Change of order assumes an available zero-dimensional basis/quotient; it does not remove the front-end elimination task here.
- [Semaev, ePrint 2004/031](https://eprint.iacr.org/2004/031). The landing page was accessible; the PDF returned 403. No unread text is used as proof.
- [PR #5099 source review](https://github.com/isogeny-labs/crypto/blob/0a3932b2801738c14fd166a7239e519a587e8346/research/prime31_four_summand_20261122/scaling-b677/payload-integration/SOURCE_REVIEW.md). All 18 selected ranges in 9 files were rechecked before adopting its two explicit whole-file pin refreshes.

KB and Conductor were unavailable. A bounded cross-repository search and
open-PR check found the original coupled-selector work and no matching open
sector lane. This is a statement of search scope, not a novelty claim.

No canonical performance chart or leaderboard receives a new point. The
change supplies construction counts and correctness controls, without an
admitted speed, memory or complete-solver measurement. The report's inventory plot describe this local component only.

**Impact assessment: 4/10.** This is an exact, implemented reduction in the
size of the individual elimination problems with meaningful failure-detecting
controls. Whether the five systems actually cost less to solve is the next
unresolved test.
