# Three-point tangent conic pencils, 27 November 2026

Question:whether a different conic base scheme(two copies of one point and
two other distinct points)adds an independent relation on the original225
columns. Prior four-distinct-point scans do not exhaust this family. Reuse
reviewed conic/local-series methods and primary sources in REPORT.md;KB MCP
and CLI unavailable. No novelty or new PCC/scalar/speed claim.

## Frozen native domain and basis

Use actual1434-point pool,map,135split lines and134-row/481-column core.
For s0..15choose one of two residual points on each seed s,s+32,s+64,and
choose each of the three points as forced tangent:384raw choices. Require
three distinct noncollinear anchors,neither other anchor on that C-tangent,
and at most one anchor on each known line. Native predicates partition
invalid geometry,image exclusions and admitted distinct choices. Record
all counters/tuples;selection cannot depend on output ranks.

Construct four constraints on six conic coefficients:three point values
and the derivative along C at the forced point. Require rank4and exactly
two independent null vectors,checking each constraint. No other pool point
may be a base point. This also distinguishes the pencil from completed
four-distinct-point base schemes. At most one anchor per known line excludes
every product of two known lines;check actual dictionary membership too.

## Complete candidates and exact multiplicities

Enumerate parameter buckets through every other pool point,first-derivative
conditions at the two ordinary anchors,and the second Taylor coefficient
at the forced tangent anchor. Its first derivative is identically zero
and cannot select a parameter. Require the second-coefficient constraint
to select a unique conic and verify its order is at least3. A full degree8
section either contains another point or has higher contact at an anchor,
so these candidates cover this base scheme. Support uses three distinct
anchors,not a duplicated list entry.

Cache exact conic-monomial coefficients from the existing native precision9
smooth local lift. Derivative-nonzero points have order1;otherwise exact
coefficients2..8determine order. Every accepted section compares these
orders with original homogeneous evaluation/order,then requires degree8,
actual image sum2kappa and BN/GMP/namedOpenSSL row checks. Retain complete
support,multiplicities,origin,zero/dependent rows and exact row-space
certificates. Only independent added rows enlarge the capped basis.

## Nonempty controls and replay

Use two known seed lines sharing old anchor0,together with one residual
point on each. Their product belongs to the same three-point/tangent
collector and must be recovered with a nonzero actual row. Keep21direct
old products,repeated-line,wrong-parent,row/support,plan and batch-coverage
controls. Using the identically zero forced first-derivative selector must
reject;the selected higher-contact candidate must be present and have
order>=3;omitting it rejects. Bad constraint/basis/tangent origin must
reject. Separate numerical rank7control never enters actual rows.

Native final combine replays accepted geometry,forced tangency,independent
rows,certificates and exact gap-free batches. Core ranks are computed;
original full/auxiliary restoration adds102private ranks and is derived.
Any positive old-base gain still requires actual KM and separate elliptic/
elimination replay before goal success.

## Limits and evidence

One CPU,1GiB address space,300sbuild,600sper worker,at most128pencils per
batch,256dense rows/1024columns,4096accepted conics,3000pool points remain
unchanged. Exhaustion is incomplete. All nonoriginal labels are auxiliary.
No new fibres or completed scientific scans. Fresh immutable source/input/
binary/plan/results,argv,exits,costs,hashes and failures. Conductor ignored
per explicit user instruction. This completes only the frozen384choices.
[pcc:unverified]
