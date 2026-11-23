#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9.3pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= One Rust executable: IC, graph elimination and rho
16 November 2026 · Runpod CPU · Restricted weak Fp6 cover family

Source #raw(s.source_revision.slice(0,9)), p=#display_value(s.p),
subgroup order #display_value(s.l), seed #display_value(s.seed).
All six workers recover scalar #display_value(s.actual_scalar), with native
elliptic verification and held-out answer agreement. Workers receive only
the public curve, generator, target and order; their answer placeholder is0.
This known-answer cohort is distinct from unknown-log T1 targets.
[pcc:unverified]

#table(columns:(155pt,170pt,125pt),align:left,inset:4pt,
 [Configuration],[Observed primitive bitops],[Complete cost],
 ..s.outcomes.map(o=>([ #display_value(o.arm) ],[ #display_value(o.accounting.observed_primitive_subtotal_bitops) ],[Unknown])).flatten(),
)

#image("observed-subtotals.svg",width:100%)

The graph arm collects algebraic cover-sieve relations with at most two
omitted factor-base points. Those points become weighted graph vertices;
elimination cancels them and retains combination witnesses over original
rows. Certified homogeneous rank controls collection stopping. Affine
descent equations then determine the target logarithm. The copied Rust
qualifier replays the graph combinations, independently solves the affine
system and reconstructs checkpoint ranks and arithmetic containment.
[pcc:unverified]

The four rho configurations cross plain/negation walks with batch1/16,
r256 and the same DP policy. Their source-bound native tower calibration
charges55Fp products/44additions per product or square,80products/42additions/
one base inverse per inverse. The qualifier checks executed options,
reconstructs explicit phase counts and reprices them. [pcc:unverified]

One cold process per arm, no warmups or automatic retries. Dedicated
4vCPU/8GiB Runpod container; actual pinned CPU #display_value(e.pinned_cpu).
900seconds per worker,5500whole,7200runner lifetime; process wall is
practicality data. Missing native-container, PRNG and other operation counts
leave every arm unrankable. Calibration work is outside attack subtotals and
inside process wall. Totals, pricing band and ratios remain null; the minimum
rho subtotal does not establish a cheapest complete reference.
[pcc:unverified]

Trial33 is preserved: its generator succeeded, then all six starts failed
before scientific code because stdout had created the worker's create-new
raw path. Trial34 uses distinct raw/stdout paths; native transport-only
controls reject the frozen old wrapper and accept the corrected one.
No failed raw was replaced. [pcc:unverified]

Original argv, flow, statuses, raw records, copied source/binaries/qualifier
and portable hashes: #raw(e.run_path). Artifact collection precedes owned
pod termination (GraphQL accepted; subsequent lookup null). Large raw data remain exact and hash-bound;
the native export claims no unavailable canonical-schema conformance.
Producer checks carry no independent signed verdict. [pcc:unverified]
