#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 9pt)
#let data = json("../../full-m4-v1/final/audit.json")
= N83: full-size prebuilt U/D public-query panel
_Frozen bounded m=4 comparison · 2026-10-20 UTC_

The card started at #data.card_start_utc and ended at #data.card_end_utc.
The native receipt audit accepts #data.natural_attempts.len() natural attempts,
four constructed controls and four separate general-field control replays.
Its complete inventory contains #data.manifest_files files.

#align(center, image("natural.svg", height: 220mm, width: 100%, fit: "contain"))

#pagebreak()
== Exact inputs and policy

Curve `icv1-f2m83-tm6151469093347-debefd74`, a=0, subgroup
2417851639230796216685689, cofactor 4, polynomial z^83+z^45+z^2+z+1.
U is the retained public-x-hash orbit base; D is the exported direct-subgroup
dimension-18 base. Each contains 16,627 signed-Frobenius columns and
2,760,082 points. Their accepted point sets differ.

Both bases use the same eight public points, worker binary, solver policy,
resource limits and deterministic search behavior. Validation scalars are
excluded. Target 0 uses U,D,D,U; odd later indices use D,U and even indices
use U,D. There is one model per attempt, four free summands, free selectors
and phases, 10,000 conflicts, 300,000 variables and 8,000,000 domain clauses.
Each actual model has 201,598 variables, 5,915,238 clauses and 3,901 XOR rows.
The satisfied-row cache is enabled for every natural attempt.

The four constructed controls, cache off/on within each base, passed exact
SAT-model, row and search-work checks before public search. Their separate
general-field replays reject changed targets, points and coefficients.
Those constructed rows are excluded from the natural rank calculation.

== Observed outcomes and charges

#for arm in ("U", "D") {
  let a = data.arm_summaries.at(arm)
  let r = data.replayed_factor_row_rank.at(arm)
  [*#arm*: #a.attempts attempts; #calc.round(a.charged_worker_intervals_s, digits: 3) seconds summed over all worker/cap intervals. Outcomes: #repr(a.outcomes). Accepted independently replayed natural rows: #r.accepted_replayed_rows; retained factor-row rank: #r.retained_factor_row_rank.

  ]
}

The rank is the rank of the retained accepted factor rows modulo the exact
subgroup order, separately for U and D. It excludes dependent rows. It does
not infer natural yield or a complete IC rank from censored searches.
The natural-yield estimate, useful-rank panel, candidate and best total
runtime remain null. Different cap endpoints do not support a U/D speed ratio.

Worker intervals include import/index, corpus validation, model construction,
root propagation and search up to the receipt timestamp or internal cap.
They end before final receipt serialization. Container start/end receipts
preserve the surrounding intervals. Construction/export, full-base
verification, control/replay processes, Docker and custody costs remain
separately recorded; they are not silently included in the worker sum.
Latest durable CPU/RSS observations on wall caps are lower bounds. A final
cgroup peak is absent for those cap receipts. Missing quantities remain missing.

== Host, custody and audit

Intel Xeon 6975P-C SSH Spot runner, pinned CPU 31, one CPU quota, hard 4 GiB,
zero swap, no worker network and read-only inputs/root. Other workloads
share the host, so isolation is L0. No new instance or remote compilation
was introduced during this card.

Worker source: #text(size: 8pt, data.source_commit).
Archive SHA-256: #text(size: 8pt, data.archive_sha256).
Manifest SHA-256: #text(size: 8pt, data.manifest_sha256).

`examples/n83_full_m4_audit.rs` checks the full inventory; pinned source,
binary, corpus and object identities; exact arguments and input mounts;
container image/resources, exits and serial order; phase/progress records;
counter and resource monotonicity; final/cap consistency; independent replay
bindings; and retained factor-row rank. A wall cap cannot be promoted to a
completed conflict endpoint or a mathematical UNSAT result.

== Original gates and next obligation

The D18 full object and generic replay are complete. Durable m=4 progress
and this frozen equal-size public panel are retained. The same-set
prebuilt/on-demand m=2 and m=3 finite-policy measurements remain a separate
experiment. Complete IC useful-rank collection, linear algebra, individual
logarithm verification, qualified cold/reuse cost, matched same-point rho,
external-fastest comparison and best-base selection remain open.

The next discriminating stage must obtain independently checked natural
rank increments under a separately frozen search or partial-relation policy.
The existing caps remain in the record. This panel supplies bounded search
evidence, with no qualified pipeline or fastest-base promotion.

Sources: `FULL_M4_PROTOCOL.md`, `FULL_M4_HOST_CARD.md`,
`full-m4-v1/final/`, this report's `RUN_REPORT.md`, and
`problems/n83-prebuilt-base-quality-m2-m4.md`. Earlier interrupted and
smaller-base cards remain separate. The SVG and 150 dpi PNG share this
plotting source; matching report and visual copies are in `benchmarks/`.
