#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
#let s=json("presentation.json")
#let e=json("local-evidence.json")
#let dv(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Factor-base and checkpoint balance
16 November 2026 · Trial37 · Runpod CPU · Restricted weak Fp6 cover family

Source #raw(s.source_revision.slice(0,9)), p=#dv(s.p), seed=#dv(s.seed),
scalar order #dv(s.l). Seven planned public-input arms, one cold process
each, zero warmups/retries. Completed #dv(s.flow.completed), recovered
#dv(s.flow.recovered), failed/capped #dv(s.flow.failed_or_capped).
The fixture generator holds out its planted answer; successful workers
must recover #dv(s.actual_scalar). [pcc:unverified]

#table(columns:(65pt,75pt,55pt,65pt,65pt,80pt),inset:4pt,align:left,
 [Arm],[Outcome],[Small base],[B values],[Sources],[Descent residuals],
 ..s.outcomes.map(r=>([#dv(r.arm)],[#dv(r.status)],[#dv(r.small_base)],[#dv(r.b_count)],[#dv(r.source_rows)],[#dv(r.descent_residuals)])).flatten()
)
#image("phase-subtotals.svg",width:100%)

The six graph arms cross75/85/95percent small factor base with rank strides
32/128. Each uses the existing structural bound to defer impossible full
rank checks. The sieve finds at least seven small roots, solves the
remaining quadratic and verifies original Jacobian relations. Every
successful graph arm retains affine combination replay, independently
checked checkpoint rank, actual target determination and elliptic replay.
The copied Rust qualifier reconstructs gate prefixes using graph traversal
and dense large-column rank; it also checks gate scheduling and known
charged events. Common descent prefixes and, within each small fraction,
original-source prefixes are checked. [pcc:unverified]

Numbers shown are observed incomplete primitive subtotals, with all recorded
phases included. Container/PRNG/RREF/DSU work remains incomplete, so complete
totals, bands, ratios, and eligibility remain unknown/false. This parameter
diagnostic provides no fastest-IC/rho-parity or wall-speed admission and
does not compose costs from previous cohorts. Failures and caps are kept.
[pcc:unverified]

Limits:60s fixture,300s worker,2300s whole,2700s runner,8GiB; dedicated
4vCPU/8GiB CPU, actual pinned CPU #dv(e.pinned_cpu). Builds finish before
timing; wall is practicality only. Exact argv/flow/status/raw/source/binary
custody: #raw(e.run_path). The source/spec/manifests precede the sole Runpod
dispatch; collection/hash checks precede owned pod deletion. Native export
claims no unavailable canonical-schema conformance or signed independent
execution verdict. [pcc:unverified]
