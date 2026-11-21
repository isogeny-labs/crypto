#set page(paper:"a4", margin:16mm)
#set text(font:"DejaVu Sans", size:9pt)
= Rust Vieta extraction: polynomial backend continuation
16 November 2026 · matched local component counts

*The opt-in Karatsuba backend reduces counted multiplication work on every
completed configuration in this panel.* The default constructor still selects
the historical schoolbook arithmetic. The Newton option performs identically
here: successful query rows invoke zero Newton divisions. Short-quotient
Euclidean GCD remains classical. All observations are producer checks pending
independent signed validation: #raw("[pcc:unverified]").

#table(columns:(1.4fr,.7fr,1.3fr,1.3fr,.8fr),inset:5pt,
 [Conditioned inputs],[m],[Schoolbook muls],[Karatsuba muls],[Paired ratio],
 [62-bit field],[8,192],[628,385,305],[300,898,741],[2.0884],
 [62-bit field],[16,384],[2,509,465,297],[1,168,899,497],[2.1469],
 [P224],[512],[2,922,285],[1,908,185],[1.5314],
 [P224],[2,048],[41,002,731],[22,039,567],[1.8604])

Medians use three matched retained conditioned targets per row. Ratios are native
medians of paired schoolbook/Karatsuba multiplication counts; the retained
random-rational population is separate. All primitive multiplications inside
inverses, square roots and output checks are included. Additions are also
recorded: at 62-bit/m16384 their median falls from 2,505,823,783 to 1,241,898,119.
Hashing, allocation and sorting are unpriced by the multiplication metric.

#image("ratios.svg",width:100%)

The exact A/A and unaffected lookup multiplication-count ratios are 1 on all
completed retained pairs. This tests determinism/invariance, not a stochastic
clock noise floor. The references use generic u128/BigUint arithmetic. Existing
optimized Montgomery/NTT/half-GCD or mature external implementations were not
measured, so no fastest-backend conclusion follows. Batched lookup still uses
much less multiplication work: 98,828 at 62-bit/m16384 and 13,419 at P224/m2048.

#pagebreak()
== Full populations and retained deadlines
#image("counts.svg",width:100%)

#table(columns:(2.3fr,1fr,1fr,1fr),inset:5pt,
 [Panel],[Planned/actual],[Succeeded],[Deadlines],
 [Primary: four configurations, six arms, two warmups + six retained],[192 / 192],[192],[0],
 [Supplement: P224/m8192, six arms, no warmups + two retained],[12 / 12],[4],[8],
 [Separate bootstrap calibration; excluded from comparisons],[12 / 12],[12],[0])

All eight supplemental polynomial calls reached their fixed 30-second stage
deadline; all four lookup controls completed. No failed row is assigned zero
cost or used as a successful pair. Original 20261115 failures and protocols are
unchanged. Separate-process wide scalar replay matches all 196 successful main
and supplemental calls, while retaining eight failed rows and returning exit1.
It is a producer replay, not an independent signed certificate.

#pagebreak()
== A conditional degree bound for split rational towers
The sibling SPLIT_TOWER_BOUND.md extends the hand argument to maps $N_i / D_i$
with coprime numerator/denominator, $"deg" N_i = q_i$ and $"deg" D_i <= q_i - 1$.
Every encountered finite fiber must split into $q_i$ distinct roots in the base
field. With $S = sum_i (q_i - 1)$, two membership chains plus the quadratic Vieta
core have proposed last fall degree at most $bold(2 S + 2)$. Binary rational
towers give *2 log2(M)+2*, for M=product(q_i).

The proof uses conditional degree-(q_i−1) Lagrange factors and degree-S lifted
projectors. A telescoping coordinate certificate carries denominator factors
without exceeding S+1. The local slope normal-form argument includes zero
target ordinates and nonreduced quadratic blocks. This remains a hand proof
awaiting independent/formal validation, not a solver-time estimate.

#image("split-tower.svg",width:100%)

The Rust exact-example checker expands all four projectors, their partition of
unity and degree-three coordinate certificates over F17. It checks homogeneous
membership composition against the four linear factors. On the separate toy
curve y²=x³+2x+2, exhaustive enumeration gives prime order 19 and the two ordered
relations for (9,1)+(10,6)=(6,14). This example is not the large-field experiment
or a general formal theorem certificate.

There is also a division-free, interpolation-free construction: compose each
map homogeneously in F_p[L][Z]/(Z²−sZ+v), starting U=Z,W=1, and extract the two
coefficients of U−cW. Repeated abscissae retain their own discriminant branch.
Off-diagonal coefficient degrees still grow with M. The measured Rust qualifier
implements Kummer cosets; a general rational-tower extractor is described here
as a construction and is not claimed implemented by that binary.

Normalized isogeny x-maps fit the degree shape, conditional on full rational
splitting of the selected auxiliary-curve fibers. Construction/search and
splitting certificates still need to be charged. This supplies a field subset;
it is not a DLP transfer between curves of different orders.

#pagebreak()
== Custody, resource scope and reproduction
The measured source is 9814b4bb5ca5949685be0f99bc992f6e1d0cc8bd, with an
immutable copied qualifier, replay and native summary command. Source/build,
distinct pre-call and GitHub publication manifests are under run/. The
post-call original hash check passed before post-call exporter/example-checker
edits. All three unified exports pass the canonical file-checking tool, with
zero findings. Frozen protocols and raw data were not rewritten.

Protocol hashes:
#raw("primary  1abbbad626f0fd1a7714eb833f61ed5cd1bbd55a9a2d84a7e922c63eb9527c1d\nsupplement b9d6705ccee48dc3b77132f050239bf7cf02e12c5c9273220489cc1eed57c477",block:true)

The unmodified isolated runner reserved CPU4 on a five-vCPU AMD EPYC9V74 KVM
host. It reports 930.815920002 seconds, 0.94 seconds of other-process CPU,
19,816 KiB peak RSS and no detected contention. PSI is unavailable and host
threads could not all be moved: *all clocks remain below-L2 diagnostics with
wall_valid=false*. No timing advantage is asserted. No builds/tests/heavy
analysis ran during the interval; only lightweight completed-panel Git
checkpoints. No cloud experiment/spend. User authorization was local/Git-only.

The primary six-arm dataset uses two warmups and six retained targets per
configuration, rotating/reversing the interleaved order. Scalar complete-set
verification is recorded separately outside each query interval. Shared
descriptor/base setup is also recorded separately. The reported ratios are
query-component counts, not cold end-to-end costs. Identity targets and both
populations have distinct group keys. No missed row or timeout is omitted.

Native checks pass: seven unit groups and four CLI/replay tests, formatting and
clippy -D warnings. The root focused release rebuild reached its explicit
180-second preparation limit while compiling; no full-root-suite pass is
claimed. The exact shared library source is compiled by the standalone crate.

After measurement, a metadata exporter host-path option and the standalone
symbolic example checker were added. They are not the measured instrument or
frozen statistical analysis. Initial export drafts used the wrong directory
for study-relative paths; corrected metadata exports pass, with the failed
attempt receipt retained. All scientific standing remains unverified.

The benchmark record is censored because the supplemental stage limits were
reached. No canonical benchmark-index/IC leaderboard row is promoted. The next
polynomial question is half-GCD on the same complete inputs; Newton division
alone did not engage on these short-quotient Euclidean chains.

Sources checked: repository isogeny_eval/poly.rs, zero_dim_la.rs, prime_fast and
pkm_smooth_tower; official FLINT polynomial/Newton material; PKM §3.3 and
Huang–Kosters–Yang–Yeo definitions2.1/2.5. KB unavailable; direct primary/internal
fallback, no novelty claim. See PLAN.md, SPLIT_TOWER_BOUND.md, run/summary.json,
run/replay.json and the three export-poly files at the study root.
