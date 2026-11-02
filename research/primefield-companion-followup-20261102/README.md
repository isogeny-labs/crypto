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
