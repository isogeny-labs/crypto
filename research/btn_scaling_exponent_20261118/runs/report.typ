#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let R = json("report.json")
#let F = R.at("scaling").at("fits_8_bits_and_up")
#let f(x, d) = str(calc.round(x, digits: d))

= Scaling of the hashed-ECDL tensor solver and the Bochum v0.2 cost at 120--500 bits

Study: `research/btn_scaling_exponent_20261118/`. Tool: `native/` (`btn-scaling`,
Rust) on the crate at crypto `main` 833a35d21e, which includes the network
reductions of PR \#4356. Every number below is read from `runs/report.json`,
produced by `btn-scaling report runs`.

*Unit.* Step counts are the crate's `Meter` charges, deterministic diagnostics
rather than time. Rho figures are analytic ($sqrt(pi r slash 4)$ group operations),
not measured, and they are used only for slopes and orders of magnitude.

== 1. Measured exponent of one exact count

Two-slot counts on the first prime-order curve over the largest prime below
$2^k$, for $k = 3, ..., 17$, at thresholds $s$ and $s slash 4$, one target. All
#R.at("scaling").at("rows") counts equal an independent $O(p)$ count:
*#R.at("scaling").at("all_match_independent_count")*. Recursive backend calls:
*#R.at("scaling").at("recursive_calls_total")*.

Least squares over $b >= 8$ bits:
- $log_2$ steps grows #f(F.at("threshold_s").at("log2_steps_vs_bits").at("slope"), 3) per bit ($R^2$ #f(F.at("threshold_s").at("log2_steps_vs_bits").at("r2"), 4)) at threshold $s$, and #f(F.at("threshold_s_over_4").at("log2_steps_vs_bits").at("slope"), 3) at $s slash 4$.
- $log_2$ visited assignments grows #f(F.at("threshold_s").at("log2_assignments_vs_bits").at("slope"), 3) per bit: the solver enumerates $2^(b+1)$ inputs, the compressed free point.
- Pollard rho grows 0.5 per bit.

So one decomposition query costs about $2^(1.17 b)$ steps. That is exponential,
and its exponent is more than twice rho's _for the whole discrete logarithm_.

#figure(image("scaling.svg", width: 100%), caption: [Left: measured steps and assignments per exact count. Middle: the recursive backend's guard at 120--500 bits. Right: Remark 2's cost composition with the implemented solver, against rho.])

== 2. Network size at 120--500 bits

No prime-order curve at these sizes can be certified here (the crate certifies
primes above 64 bits only through supplied certificates), so sizes rest on
two exact anchors: one free-by-free modular product built at each width
(`gadget.jsonl`; any odd modulus of that width), and the exact ECCp-131 network
(#R.at("eccp131").at("two_slot_edges") edges with two slots,
#R.at("eccp131").at("three_slot_edges") with three; from
`research/hashed_ecdl_dedupe_20261115`). Its ratio to one 131-bit product
(#f(R.at("eccp131").at("ratio_two_slot"), 1) and
#f(R.at("eccp131").at("ratio_three_slot"), 1)) scales the product to an
estimate; one product is a strict lower bound.

== 3. The recursive backend never engages

Appendix D.4, as implemented in `contraction/recursive.rs`, uses the recursion
only when $lambda = ceil(log_2 2 s) >= 131072$, i.e. $s >= 2^131071$ edges.

#table(
  columns: 9,
  [bits], [m], [product edges], [edges (est.)], [$lambda$], [engages], [$log_2$ calls], [$log_2$ implemented (lower bd)], [$log_2$ rho],
  ..R.at("cost_table").map(t => (
    str(t.at("bits")), str(t.at("slots")), str(t.at("modular_product_edges")),
    str(calc.round(t.at("edges_estimate"))), str(t.at("lambda_estimate")),
    if t.at("recursion_engages") { "yes" } else { "no" },
    f(t.at("log2_solver_calls"), 1), f(t.at("log2_fallback_total_lower"), 1), f(t.at("log2_rho"), 1),
  )).flatten()
)

Calls follow Remark 2, $O(m^2 b r^(1/m))$ with constant 1. The implemented
solver visits $2^((b+1)(m-1))$ assignments per call, at least one step each,
which gives the lower bound shown. It exceeds rho at every size, by $2^130$ at 120
bits and growing.

== 4. If Theorem 2 held: the break-even constant

Theorem 2 claims time $2^(O(s slash sqrt(log s)))$ per contraction, with no
constant stated. Write it $2^(c s slash sqrt(log_2 s))$ and ask for the $c$ at
which the whole reduction costs as much as rho, using the estimated $s$
(the bipartite subdivision of Remark 1 doubles $s$, which only lowers $c$):

#table(
  columns: 4,
  [bits], [m], [budget per call, bits], [break-even $c$],
  ..R.at("cost_table").map(t => (
    str(t.at("bits")), str(t.at("slots")), f(t.at("theorem2_budget_bits_per_call"), 1),
    str(calc.round(t.at("theorem2_break_even_constant"), digits: 7)),
  )).flatten()
)

With two slots the budget is negative: $O(m^2 b r^(1 slash 2))$ calls already
cost more than rho, whatever the solver. With three slots the hidden constant
would have to be about $10^(-4)$ or less at every size here. The proof in
[Ope26] gives no saving at all below $2^(2^17)$ edges (the manuscript's own
Limitations paragraph), which is what the $lambda$ guard implements.

== What this shows and does not show

It shows that the implemented method is exponential, about $2^(1.17 b)$ per
query on toy curves, and that at 120--500 bits it is many orders of magnitude
above rho. It also shows that the conditional theorem could only help there
with a hidden constant near $10^(-4)$, for which the paper gives no evidence. It does
not test Theorem D.1 itself, and it computes no discrete logarithm at these
sizes. No wall-clock time is reported, so no `benchmarks/` or leaderboard row
is added.
