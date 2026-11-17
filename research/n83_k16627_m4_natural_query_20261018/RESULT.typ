#set page(paper: "a4", margin: 14mm)
#set text(font: "New Computer Modern", size: 10pt)
= N83 K = 16,627 prebuilt base: natural m = 4 gate

The frozen spot-host sequence completed. P2 validated the exact retained
base and public target. A1 and A2 each returned the same independently
replayed rank-one constructed-selector relation. B was an unpinned public
query and wrote `UNKNOWN_budget` at 1,790.010 s. It wrote no attempt or
relation receipt and its 4 GiB cgroup recorded no OOM event.

#image("reports/run-20261018T2052Z/sequence.svg", width: 100%)

The run was on a shared host, with one CPU and zero swap. P2/A1/B/A2 worker
walls were 43.737231/60.034319/1,790.010187/68.854539 s. A2 was delayed
by an interrupted local session and began about four hours after B ended.
The constructed controls supplied representative selectors, so their times
do not yield a natural-query speed ratio. The single capped B arm gives no
natural-yield estimate. Its internal search-progress counters were not
durably sampled. `selected_best_total_runtime` and complete IC runtime stay
null.

The next base-design step is an exact native census of direct-subgroup
points with x in the dimension-18 polynomial subspace. A passing count would
support an exported and replayed structured K = 16,627 closure for a frozen
common-query comparison. Natural relation/rank, m = 2/3 policies,
prebuilt-versus-on-demand full costs and matched rho remain open. See
`RESULT.md`, `REMOTE_PROTOCOL_V2.md`, the run report and
`spot-raw-files.sha256` for source, inputs and every receipt.
