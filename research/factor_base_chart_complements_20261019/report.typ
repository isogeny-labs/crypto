#set page(paper: "a4", margin: (x: 17mm, y: 15mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: false, leading: .55em)
#set heading(numbering: none)
#let v = json("validation-summary.json")
#align(center)[#text(size:18pt,weight:"bold")[New factor-base leads]\
#text(size:11pt)[Small chart grafts, cofactor fibres and witness diversity]\
2026-10-19 · finite native structural screen]

= A concrete smaller-base candidate
For `icv1-f2m13-t181-515ee569`, a 14-column union supports *242* of 2,002
nonidentity subgroup targets. Earlier GP seed 17 supports *226* with 15
columns. The union grafts a two-column auxiliary chart onto a 12-column base.
This example was selected after inspection; it has no fresh holdout test.

#table(columns:(1.9fr, 1fr, 1fr, 1.25fr),inset:5pt,
[*Case*],[*Columns*],[*Support*],[*Collision rank*],
[Coordinate B029],[16],[144],[0],
[GP17 B031],[15],[226],[3],
[GP29 B035],[15],[214],[4],
[Graft B027+B033],[14],[242],[3])

Rank means differences between alternative witnesses for the same target,
modulo r, after removing zero prime-projection columns. It is not training
rank or rank per unit of online work. The graft raises finite support but
does not raise this rank relative to GP17.

#image("chart-support.svg", width:100%)

*Observed:* the graft's two separate supports cover 200 targets; 42 more need
cross-chart witnesses. *Proposed:* test the four signed auxiliary points by
point subtraction and exact main-base membership, retaining the main-main
solver. This branch is not implemented or priced in this study.

*No claim:* successful F4 work, certified regularity, runtime speedup,
statistical significance, full pipeline cost or wide-curve applicability.

#pagebreak()
= What the new screen checked
Protocol/source commit `4bce792529a2ed81103a91809f65853e9ccde213`; standalone
lock pinned in `6811c867f8b86aa3880d48aa8ddaa5f37efed89c` before execution.
The 60 inventories came unchanged from the completed 2026-10-18 study:
one empty failed inventory retained, 59 nonempty records, 42 exact distinct
signed point sets. Every within-curve two-base union was included, for 313
unions and 355 total cases. No 256-signed-point cap was reached. Repeated
union sets are correlated cases, not independent replicates.

#table(columns:(2.6fr,.8fr,.8fr, 1fr, 1fr),inset:4pt,
[*Exact ICV1 model*],[*r*],[*Bases*],[*Unions*],[*Pair checks*],
[`icv1-f2m7-tm13-ac10a42c`],[71],[17],[136],[4,278],
[`icv1-f2m9-tm5-4a3ea183`],[37],[3],[3],[465],
[`icv1-f2m13-t181-515ee569`],[2,003],[19],[171],[43,365],
[`icv1-f2m31-tm90707-c95f16f5`],[1,439,393],[3],[3],[14,878])

All 62,986 unique unordered point pairs (repetition included) were added as
actual curve points. Direct multiplication of each sum by r decided subgroup
membership. Identity sums were counted separately. The cofactor-image test
agreed for every pair. Every completed singleton reproduced the prior
independent Rust census. All targets and witness pairs are retained.

The arithmetic adapts the published independent audit's polynomial-basis
field/group implementation. It is not a new independent arithmetic
implementation or independent-host certificate. Exact generators, EC1/full
UID, subgroup, field, FB1 identities and derivations remain in copied inputs.
No scalar logarithms were searched for or recovered.

== Preregistered selections and their limits
Per curve: most new cross targets; fewest columns with any new cross target;
highest useful witness-difference rank, with deterministic ties. All 355
compact cases and these selections appear in `summary.json`.

The maximum-cross degree 13 union `B031+B041` uses 36 columns and supports
1,036 targets; 654 already occur in the union of singleton supports, 382 are
new cross targets. Its rank is 35. The largest useful-rank selection is
`B034+B041`, 38 columns, support 758, rank 36. Size and rank are different
objectives. At 21 or fewer columns, no union exceeds the 21-column GP43
singleton's support 482. The smaller graft on page1 is explicitly post-hoc.

== Execution and accounting
Card `CARD-chart-complements-20261019-1` executed one exact native command,
with 70 bindings and 900-second timeout, exit 0. The director assessed
`executed_as_specified`, relay verbatim. Tool elapsed time is an operational
receipt, not a benchmark. The shared managed host earned no timing isolation;
only deterministic structural counts are interpreted.

An initial tool-process refusal occurred before native execution. Post-run
CLI relay checking hit an output-as-input lint defect; unchanged canonical
verification functions checked the receipt instead. Exact failure and
fallback records are preserved. No native result was retried.

#pagebreak()
= Three implementation leads
#image("graft-route.svg", width:100%)

== 1. A tiny-chart graft with an explicit membership fast path
B033 (Frobenius-squared dimension 5 seed 17) has 12 columns/support 196;
B027 (trace-zero/Frobenius dimension 4 seed 29) has 2 columns/support 4.
Their union has 14 columns/support 242, 42 mixed-only targets. Freeze a new
matched-target protocol before implementing the proposed branch. Count
point subtraction, membership, setup, failures, new log columns and all
other arms. A general two-chart solver has up to four ordered branch cases,
or three with justified symmetry handling; this is not free extra support.

== 2. Search for useful cofactor-fibre collisions
On `icv1-f2m31-tm90707-c95f16f5`, coordinate B000 has 68 signed points and 68
distinct images under multiplication by r. Its 34 identity pairs yield zero
nonidentity subgroup pairs. Random B001 similarly has 48 points, 48 images,
24 identity pairs and zero support. Their 58-column union supports only2
targets. Coordinate plus B002 supports 4 with 62 columns. Every degree 31
case has useful homogeneous rank 0.

Proposed filter: seek low-degree chart parameters that make compatible
fibres contain more than inverse pairs. Require actual subgroup support
before online solving; charge the construction and scalar checks as setup.
Projection by the cofactor may destroy simple membership. No degree 83/130
confidence gate or transfer has been performed.

== 3. Admit witnesses for rank and cost, not yield alone
The same-target collision matrix and active witness-pair graph are retained.
Graph connectivity alone does not guarantee rank; rank alone does not give
cheap successful decomposition. Proposed next step: compare first-witness
and rank-aware admission on fresh targets, including the cost of enumerating
alternative roots. No online selector ran here, and old draft cards remain
unrun, zero-funded and unleased.

== Mathematical status
This is a finite checked census, not a new general theorem or polynomial
identity. No Lean proof, IDC1 certificate, regularity certificate or formal
verification of the arithmetic is claimed. Direct scalar checks decide
support; a cofactor identity is not assumed as a substitute. See the named
obstruction `problems/factor-base-shape-to-successful-work.md`.

#pagebreak()
= Validation, reproducibility and prior art
*Standalone native checks:* #v.standalone\
*Required root library command:* #v.root_library\
*Existing root inputs:* #v.root_inputs\
*Local dedicated CI replay:* #v.replay\
*Coordination:* Conductor absent 127; pinned workflow 404; neither is a pass.

The five tests cover field inverses, repeated scalar addition, inverses and
associativity branches, modular-rank dependencies and exhaustive small-curve
cofactor checks. Strict all-target Clippy and release build pass. All input
and source hashes, raw member hashes and extracted-tree deterministic
repacking are checked. Current source/records, failures and operational
limitations are in `VALIDATION.md` and `SETUP_OBSERVATIONS.md`.

Rebuild the standalone research executable from `screen/Cargo.toml`, then
invoke `factor-base-chart-screen screen STUDY_ROOT NEW_OUTPUT_JSON` directly.
Compare the result with `runs/census.json` after extracting `runs.tar.xz`.
Check the archive sidecar, `runs/SHA256SUMS` and `SOURCE_SHA256SUMS` first.
The dedicated CI workflow repeats this; local validation is not a GitHub pass.
The installed unified `factor-base-sweep` executable and its resume bindings
were not replaced. This study's verifier is not a new public launcher.

== Relevant primary literature
#link("https://eprint.iacr.org/2014/318")[Gorla–Massierer, Index calculus in the trace zero variety (2014/318)].
Their extension-degree 3/5 setting supplies context, not validation of the
present degree 31 fixture or prime-degree wide challenges.

#link("https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf")[Faugère–Huot–Joux–Renault–Vitse, Symmetrized summation polynomials (EUROCRYPT 2014)].
Torsion symmetries can compact decomposition systems in the studied regimes.
A fibre histogram is not that implemented solver. Abstract retrieved through
primary archive search; direct PDF open returned 403 this session.

#link("https://arxiv.org/abs/1909.11326")[Euler–Petit, New results on quasi-subfield polynomials (2019/2021)].
Structured-root-set families and their restrictions suggest a separate
membership direction; the abstract does not establish a resulting ECDLP
speedup. Not tested here. No global novelty claim is made for any lead.

== What remains open
Successful solver work for chart unions, fresh-target incremental rank,
complete symmetric native-operation pricing, matched fastest external tools,
certified regularity and large-field scaling remain unmeasured. The new
follow-up card stays draft/unrun/zero-funded/unleased, with no runnable frozen
protocol or checkpoint. No S3 operation, paid host, historical Python rerun,
new campaign through old cards or modification to frozen evidence occurred.

The canonical IC cost scoreboard, progress timeline, curve/leaderboard data
and prior frozen plots remain unchanged: no admitted performance incumbent
or end-to-end claim changed. No timing benchmark entry is appropriate for
these structural counts. Complete artifact links and requirements status
appear in the study README.
