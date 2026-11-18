#set page(paper:"a4",margin:16mm,footer:context align(right)[#counter(page).display()])
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:0.5em)
#let s=json("../summary.json")
#let t=s.target_cases.at(0)
= The full-phase construction frontier
18 November 2026 · fixed 80-bit target · pinned native source

The minimum single-rectangle necessary coefficient-slot model over every
baby window is *#t.all_window_single_rectangle_minimum_bytes bytes*.
Under the example *#(t.cap_bytes)-byte* cap, every fitting tiling in the
current traversal entails at least
*#t.all_window_contexts_minimum_under_cap coordinate contexts* and
*#t.all_window_rectangles_minimum_under_cap rectangles*.
These are conditional source-model bounds, not observed RSS or timings.
PCC-20261118-61e7ed is unsigned/unverified, verdict not_covered.
[pcc:unverified]

#image("frontier.svg",width:100%)

The hatched bar is a lower bound, not an achieved execution. The two other
context bars are exact counts for the retained fixed block/window models.
No count is converted to seconds, field operations or dollars.

== Two separate live phases
#table(columns:(1fr,1.5fr,0.8fr),inset:5pt,stroke:0.4pt,
 [*Phase*],[*Mandatory live data*],[*Slots*],
 [Baby product],[b roots plus monic h of degree b],[2b+1],
 [Giant norm],[h plus zeroth giant jet of degree 2g],[b+2g+2],
)
The giant jet is a product of g monic quadratics (Z−x_j)². It therefore
has exactly 2g+1 coefficients and coexists with h at evaluation. The old
baby-only necessary bound remains valid; this adds another mandatory phase.

The model excludes GMP payloads, all other jets, prepared cache/scratch,
points, multiplication storage, serialization and allocator overhead.
A fitting bound does not establish actual memory fit or launch admission.
The example cap is not a live quota or capacity readback.

#pagebreak()
= All-window derivation and native controls
Let n=(ell−1)/2, G=floor(n/(2B)), L=n−2BG, and i/j be the numbers of
baby/giant blocks. Then 0≤L#(sym.lt)2B. For C slots in the cap, set
M=floor((C−1)/2). Necessary fit implies each actual chunk is at most M,
so B≤iM and G≤jM.

The pinned traversal processes W=B+iG+L coordinate contexts. Hence

  2MW ≥ n+2M,    W ≥ 1+ceil(n/(2M)).

Also n#(sym.lt)2B(G+1)≤2ijM(M+1), giving

  rectangles ≥ floor(n/(2M(M+1)))+1.

These count source-loop contexts, including regenerated giant contexts and
leftovers; they are not distinct points or full operation counts.

== Exact optimum of the necessary single-rectangle model
For fixed C, a permitted B fits max(2B+1,B+2G+2) iff

  n #(sym.lt) 2B floor((C−B)/2).

This expression increases separately on each parity in the allowed B
interval. Testing the two rightmost B values gives an exact feasibility
predicate; binary search in C finds the minimum. Its witness and preceding
cap are checked. This optimizes the stated bound, not runtime or physical RAM.
DERIVATION.md gives the integer argument and all hypotheses.

#table(columns:(1fr,1.65fr),inset:5pt,stroke:0.4pt,
 [*Native control*],[*Outcome*],
 [Exhaustive windows],[1,048,576 B configurations over n=2..2048; optimizer equals enumeration],
 [Blocked inequalities],[6,291,456 configurations at six block sizes; both bounds hold],
 [Actual polynomial objects],[Nine monic h/zeroth-jet length/coexistence checks on existing fixtures],
 [Prior output metadata],[Seven B/G/coverage/block records match retained native pipeline output],
 [Admission controls],[Five invalid inputs reject; small bound fitting cap still refuses launch admission],
)
All qualifiers exit 0 with empty stderr and unchanged pre/post hashes.
This is local integer/layout proof qualification. No new curve search,
polynomial construction at target scale or benchmark session ran.
[pcc:unverified]

#pagebreak()
= Remaining construction and measurement obligations
#table(columns:(1fr,1.65fr),inset:5pt,stroke:0.4pt,
 [*Route or requirement*],[*Current status*],
 [One rectangle, example 128 GiB],[Fails the necessary bound for every B in the pinned source model],
 [Bounded current traversal],[Context and rectangle lower bounds above; actual total cost unmeasured],
 [Prepared norm backend],[Correctness retained; the constrained windows/loops remain],
 [External-memory or new traversal],[Not implemented or ruled out by these inequalities],
 [At least 80-bit conductor crossing],[No target quotient/non-kernel images or lower endpoint supplied],
 [Paired scalar/ECDLP/IC trials],[Not executed; transport, reference/IC and cloud admission remain open],
)

Window tuning alone cannot make the single-rectangle pinned model fit the
example cap. An external-memory or different algorithm/data plan requires
its own construction, work/I/O and cost evidence. The inequalities do not
prove that every such route fails or that a tiled run is impossible in time.
[pcc:unverified]

The minimum-model witness has B=604088333472, G=302044166736 and
L=1180929268736. Its leftovers differ from the retained zero-tail choice;
it is not a performance recommendation. All endpoint transport, verification
and measurement costs remain in the original request. [pcc:unverified]

== Evidence and retained build failures
The first audit build stopped on indentation lint; the first layout build
used get_ui on a GMP expression. Preserved source overlays/errors explain
both corrections. Corrected builds pass with warnings denied. No arithmetic
attempt failed, and no production evaluator source was changed.

The source/input/binary/qualifier hashes, native outputs and derivation are
retained. Named proof/manifest subjects stay loose; remaining raw records
are deterministically archived. The producer evidence is unsigned and
unverified/not_covered; no Lean/lake executable is exposed. The readable
argument and bounded checks are not a compiled formal theorem.
[pcc:unverified]

Neutral progress metrics label exact necessary source-model bounds. They
remain separate from certified conductor bit lengths and measured-crossing
series. No benchmark/leaderboard row moves. Earlier narrower source/phase
graphs remain pinned and valid.

The earlier S3 archive obligation remains unresolved, so the existing PR
stays draft. No target, certificate or benchmark job is active. [pcc:unverified]

Sources: pinned complete_pipeline.cpp (baby_product, giant product/finish,
coverage), prepared_norm.hpp, original CONSTRUCTOR.md, previous native
schedule audit and installed FLINT layout. README.md and RESULTS.md link
all source and receipt paths. KB unavailable, wrapper 127.
