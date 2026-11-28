#set page(paper:"a4",margin:15mm)
#set text(font:"DejaVu Sans",size:9.5pt)
#set heading(numbering:none)
= Authorized local isogeny/ECDLP measurements
19 November 2026. Actual existing native solvers; all timings descriptive L0.

Both field pilots completed 24 parent-verified solve attempts and 24 exact native
replays. Independent PARI checks confirm all recovered scalars. Each field has
one shared ECDLP target, two endpoint models, two measured rounds and three
arms. This is not 24 independent targets. No L2 wall admission or conductor-
effect conclusion is established. [pcc:unverified]

#table(columns:(1fr,1.3fr,1.3fr,1.3fr),inset:5pt,stroke:0.35pt,
 [*Field bits*],[*Method*],[*Source mean s*],[*Image mean s*],
[202], [rho], [7.53], [4.51],
[202], [BSGS], [13.55], [13.08],
[202], [rho A/A], [7.32], [4.48],
[96], [rho], [35.67], [49.07],
[96], [BSGS], [86.72], [83.40],
[96], [rho A/A], [34.00], [47.75],

)
#image("ecdLP.svg",width:100%)

The 202-bit field uses prime subgroup 1125023250121 and a certified conductor
1→2 degree 2 map. The 96-bit field uses subgroup 125000000302579 and a degree 5
horizontal map (conductors 1→1). Different subgroup orders preclude reading
the two field rows as a scaling comparison. [pcc:unverified]

The rho partitions hash coordinates, so paired scalar/algorithm seeds do not
make identical endpoint walks. Their work counts differ; A/A controls replay
the same work counts on each model. BSGS rebuilds its full table every run;
its table comparison/allocation work and rho canonicalization remain unpriced.
GAE figures are lower bounds. No operation-cost or wall ranking is admitted.
[pcc:unverified]

= Execution conditions
The host reports 16 GiB cgroup memory and four CPU cores quota. Automatic
reserved-core admission refuses unavailable CPU/memory PSI. The separately
preregistered local diagnostic uses --cpus none and keeps actual L0 records;
perf events are also unavailable. No build/test/other benchmark overlapped
measured calls. Timings cover the harness solve interval, including per-call
solver setup, not complete large-isogeny construction. [pcc:unverified]

#pagebreak()
= Scalar multiplication across varied points
The new scalar-timing adapter calls the original CountedGroup::mul implementation
in the qualified release library. It adds timing/recording, not new arithmetic
or an ECDLP solver. Three focused native controls and clippy pass; historical
pilot plan bytes match. [pcc:unverified]

#image("scalar.svg",width:100%)
#table(columns:(1fr,1.3fr,1fr,1fr,1fr),inset:4pt,stroke:0.35pt,
 [*Field*],[*Model*],[*Mean µs*],[*Median µs*],[*Samples*],
[202], [source], [330.1], [327.7], [192],
[202], [image], [329.8], [325.0], [192],
[202], [u=2], [329.4], [326.8], [192],
[96], [source], [201.0], [198.0], [192],
[96], [image], [202.7], [199.8], [192],
[96], [u=2], [202.6], [198.6], [192],

)
Each model uses 32 varied target bases Q, six measured rounds plus one warmup.
Every timed sample repeats the same [k]Q 64 times; k varies by paired round seed.
Means/medians above exclude warmups and describe batched throughput, not cold
single-call latency. Source/image/u2 models retain their exact original IDs.
Paired reversal balances pairwise orientation, not every three-model position.
[pcc:unverified]

The timer includes counted multiplication, loop/black-box and ledger updates.
Curve/target construction, independently computed expected results, verification
and output are excluded and retained separately. Field/group ledgers are written
for every sample. All 1344 batch outputs independently match PARI, including
warmups. Wrong output and truncated-coverage audit controls reject.
[pcc:unverified]

The audit's first three versions are preserved: a reserved GP name, duplicate
hex prefix and failed text replacement caused parser/coverage failures. Stderr
and expected-count checks rejected their zero-sample outputs. The final native
checker catches exceptions and requires exact coverage. No failed audit is
reported as successful. [pcc:unverified]

#pagebreak()
= Original 80-bit construction: actual capped attempt
The unchanged prime degree is 729845429276318409687041. The qualified prepared
native constructor was executed locally with G plus 32 frozen source points,
64-sized polynomial tiles, projective batches 64, a 12 GiB address-space cap and
60 second watchdog. The maximum global window is an index guard, not a full
allocation. [pcc:unverified]

#table(columns:(1.2fr,2.3fr),inset:5pt,stroke:0.35pt,
 [*Actual disposition*],[*Recorded value*],
 [Attempt exit],[124; watchdog interrupt retained],
 [Quotient and images],[Not produced],
 [Durable partial coverage],[Not produced; prototype has no resumable journal],
 [Original crossing complete],[No],
)
The 9.7 TB necessary model minimum concerns a single full rectangle; blocking
can avoid that allocation. This timeout does not establish nonexistence or a
universal runtime lower bound. A complete affordable schedule remains open.
No stored prefix or partial tile is turned into a completed quotient.
[pcc:unverified]

== Index-calculus request
The actual wide-field run request for ic.pipeline is rejected before solving:
“ic.pipeline has no prime_mont4 adapter”. The supported wide adapter methods
are rho.negation and bsgs.prime_mont4_v1. No IC runtime or complexity conclusion
is invented. Existing word-field CM controls have their own applicability and
are not substituted for the requested wide-field run. [pcc:unverified]

== Requirement status
#table(columns:(1.3fr,2.2fr),inset:4pt,stroke:0.35pt,
 [*Requirement*],[*Evidence and remaining scope*],
 [Run locally],[Executed with explicit user authorization; EKS is not a prerequisite],
 [Endpoint ECDLP],[Two small-degree control maps, one target per field; all 24 attempts/replays/independent checks pass],
 [Dozens of scalar points],[32 points per model in both fields; six measured rounds, warmup and coordinate controls],
 [Full ECDLP 32-target protocol],[Still open; pilots do not replace it],
 [80-bit prime crossing],[Still open: no target quotient/images or endpoint timings],
 [Admitted performance result],[None: L0 clocks, small pilot sample and unpriced work retained],
 [Raw-data publication],[Local bytes/hashes preserved; S3 identity remains unavailable],
)
All producer evidence remains unsigned/not_covered by a signed independent
verdict. PCC-20261119-22bcb7/77c30f/5c4b81 bind native data, audits and caps;
PCC-20261119-4a8667 binds the descriptive first-field analysis. No leaderboard
row or world-record claim is promoted. [pcc:unverified]

#pagebreak()
= Next larger map: concrete proposal, not executed
Retained CM case 0 has field prime 5629830687772387, ordinary source order of
conductor 1, and a certified floor model of conductor 33555419. The original
cofactor-one subgroup order is 5629830654216971. Its explicit point transport
is still missing here; these retained equations are not a new evaluated crossing.
[pcc:unverified]

The existing complete evaluator takes a rational kernel on auxiliary curve T.
The old benchmark points are on its quadratic twist E. With twist factor d,
E has coefficients d²A,d³B. If f is the normalized quotient x-map on T, the
conjugated map has formula (x,y) ↦ (d f(x/d), y f′(x/d)); its codomain has
coefficients d²A′,d³B′. Square-root factors cancel. Source/output validation
must use the actual twisted models. Implementation and cost remain open.
[pcc:unverified]

#image("twist-bridge.svg",width:100%)

Implement a separately versioned native evaluator, preserve immutable defaults,
and qualify tiny square/nonsquare twists against direct PARI kernel-polynomial
maps. Then freeze the retained large case and attempt its full map locally,
verify the retained codomain, transported prime-order targets and dual
composition on the original subgroup. Do not silently replace that subgroup
or infer map correctness from j or field size. [pcc:unverified]

This proposal is a lower milestone than the original 80-bit criterion. The
separate 4774-bit Frobenius-conductor arithmetic certificate supplies no evaluated
crossing or endpoint timings. No broader record search is completed by this
local report. [pcc:unverified]

== Sources and exact reproduction
Current source: native/complete_pipeline.cpp and its frozen prepared evaluator;
actual ecbench runner/spec/record and prime_mont4 implementation; paired-schedule
and retained-fixture conductor certificates. Old CM input lineage is
conductor_gap_cm_orbits_20261015/prepared/kernels-reference.txt and catalog.json;
the constructor derivation is its native/CONSTRUCTOR.md. See exact argv/source/
rlib hashes in build and pre-call manifests. [pcc:unverified]

Primary ordinary-order background:
#link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[Galbraith, introduction/section 2],
#link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf")[Galbraith–Gilchrist–Robert, section 2.1].
KB remains unavailable; prior exact primary PDF review is reused. No latest-
literature or novelty assertion. [pcc:unverified]
