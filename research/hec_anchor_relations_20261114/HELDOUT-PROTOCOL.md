# Held-out dense curves and increasing field sizes (frozen before code/run)

Question: does the anchor/forest construction's cold-cost advantage survive new curve
coefficients, independent target scalars and larger fields? The mathematical10x objective
remains unchanged. Prior compact pilot is terminal: all correctness gates pass, but gains over
orbit are0.83..1.32 and do not justify equivalent arithmetic rewrites [pcc:unverified].

## Cohort and immutable selection

Twenty attempted fixtures: two coefficient families at each (genus,prime):
g2={1009,4001,16381,65537}; g3={127,503,1009}; g4={31,61,127}.
Primality is checked before field construction. For family j=0,1, seed SplitMix64 with
(2026111501+j) XOR rotate_left(p,17) XOR rotate_left(g,40). Draw all2g+1 lower coefficients
uniformly in Fp and set leading coefficient1. Reject nonsquarefree f, preserving attempt count.
These dense one-infinity curves differ from the development thread's sparse coefficient laws.

Try at most128 polynomials, first8 canonical degree-one places each. Native order_bsgs budget
200000; largest prime factor ell must be>=1000 and4ell>=Hasse lower bound. Require ell^2 above
the conservative integer Hasse upper bound, D1 nonzero/valid, ell D1=0. Select the FIRST valid
fixture, without timing any solver. Serialize exact f,D1,ell and selector metadata; no field or
seed replacement on unavailable/timeout. This is a generator-order fixture, not a full Jacobian
order assertion. Fresh dense curves are newly generated inputs, not claimed existing registry facts.

All rejected attempts and selection failures remain in stderr/receipts. Selection is common
fixture preparation excluded equally from algorithm clocks and reported separately. Selection
child cap30seconds, aggregate child wall cap600seconds. No curve is selected on solve timing.

## Target panel and arms

Each fixture has five independent known-answer single-target cases, seed index s=0..4.
Scalar generator seed=9000001 XOR (g<<48) XOR (p<<16) XOR (family<<8) XOR s; choose
k=1+rng.below(ell-1). Construct D2=k D1 outside every algorithm clock. Every arm receives the
same exact D1,D2,ell and walk seed20260916+s. Print target u,v and expected/recovered scalars.

Primary candidate: compact anchor source840006608f. Secondary: original orbit, unchanged.
Twelve controls: native and table; branches1/64; Fast/Hybrid; native reference/direct oracle.
This gives8 native+4 table arms. Add a SECOND independently cold table/Fast/branches1 arm as
A/A diagnostic, excluded from choosing fastest control. Fifteen arms x100targets=1500 potential
timed cells if all fixtures and gates succeed. Rotate arm order by target/fixture index and
reverse on odd target seeds; do not choose arms from the new timing results.

First fully group-gate compact on every target (100potential gate cells). Its existing gate
checks every generated orbit against reference, raw principal identities, projected anchor
identities, descents and final scalar. Gate failures censor BOTH candidates on that target;
controls still run. No time ratio is reported for a capped/invalid candidate or missing control.

## Execution and cost boundaries

One sequential local macOS worker at a time; L0 observational, no isolation assertion. No cloud
workers or spend. Method clocks include all cold tables/catalogue, anchors, inverses, descents,
all attempts/retries, forest and solve, and the implementation's final group verification.
Process launch, fixture parsing/validation and known-answer construction are excluded equally.
A further harness group replay is timed separately, without treating it as independent replay.

Timed child wall cap30seconds, gate child cap60seconds. Aggregate gate child wall cap1800s,
timed child wall cap1800s. Native/table trial cap2million; existing candidate cap1million,
descent10000 and dense forest16million words unchanged. These are explicit logical/time caps,
not an asserted OS RSS limit. Preserve exit, panic, wrong answer, gate failure, cap and remaining
unstarted cells. Stop on aggregate budget without resetting it on resume. Expected raw<100MB.

Run each cell in a fresh child process. Parent writes terminal receipts with frozen build ID,
exact args and input SHA256, stdout/stderr paths, exit/timeout and child wall. Resume only an
identical terminal cell; mismatched identity fails closed. An incomplete temporary output is
not a success receipt. Kill/reap only the parent's own timed-out child. No concurrency.

## Interpretation and rejection

Keep per-target paired cold totals and per-arm summaries. Compare primary candidate to ALL
twelve controls; report the fastest measured control conservatively and A/A spread. Five-target
median ratios are secondary summaries, not multi-target amortization. Never multiply earlier
ratios, hide unavailable fixtures/caps, or fit an asymptotic exponent from this short ladder.
Loss or capped targets falsify broad claims at that scope. >=10x in this L0 screen would justify
controlled independent replay and novelty review, not by itself establish the requested result.
Mathematical novelty, external fastest-tool comparison and signed replay remain requirements
before any final breakthrough claim. No benchmark-index/leaderboard promotion from this screen.

Sources checked: existing principal/orbit derivation, COMPACT-PROTOCOL.md and terminal raw;
GTTD signed-cycle/principal-divisor prior art retained there. KB revision9141e386254340a9bdba3036f2bb706a,
last_refresh2026-11-14T21:44:48Z, ready true/lexical, predates current work. Queried held-out
hyperelliptic/large-prime and signed-graph/rank terms. Opened source:c64201c3064a6795a01664dede4a7f3d,
crypto:docs/curves/HYPERELLIPTIC_INFRASTRUCTURE.md, boundary: use explicit odd-degree/one-infinity
models, do not infer an even-sextic conversion. Other hits are pointers, not novelty evidence.
Conductor check/claim attempts returned500 and HTTPS/HTTP mismatch; no service repair was
attempted. Work remains within the previously claimed anchor-study directory.
