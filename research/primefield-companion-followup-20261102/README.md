# Companion inverse constructions and frozen binary validation

This follow-up preserves the work after PR #3266, from standalone source commit
`f208412`. The earlier `primefield-companion-schur-20261101` snapshot is unchanged.
The source elliptic-curve scope remains prime fields, with the original method
exclusions. No numerical experiment is launched by this publication.

## Construction and evidence

| Component | Delivered scope |
|---|---|
| Recursive inverse | Two-query leading generators of width s+2; explicit recursive inverse preparation and reusable width-s inverse generators. Derived and syntax checked; numerical validation pending. |
| Pivot transport | Same-field row swap with two transpose queries and width s+2; first-column pivot and right-null certificate primitives. Derived and syntax checked; numerical validation pending. |
| Binary trace codec | Canonical streaming fingerprint, independent decoder and eight software tests. Serialization cost has not been benchmarked. |
| Binary validation | Frozen full 1,698-row / 34,248-product contract, 24 source locks, two input locks, all seeds, exact launch command and single-owner launcher. Thirteen software tests; zero numerical trials. |
| Original fused run | Terminal exit 124 at its original deadline. All 877 completed rows retained: complete 849-context candidate arm and 28 paired A/A contexts. Full-panel acceptance remains pending. |
| Schur/lift integration | Full 642-context checker preparation and locks; zero numerical trials. |

The recursive inverse replaces supplied solves with explicit preparation, under
the stated nonzero recursive-pivot condition. The pivot primitive is not an
automatic balanced-tree fallback; repeated scalar pivots can increase width.
See the original derivations and their Bostan et al. 2017 Section 6.2 reference.

The original fused raw archive contains 431,414,955 bytes, SHA-256
`db199b944109851a17bbd77b3e0d547310d62b89bcaf538804288d06357ce4a2`.
The complete candidate arm records 17,124 products. All 28 completed A/A payload
pairs match; 821 planned rows lack completion receipts. These are the saved
custody results, not a fresh numerical replay. The original invocation must not
be restarted or its cap extended.

## Restore and inspect

From the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/primefield-companion-followup-20261102
python3 research/primefield-companion-followup-20261102/verify_publication.py
```

Native adjacent artifact manifests retain the exact original paths, hashes and
byte counts. `publication-manifest.json` binds all 110 imported files to their
standalone source revision. The verifier checks every imported file and all
three included source-lock sets without invoking a numerical kernel.

The two archived numerical inputs remain in the predecessor publication. Restore
them using that snapshot's README and `verify_bundle.py --hydrate`; their exact
hashes are retained in this snapshot's input locks. This follow-up is an overlay:
for a separate replay workspace, first restore the predecessor, then overlay the
imported follow-up files. Do not overwrite an existing immutable evidence tree.
All other historical path and host assumptions remain visible in the source.

The planned experiment command targets the original standalone workspace.
`launch.py` requires exact local source/input hashes, a clean committed source,
and a receipt binding every source to an actual immutable GitHub revision.
Publication does not itself create that receipt or authorize a rerun of old
invocations. A fresh invocation retains one CPU, 8 GiB per-process address space,
4 GiB output, a shared 3,600-second deadline and one attempt with no retries.

## Reports and validation

- [Recursive inverse derivation](companion-recursive-inverse/DERIVATION.md)
- [Pivot transport derivation](companion-pivot-transport/DERIVATION.md)
- [Codec design](companion-trace-codec/DESIGN.md)
- [Capped fused report](companion-fused-validation/reports/capped/report.pdf)
- [Frozen validation contract](companion-fused-binary-validation/CONTRACT.md)

The copied tree passes all 21 software tests (13 instrument/admission/resource
tests and eight codec tests). The preparation AST check confirms unchanged
numerical stages, verification body before fingerprinting, and main schedule
after the declared guard/output-routing substitutions. These checks do not
execute the research kernels or establish runtime improvement. Repository-wide
checks and artifact publication status are recorded separately as they finish.

## Added construction: exact generator compression

`companion-generator-compression/` adds deterministic two-sided rank
factorization and a separately implemented certificate checker. The derived
output width is rank(G K^T), so all algebraic generator redundancy is removed
without forming the full displacement. The same field and operator are kept.
Source commit: 3c49347e183e08baca60365ead000b3438de64d8. Its separate publication.json binds
the ten original files; the initial 110-file manifest remains unchanged.

Validation is syntax-only for three Python files, plus a rendered and visually
inspected one-page report. Numerical/corruption/performance trials are zero.
The certificate checks factor identities and triangular rank minors; actual
Meter-count validation remains pending. Setup has derived classical O(n s^2)
cost, and smaller width helps only when later query savings repay that cost.
No frozen numerical caller was changed. See its DERIVATION.md and
reports/design/report.pdf for the proof, exact arithmetic schedule and prior art.

## Added construction: structured solve or normalized witness

`companion-rank-solver/` connects compression, explicit same-field row/column
pivots, scalar leading inverses and Schur primal/dual lifts. It is a separate
iterative scalar-pivot route; no frozen program is changed. Under the stated
field/operator and primitive-correctness assumptions, the derivation covers
singular compatible and incompatible systems, returning either Ax=b or a
normalized A^T w=0, w^T b=1 witness checked against the original operator.
Feature annihilation remains a separate unchecked requirement.

Source revision afc04886374411a8d982c67bc315f506f89fc8d2; nine original files
bound by its own publication.json. Two Python files are syntax checked; the
one-page report is rendered and visually inspected. Numerical trials remain
zero. This path charges pivot searches, swaps, scalar solves, compression,
Schur setup and root verification; it does not inherit balanced-inverse speed
or memory bounds. See DERIVATION.md for transport identities and costs.

## Added construction: cached pivot-column transport

`companion-cached-pivot/` reuses the first nonzero column and the zero prefix
established by search. Derived additional transport queries change from two
to one for interior columns and from two to zero for the last column. The
first-column identity branch is unchanged. Explicit scalar products drop
from two to zero; column-1 interior adds n subtractions. These are derived
schedules, not timings or a total-cost result. Nonsingular nodes take the
unchanged first-column branch.

A separate solve_cached.py preserves the original solver; a reproducible
normalized-AST audit permits only one import and two call-site changes.
The internal cache is one-use and source-bound, not a public certificate.
Four Python files are syntax checked and one-page report visually inspected;
zero numerical/performance trials. The twelve original files are separately
bound in publication.json at source revision 0174135d9384b1ffec715de9fb5fd714a7fce56d.


### Leading-border Schur cache

Adds `companion-border-cache/`: reuses the transported pivot column, obtains the first row with one transpose query, and answers scalar-leading-vector requests by charged scaling. Borders are released before descent. For the specified setup/RHS/lift phases, the derived full-source query count changes from 2w+8 to 3+ell (ell is zero or one), with n(2w+5) additional field products. Static inspection records 4w+8 instrumented native integer products per successful fused application; these units are kept separate and runtime is unmeasured.

Five Python files parse; the integration AST and preserved-kernel syntax audits pass; the one-page PDF is visually inspected. Fourteen original files have an independent publication manifest. Numerical, mutation-control and performance trials remain zero. Full reference, release-lifetime, cache and counter validation awaits funded execution.


### Compact scalar unwind frames

Adds `companion-compact-lift/`: retain two border tails, a scalar inverse and one RHS entry, without keeping Schur/source/generator objects or a full parent RHS in unwind frames. Derived primal/dual head identities replace each nonzero lift source query with n-1 field multiplications and additions; common scalar solve arithmetic and original-system verification stay intact. Zero input tails introduce dot arithmetic where the reference already skipped its query. The specified setup/RHS/lift source-query count is three per nonterminal node. No runtime dominance claim.

Each frame has 2n field-value slots, giving N(N+1)-2 across full scalar descent, excluding audit/certificate/current-operator memory. This remains quadratic; no heap measurement is claimed. Four Python files parse, complete normalized integration AST and static ownership/source-call audits pass, and the one-page report is visually inspected. Thirteen original files have a separate source manifest. Numerical, corruption-control, heap and performance trials remain zero under the existing cost gate.


### Conditional border sharing and rank-one representation limit

Adds `companion-shared-border/`: checks t=aJc exactly at each frame and shares the column tail for zero-row, reversal or scaled-reversal modes; failures retain both tails. With q=n-1, retained border field slots are q, q or q+1 instead of 2q. Detection failures and scaled-dot overhead are charged; no guaranteed inheritance through Schur reduction or runtime gain is claimed. Independent coefficient checker is implemented but unexecuted.

Derives C_a(g)=sum g_i Z_a^i with displacement (a-b)g e_last^T. Fixing a=1,b=2,g0=1 leaves q arbitrary tail values at displacement rank one. Exact self-contained encodings for arbitrary dot queries therefore need q log2(p) bits in the worst case; recurrences stored by d initial values and d coefficients cannot cover this family if 2d<q. This concerns the stated operator/query model, not an elliptic lower bound. It directs further compression toward actual feature-map structure or charged recomputation.

Six Python files parse; frame and solver normalized-AST checks pass; one-page report inspected. Fifteen originals are independently manifested. Numerical, certificate mutation, heap and performance trials remain zero under the existing cost gate.


### Feature-metric principal-pivot primitives

Adds `companion-metric-pivot/`: derives DA symmetric from the actual public compiler A=D^-1JBJ, selects a nonzero scalar or invertible antidiagonal 2x2 principal pivot using at most one query beyond the first nonzero column, applies simultaneous row/column generator transports, and supplies explicit small inverses. Principal Schur reduction preserves the trailing permuted diagonal metric. Leading rows can be reconstructed from cached columns with n+2n*cut charged products including metric validation and no source queries.

Each nontrivial simultaneous swap still costs four source queries and adds four generator columns; the pair branch needs at most two swaps. Local metric/reciprocal checks do not certify global symmetry. This is a set of implemented primitives and a derived closure argument, not a complete new driver. Next integrate shared weighted block borders and full singular solve/witness behavior. Dumas–Pernet2018 supplies established exact-field 1x1/2x2 pivot context; no rank-profile or fast-complexity claim is inherited.

Six Python files, including the preserved constructor dependency, parse and dependency hashes are recorded; the one-page report is inspected. Fourteen originals are separately manifested. Numerical, corruption-control and performance trials remain zero under the existing cost gate. Frozen callers are unchanged.


### Complete conditional weighted-symmetry driver

Adds `companion-metric-solver/`: connects exact compression, principal scalar/antidiagonal-pair pivots, simultaneous transport, explicit small inverses, Schur descent, compact block lifts and original-system solution/witness verification. Zero-source and singular compatible/incompatible behavior is retained. The caller must establish DA=A^T D; inverse-metric checks alone do not certify it. The public weighted Gram compiler supplies this invariant.

Cached weighted rows/columns handle leading-supported requests, leaving two full source applications per nonterminal node across the specified generator/RHS/lift phases, versus 2w+8 for the same uncached block. Cache arithmetic, row reconstruction, metric validation and changing transport widths are charged separately. Primal lifts use C tails and shared root metric through Fz=D1^-1 C^T D2 z; dual lifts use C^T v. Frames retain kq+2k field values plus two length-n index tuples, with shared root metric2N; total heap and runtime remain unmeasured.

Five Python files parse; the zero-terminal and final root-verification ASTs exactly match the preserved rank solver; dependencies are recorded. Thirteen originals are separately manifested; the one-page report is inspected. Numerical, corruption-control, heap and performance validation remain unrun under the cost gate. Feature annihilation remains separate. This completes the previously missing driver implementation; acceptance still requires its full frozen cohort and controls.


### Paired symmetry transport with selected-column reuse

Adds `companion-paired-transport/`: combines simultaneous row/column swaps into the exact same ordered generator columns as the sequential reference. Cached pivot columns provide Au and A^T u through the known diagonal metric. Each nontrivial swap needs two structural queries plus zero or one missing-column query, all at input width w, versus four reference queries at widths w,w,w+2,w+2. Output width remains w+4; cache keys and column coordinates are transported together across pair pivots.

Outside-query cost is 3n+4 field products and 4n+1 additions, versus 4 and 2n+2. Derived fused native-product counts change from 16w+48 to (2+h)(4w+8), h in {0,1}; these units are separate and no net runtime gain is claimed. The complete driver changes only its transport import, with normalized whole-driver AST equality checked. Identity swaps and upstream cohorts are preserved.

Four Python files parse; twelve originals are separately manifested and the one-page report is visually inspected. Numerical, corruption-control, heap and performance validation remain unrun under the cost gate. No zero-prefix query elision or reduced metric validation is included in these counts.


### Immutable inverse-metric views

Adds `companion-metric-views/`: validate canonical inverse pairs once in an immutable root, then derive index views by checked permutations and trailing-coordinate selection. Selector and row-cache entry points still match every supplied metric pair to the same-meter root coordinates and captured prime, replacing repeated field products with native checks. Global DA symmetry remains caller-established; frozen Python objects provide internal provenance, not public attestation.

Derived removed products equal the sum of nonzero selector-node dimensions plus the sum of nonterminal cache-node dimensions. Full scalar descent removes N(N+1)-1; full even pair-only descent removes N(N+2)/2-2; initially zero source removes zero. The initial N products remain. Native label/permutation/coordinate checks, tuple allocations and index metadata remain explicit; no runtime or whole-heap claim. Source queries, pivot rules, paired transport, block equations and root verification are unchanged.

Seven Python files parse; four complete AST substitution audits cover driver, selector, row helper and block cache. Sixteen originals are separately manifested and the one-page report is inspected. Numerical, corruption-control, heap and performance trials remain unrun under the existing cost gate.

### Proven-zero prefix transport

`companion-zero-prefix/` preserves the first nonzero search column separately from the selected pivot. Proven-zero column indices follow both simultaneous swaps; missing front columns reuse this information. Derived query savings for a nonempty zero prefix: one scalar or two pair transport queries. All generator arithmetic and final checks are preserved by three static substitution audits. Five Python files parse; report inspected. Numerical, corruption, heap and performance trials: zero. Native set/materialization costs and internal provenance assumptions are explicit in DERIVATION.md. Source `3090ba584b463d078516a478257b85e987d6a086`.

### Zero-coordinate deletion

`companion-zero-delete/` removes a proved-zero prefix using weighted symmetry. The restricted operator has dimension n-j and width w+2 before compression, constructed with one transpose query and the retained first nonzero column. A nonzero prefix RHS instead gives an immediate normalized witness and leaves the full rank unknown. Compact zero-padding frames preserve both primal and dual lifts; final root checks remain. Three declared-substitution AST audits pass; six Python sources parse; report inspected. Numerical/corruption/heap/performance trials zero. Repeated compression/selection, copies and width growth remain charged. Source `d6a7316b2818bb633e3aa6ebd7c2aa94e5ed29ec`.

### Selected columns through checked compression

`companion-selected-reuse/` restricts cached pivot columns after deletion and independently checks compression factor identities before rebinding them to the new operator. Derived repeated-search query reduction is one for the first-diagonal branch, two for secondary-diagonal/pair branches; checker products/reductions and copies remain separate costs. Driver preserves pivot choices and root checks. Two static AST audits pass; five Python files parse; report inspected. Numerical/corruption/heap/performance trials zero. Source `d072d3cb829735f2ccd03fa43e6c0104d649d4ad`.

### Compact actual-feature diagonal

`feature-diagonal-events/` derives a constant background plus q exceptional diagonal corrections for the original weighted character-feature Gram. Geometrically scaled interval events construct these with O(q^2) field operations and O(q) arithmetic workspace, without an r-point sweep. C nonzero bounds the root zero-column prefix by q-1; diagonal zeros alone do not certify zero columns. A constant-weight diagnostic reports at most q+1 forbidden values without changing inputs. Five Python files parse; original constructor bytes/pattern AST checked; report inspected. Independent direct-character checker implemented but unexecuted. No numerical/corruption/heap/performance trial. Schur-diagonal closure remains a separate construction. Source `0de2baf50632fd07aceca933541334c8526dbd79`.

### Constructor-bound root diagonal pivot

`feature-root-pivot/` adds a public-constructor solve entry. One fixed weight tuple feeds source and diagonal; generator snapshots and independent compression verification bind a root-only scalar proposal. Its queried column must match the predicted nonzero diagonal. All-zero diagonals use the ordinary selector. Separate RootSelection carries an empty zero prefix and unknown first column, avoiding false skipped-column provenance. The binding is consumed before children. Two full static integration audits pass; six Python sources parse; report inspected. Numerical/corruption/heap/performance trials zero. Costs include snapshots/checker/setup; changed pivots require equation-based validation. Source `ad5439372ce969521f19069d5b75e0a6303511c5`.

### Feature-coefficient Schur construction (2026-11-03)

`feature-coefficient-schur/` adds exact supplied-point scalar/pair updates to
the fixed character-feature coefficient matrix and residual RHS, sparse dual
lifts, a sufficient pointwise terminal and an independent all-feature certificate
checker. It uses no r-row array; state is O(m^2)=O(q^6), and no claim that this
is always smaller than r is made. Derived schedules include evaluations,
small inverses and lifts; public pivot/terminal selection remains external.
See `DERIVATION.md` and the inspected `reports/design/report.pdf`. Five Python
sources parse; source/pattern/power preservation and checker separation are
static audits. Numerical/corruption/heap/performance trials: zero.

### Character-signature exclusion and sparse selection (2026-11-03)

`feature-signature-selection/` derives the agreement bound K=min(r,2^q+q)
for every function of q shifted quadratic characters. K<r analytically excludes
X; otherwise the precheck is inconclusive. A bounded supplied-point stream
constructs sparse cubic feature-annihilation witnesses by retaining independent
feature rows. Under independent uniform sampling in the strict regime, expected
offers are at most (m+1)r/(r-K). No random sampler or field sweep is launched.
The saved F479 width-15 failure and retained recovery panels are outside the
strict regime. See the derivation and inspected PDF. Six Python files parse;
dependency/import checks are static. All numerical trials remain zero.

### Saved character fibers and targeted repair (2026-11-03)

`feature-fiber-repair/` isolates two normalized pair differences from the saved
F479 width-15 duals. Independent singleton checks prove no function of the old
characters can represent X, regardless of degree. New shift15 and old shift1
give a rank-two projected repair with target coefficients [405,0]. Thus
X-405*chi(X+15) lies in the old cubic span conditional on the preserved rank477
result. Constructing its old-feature coefficients remains next. Five bounded
local exact/software test methods passed, including F7 checks for the preceding
coefficient and signature primitives. No cloud run or performance comparison.
See the exact packet, independent check, test receipt, derivation and inspected PDF.

### Explicit corrected-span program (2026-11-03)

`feature-consistent-repair/` constructs the missing old-feature coefficients.
The published polynomial has476 nonzero cubic terms at offsets0..14 plus
405chi(X+15), and its identity with X passes independent square-set checking
at all479 points. The representative-only uncorrected fit fails exactly at
the two omitted mates,173/440. A reusable rectangular reference factor and
full coefficient program are preserved; no timing or recovery-panel claim.
See `certificate/program.json`, `check_certificate.py`, the exact receipt and
inspected PDF. Factor reload solving remains a separate unchecked operation.

## Preserved decoder binding

`feature-decoder-binding/` binds the verified repaired coefficients to the byte-preserved original decoder. Three receipted local test methods cover four recorded sign-oracle fixtures, strict sign validation and mutation isolation, and persisted-factor reload reproducing the full-field verified program. These are recorded API checks; fresh source cases remain zero. See its derivation, report, receipt and complete-cohort validation plan.

This publication preserves the completed standalone Python source snapshot and receipts as historical evidence under the repository language policy. It is not a native execution deliverable. Further algorithm execution-path development must use a native implementation retaining these contracts and fixtures.

## Native sign-mask evaluator

`feature-native-signmask/` replaces coefficient-by-sign multiplication with support-zero and negative-parity masks in dependency-free Rust. All3 native unit methods passed, including the saved polynomial on all479 field inputs against direct multiplication and X. For that477-term program,1291 field multiplications become zero;16 oracle calls and477 terms remain. This is a derived operation count, not a timing claim. Width<=64 and modulus<=u64::MAX/2 are explicit. See its derivation, source locks, native test receipt and inspected report. The constructor does not certify primality or the supplied polynomial identity.

## Native normalization and division core

`feature-native-division/` ports the original public-point division recurrences and invariant-differential jets to dependency-free Rust. One native invocation passed3 methods: saved normalization seeds on all3 original generators,9 saved division values, Ward addition identities in value and derivative, and domain controls. A point-bound cache supports reuse across requests. Value-only requests currently compute derivatives too; no total-cost saving is claimed. The original kernel is preserved as text for review. Geometry and resultant stages remain to port before the full source is executable natively.

## Scalar-only native source values

`feature-native-value-division/` supplies a separate ValueDivision for source calls that already have a normalization seed. Its production path constructs no derivatives; use the prior jet engine when phase/derivatives are needed. Three selected native tests pass, with three historical reference tests filtered out:9 saved values, agreement at original generators and negatives through fixed indices including u64::MAX, parity and domain controls. Six products per new nonbase recurrence value is a derived count excluding setup and map overhead, not timing. Separate caches do not share setup. See its receipt, derivation and inspected report.

## Native prescribed source grid

`feature-native-grid/` ports the original known-multiplier Jacobian grid and one-inversion affine batch. Three unit methods pass: all71 saved slots of the three original public setups match labels/projective/affine/inverse-Z values, independent affine multiplication agrees, and exceptional/invalid cases pass. Conversion uses7N-3 field products and one inversion excluding group/setup work, as in the original. This exposes individual-grid conversion; projective-only and mixed-stride shared-window paths remain next. See its source/input locks, projection script, native receipt and inspected report.

## Native mixed-stride shared window

`feature-native-window/` preserves identity/prepared-P/generated projective initialization and mixed stride updates, then converts all finite lanes once. Four native unit methods pass, including exact agreement with1426 saved slots and64 public shifts across O,P,2P,-P windows. Extra public-prefix validation is charged separately. Prepared state is immutable and curve-borrowed. See source/fixture locks, projection script, receipts and inspected report. The production API currently consumes public shifted points; a next cost-reduction candidate is generating those points projectively and converting them alongside the grid lanes. That candidate is not implemented or measured here.

## Joint conversion of generated shifts and grid lanes

`feature-native-joint-window/` now takes Q and prefix length, generating shifts projectively and co-converting them with the grid. Three native methods pass:64 generated public points and1426 golden slots agree; target product/inversion assertions pass. On O/P/2P/-P, inversions15/16/16/14 become1 each, with296/321/330/298 additional products including new point checks. Setup/additions/native overhead remain outside this arithmetic table; no elapsed-time claim. See its chart, report, original counts and receipts. Shared daily log was reserved by another task; component SESSION.md preserves the handoff.

## Native polynomial engine

[feature-native-polynomial/DERIVATION.md](feature-native-polynomial/DERIVATION.md) ports the original cutoff-12/Karatsuba, Newton division and product/remainder-tree source path. Four bounded Rust methods pass, including three original resultants and partial values, independent schoolbook/long-division controls and ordered multipoint checks. Exact receipts and an editable PDF report accompany the source. Normalized-source integration remains next.

## Native normalized source integration

[feature-native-source/DERIVATION.md](feature-native-source/DERIVATION.md) joins phase, value division, joint geometry and product-tree resultant with exact homogeneous recurrence, partial tails and calibration. Three integration methods pass: three original raw records and three public identity-crossing windows. Phase sharing and retained stride setup are next; complete costs and full cohort remain separate.

## Shared source seeds and retained preparation

[feature-native-shared-seeds/DERIVATION.md](feature-native-shared-seeds/DERIVATION.md) adds borrowed immutable prepared state, four seed anchors and the original outer seed recurrence. Five fixed F509 windows match all 80 signs and 78 finite direct phases; recorded direct calls are 0/0/1/0/2. Reuse on three original curves and two tiny tail-branch fixtures passes. Costs retain recurrence inversions and all downstream work. Decoder binding is next.

## Fresh native verified recovery

[feature-native-recovery/DERIVATION.md](feature-native-recovery/DERIVATION.md) binds shared fresh source signs to the repaired native program and verifies every candidate by independent affine multiplication. Eight fixed public targets, including the four repair-fiber points, recover exactly; deleting the repair term fails verification. Three methods pass once. Complete-cost instrumentation and original full cohort remain next.

## Native stage accounting

[feature-native-accounting/DERIVATION.md](feature-native-accounting/DERIVATION.md) instruments all seven native layers with exact reversible source mapping. Eight receipts match unchanged recovery; three checks pass. Separate setup/preparation/query/verifier records identify polynomial remainder work and allocations as concrete next targets. Unit inversions are distinguished; no timing weights or peak-memory claim.
