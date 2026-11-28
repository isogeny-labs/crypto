# Half-GCD costs: fewer multiplications did not yield a Newton speedup

The new Rust run completed504/504 calls (72 warmup,432 retained), with no failed
cells or retries; all504 complete relation sets replayed on84 distinct inputs.
All12 groups pass exact A/A and double-query count controls and the frozen
positive timing separation. Calibration is separate:72/72 calls replayed on12
inputs. Native raw summaries retain every value, range and descriptive95%
bootstrap median interval. Each population/configuration has n=6 retained inputs.

On conditioned finite targets at word62/m16384, half-GCD used1.4437× fewer query
multiplications than Euclid/Newton but its local median query time was8.072s
versus4.944s. It beat schoolbook's12.448s. Full construction-plus-query medians
were8.192s half,5.065s Euclid/Newton and0.129s lookup. These are per-arm medians;
paired-ratio medians below are computed separately by the frozen Rust analyzer.

| Conditioned finite targets, n=6 | School / half query | Euclid-Newton / half query | Euclid-Newton / half cold | Lookup / half cold |
|---|---:|---:|---:|---:|
| Coset word62 /8192 |1.1430|0.4857|0.4971|0.0229|
| Coset word62 /16384 |1.5441|0.6165|0.6220|0.0157|
| Coset P224 /2048 |1.1797|0.5779|0.6310|0.1295|
| Rational word62 /2048 |1.3235|0.4805|0.8503|0.7095|
| Rational P224 /128 |0.8843|0.6307|0.9751|0.9202|
| Rational P224 /512 |1.2615|0.5860|0.9411|0.8590|

Ratios above1 favor half-GCD; below1 favor the reference. Random-abscissa targets
remain separate and show the same direction against Euclid/Newton in this run.
Lookup has the lowest median full cost in every group. Half-GCD remains explicit
and opt-in; these observations do not justify replacing the default globally.
The counter saving did not establish elapsed improvement. No bottleneck profile
was run, so allocation/addition/matrix overhead explanations remain hypotheses.

**Evidence limits:** local clocks are belowL2 and diagnostic only. Furthermore,
canonical exports fail360 main/60 calibration seed-table checks because the
frozen registration encoded six per-configuration seeds using a cell-cycling
`explicit_list` field. The actual published configuration/source rule was
followed and separately byte/scope audited, but this does not repair strict
protocol qualification. See [REGISTRATION_ERRATA.md](REGISTRATION_ERRATA.md).
No new benchmark, seed relabeling, posthoc protocol change or gate weakening.
All observations remain producer-owned [pcc:unverified].

The isolated main wrapper recorded3535.874247s wall,3527.613067s user CPU,
2.965507s system CPU,12928KiB peak RSS and2.89s foreign CPU, contended=false.
Persistent supervisor04:56:42–05:55:40UTC exited0. Native calibration point forecast
3553.997345s was only a planning estimate. Missing PSI/unmovable host threads
prevent L2 admission despite the foreign-CPU result. No cloud/S3 operation.

The shared Rust core already implements recursive half-GCD with Euclid base
cases, schoolbook/Newton references and general homogeneous rational towers.
The new experiment reuses those exact bytes and charges full descriptor/fiber
certification and point-base lifting to every arm; lookup adds its index.
[PAIRED_COSTS.md](PAIRED_COSTS.md), paired-costs.tsv and run/summary.json retain
both populations and query/full-cost counts/elapsed ratios with uncertainty.
[COST_MODEL.md](COST_MODEL.md) lists exclusions and fixture applicability.

The degree audit checks21 Lean theorems with0 trusted exceptions, including the
actual-degree NF implication, its bridge to existing V/lastFallDegree, finite
literal projector/full-splitting/pole/coprimality certificates and nilpotent
controls. A separate nonempty diagonal test uses the order19 curve overF17.
The arbitrary-depth tower NF instance and independent signed standing remain
unfinished: the full hand bound is **not established**. Shared crypto-lean PR26
CI never started due to GitHub's account billing lock. See PROOF_MAP.md,
ASSUMPTION_AUDIT.md and REQUIREMENTS.md for the exact scope and remaining step.
