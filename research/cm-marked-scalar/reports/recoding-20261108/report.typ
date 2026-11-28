#set page(paper: "a4", margin: 15mm)
#set text(font: "Libertinus Serif", size: 10pt)
#set par(leading: 0.5em)
#align(center)[
 #text(size: 18pt, weight: "bold")[Bounded modular recoding]
 #linebreak()
 P-192 primary; five-curve exact cost continuation
 #linebreak()
 2026-11-08 · T-1888 · PR 3674
]
= Completed construction and controls
For each fixed marked label mu, let mu0 and k0 be the centered representatives of mu and mu/ell modulo n. The native evaluator uses mu0+j*n or k0+j*n. This preserves the order-n subgroup action. Both direct and fused methods receive every shift -256..256 and width 2..7. The fused route includes the complete small-scalar multiplication by ell and both point tables. This is a subgroup arithmetic recipe; no new global quotient polynomial is asserted.

The frozen census completed all 30,780 schedules. Every signed-digit reconstruction is checked. A separate native counter/selector recomputes every raw row and all 20 minimum sets. Software qualification passes 1,500 boundary point/count cases and 200 selected-plan cases, using independent OpenSSL point references. Five corrupted-data controls are rejected at their intended checks. Fresh archive replay is byte-identical; deterministic repacking reproduces its digest.

= Preregistered primary comparison
#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 5pt,
 [*Curve*],[*Centered direct*],[*Translated direct*],[*Centered fused*],[*Translated fused*],
 [P-192],[229],[228],[238],[235],
 [P-224],[260],[260],[276],[273],
 [P-256],[300],[300],[311],[309],
 [P-384],[450],[449],[459],[458],
 [P-521],[606],[606],[613],[613],
)
Every value is minimum unweighted D+A over the frozen finite design. Weights D+2A, D+3A and D+4A are retained as sensitivities. The translated fused route remains above the equally optimized direct route on each curve in the primary comparison, and in each declared sensitivity. No population inference or timing result is made.

#image("counts.svg", width: 100%)

#pagebreak()
= P-192 component improvement
The translated fused plan uses shift -7 and width 5: 196 doubles, 39 additions, 17 negations and 9 cumulative table points. Its centered width-5 counterpart uses 196, 42, 20, 9. Thus three additions and three negations are removed with the same doubling and table counts. The minimum-D+A centered configuration at width 4 has different table size; all configurations remain in the census.

The translated direct plan uses shift 6, width 5: 192 doubles, 36 additions, 12 negations and 8 table points. The centered minimum uses width 4 with 190,39,19,4. Its one-call D+A reduction trades more doubles and table points for fewer additions/negations. These vectors cannot be substituted for full native time.

= Integrity, failures and scope
Protocol CM-MODULAR-RECODING-V1 was frozen and published before the scan, body SHA256:
#text(size: 8pt)[328c33e50fe9cc4d6fd55acc3f103b317f93618b3161e022c45d443ff6d0cb3a]
Source/checker commit 41e44c26a057 preceded freeze publication bb798850daab. Both binary hashes, both source files, seven input hashes, fixed bounds and the tie key are bound. No scalar, bound, score or point-specific choice changed after data.

The first software qualification exposed a noncumulative table-point counter. The fix accumulates across both fused scalar calls and passes the positive controls. Compiler warnings, the failed qualifier, schema-metadata corrections and a thin summary formatting error are preserved. No scientific scan was repeated. The five post-scan rejection controls cover an altered selected plan, wrong raw count with a correspondingly updated checksum, missing row, duplicate row and changed protocol.

The schema-valid experiment.export/v1 contains 80 exact derived minimum cases: five curves x four method records x four declared weights, from one census. They are not 80 independent process runs or timing trials. Centered minima reuse zero-shift rows. The 91-KiB archive retains every recipe and control input, with exact byte counts and hashes. Local invocations were bounded at 120 seconds, one thread/process and 2 GiB. No cloud submission occurred.

= Measurement admission: actual readback
The documented existing kubeconfig authenticates as isolab:submitters. It resolves the old default-context role-assumption failure without an IAM or Kubernetes permission change. Actual NodePool gets succeed: ordinary 16CPU/128Gi; L2 8CPU/40Gi. The supplied policy retains L2 32Gi. Both quota-list and named isolab/jobs reads are forbidden. Nine retained Jobs show no active job and one suspended; none was modified.

AWS currently reports monthly ActualSpend USD 1044.578, above the supplied USD 1000 monthly ceiling. The live object's USD 1100 limit does not supersede the owner instruction. Its configuration timestamp is not a guaranteed billing-freshness timestamp. No new cloud spending, role grant, budget increase or pool patch was made. Previous local quietness checks remain unadmitted; no cryptographic time was collected in this round.

The original end-to-end runtime objective remains open. A future run must include scalar preparation, curve/table setup, maps or fusion, conversion, output verification and destruction, with plan-generation work retained in first-use accounting. The next arithmetic investigation is a different closed relation or split that repays the correcting-scalar work; a larger recoding count alone is not treated as faster.

#text(size: 8pt)[Replay: recoding-v1/README.md and its raw archive; independent source recoding_replay.cpp. Standard digital-expansion background: Heuberger–Krenn, arxiv.org/abs/1110.0966 (author abstract checked; no novelty/optimality claim). Live gate receipts: qualification/environment-v2/. AWS API semantics: docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_CalculatedSpend.html and API_budgets_Budget.html.]
