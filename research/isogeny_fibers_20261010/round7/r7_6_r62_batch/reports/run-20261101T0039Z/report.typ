#set page(paper: "a4", margin: 1.8cm)
#set text(size: 9.5pt)
#let rec(n) = json("../../runs/" + n)
#let cmpf(L) = json("../../sessions/pooled-vs-rho-l0/comparisons/rho-neg__pooled-L" + str(L) + ".json")
#let yn(b) = if b { "yes" } else { "no" }
#let f(x, d: 2) = str(calc.round(x, digits: d))
#let ci(x, d: 2) = f(x.est, d: d) + " [" + f(x.lo, d: d) + ", " + f(x.hi, d: d) + "]"
#let names = ("batch_r24_sym0_v1.json", "batch_r24_sym1_v1.json", "batch_r28_sym0_v1.json", "batch_r28_sym1_v1.json", "batch_r32_sym0_v1.json", "batch_r32_sym1_v1.json", "batch_r36_sym1_v1.json")
#let runs = names.map(n => rec(n))
#let fit = rec("scaling_fit_v1.json")
#let sc = (rec("scaling_v1.json"), rec("scaling_v2.json"))
#let audit = json("../../sessions/pooled-vs-rho-l0.audit.json")
#let tag(d) = "r=2^" + f(d.base_curve.rbits_actual) + (if d.class.sym_primes_only { ", p=±1 mod 5,7" } else { ", p free" })

= R7.6 run report: pooled rho across an isogeny class, and path-finding cost against component size

*Run id* `run-20261101T0039Z`. *Branch* `research/r7-6-r62-batch`, base `origin/main` 332716c307 (library `src/` unchanged; new example `examples/r7_6_r62_batch.rs`). *Host* a 4-vCPU shared VM (`ecbench host`: env class ECBENV3h13e37662a118), other build jobs running: *no isolation earned, no wall time is claimed*; every number below is an operation count. Toy instances only; nothing here concerns a deployed curve and no sub-rho claim is made. It finishes R6.2 (#2230, corrected by #2543).

== Verdicts (details and numbers in `RESULTS.md`)

- *H1 (pooling across a class of size $C$ gives about a $sqrt(C)$ gain over separate rho runs):* supported as a scaling law, falsified as the equality "gain $= sqrt(C)$". The gain is above 1 with an interval excluding 1 at every $C >= 4$ in all 7 runs; its exponent in $C$ over 2..256 is 0.43 to 0.52 (ideal Kuhn–Struik: 0.489); at $C = 256$ the gain is 11.0 to 13.9 against 16, approaching the ideal 14.2 as $r$ grows; the gain is below $sqrt(C)$ by more than its interval from $C = 32$ in every run. Transported and same-curve pooling do not differ consistently (5 of 56 intervals exclude 1, mixed directions).
- *H2 (path-finding cost per pair is $O(sqrt(C))$):* refuted for the corrected breadth-first bidirectional search: exponent 1.13 [1.06, 1.21] over 40 classes and 32 distinct component sizes from 8 to 16 702 curves (3.3 decades); 0.5 is outside every interval.
- *Limits:* toy sizes; ecbench arm is same-curve pooling at L0 with a same-host replay certificate, not an independent one, and no admitted benchmark.

== What ran

- *Batch half* (native, `examples/r7_6_r62_batch.rs --mode batch`): one planted key on each of the first $L$ curves of a breadth-first class of a base curve $E_0$ (class size $C = L$), transported to $E_0$ along the `SmallIsogeny::dual`-certified duals (an $ell$ = 5, 7 edge enters the class only where `dual` certifies it; `--sym 1` draws $p equiv plus.minus 1$ mod 5 and mod 7, `--sym 0` leaves $p$ free). Arms on the library walk that ecbench's `rho.multitarget_pooled` and `rho.negation` run: *separate* (each transported target by its own `rho_walk_with`), *pooled B* (the first $L$ targets in one `rho_batch_with`), *pooled C* (the control: $L$ fresh keys on $E_0$, same pooled batch, same seed). Pooled B and separate share keys and seeds; the separate cost at $L$ is the prefix sum. Every recovered log is verified on its original curve (#runs.map(d => d.verified_logs).sum() logs over #runs.len() runs). Intervals: percentile bootstrap over trials, 10000 resamples.
- *ecbench half*: spec `spec.json` (`ECS1h0f7350931294`), session `ECBS1h4b0bd52ebc3a`, 640 records, all verified, `--cpus none` (L0, counts only), arms `rho.negation` and `rho.multitarget_pooled` with `batch` 4, 16, 64, 256, on registered `prime_search` curves of 20, 24, 28, 32 bits (16 workloads each, 2 measured rounds, no warm-up). Same-curve companions only (the control arm of the class experiment).
- *Path half* (native, `--mode scaling`, then `--mode fit`): whole certified components at $p$ of 12 to 34 bits, random pairs by the corrected bidirectional search, routes re-validated forward; cost per pair = search enumerations + certifications + validation enumerations.

== Audit and replay

`ecbench verify --replay-all --exit-code`: #audit.session_status session, #audit.records records, #audit.verified_records verified, #audit.replay_candidates replays, all reproduced, problems: #audit.problems.len(). Receipt `sessions/pooled-vs-rho-l0.audit.json`, SHA-256 `e120c0cb25583e61b40c14053070fc87240def172853a7b319d725114f783780`. The auditor ran on the same host class as the session, so this is a *same-host replay certificate, not an independent one*. The native example's records are replayed by `replay_check.py` (`replay_certificate.json`, below).

#let rc = json("../../replay_certificate.json")
#table(columns: 3, align: (left, left, left), stroke: 0.4pt + luma(160),
  [native record], [SHA-256 (committed file)], [replay],
  ..rc.records.map(r => (r.file, if "sha256" in r { r.sha256.slice(0, 16) + "..." } else { "-" }, if r.replayed { if r.identical_except_wall { "identical except the wall-time note" } else { "DIFFERS" } } else { "not replayed (" + r.reason + ")" })).flatten())

Replay binary SHA-256 #rc.binary_sha256.slice(0, 16)...; all replayed records identical: #yn(rc.all_replayed_identical).

== Batch half: gain of pooling over separate runs

Gain $= sum$ separate $slash sum$ pooled B (counted group additions); strict prediction $sqrt(L)$; the ideal Kuhn–Struik gain is $1 / (sum_(i<L) binom(2i, i) 4^(-i) / L)$ (closed form, recorded by the example as `ks_prediction_ratio`).

#figure(image("fig1_batch_gain_vs_L.svg", width: 100%), caption: [Recorded gain against $L = C$ for seven runs (95% bootstrap over trials), and the same divided by $sqrt(L)$. Source `runs/batch_r*_sym*_v1.json`; script `plot.py`.])

#table(columns: 7, align: (left, right, left, left, left, left, left), stroke: 0.4pt + luma(160),
  [run], [trials], [gain at $L = 64$], [gain at $L = 256$], [gain / $sqrt(L)$ at 256], [pooled / separate over ideal KS at 256], [gain exponent in $L$ (2..256)],
  ..runs.map(d => (tag(d), str(d.trials.len()), ci(d.summary.at(5).gain_separate_over_pooled), ci(d.summary.at(7).gain_separate_over_pooled), ci(d.summary.at(7).gain_over_sqrt_L, d: 3), ci(d.summary.at(7).ratio_over_ks_prediction, d: 3), ci(d.exponents_over_L.gain_ln_on_ln_L, d: 3))).flatten())

Predicted exponent 0.5 (strict) and #f(fit.ks_ideal_gain_exponent_L2_to_256, d: 3) (the ideal Kuhn–Struik curve over $L = 2 ... 256$). Pooled-cost exponents: #runs.map(d => f(d.exponents_over_L.pooled_cost_ln_on_ln_L.est, d: 2)).join(", ") (predicted 0.5); separate-cost exponents: #runs.map(d => f(d.exponents_over_L.separate_cost_ln_on_ln_L.est, d: 2)).join(", ") (predicted 1).

#figure(image("fig2_batch_exponent_and_control.svg", width: 100%), caption: [Left: exponent of the gain in $L$ with 95% bootstrap interval, per run, against the strict 0.5 and the ideal Kuhn–Struik exponent. Right: transport control B/C per $L$ (the interval and the band [0.95, 1.05]). Source `runs/batch_r*_sym*_v1.json`; script `plot.py`.])

== ecbench session (same-curve pooled arm, paired with `rho.negation`)

#table(columns: 5, align: (right, left, left, left, left), stroke: 0.4pt + luma(160),
  [$L$], [ecbench ratio pooled / `rho.negation` (cluster bootstrap)], [pooled / separate $=$ ratio $slash L$ (derived)], [gain $= L slash$ ratio (derived)], [per curve gain, $r = 2^19.1, 2^24.0, 2^28.0, 2^31.7$],
  ..(4, 16, 64, 256).map(L => {
    let o = cmpf(L).ops
    (str(L), f(o.ratio_b_over_a, d: 3) + " [" + f(o.ci95_bootstrap.at(0), d: 3) + ", " + f(o.ci95_bootstrap.at(1), d: 3) + "]", f(o.ratio_b_over_a / L, d: 4) + " [" + f(o.ci95_bootstrap.at(0) / L, d: 4) + ", " + f(o.ci95_bootstrap.at(1) / L, d: 4) + "]", f(L / o.ratio_b_over_a) + " [" + f(L / o.ci95_bootstrap.at(1)) + ", " + f(L / o.ci95_bootstrap.at(0)) + "]", cmpf(L).curves.map(c => f(L / c.ratio_b_over_a)).join(" ; "))
  }).flatten())

The separate cost is $L$ times the mean single-run $S$ of `rho.negation` on the same targets (one reference run per workload and round, 64 workloads, 128 pairs), so these intervals are wider than the example's, which runs every separate walk. Pricing band (unpriced `canonicalisations_uncharged`, priced 0, 0.1, 1 GAE): the order pooled $>$ `rho.negation` in $S$ holds at every weight at every $L$ (recorded `ops.band.order`).

#figure(image("fig3_ecbench_pooled_gain.svg", width: 70%), caption: [ECBS1h4b0bd52ebc3a: gain $= L slash$ (ecbench ratio) with the cluster bootstrap interval, overall and per curve. Source `sessions/pooled-vs-rho-l0/comparisons/rho-neg__pooled-L*.json`; script `plot.py`.])

== Path half: cost against component size

#table(columns: 7, align: (left, left, right, left, left, left, left), stroke: 0.4pt + luma(160),
  [subset], [cost], [classes], [sizes], [exponent (95% bootstrap)], [contains 0.5], [contains 1],
  ..fit.fits.map(x => (x.subset, x.cost, str(x.classes), str(x.size_min) + ".." + str(x.size_max), f(x.exponent, d: 3) + " [" + f(x.lo, d: 3) + ", " + f(x.hi, d: 3) + "]", yn(x.sqrt_in_interval), yn(x.one_in_interval))).flatten())

#fit.sizes.classes classes, #fit.sizes.distinct_sizes distinct component sizes, #f(fit.sizes.min, d: 0) to #f(fit.sizes.max, d: 0) curves (#f(fit.sizes.decades) decades); every returned route re-validated forward (asserted in the run).

#table(columns: 6, align: (left, right, right, right, left, right), stroke: 0.4pt + luma(160),
  [size bin], [classes], [mean size], [mean total per pair], [total / $C$ (min..max)], [total / $sqrt(C)$],
  ..fit.size_bins.map(b => (str(b.size_from) + ".." + str(b.size_to - 1), str(b.classes), f(b.mean_size, d: 0), f(b.mean_total_cost, d: 0), f(b.mean_cost_over_size) + " (" + f(b.min_cost_over_size) + ".." + f(b.max_cost_over_size) + ")", f(b.mean_cost_over_sqrt_size, d: 1))).flatten())

#figure(image("fig4_path_cost_vs_component.svg", width: 100%), caption: [Recorded mean counted enumerations per pair against component size, one point per class (5 to 6 pairs each); fit and reference slopes through the same centre; right: the same divided by $C$ and $sqrt(C)$. Source `runs/scaling_v1.json`, `runs/scaling_v2.json`, `runs/scaling_fit_v1.json`; script `plot.py`.])

== Limits

- ecbench: same-curve pooled arm only (not the transported arm), L0 counts only, one host class, same-host receipt; the reference is one run per workload, so its intervals are wider than the example's. No isolated or independent-runner measurement, no EKS/isolab run, no admitted benchmark, no wall time.
- Batch half: the pooled class is the first $C <= 256$ curves of a breadth-first class, one base curve per run (seven base curves, $r$ from $2^24.7$ to $2^36.9$), 6 to 30 trials; unregistered toy curves identified by $p, a, b$ in the records. The distance from the ideal Kuhn–Struik cost at large $C$ and small $r$ is measured, its cause is not.
- Path half: 40 classes, 5 to 6 pairs each, components capped at 12 000 (v1) or 24 000 (v2) curves; only 11 classes exceed 1000 curves; the search is breadth-first bidirectional on a toy class, not an Elkies walk; `scaling_v2.json` was not replayed (33 minutes).
- Replay of the native records (`replay_certificate.json`): see `RESULTS.md`.
