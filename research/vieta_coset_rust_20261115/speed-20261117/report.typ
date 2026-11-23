#set page(paper:"a4", margin:14mm)
#set text(font:"DejaVu Sans", size:8.7pt)
#let s=json("run/summary.json")
#let configs=(
 (key:"coset:word62:m8192",label:"Coset word62 / 8192"),
 (key:"coset:word62:m16384",label:"Coset word62 / 16384"),
 (key:"coset:p224:m2048",label:"Coset P224 / 2048"),
 (key:"rational:word62:m2048",label:"Rational word62 / 2048"),
 (key:"rational:p224:m128",label:"Rational P224 / 128"),
 (key:"rational:p224:m512",label:"Rational P224 / 512"),
)
#let group(key,pop)=s.groups.find(g=>g.group==key+":"+pop+":finite")
#let ratio(g,arm,phase)={if g==none {[missing]}else{let x=g.paired_ratios.at(arm+"/half:"+phase);if x.n==0{[missing]}else{str(calc.round(x.median,digits: (if x.median < 0.01 { 5 } else { 3 })))}}}
#let n(g)={if g==none{[0]}else{str(g.arms.at("half").successful)}}
#let count(g,arm,phase)={if g==none{[missing]}else{let x=g.arms.at(arm).at(phase);if x.n==0{[missing]}else{str(int(x.median))}}}
#let elapsed_table(pop)=table(columns:(1.7fr,.25fr,.55fr,.55fr,.55fr,.55fr,.55fr,.55fr),inset:4pt,
 [Configuration],[n],[S/H query],[N/H query],[L/H query],[S/H cold],[N/H cold],[L/H cold],
 ..configs.map(c=>{let g=group(c.key,pop);(c.label,n(g),ratio(g,"school","query_ns"),ratio(g,"newton","query_ns"),ratio(g,"lookup","query_ns"),ratio(g,"school","cold_ns"),ratio(g,"newton","cold_ns"),ratio(g,"lookup","cold_ns"))}).flatten())
= Vieta half-GCD: matched query and complete costs
17 November 2026 · VIETA-SPEED-MAIN-20261117

*Observed result:* At word62/m16384, conditioned finite targets have half-GCD
query median8.072s versus4.944s Euclid/Newton and12.448s schoolbook. Half uses
1.4437 times fewer query multiplications than Euclid/Newton, yet takes longer.
Lookup has the lowest median full cost in every group. The backend stays opt-in.

*Registration failure:* Canonical exports retain360 main/60 calibration seed
mismatches: the frozen explicit_list cycles cells, while published configuration
seeds were held per configuration. Exact published configs/source were followed
and separately audited. This is not strict protocol qualification; frozen files
and failed checks remain. See REGISTRATION_ERRATA.md.

The shared Rust engine provides schoolbook reference, classical Euclid with
Karatsuba/Newton arithmetic, and recursive half-GCD. General homogeneous
rational towers certify every sibling fiber and handle denominators without
interpolation. Each comparison arm constructs the complete point factor base;
lookup additionally constructs its hash index. The query phase is reported
separately from cold construction-plus-query cost.

*Timing standing:* every elapsed ratio below is a local diagnostic, not an
admitted L2 speedup. CPU4 is reserved, but PSI is unavailable and unmovable host
threads remain. Counters and exact complete relation sets are separate evidence.
All scientific observations remain #raw("[pcc:unverified]").

#table(columns:(1fr,1fr,1fr,1fr,1fr),inset:5pt,
 [Planned],[Attempted],[Succeeded],[Failed],[Warmup calls],
 [504],str(s.attempted),str(s.successful),str(s.failed),str(s.warmup_calls))

== Seeded random-abscissa finite targets
#elapsed_table("seeded-random-abscissa")
== Conditioned-sum finite targets
#elapsed_table("conditioned-sum")

These are *precomputed medians of matched-input ratios*, with up to6 retained
inputs per population/configuration. S=schoolbook, N=Euclid/Newton,
L=lookup, H=half-GCD. A ratio above1 favors half-GCD at that phase; below1
favors the numerator. No populations, fields, sizes, representations, warmups,
identity targets or calibration inputs are pooled. Exact observed ranges,
4096-replicate descriptive percentile median intervals, all raw values and
failures are in run/summary.json. Small-N intervals are conditional descriptions,
not population confidence or exponent claims.

#pagebreak()
== Charged arithmetic and elapsed ratios
#image("counts.svg",width:94%)
#image("timing.svg",width:94%)

Graphs reproduce precomputed paired medians and descriptive median intervals.
The logarithmic ratio axis places1 at equal cost. Counted multiplications include
inversion and square-root internals. Allocation, indexing, hashing, sorting and
memory traffic have no field-operation price, so count reductions cannot be
renamed wall-clock speedups. Diagnostic clocks capture those local costs.

#pagebreak()
== Complete charged multiplication counts
#for pop in ("seeded-random-abscissa","conditioned-sum") [
=== #pop
#table(columns:(1.7fr,.2fr,1fr,1fr,1fr,1fr),inset:4pt,
 [Configuration],[n],[Schoolbook],[Euclid/Newton],[Half-GCD],[Lookup],
 ..configs.map(c=>{let g=group(c.key,pop);(c.label,n(g),count(g,"school","total_work_muls"),count(g,"newton","total_work_muls"),count(g,"half","total_work_muls"),count(g,"lookup","total_work_muls"))}).flatten())
]

== Controls, sampling and exclusions
The same target is sent to six fresh child processes from one immutable binary.
Two warmups and12 retained inputs per configuration follow a frozen Williams
order, balanced within each population. Half A/A is the same negative/null
panel, not a second independent control panel. Half-double makes two identical
queries after one construction. Its query counts must be exactly2 times half;
A/A query counts must be identical. Elapsed positive-control separation is
judged against the frozen1.25 threshold and A/A noise before any timing
interpretation. Controls failing timing separation leave counts intact and
block clock interpretation. All failures/deadlines/missing inputs remain visible;
there are no retries, restarts or imputed values.

Random targets use seeded abscissae and a canonical square root; no uniform
curve-point claim is made. Conditioned targets are sums of two base points and
must have a nonempty answer. Identities, if any, form separate groups. The word62
curve has p=4179340454199820289, A=2, B=3; no prime-group-order assertion.
The wide curve is standard NIST P224. Rational performance fixtures are Mobius
conjugates of squaring; auxiliary-isogeny search is outside these timings.

Algorithm clocks include construction of the descriptor/full split certificate,
all curve lifts, and lookup's extra index. Setup objects stay alive through the
clocks; teardown is excluded. Fixture/target generation, scalar complete-set
verification, process startup and evidence serialization are excluded from these
algorithm clocks and retained separately. Fixed worker/stage/class/outer deadlines
are in the frozen protocol. This is a component-query experiment, not a DLP run.

#pagebreak()
== Degree argument: exact checked scope
The proposed hand bound is $d_F <= 2 sum_i (q_i-1)+2$ for the actual two tower
chains and quadratic Vieta generators, under distinct complete sibling splitting,
coprime numerators/denominators and $deg D_i < q_i$. Terminal values are constants.
No radicalization or field equations may silently replace these generators.

The Lean proof gate passes21 theorems with zero trusted exceptions. The new
shared generic theorem proves the actual-degree normal-form induction. Its bridge
uses the existing constructible spaces and last-fall definition. Finite F17
certificates check literal non-cartesian partition, all coordinate identities,
full splitting, coprimality and denominator nonvanishing. The quotient by X squared
retains a provably nonzero nilpotent; nonsplit slope blocks are not replaced by
empty field-point sets. The arbitrary-depth tower normal-form instance is still
unfinished, and independent signed standing is absent. These conditional and
finite checks do not establish the full quantified bound.

A separate native prime-order19 curve test on y squared=x cubed+2x+2 overF17
checks the nonempty relation (5,1)+(5,1)=(6,3) through a normalized Velu fiber.
Repeated abscissae need the diagonal branch: imposing repeated-quadratic
polynomial divisibility would discard this valid relation. Repeated tower fibers
instead violate distinct splitting and are rejected. See ASSUMPTION_AUDIT.md,
PROOF_MAP.md and the pinned Lean project for exact obligations.

== Frozen custody and reproduction
Measured source: #raw("27bc4bf511cda40f904324901895a246f40a9c75").
Unchanged shared core source derives from the previously measured implementation
9d2106eb2a; the new driver changes accounting and timing, not that arithmetic.
Qualifier SHA-256:
#raw("ecf84d40b2638f80c590142d6388c4f3ca95670de60bb5f1625a8c91da142be5").
Analysis/source SHA-256:
#raw("222598cbc12a9a98eb576a4f6aa1d953186e6e5b535d88167ae8f1b1c77dfa4b").
Primary protocol canonical hash:
#raw("f88b57e6b52dfd4a0fe21d41acba34742865ef4c25c959494b082c63ad3c9851").

Calibration has its own protocol af0985e46cd0...,72 successful/replayed calls and
no warmups. Its native forecast was3553.997s; this is a planning estimate, not
an elapsed outcome. Main has distinct seeds. Analysis, matrix, budgets and sample
size were frozen before affected data. run/main-publication.json verifies the
actual remote Git commit and exact local tree blobs; all10 qualifier manifest
entries match. Git-only/local custody is authorized. Immutable binaries remain
in ignored local storage; no local S3 operation or remote availability is claimed.

Proof project: Lean4.35.0, Mathlib cfc55fc58714..., shared crypto-lean23569e7f1da1
(PR26), original CryptoResearch framework0ca36c9d5b49. The focused gate checks
transitive axioms; no sorry/admit/native_decide/custom axiom is introduced.
Shared PR26 CI jobs never started because GitHub reports an account billing
lock; this is separate from the passing focused local kernel checks.
The alternate checker probe failed on incompatible object headers; no alternate
checker success is claimed. Focused native unit/CLI/integration/fmt/clippy checks
are reported separately from the unrun full-root suite.
