#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
#let summary = json("../../followup-run/summary.json")
#let lp = json("../../followup-run/lp_summary.json")
#let pct(x) = str(calc.round(x*100,digits:1)) + "%"
= Hyperelliptic IC follow-ups
14 November 2026. Record: 20261114T2210Z-hec-followups. Source: ad74146534.

The hybrid solver passed its frozen genus-2 criterion: median solve time is 22–50% lower at
p >= 503, with lower median total time. The direct squarefree oracle passed at all genus-3
rows and genus-4 p=61, but missed the 10% oracle reduction at p=31. Both remain selectable;
Fast and the reference oracle remain the defaults.

== What ran and what was checked
Protocol: FOLLOWUPS.md, frozen at d11ab32f84. Same curve families, instance rule, subgroup orders
and full-base walk as S1/S2a. Genus 2: y²=x⁵+3x³+2x²+x+c; genus 3: y²=x⁷+x³+cx+1;
genus 4: y²=x⁹+x³+cx+1. The exact c, ell and generator are reproduced by pick_instance(p,g).
ICV1 covers elliptic curves; these instances use the equation and subgroup.

The paired run used 20 seeds (20260916..20260935) on eleven instances, with Fast/reference-oracle,
Hybrid/reference-oracle and Fast/direct-oracle. The order reverses on odd seeds. All 660 answers
verified k D1=D2. Oracle arms also agree on trials, rows, pivot hash and modular multiplications.
The hybrid's pivot order changes, so its answer is checked directly. Release and debug tests pass,
including all 55 original round-eight hybrid instances and oracle/brute-force equivalence.

Single-thread local arm64 macOS host, L0 isolation; no exclusive CPU reservation or second host.
Ratios below divide medians over seeds; no confidence interval is claimed. Timing noise affects
whole-pipeline ratios: the genus-2 oracle arithmetic is unchanged and its ratios are controls.

#image("ratios.svg",width:100%)
#table(columns:6,inset:3pt,table.header([g],[p],[Hybrid solve],[Hybrid total],[Oracle stage],[Oracle total]),
..summary.map(r => ([#r.g],[#r.p],[#pct(r.hybrid_solve_ratio)],[#pct(r.hybrid_total_ratio)],
[#pct(r.oracle_ratio)],[#pct(r.oracle_total_ratio)])).flatten())

#pagebreak()
== Reduced-base 0/1/2-large-prime baseline

The bounded baseline materializes the full rational-place catalogue, truncates the base to its
first m places, and walks only in that base. It retains split relations containing up to two
distinct off-base places, with signs and multiplicities. Exact elimination cancels off-base
columns modulo ell. This handles coefficients without assuming that every unsigned cycle cancels.

All 36 runs verified, with no cap reached (100000 trial cap). Every accepted decomposition and
its partially eliminated row was checked on the Jacobian. The latter check uses a certified
projection into ell-torsion: compute every catalogue place's order, take their common annihilator,
remove its single ell factor, then check every projected place is annihilated by ell. Projection,
validation and linear algebra are included in the reported wall time.

#image("large_primes.svg",width:100%)
#table(columns:6,inset:3pt,table.header([p],[m],[LP limit],[Seeds],[Median trials],[Median total ms]),
..lp.map(r => ([#r.p],[#r.m],[#r.lp],[#r.n],[#r.trials_median],[#calc.round(r.total_ms_median,digits:2)])).flatten())

The projection/order checks dominate this baseline: totals range from about 43 ms to 2.53 s.
The trial savings establish the collection mechanism, but total cost needs an implementation
that reuses a separately charged certified projection and a sparse graph/filter. The current
partial-pivot storage is dense in the full catalogue size. It is bounded here, not a claim of
the published forest algorithm's complexity.

== Artifacts, limits and next work
Raw: followup-run/paired.txt, lp.txt, binary.sha256. Derived: summary.json, lp.json, lp_summary.json.
Jq/shell computes summaries; plots.py lays out recorded fields. No external same-input comparison
was run. KB IC_TECHNIQUE_INTEGRATION_20261012 supplied repository precedents for exact partial
elimination. GTTD (2007), eprint 2004/153, was queried; the primary PDF returned HTTP403. No
published record timing or novelty claim is made. No validation certificate is attached.

Next: independently replay the gains, export the same matrices for an external solver comparison,
and replace dense partial storage with a graph/filter while retaining the signed identity gates.
