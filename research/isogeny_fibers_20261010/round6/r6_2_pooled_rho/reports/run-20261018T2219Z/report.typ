#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let b = json("../../batch.json")
#let p = json("../../paths.json")
#let f(x, d: 3) = str(calc.round(x, digits: d))

= R6.2 run report: pooled multi-target rho on an isogeny class, and path-finding cost

*Run id* `run-20261018T2219Z` (UTC start of the batch run). *Commit* base `abee07ee73` (origin/main) plus the uncommitted R6.2 changes on branch `research/r6-pooled-rho`, landed in the same PR as this report. *Host* `vm`, 4 vCPU, shared with other build jobs: *no isolation earned*; every recorded result is a count, wall times are contended notes only. Toy instances; nothing here concerns a deployed curve and no sub-rho claim is made.

*Commands* (from the worktree root):
- `target/release/examples/r6_2_pooled_class_rho --mode batch --out research/isogeny_fibers_20261010/round6/r6_2_pooled_rho/batch.json` (defaults: `--rbits 28 --trials 20 --seed 1 --ls 1,4,16,64,256`)
- `… --mode paths --out …/paths.json` (defaults: `--pbits 24,28,32 --classes 6 --pairs 10 --cap 3000 --seed 7`)

*Raw records*: `round6/r6_2_pooled_rho/batch.json` (schema #raw(b.schema)), `paths.json` (schema #raw(p.schema)), `batch.log`, `paths.log`. Both runs completed; no failures.

== Batch mode (measured)

Base curve $E_0$: $p = #b.base_curve.p$, $\#E_0 = h r$ with $h = #b.base_curve.h$, $r = #b.base_curve.r$ ($2^(#f(b.base_curve.rbits_actual, d: 2))$). Class: #b.class.nodes curves by BFS over rational kernels, #b.class.duals_certified edges each with a certified dual ($hat(phi) compose phi = [ell]$ checked on a point of order $> 4 ell^2$ and on the generator). Edges by $ell in {2,3,5,7}$: #b.class.edges_by_ell.at("2"), #b.class.edges_by_ell.at("3"), #b.class.edges_by_ell.at("5"), #b.class.edges_by_ell.at("7"). Edges dropped for lacking a rational-kernel dual: #b.class.edges_without_dual_by_ell.map(str).join(", ") ($p equiv 2 med (mod 5)$, $p equiv 5 med (mod 7)$, so every $ell = 5, 7$ edge is dropped; see the note on duals below). Logs verified on their own curves: #b.verified_logs.

Arms per trial (same algorithm seed for B and C): *S* one key on $E_0$ with `rho_walk_with` (ecbench `rho.negation`); *B* $L$ keys on $E_1 … E_L$, mapped to $E_0$ along the duals, one pooled batch (`rho_batch_with`, ecbench `rho.multitarget_pooled`), each log mapped back and verified; *C* $L$ keys on $E_0$, same pooled batch. Unit $S$ = total counted group-operation equivalents divided by $sqrt(r)$, including setup. Transport is reported separately in field $M$ and $I$ per target.

#table(columns: 8, align: right,
  [$L$], [trials], [mean $S_B$ ± SE], [mean $S_C$ ± SE], [mean $S_"single"$ ± SE], [KS ref.], [$L dot S_"single"$ (derived)], [transport per target],
  ..b.summary.map(r => (str(r.L), str(r.trials), f(r.mean_B) + " ± " + f(r.se_B), f(r.mean_C) + " ± " + f(r.se_C), f(r.mean_single) + " ± " + f(r.se_single), f(r.ks_reference), f(r.L * r.mean_single, d: 2), f(r.transport_M_per_target, d: 1) + " M + " + f(r.transport_I_per_target, d: 1) + " I")).flatten())

KS ref. is the Kuhn–Struik expected pooled cost on ${P, -P}$, $sum_(i<L) binom(2i, i) 4^(-i) sqrt(pi/4)$. $L dot S_"single"$ is derived as $L times$ the recorded mean of arm S (the cost of $L$ independent solves).

#figure(image("batch_cost_vs_L.svg", width: 92%), caption: [Batch run `run-20261018T2219Z`: recorded mean total cost per arm against $L$, $r = 2^(#f(b.base_curve.rbits_actual, d: 2))$, 20 trials per $L$, bars 2 SE. Source `batch.json`; plot script `plot.py`.])

*Observations.* At every $L$ the means of B and C lie within about 2 SE of each other. Transport costs at most #f(b.summary.last().transport_M_per_target, d: 1) $M$ + #f(b.summary.last().transport_I_per_target, d: 1) $I$ per target, at path depth up to #b.class.depth_max. Both pooled arms stay above the KS reference (at $L = 256$: #f(b.summary.last().mean_B) and #f(b.summary.last().mean_C) against #f(b.summary.last().ks_reference)) and far below $L dot S_"single"$. No paired interval between B and C was computed by the harness, so no difference or equivalence claim is made beyond these recorded SEs.

== Paths mode (measured)

For each $p$ size, random curves until 6 components of the rational $ell in {2,3,5,7}$ graph of at most #p.cap curves were enumerated (components over the cap skipped and counted). For 10 random pairs per component, a bidirectional BFS found a path from scratch. Cost is counted in neighbour enumerations (one per (curve, $ell$)).

#table(columns: 7, align: right,
  [$p$ bits], [$p$], [$r$], [$h$], [component], [mean enum./pair], [pairs],
  ..p.classes.filter(c => "p" in c).map(c => (str(c.pbits), str(c.p), str(c.r), str(c.h), str(c.component_size), f(c.mean_enumerations, d: 1), str(c.pairs.len()))).flatten())

Components skipped over the cap: #p.classes.filter(c => "skipped_over_cap" in c).map(c => str(c.pbits) + " bits: " + str(c.skipped_over_cap)).join("; ").

#figure(image("paths_enumerations_vs_component.svg", width: 92%), caption: [Paths run `run-20261018T2219Z`: recorded mean neighbour enumerations per pair against component size, 10 pairs per component. Source `paths.json`.])

*Observations.* Mean enumerations per pair range from #f(calc.min(..p.classes.filter(c => "p" in c).map(c => c.mean_enumerations)), d: 1) to #f(calc.max(..p.classes.filter(c => "p" in c).map(c => c.mean_enumerations)), d: 1). They grow with component size and stay on the order of the component size, here up to #calc.max(..p.classes.filter(c => "p" in c).map(c => c.component_size)) curves. Selection is biased toward small components by the cap; most 32-bit components exceeded it.

== Note on duals for $ell >= 5$ (derived, test-checked)

`SmallIsogeny::dual` builds $hat(phi)$ from a rational root of the $ell$-division polynomial on the codomain. If $phi$ has rational kernel abscissas, Frobenius acts on $ker phi$ by $lambda = plus.minus 1$ and on $ker hat(phi)$ by $p lambda^(-1)$, so $hat(phi)$ has rational kernel abscissas iff $p equiv plus.minus 1 med (mod ell)$. The test `degree_five_and_seven_isogenies_and_certified_duals` now asserts exactly this equivalence; it previously assumed the dual always exists and failed. The example drops such edges and records them in `edges_without_dual_by_ell`.

== Missing data and limits

No isolated or independent-runner measurement; no ecbench session or paired bootstrap; one base curve for batch mode; ℓ = 5, 7 transport untested on this $p$.
