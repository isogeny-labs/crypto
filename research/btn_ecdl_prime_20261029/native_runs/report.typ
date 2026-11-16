#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let R = json("report.json")
#let E = R.at("eccp131")
#let net = if E.at("network") == none { none } else { E.at("network").at("networks") }

= Native hashed-ecdl-tensor measurements on small prime-order curves

Study: `research/btn_ecdl_prime_20261029/` (PR \#2888). Tool:
`native/` (`btn-prime-scaling`, Rust), depending on the unmodified
`crates/hashed-ecdl-tensor` crate (\#2878). All numbers below are read from
`native_runs/report.json`, produced by `btn-prime-scaling report native_runs`
from the raw records `sizes.jsonl`, `counts.jsonl`, `solves.jsonl` and
`eccp131_size.json`.

*Unit.* Step counts are the crate's `Meter` charges, deterministic diagnostics
and not calibrated time units. *No wall-clock time is reported*, so no timing
isolation applies. Host: a 4-core Claude Code cloud container (Linux).

== Exact counts against brute force

#R.at("counts").at("n") contractions of the decomposition network (two slots,
full and hash-filtered factor bases) at p = 5, 7, 11, 13. All match brute
force: *#R.at("counts").at("all_match_brute_force")*. Backend `Functional` in
every count: *#R.at("counts").at("all_functional_backend")*. Visited
assignments equal $2^(4d)$ in every count:
*#R.at("counts").at("functional_assignments_equal_2_pow_4d")*. Recursive
(OpenAI Appendix D) backend used anywhere: *#R.at("recursive_backend_used")*.

#table(
  columns: 7,
  [p], [d], [brute], [backend], [assignments], [steps], [steps / assignment],
  ..R.at("counts").at("rows").map(c => (
    str(c.at("p")), str(c.at("d")), if c.at("ok") { "match" } else { "MISMATCH" },
    c.at("backend"), str(c.at("functional_assignments")), str(c.at("steps")),
    str(calc.round(c.at("steps_per_assignment"), digits: 0)),
  )).flatten()
)

== End-to-end solves (`reduction::solve`, seeded)

#table(
  columns: 8,
  [p], [d], [order r], [seed], [secret], [recovered], [relations], [steps],
  ..R.at("solves").at("rows").map(s => (
    str(s.at("p")), str(s.at("d")), str(s.at("r")), str(s.at("seed")), str(s.at("secret")),
    if s.at("ok") { s.at("recovered") + " (verified)" } else { "FAILED" },
    str(s.at("relations", default: "-")), str(s.at("steps")),
  )).flatten()
)

== Network size

#table(
  columns: 5,
  [d], [p], [edges], [vertices], [max degree],
  ..R.at("sizes").at("rows").map(s => (
    str(s.at("d")), str(s.at("p")), str(s.at("edges")), str(s.at("vertices")), str(s.at("max_degree")),
  )).flatten()
)

#if net != none [
  ECCp-131 (d = 131, published challenge input, certified primes): #net.first().at("slots") slots, #net.first().at("edges") edges, #net.first().at("vertices") vertices, max degree #net.first().at("max_degree"), #net.first().at("interface_bits") interface bits (4 · 131 free coordinate bits plus 2 constant identity flags).
] else [
  ECCp-131 network size: *not recorded*.
]

#figure(image("native_scaling.svg", width: 100%), caption: [Recorded sizes, per-count assignments and steps, and the derived ECCp-131 comparison. Source: `native_runs/report.json`.])

== Setup: primality certification above 64 bits (resolved with certificates)

`is_prime` (used by `Curve::new` and `FactorBase::new`) dispatches to a fast
u64 path at <= 64 bits and to *AKS* above.

#let S = R.at("setup_bottleneck")
#let pu = S.at("is_prime").at("u64_path_max_micros")
#let pa = S.at("is_prime").at("aks_at_65_bits")
Without certificates, every certification at <= 64 bits took at most #pu µs,
while AKS at 65 bits did *not finish in #pa.at("did_not_finish_seconds") s*.
That is why the first uncertified ECCp-131 build never finished.

The crate on `main` now accepts independently checked Pocklington certificates
(`Curve::with_cofactor_and_certificates`, landed by a separate change). With the
bundled `eccp131-certified-hash.json` (5 certificates, certified 132-bit hash
prime) the ECCp-131 curve validates in about 13 ms and its full decomposition
network builds (below). Network construction itself is fast at every width: the
Fourier integer-multiply builds in
#S.at("multiply_construction").at("rows").map(m => [#m.at("width")-bit: #m.at("micros") µs]).join(", ").
So setup is no longer the wall; the recursion crossover is.

== Speeding up the exact fallback that actually runs

#let U = R.at("upstream_pruning")
#let D = R.at("derived_last_point")
At every reachable size the count runs on the exact `Functional` fallback, so
that is where speed-ups land. Two were measured on the same 16 instances, in
the crate's charged steps (deterministic `Meter` diagnostics, not time).

*Upstream functional pruning* (crate change landed separately, measured here):
#U.at("pairs_matched") of #U.at("of") instances paired, counts unchanged, charged
steps #calc.round(U.at("step_ratio_min"), digits: 1)–#calc.round(U.at("step_ratio_max"), digits: 1)× lower.

*Derived last point* (this study): for two slots the decomposition network
searches both A#sub[1] and A#sub[2] as free inputs, but A#sub[2] = target − A#sub[1] is
determined. Computing it with the encoder instead of searching it is a
bijection on the same identity-free language, so counts cannot change, while
the free input bits halve
(#D.at("free_bits").map(f => [d = #f.at("d"): #f.at("original") → #f.at("derived")]).join(", ")).
Two refinements on top: mark only A#sub[1] as the interface, so the crate's
witness search may stop at its first hit (A#sub[1] is the word's prefix and
fixes A#sub[2], so the canonical witness is unchanged); and check A#sub[1]'s
factor-base hash before building A#sub[2] ("hash-first"), so an A#sub[1] outside
its base is rejected before the point addition runs. All variants are built
only from the crate's public `Encoder` API; the crate is not modified.

Measured on #D.at("n") instances (p = 5, 7, 11, 13; two targets; hash
thresholds τ = s, s/2, s/4, s/8, where s is the hash prime):
- Every variant's count equals the original network and brute force: *#D.at("all_variants_exact")*.
- Witnesses: on the fully marked derived network, #D.at("witnesses_found") found and each checked by reference arithmetic and `FactorBase::contains`; on the A#sub[1]-only interface, all checked (*#D.at("a1_only_witnesses_ok")*) and the same canonical witness in every solvable case (*#D.at("a1_only_same_canonical_witness")*).
- Combined (derived, A#sub[1] interface, hash-first) versus the current pruned original: charged steps #calc.round(D.at("best_ratio_min"), digits: 1)–#calc.round(D.at("best_ratio_max"), digits: 1)× lower, worse on #D.at("best_worse_than_original") instances. Hash-first alone contributes #calc.round(D.at("hashfirst_ratio_min"), digits: 2)–#calc.round(D.at("hashfirst_ratio_max"), digits: 2)×, largest at small τ.
- Limit: without hash-first, the derived network was worse than the original on #D.at("derived_a1_worse_than_original") instances (p = 7, τ = 2), where the original's own pruning leaves few assignments and a fixed per-call overhead (about 84k charged steps at d = 3, 140k at d = 4, roughly 5 per edge) dominates. The gain is largest when many assignments survive, i.e. at large τ and larger p.

#table(
  columns: 7,
  [p], [τ], [count], [steps original], [steps derived], [steps combined], [combined ×],
  ..D.at("rows").map(r => (
    str(r.at("p")), r.at("threshold"), r.at("original").at("count"),
    str(r.at("original").at("steps")), str(r.at("derived").at("steps")),
    str(r.at("derived_a1_hashfirst").at("steps")),
    str(calc.round(r.at("original").at("steps") / r.at("derived_a1_hashfirst").at("steps"), digits: 1)) + "×",
  )).flatten()
)

*Where the remaining cost is, and when to prune.* Pinning every input so one
assignment remains isolates the fixed per-call cost on the combined network:
#D.at("overhead").map(o => [d = #o.at("d"): #o.at("pinned_count").at(0).at("steps") charged steps with the crate's pruning plan, #o.at("pinned_count").at(1).at("steps") without, on #o.at("edges") edges]).join("; ").
Building the pruning plan is about 85% of the overhead (about 4 charged steps
per edge). With the derived network's search space halved, the plan can cost
more than it saves: switching pruning off (the public
`Counter::functional_pruning` field) keeps every count exact on all networks
(*#D.at("all_exact_with_and_without_pruning")*) and makes the combined
network #calc.round(D.at("noprune_vs_prune_hashfirst_min"), digits: 2)–#calc.round(D.at("noprune_vs_prune_hashfirst_max"), digits: 2)× cheaper at d = 3–4,
for #calc.round(D.at("best_overall_min"), digits: 1)–#calc.round(D.at("best_overall_max"), digits: 1)× below the pruned original. The original network
needs its pruning (off costs it #calc.round(D.at("original_noprune_penalty_min"), digits: 2)–#calc.round(D.at("original_noprune_penalty_max"), digits: 1)×). The advantage shrinks
with d, because the search space grows 4× per bit while the plan grows with
network size:

#table(
  columns: 6,
  [p], [d], [τ], [free bits], [pruned / unpruned steps], [faster],
  ..D.at("prune_crossover").map(r => (
    str(r.at("p")), str(r.at("d")), r.at("threshold"), str(r.at("free_input_bits")),
    str(calc.round(r.at("prune_over_noprune"), digits: 2)),
    if r.at("prune_over_noprune") > 1 { "pruning off" } else { "pruning on" },
  )).flatten()
)

So the plan should be built only when the search space is large relative to
the network: off is better through d = 5 here, and at d = 6 pruning already
wins at a small threshold. A size-based rule, not a blanket switch.

*A held-out rule for the pruning plan.* From the measurements above, build the
plan only when $2^"free bits" dot 8 >= "edges"$. Tested on the crate's own
`decomposition_network` (the derived last slot since \#3522) and on the general
construction, including sizes not used to choose it (d = #R.at("derived_last_point").at("pruning_rule").at("held_out_d").map(str).join(", ")):
#let PR = R.at("derived_last_point").at("pruning_rule")
the rule picks the cheaper option in #PR.at("correct") of #PR.at("n") cases, is never more than
#calc.round((PR.at("worst_rule_vs_best") - 1) * 100, digits: 1)% above the best, and counts agree with and without the plan in every case
(*#PR.at("all_counts_agree")*). Where it switches the plan off on the crate's network it saves
#PR.at("crate_gain_when_off").map(x => str(calc.round(x, digits: 2)) + "×").join(", "). At cryptographic sizes the rule always builds the
plan, so it only helps small instances.

*It does not generalise.* Added to the crate as a default (`Counter`), the rule
broke the crate's own `tests/structured.rs` test: on the isogeny-fibre
`Prepared` network at arity 3 it switched the plan off and the count exceeded
its 500,000,000-step budget, because pruning skips almost everything there and
each unpruned assignment is far more expensive. A rule that sees only the
network's size cannot know the per-assignment evaluation cost, so it holds for
decomposition networks only. The crate change was dropped, not pushed.

*Dropping a duplicated validity check ("lean").* `input_point` validates
A#sub[1] (canonical coordinates, on the curve), and `point_add` then validates
−A#sub[1], which holds exactly when A#sub[1] is valid (same x; y made canonical by
`modular_subtract`; (−y)#super[2] = y#super[2]). Building A#sub[1] from raw input words removes
the first check: #D.at("lean").at("edges").map(r => [d = #r.at("d"): #r.at("hashfirst") → #r.at("lean") edges]).join(", ").
On #D.at("lean").at("n") instances every count stays exact with and without pruning and
every witness checks out (*#D.at("lean").at("exact")*). The effect depends on the threshold:
at τ ≤ s/4 lean is #calc.round(D.at("lean").at("gain_small_threshold_min"), digits: 2)–#calc.round(D.at("lean").at("gain_small_threshold_max"), digits: 2)× faster (the hash rejects most
A#sub[1] before any curve check runs), at τ ≥ s/2 it is slower
(#calc.round(D.at("lean").at("gain_large_threshold_min"), digits: 2)–#calc.round(D.at("lean").at("gain_large_threshold_max"), digits: 2)×), because the curve check, which rejects most (x, y) pairs, now
runs only inside `point_add`. Real factor bases use tiny thresholds (at ECCp-131
about 2#super[65] of 2#super[131] points, τ/s ≈ 2#super[−66]), which is the lean regime; like the
pruning plan, it is a threshold-based choice.

At ECCp-131 the exact fallback's worst case drops from
$2^#D.at("eccp131_log2_assignments").at("original")$ to $2^#D.at("eccp131_log2_assignments").at("derived")$
assignments per count. That is a large constant-exponent improvement and still
far beyond Pollard rho's $2^65.3$ group operations for the whole problem; it
does not change the conclusion below.

== The recursion crossover (headline)

#let X = R.at("recursion_crossover")
The only part of the algorithm that could beat exponential cost is the
recursive backend (OpenAI Appendix D.4). In the crate's `Counter::count`, it is
selected only when $"root_u" = lambda \/ #X.at("epsilon_denominator")$ is at least 1,
with $lambda = ceil(log_2 (2 dot "edges"))$. That needs
$lambda >= #X.at("min_lambda_for_recursion")$, i.e. at least
$2^#calc.round(X.at("lower_bound_log2_edges"))$ edges (further gates only raise it).

The largest network built here is
$2^#calc.round(X.at("log2_max_edges_built_here"), digits: 1)$ edges. The gap to the
crossover is about $2^#calc.round(X.at("lower_bound_log2_edges") - X.at("log2_max_edges_built_here"))$. Across all
#R.at("counts").at("n") counts and #R.at("solves").at("n") solves the recursive backend was selected
*#R.at("recursive_backend_used")* times: every exact count used the fallback
(`Functional`/`Direct`), whose cost is $2^"free input bits"$ per contraction.

This threshold is faithful to the manuscript, which states a saving only above
networks of size #X.at("paper_threshold_log2_edges_approx"). No prime-field curve
produces a network near it, so this implementation gives no sub-exponential
behaviour at any reachable size, including ECCp-131.

== Derived: ECCp-131

The backend actually used at every measured size is `Functional`, which visits
$2^"free input bits"$ assignments. With two slots at d = 131 that is
$2^#E.at("functional_backend_log2_assignments_per_contraction")$ assignments per
contraction (#E.at("derivation")). With A#sub[2] derived (above) it is
$2^#D.at("eccp131_log2_assignments").at("derived")$. Pollard rho on ECCp-131 needs about
$2^#calc.round(E.at("pollard_rho_log2_group_ops"), digits: 2)$ group operations
for the whole discrete logarithm. The recursive backend did not run at any
measured size; this report says nothing about its cost.

== What this does not show

No ECCp-131 scalar is computed or attempted, and nothing here approaches
Pollard rho. The fallback speed-ups above are measured in the crate's
charged steps on small curves, against the same code's earlier behaviour; they
are not wall-clock benchmarks and not a comparison with rho. The construction
and primality timings are unisolated diagnostics. The recursive backend's
asymptotic bound (Bochum v0.2, conditional on OpenAI's unreviewed Theorem D.1)
is not tested. No row is added to `benchmarks/README.md`: the validation gate
admits no row without an independent replay certificate, validator report and
matched rho reference.
