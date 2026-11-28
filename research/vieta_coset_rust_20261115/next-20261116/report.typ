#set page(paper:"a4", margin:15mm)
#set text(font:"DejaVu Sans", size:9pt)
#let s=json("run/summary.json")
#let cfg=(
 (key:"coset:word62:m8192",label:"Coset · word62 / 8192"),
 (key:"coset:word62:m16384",label:"Coset · word62 / 16384"),
 (key:"coset:p224:m2048",label:"Coset · P224 / 2048"),
 (key:"rational:word62:m2048",label:"Rational · word62 / 2048"),
 (key:"rational:p224:m128",label:"Rational · P224 / 128"),
 (key:"rational:p224:m512",label:"Rational · P224 / 512"),
)
#let group(key,pop)=s.groups.find(g=>g.group==key+":"+pop+":finite")
#let value(g,arm,phase)={if g==none {[—]} else {let v=g.arms.at(arm).metrics.at(phase).muls;if v.n==0 {[—]}else{str(int(v.median))}}}
#let ratio(g,phase)={if g==none {[—]}else{let v=g.paired_ratios.at(phase+":newton/half");if v.n==0 {[—]}else{str(calc.round(v.median,digits:4))}}}
#let count(g)={if g==none {[0]}else{str(g.arms.at("half").metrics.work.muls.n)}}
= Half-GCD and certified rational Vieta towers
16 November 2026 · fresh construction plus query arithmetic

The counted Rust engine now has an opt-in recursive half-GCD backend and a
general homogeneous rational-tower extractor. Every backward sibling fiber is
certified before use; finite-target evaluation uses no interpolation or field
division in the quadratic algebra. These are producer observations and hand
arguments pending independent signed validation: #raw("[pcc:unverified]").

Half-GCD lowers the matched multiplication count at the two large word-field
coset sizes, but increases it at P224/m2048 and all three rational configurations.
The option remains explicit; the historical default is unchanged. The largest
word-field paired count ratio is1.4437. Data statement c8911d is unverified.

== Conditioned finite targets: total charged multiplications
#table(columns:(1.65fr,.25fr,1.05fr,1.05fr,1.05fr,.55fr),inset:4pt,
 [Configuration],[n],[Newton / Euclid],[Half-GCD],[Lookup],[N/H],
 ..cfg.map(c=>{let g=group(c.key,"conditioned-sum");(c.label,count(g),value(g,"newton","work"),value(g,"half","work"),value(g,"lookup","work"),ratio(g,"work"))}).flatten())

Values are native medians per target population. N/H is the native median of
matched Newton/half-GCD count ratios. A ratio above1 means fewer multiplications
for half-GCD at that scope. All values/ranges and every arm are retained in the
native summary. Counts include inversion/square-root internals; additions are
reported separately. Memory allocation, hashing and sorting have no field price.
No elapsed-time, fastest-backend, exponent or complete-DLP gain follows.

== Flow
#table(columns:(1fr,1fr,1fr,1fr,1fr),inset:5pt,
 [Planned],[Attempted],[Succeeded],[Failed],[Missing],
 [240],str(s.attempted),str(s.succeeded),str(s.failed),str(240-s.attempted))

Two warmups per configuration were predeclared; their calls are kept but excluded
from comparative summaries. Six retained targets per configuration alternate a
seeded random-abscissa/square-root draw and a constructed sum. Three targets per
population is descriptive only. Arms on the same target are matched repeats,
not additional independent targets. Identity targets are separated.

Fresh construction is executed for every arm. It includes descriptor/full-fiber
certification and each method's required point-table lifting; lookup builds its
table/hash set, finite Vieta candidates build no unused point table. Query
counters remain available separately. Fixture generation and the external
complete-set verifier are excluded and recorded.

#pagebreak()
== Separating the arithmetic effect from construction
#image("counts.svg",width:100%)

Markers show precomputed native medians for construction-plus-query and query
separately, for the conditioned finite-target population. Missing successful
cells are marked, never imputed. SVG/PDF carry vector figures; PNG is a preview.

#image("ratios.svg",width:100%)

Ranges are observed matched-input ranges, not confidence intervals. A/A repeats
test count determinism. The unused-backend lookup control tests count invariance;
neither is a clock-noise estimate. The native summary records every control,
failure and target population.

#pagebreak()
== Other retained population: finite random-abscissa targets
#table(columns:(1.65fr,.25fr,1.05fr,1.05fr,1.05fr,.55fr),inset:4pt,
 [Configuration],[n],[Newton / Euclid],[Half-GCD],[Lookup],[N/H],
 ..cfg.map(c=>{let g=group(c.key,"uniform-x-rational");(c.label,count(g),value(g,"newton","work"),value(g,"half","work"),value(g,"lookup","work"),ratio(g,"work"))}).flatten())

The sampler uses seeded modular reduction and a canonical square-root choice;
the population label does not assert uniform sampling of curve points. No
comparison pools populations, representations, fields, sizes or identity targets.

== Degree argument: a separate exact audit
The proposed hand bound for the exact lifted generators is
$d_F <= 2 sum_i (q_i-1)+2$ under full splitting, separability, coprimality and
$deg D_i <= q_i-1$. It is not a running-time bound or a formal theorem verdict.
The old pinned statement and proof files are unchanged.

A different Rust calculation constructs the finite space $V_T$ by graded
Gaussian elimination and all permitted coordinate multiplications. It checks
literal projector identities, discarded blocks, normal-form coordinate actions
and truncated quotient dimensions on the predeclared cases. It retains
nilpotent and nonsplit quadratic algebras when the target ordinate is zero.

#let audit=json("degree-audit-final.json")
#table(columns:(2fr,.4fr,.4fr,.6fr,.7fr),inset:4pt,
 [Exact case],[S],[T],[Rank],[Model dim.],
 ..audit.cases.map(c=>(c.case,str(c.S),str(c.T),if c.constructible_closure_computed{str(c.rank)}else{[not run]},str(c.predicted_algebra_dimension))).flatten())

The mixed-degree case checks projector/coordinate certificates only, as declared
before the exact run. These finite producer checks do not prove the quantified
theorem or supply an independent signature. General certificate/normal-form
review remains an independent obligation #raw("[pcc:unverified]").

The general API also accepts normalized Vélu maps. A separate nonempty F17
Vélu-fiber test checks two ordered relations on a prime-order target curve.
The performance fixtures here are Möbius conjugates of squaring; they do not
measure auxiliary-isogeny search or assert transfer of a discrete logarithm.

#pagebreak()
== Source, checks and custody
Measured source: #raw("9d2106eb2a9f65ffe94020b237301ce910a799e1").
Qualifier SHA-256:
#raw("42d0cfd089122041b85c935f55ed7a4cf548c157c439889073ba574106f41e78").
The frozen native analyzer is #raw("native/next_compare.rs").
Build/lockfile/compiler/hash receipts and official pre-call Git publication
receipts are retained. Immutable executables live in the ignored local cache;
no remote binary-archive availability is asserted.

Preflight protocol:
#raw("72e70723e7472370cec78bb1b61af11ad325edc2d4586595c28ee5803d956aa6").
Main protocol:
#raw("ac9f3009b19ef044ca5a7ae0b483dd8f4f2924e8ccb827ed6bba584447dbb586").
Both were frozen/published before their affected calls. The primary matrix,
sample size, stage limits, analysis and stopping rule were unchanged after data.

The separate calibration retained/replayed all60 calls. Environment reconnect
lost its benchrec/isolation supervisors while the original bounded native driver
continued without restart; aggregate launcher exit and isolation receipt are
unknown. Its unified export records contended=unknown and invalid clocks.
The native per-class point forecast is for planning only. Main used fresh
isolated admission and a persistent supervisor under the unchanged3600s envelope.
Every clock remains below-L2 diagnostic, with wall_valid=false.

Qualified shared source:12 unit groups and5 original/new comparison CLI tests;
a separate positive Vélu integration test was added after calibration without
changing the qualifier. Post-run tests also check nonempty distinct/diagonal
two-torsion cases and budget rejection. Formatting/clippy pass. Full root release-library
preparation reached180s while compiling; no full-root-suite pass is claimed.
Post-run receipts describe the final checks and wide scalar replay separately.

No cloud experiment, local S3 operation, protected benchmark-index/leaderboard
promotion, administrative merge bypass, force push or empty CI commit.
The bench record is censored. Prospective S3 addresses are not upload or
availability statements. Scientific observations and the hand argument remain
unverified pending independent signed standing.

== Reproduction and limits
Use the committed #raw("run-panel.sh"), frozen source/binary hashes and protocols.
Raw checkpoints are one verifier plus five arms per input; fixture files and
all stderr/driver/supervisor outputs are retained. Re-run scalar replay and
summary with the source-pinned native binary; check unified exports against
their protocols with file verification. No timing advantage is established.

Sources read: FLINTv3.3.1 nmod_poly/hgcd.c and gr_poly/hgcd.c; internal polynomial
and prime_tower sources; original HKYY last-fall definitions and PKM section3.3.
KB MCP/wrapper unavailable; primary/internal fallback, no novelty claim.
