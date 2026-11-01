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
  ECCp-131 (d = 131, published challenge input): #net.map(n => [#n.at("slots") slots: #n.at("edges") edges, max degree #n.at("max_degree")]).join("; ").
] else [
  ECCp-131 network size: *not recorded* — the `size-json` build did not finish,
  stuck in AKS primality certification of the 131-bit prime (see "Setup
  bottleneck"), not in network construction.
]

#figure(image("native_scaling.svg", width: 100%), caption: [Recorded sizes, per-count assignments and steps, and the derived ECCp-131 comparison. Source: `native_runs/report.json`.])

== Setup bottleneck: primality certification above 64 bits

Before the contraction cost matters, there is a harder practical wall:
`is_prime` (used by `Curve::new` and `FactorBase::new` to certify the field and
subgroup primes) dispatches to a fast u64 path at <= 64 bits and to *AKS* above.

#let S = R.at("setup_bottleneck")
#let pu = S.at("is_prime").at("u64_path_max_micros")
#let pa = S.at("is_prime").at("aks_at_65_bits")
At <= 64 bits every certification took at most #pu µs. At 65 bits (AKS) it did
*not finish in #pa.at("did_not_finish_seconds") s*, a jump of more than seven
orders of magnitude at a single bit. ECCp-131's prime is 131-bit, so the curve
cannot even be constructed in reasonable time; the 131-bit `size-json` build was
stuck here, not in the network.

By contrast, *network construction is fast at every width*: the Fourier
integer-multiply builds in
#S.at("multiply_construction").at("rows").map(m => [#m.at("width")-bit: #m.at("micros") µs]).join(", ").
So the blockers, in order, are (1) prime certification above 64 bits (AKS),
then (2) the recursion crossover below. Neither is close to feasible.

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
contraction (#E.at("derivation")). Pollard rho on ECCp-131 needs about
$2^#calc.round(E.at("pollard_rho_log2_group_ops"), digits: 2)$ group operations
for the whole discrete logarithm. The recursive backend did not run at any
measured size; this report says nothing about its cost.

== What this does not show

No ECCp-131 scalar is computed or attempted. No speed-up is claimed. The
recursive backend's asymptotic bound (Bochum v0.2, conditional on OpenAI's
unreviewed Theorem D.1) is not tested. No row is added to
`benchmarks/README.md`: no wall-clock number is reported, and the validation
gate admits no row without an independent replay certificate, validator report
and matched rho reference.
