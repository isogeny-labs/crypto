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
Built only from the crate's public `Encoder` API; the crate is not modified.

- Counts equal to the original network and to brute force on all #D.at("n") instances: *#D.at("counts_equal_and_match_brute_force")*.
- Lemma-1 witness extraction on the derived network: #D.at("witnesses_found") decompositions found, every one checked by reference arithmetic and the crate's membership test (the remaining instance has none, as brute force agrees): *#D.at("witnesses_ok")*.
- Charged steps #calc.round(D.at("step_ratio_min"), digits: 1)–#calc.round(D.at("step_ratio_max"), digits: 1)× lower than the original network, on top of the pruning; the gain grows with p.

#table(
  columns: 8,
  [p], [threshold], [count], [free bits], [assignments (orig → derived)], [steps original], [steps derived], [ratio],
  ..D.at("rows").map(r => (
    str(r.at("p")), r.at("threshold"), r.at("derived").at("count"),
    str(r.at("original").at("free_input_bits")) + " → " + str(r.at("derived").at("free_input_bits")),
    str(r.at("original").at("functional_assignments")) + " → " + str(r.at("derived").at("functional_assignments")),
    str(r.at("original").at("steps")), str(r.at("derived").at("steps")),
    str(calc.round(r.at("step_ratio"), digits: 1)) + "×",
  )).flatten()
)

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
Pollard rho. The two fallback speed-ups above are measured in the crate's
charged steps on small curves, against the same code's earlier behaviour; they
are not wall-clock benchmarks and not a comparison with rho. The construction
and primality timings are unisolated diagnostics. The recursive backend's
asymptotic bound (Bochum v0.2, conditional on OpenAI's unreviewed Theorem D.1)
is not tested. No row is added to `benchmarks/README.md`: the validation gate
admits no row without an independent replay certificate, validator report and
matched rho reference.
