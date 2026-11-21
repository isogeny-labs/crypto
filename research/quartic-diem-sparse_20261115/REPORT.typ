#set page(margin: 1.6cm)
#set text(size: 9.5pt)
#set heading(numbering: none)
#show raw: set text(size: 8.5pt)
= Generic plane quartics: Diem index calculus with a c sqrt(q) factor base, double large primes and sparse linear algebra
2026-11-15. Study `research/quartic-diem-sparse_20261115/`, plan frozen in `PLAN.md` before the first measured run. Local development check at small parameters on one shared 4-CPU VM, not a lab benchmark. Every number is `[pcc:unverified]`. No improvement is claimed; gaps to the matched rho are distances. Repo HEAD at run time `3c24520f7cdc1c47bac26cc7778d058e655b79a8`.

== Question
The generic plane-quartic Diem tool (`examples/quartic_ic.rs`, study `tian_n3_quartic_route_20261024`) beats rho when native work is free and loses at 0.1 and 1 GAE per native operation, with dense linear algebra said to grow like q^1.4. The N4 study (`quartic_n4_la_20261021`) reached exponent 1.045 on the GHS-weak family. Does the generic quartic reach the same place when the same pipeline (c q^(1/2) factor base, double large primes, rows of at most four points plus the hyperplane unknown, sparse solver) is built on it?

== Answer in brief
Every IC unit with c \>= 2 verified `kG = Q` in the prime-order subgroup (30 of 30 primary units; c = 1 never produced a solvable system: the pair graph stays subcritical). The tool is dense because of its *solver* (Gauss-Jordan on a b x b system with b = c sqrt(q), about 0.6 b^3 mod-ell operations), not because of its factor-base size, which is already c sqrt(q). Replacing the solver by Wiedemann does *not* bring the linear algebra to q^1.0: the mean row weight after the large primes are eliminated grows like q^0.29 \[0.28, 0.31\], so Wiedemann costs about 6 w b^2 and scales like q^1.23 \[1.21, 1.24\] (dense: q^1.41 \[1.40, 1.42\]), and on the measured range the dense solver still uses fewer operations (dense/sparse operation ratio 0.67 at q = 131071).

Preregistered predictions (details below): P1 (LA share of S below 20% at q \>= 2^14) REFUTED; P2 (exponent in \[0.95, 1.15\]) REFUTED; P3 (S/rho below 1 at all three prices for q \>= 2^14) REFUTED.

Distance to the matched rho, c = 2, primary arm (S\_IC / S\_rho with rho modelled from measured calibration, 95% bootstrap over instances; the same table with measured rho is in `RESULTS.md`):

#table(columns: 7, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[w = 0],[w = w\_cal],[w = 10 w\_cal],[w = 0.1],[w = 1],[break-even w],
[1021],[0.0061 \[0.0029, 0.02\]],[0.034 \[0.015, 0.12\]],[0.28 \[0.13, 1\]],[5.09 \[2.31, 18.9\]],[20.2 \[9.17, 74.9\]],[0.017],
[4093],[0.0015 \[0.001, 0.0052\]],[0.0073 \[0.0051, 0.026\]],[0.06 \[0.042, 0.21\]],[2.79 \[1.94, 9.8\]],[11.1 \[7.69, 38.9\]],[0.032],
[16381],[0.0011 \[0.00096, 0.0012\]],[0.02 \[0.018, 0.023\]],[0.19 \[0.17, 0.22\]],[6.01 \[5.34, 6.79\]],[23.9 \[21.2, 27\]],[0.014],
[65521],[0.00022 \[0.00013, 0.00048\]],[0.0053 \[0.0037, 0.01\]],[0.051 \[0.036, 0.099\]],[2.39 \[1.71, 4.67\]],[9.48 \[6.79, 18.5\]],[0.037],
[131071],[5.1e-05 \[4.6e-05, 6.3e-05\]],[0.0022 \[0.0018, 0.0028\]],[0.021 \[0.018, 0.028\]],[1.01 \[0.834, 1.31\]],[4 \[3.31, 5.19\]],[0.099],
)

Extrapolated parity (beyond the measured range, not a measurement; fitted slopes 1.22 \[1.21, 1.24\] for IC and 1.52 \[1.38, 1.86\] for modelled rho at w = 0.1, starting from the ratio at q = 131071): w = 0.1: ratio 1.01 at q = 131071, parity at about 2^17; w = 1: ratio 4.00 at q = 131071, parity at about 2^24.

At zero-priced native work and at the calibrated price of a mod-ell operation (w\_cal, about 2e-4 to 5e-4 GAE here) the IC is far below rho; at the band price 0.1 GAE the ratio is 1.01 \[0.834, 1.31\] at q = 131071 (5.09 \[2.31, 18.9\] at q = 1021) and at 1 GAE it is 4 \[3.31, 5.19\] (from 20.2 \[9.17, 74.9\] at q = 1021), so the order changes inside the band and by the section 5a rule the ranking is *unresolved*, not a win. The price at which IC and rho tie (break-even w) is the distance to read: about 0.099 GAE per native operation at q = 131071. The ratios are not monotone in q because each instance has its own ell (cofactor 3 to 61), which moves its rho by factors of several; three instances per cell cannot average that out.

== Why the generic tool is dense (task a)
Read from `examples/quartic_ic.rs`, function `run_ic`, and checked by the A/B run below:

1. Factor-base size. `b = ceil(bmult/100 * sqrt q)`: the tool already uses a c sqrt(q) base (c = 1 first, +1 after a failure). Not the cause. In this study c = 1 failed at every q (the graph of residual-point pairs is subcritical: about q/4 edges on q vertices) and c = 2 is the smallest base that works.

2. Solver. The relations go to `nullspace`, a dense Gauss-Jordan over `Vec<Vec<u64>>` of width b + 1 with all cycle rows (about 2b + 30). It costs about 0.6 b^3 counted mod-ell operations, i.e. q^1.5 at b = c sqrt(q); measured exponent of the dense operation count on q \>= 2^12: 1.41 \[1.40, 1.42\]. This is the dense linear algebra of the earlier report.

3. Relation shape. The tool builds rows from tree-path potentials of the contracted graph, so a row is a sum of path vectors, not a line relation: after the large primes are removed the mean row weight is 21.7 at q = 1021 and 84.6 at q = 131071 (raw rows: at most 4 points plus h, weight at most 5). The weight grows like q^0.29 \[0.28, 0.31\]. Rows are sparse only before the large primes are eliminated, and then the columns number about q.

4. A bookkeeping flaw found while building: a line through three or four base points is enumerated once per pair of them and gave identical rows, so the tool's stopping count of independent cycles was inflated (in a q = 1021 debugging run 141 counted cycles left 83 nonempty rows after elimination). The new code keeps each line once.

== Method
Arms (same instance, same group arithmetic, same counters, one process): *IC-sparse*, factor base of b = ceil(c sqrt q) random rational points plus the pinned base point O; pairs of base points in random order; each line gives the two residual points from a quadratic and a square-root table; rows `x_i + x_j + x_X + x_Y - h = 0` with large primes as extra columns; large-prime columns removed by minimum-weight elimination (`diem_quartic::eliminate_lp`); sequential Wiedemann on the normal equations mod ell (`diem_quartic::wiedemann_solve`, 8 held-out rows); large-prime logs by propagation over the edge graph; the base tool's target stage; result checked `kG = Q`. *IC-dense A/B*: the same eliminated rows solved by the base tool's `nullspace`; its kernel must equal the Wiedemann solution (agreement flag, checked on every unit with b \<= 800). *rho / rhoneg*: the base tool's distinguished-point rho and its negation-map variant on the same subgroup.

Cells: q in {1021, 4093, 16381, 65521, 131071}, three instances (curve seeds 7000 to 7002) per q, c in {1, 2, 4} plus an A/A repeat of c = 2 with a second IC seed; primary configuration c = 2. Rho measured at q = 1021 (20+20 runs per instance), 4093 (5+5), 16381 (1+1) and 65521 (plain only: 1 run on instances 0 and 1, 15 on instance 2); above that rho is modelled as kappa sqrt(ell) steps with kappa = 1.254 the pooled mean of steps/sqrt(ell) over 95 verified plain runs (textbook sqrt(pi/2) = 1.253) and the per-step cost of the plain runs at q = 65521 (1.002 GAE and 2.03 native operations per step). Negation-map rho: kappa 1.322 over 78 runs but two group operations per step, so plain rho is the cheaper reference and is the one used.

Cost unit: `S(w) = (M + cI I) / (gM + cI gI) + w (NAT + LA)` in GAE, cI = 3, with M, I the F\_q multiplications and inversions of the whole unit, gM, gI those of one group addition measured on the instance, NAT the hash probes, table lookups, union-find steps and elimination entries, LA the mod-ell multiply-adds of elimination, Wiedemann and Berlekamp-Massey. rho uses the same formula. w in {0, 0.1, 1} (the band) and the calibrated w\_cal = time of one mod-ell multiply-add / time of one group addition on the same curve and host (best-of-N on the shared VM, not isolated): q = 1021: 4.54e-04; q = 4093: 1.75e-04; q = 16381: 2.66e-04; q = 65521: 1.76e-04.

== Results
=== IC-sparse, c = 2 (verified units, means over three instances)
#table(columns: 14, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[b],[ok],[pairs],[rows],[weight],[LA elim],[LA Wiedemann],[LA dense],[target adds],[S w=0],[S w=0.1],[S w=1],[LA share w=0.1],
[1021],[64],[3/3],[2027],[122],[21.7],[1.05e+05],[9.58e+05],[2.04e+05],[88],[110],[1.11e+05],[1.11e+06],[0.976],
[4093],[128],[3/3],[6539],[205],[30.8],[3.46e+05],[4.72e+06],[1.46e+06],[159],[227],[5.19e+05],[5.19e+06],[0.985],
[16381],[256],[3/3],[23253],[362],[44.8],[1.13e+06],[2.45e+07],[1.05e+07],[181],[384],[2.6e+06],[2.6e+07],[0.991],
[65521],[512],[3/3],[85483],[680],[68.6],[3.71e+06],[1.42e+08],[8.24e+07],[468],[1122],[1.47e+07],[1.47e+08],[0.995],
[131071],[725],[3/3],[165803],[951],[84.6],[6.85e+06],[3.48e+08],[2.33e+08],[323],[1498],[3.58e+07],[3.58e+08],[0.996],
)

=== Matched rho
#table(columns: 8, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[arm],[runs],[mean steps],[steps/sqrt(ell)],[S w=0],[S w=0.1],[S w=1],
[1021],[rho],[60],[20849],[1.27],[2.31e+04],[2.73e+04],[6.54e+04],
[1021],[rhoneg],[60],[22030],[1.41],[5.08e+04],[5.75e+04],[1.18e+05],
[4093],[rho],[15],[174702],[1.24],[1.78e+05],[2.14e+05],[5.33e+05],
[4093],[rhoneg],[15],[112932],[0.95],[2.46e+05],[2.8e+05],[5.89e+05],
[16381],[rho],[3],[297426],[1.04],[2.99e+05],[3.6e+05],[9.03e+05],
[16381],[rhoneg],[3],[409570],[1.40],[8.45e+05],[9.69e+05],[2.09e+06],
[65521],[rho],[17],[2826462],[1.23],[2.83e+06],[3.4e+06],[8.57e+06],
)

Modelled rho on the instances of the IC cells (mean over the three instances of kappa sqrt(ell) steps at the measured per-step cost):

#table(columns: 5, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[mean ell],[S w=0],[S w=0.1],[S w=1],
[1021],[3.73e+08],[1.81e+04],[2.18e+04],[5.48e+04],
[4093],[1.93e+10],[1.55e+05],[1.86e+05],[4.68e+05],
[16381],[8.24e+10],[3.6e+05],[4.33e+05],[1.09e+06],
[65521],[1.9e+13],[5.13e+06],[6.17e+06],[1.55e+07],
[131071],[5.63e+14],[2.95e+07],[3.55e+07],[8.94e+07],
)

*Check of the rho model.* The modelled rho uses the pooled kappa = 1.25 (95 verified plain runs). Plain-rho runs at the larger sizes, steps/sqrt(ell) per instance: q = 16381, instance 0: 1 run(s), mean 0.90 (cofactor 58); q = 16381, instance 1: 1 run(s), mean 0.99 (cofactor 44); q = 16381, instance 2: 1 run(s), mean 1.24 (cofactor 60); q = 65521, instance 0: 1 run(s), mean 0.59 (cofactor 8); q = 65521, instance 1: 1 run(s), mean 0.38 (cofactor 16); q = 65521, instance 2: 15 run(s), mean 1.33 (cofactor 61). The first three runs at q = 65521 (one per instance) were low (0.59, 0.38, 0.29); 14 further runs on instance 2 brought the mean over all 17 runs at that size to 1.23, against the pooled 1.25, so the low start was chance and the pooled value is not contradicted at q = 65521 (it is not tested above that). Sensitivity: with kappa taken at the largest measured q the ratio S\_IC/S\_rho at w = 0.1 (c = 2, primary arm) is q = 65521: 2.42 \[1.74, 4.74\] instead of 2.39 \[1.71, 4.67\]; q = 131071: 1.02 \[0.847, 1.33\] instead of 1.01 \[0.834, 1.31\]. Caveat: at q = 65521, 15 of the 17 runs sit on one instance, so the measured-rho columns at that q rest mostly on one subgroup; above q = 65521 rho is modelled.

=== Exponents on q \>= 2^12 (log-log slope, 95% bootstrap over instances)
#table(columns: 3, align: right, inset: 3pt, stroke: 0.4pt + gray,
[quantity],[slope],[n\_q],
[ic\_main\_c2\_w0],[0.57 \[0.54, 0.60\]],[4],
[ic\_main\_c2\_w0.1],[1.22 \[1.21, 1.24\]],[4],
[ic\_main\_c2\_w1],[1.22 \[1.21, 1.24\]],[4],
[ic\_main\_c2\_wcal],[1.14 \[1.12, 1.16\]],[4],
[ic\_main\_c2\_wcal10],[1.19 \[1.17, 1.20\]],[4],
[wfb\_main\_c2],[0.29 \[0.28, 0.31\]],[4],
[pairs\_main\_c2],[0.93 \[0.92, 0.94\]],[4],
[relGAE\_main\_c2],[0.83 \[0.82, 0.83\]],[4],
[tgtadds\_main\_c2],[0.28 \[0.20, 0.35\]],[4],
[wfb\_bfs\_c2],[0.25 \[0.24, 0.26\]],[3],
[wfb\_light\_c2],[0.21 \[0.20, 0.22\]],[4],
[LAops\_wied\_c2],[1.23 \[1.21, 1.24\]],[4],
[LAops\_dense\_c2],[1.41 \[1.40, 1.42\]],[4],
)

Reading: the pair-trial count grows like q^0.93 \[0.92, 0.94\] and the relation-phase M cost like q^0.83 \[0.82, 0.83\]; the target stage is nearly flat (q^0.28 \[0.20, 0.35\], about 150 group additions); S at w = 0 is therefore dominated by the descent at small q (slope 0.57 \[0.54, 0.60\]). At every priced w the cost is the linear algebra (LA share of S \>= 0.99 at q \>= 16381).

=== Dense A/B on the same rows (c = 2)
#table(columns: 6, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[b],[row weight],[sparse LA ops],[dense LA ops],[dense / sparse],
[1021],[64],[21.7],[1.08e+06],[3.09e+05],[0.29],
[4093],[128],[30.8],[5.11e+06],[1.81e+06],[0.35],
[16381],[256],[44.8],[2.58e+07],[1.16e+07],[0.45],
[65521],[512],[68.6],[1.46e+08],[8.61e+07],[0.59],
[131071],[725],[84.6],[3.56e+08],[2.4e+08],[0.67],
)

Dense and Wiedemann kernels agreed on 111 of 111 units with a dense run. With exponents 1.23 \[1.21, 1.24\] (Wiedemann) against 1.41 \[1.40, 1.42\] (dense) the two operation counts cross where dense/sparse reaches 1, an extrapolation beyond the measured range, near q of the order of 10^6.

=== A/A and controls
A/A (same instance, two IC seeds of c = 2): S(seed 1)/S(seed 2) lies in \[0.84, 1.05\] at w = 0.1 and 1 (15 pairs) and in \[0.73, 1.48\] at w = 0, so the IC-seed noise is far smaller than the gaps to rho reported above (the w = 0 spread is the descent, about 100 to 500 additions, which varies with the seed). Re-running the primary c = 2 cells at q = 4093 with a later build of the binary gave bit-identical counters (6 of 6 records). Rho calibration steps/sqrt(ell) = 1.25 (criterion in the N4 study: 1.0 to 1.5).

== Preregistered predictions against what happened
*P1: LA share of S below 20% at q \>= 2^14 (at w = 0.1 and at w = 1): REFUTED.* The share is 0.991 (w = 0.1) and 0.991 (w = 1) at q = 16381; at the calibrated price it is 0.94 at q = 16381, 0.95 at q = 65521, 0.97 at q = 131071.

*P2: exponent of S in \[0.95, 1.15\] on q \>= 2^12: REFUTED.* w = 0: 0.57 \[0.54, 0.60\]; w = 0.1 and 1: 1.22 \[1.21, 1.24\]; w\_cal: 1.14 \[1.12, 1.16\]; 10 w\_cal: 1.19 \[1.17, 1.20\]. At w = 0 the cost is the relation phase plus a nearly constant descent (below the range); at the band prices the slope is set by the linear algebra (above the range); at w\_cal it is straddling the edge of the range and at 10 w\_cal above it.

*P3: S/rho below 1 at all three prices for every q \>= 2^14: REFUTED* (w = 0: holds; w = 0.1: fails; w = 1: fails; rho modelled at q \>= 65521 for the headline ratios). Variant at the calibrated price (order at w = 0, w\_cal and 10 w\_cal): holds at every q \>= 2^14 measured (ranked at calibrated price, the band verdict still governs).

The forecast written into `PLAN.md` before the runs said P2 would hold and P3 would hold at w = 0.1 but not at w = 1. It was wrong on both: the row weight growth (q^0.29 \[0.28, 0.31\]) pushed the linear-algebra exponent above 1.15, and rho on the actual subgroups (ell is the largest prime factor of the group order with cofactor at most 64) is several times cheaper than the nominal q^1.5, so at w = 0.1 the IC is above rho up to q = 65521 (ratio 2.39 \[1.71, 4.67\]) and level with it at q = 131071 (ratio 1.01 \[0.834, 1.31\]).

== Exploratory arms (PLAN Amendment 1, outside P1 to P3)
#table(columns: 9, align: right, inset: 3pt, stroke: 0.4pt + gray,
[q],[mode],[ok],[row weight],[LA ops],[LA ops / primary],[pairs],[S w=0.1],[S/rho w=0.1],
[1021],[bfs],[3/3],[19.8],[8.92e+05],[0.83],[2027],[9.14e+04],[4.2 \[1.92, 15.2\]],
[1021],[light],[3/3],[18.2],[7.99e+05],[0.74],[2080],[8.22e+04],[3.78 \[1.75, 13.2\]],
[4093],[bfs],[3/3],[27.5],[4.27e+06],[0.84],[6539],[4.35e+05],[2.34 \[1.61, 8.37\]],
[4093],[light],[3/3],[17.5],[2.65e+06],[0.52],[8256],[2.75e+05],[1.48 \[1.01, 5.17\]],
[16381],[bfs],[3/3],[37.6],[2.08e+07],[0.81],[23253],[2.11e+06],[4.87 \[4.49, 5.25\]],
[16381],[light],[3/3],[22.5],[1.25e+07],[0.48],[29611],[1.29e+06],[2.97 \[2.78, 3.22\]],
[65521],[bfs],[3/3],[55.6],[1.16e+08],[0.79],[85483],[1.17e+07],[1.9 \[1.33, 3.71\]],
[65521],[light],[3/3],[31.2],[6.56e+07],[0.45],[102997],[6.71e+06],[1.09 \[0.759, 2.07\]],
[131071],[light],[3/3],[36.3],[1.51e+08],[0.42],[195136],[1.54e+07],[0.434 \[0.373, 0.536\]],
)

`bfs` takes breadth-first potentials from the base points as rows (no elimination); `light` collects 6b cycles and hands the lightest rows to Wiedemann after a coverage guard. S/rho (modelled rho) stays above 1 for both modes at every q and price in the band except: light at q = 131071, w = 0.1 (0.434 \[0.373, 0.536\]). `light` needs 0.42 of the primary linear-algebra operations at q = 131071 (c = 2); the pre-written expectation was about 0.5. Weight exponents: primary q^0.29 \[0.28, 0.31\], bfs q^0.25 \[0.24, 0.26\], light q^0.21 \[0.20, 0.22\].

== Limitations and deviations from the task text
- Sizes: q up to 2^17; three instances per cell, so intervals are wide and the ell of each instance (cofactor up to 64) moves the rho reference by a factor of several; the q = 131071 instances were run in parallel processes (see `run_q131071_parallel.sh`).
- Rho is measured to q = 65521 only (1 run per instance at 16381; 17 runs at 65521, 15 of them on one instance); at q = 131071 it is modelled with the pooled kappa, and the model is also used for the headline ratios at 65521 (ratios against measured rho are in `RESULTS.md`).
- The N4 graph code (`research/quartic_n4_la_20261021/ic.rs`) is tied to the GHS instance type and was not reused; the graph, breadth-first potentials and elimination here follow the base tool and `diem_quartic.rs`. The N4 rho records and tool do not apply to generic quartics (GHS instance type), so rho is the base tool's. The Pic^0 arithmetic is the base tool's two-cubic Mumford-like arithmetic (`examples/quartic_ic.rs`), the same for IC and rho, not `plane_quartic_pic.rs`.
- Wiedemann is the sequential solver on the normal equations of `diem_quartic.rs`, not block Wiedemann; block Wiedemann does not change the operation count.
- LA mod-ell operations are priced at w like other native operations (the convention of the Tian study and of this task). The N4 study prices them at a measured field-multiplication equivalent instead; the calibrated rows above bridge the two conventions (AGENTS.md section 5a, 2026-11-12).
- w\_cal is best-of-N on a shared VM; q = 131071 uses the value of the largest calibrated q. Hash probes are priced at the same weight as mod-ell operations.
- Generic random smooth quartics with a prime-order subgroup of the Jacobian over F\_q; no claim about structured, deployed or extension-field targets.
== Reproduce
`cd research/quartic-diem-sparse_20261115 && cargo build --release -j2 --bin quartic_diem_sparse --target-dir ../../target && QDS_GIT=$(git rev-parse HEAD) ./run_ladder.sh ../../target/release/quartic_diem_sparse results/raw && QDS_GIT=$(git rev-parse HEAD) ./run_explore.sh ../../target/release/quartic_diem_sparse results/raw_explore && python3 summarize.py results . && python3 plot.py && python3 make_report.py` (smallest cell: `QDS_GIT=x ./quartic_diem_sparse run out.jsonl 4093 3 7 200 none 800`, a few seconds; the calibration is `quartic_diem_sparse calib <q> 7000`). At q = 131071 the instances ran in parallel processes (`run_q131071_parallel.sh`, `QDS_INST0`); the extra rho runs are `QDS_INST0=2 QDS_RHO_RUN0=1 ./quartic_diem_sparse run results/raw/rho_q65521_inst2_more.jsonl 65521 1 7 - plain 0 14` and one run per instance at 65521 with `QDS_INST0=i ... 65521 1 7 - plain 0 1`; the `light` arm at 131071 ran per instance with `QDS_ROWMODE=light QDS_LIGHTMULT=6 QDS_INST0=i`. Binary hashes are in `results/binary_sha256.txt` (several builds; counters of the primary cells are bit-identical across them where re-run). Unit tests: `cargo test --release --bin quartic_diem_sparse sparse_tests`.

== Sources read
`research/quartic_n4_la_20261021/README.md`, `PROTOCOL.md`, `REPORT.typ` (head), `solver.rs`, `n4.rs`, `common.rs`, `summ.rs` (signatures and pricing lines; `ic.rs` not read in full); `research/tian_n3_quartic_route_20261024/REPORT.md` (items a and b), `PROTOCOL.md` (head); `examples/quartic_ic.rs` (group law, `run_ic`, `run_rho`, `build_instance`, `cmd_run`, `cmd_summarize`); `src/cryptanalysis/diem_quartic.rs` (header, sparse stage: `build_rows`, `eliminate_lp`, `wiedemann_solve`, cost accounting); `src/cryptanalysis/plane_quartic_pic.rs`, `pq_wiedemann.rs`, `large_prime_filter.rs`, `src/bin/quartic_ic.rs` (headers and signatures only; not used); `AGENTS.md` section 5a.

#figure(image("figure.svg", width: 100%), caption: [Cost, distance to rho, linear-algebra operations and row weight against q (c = 2). Dashed cost curves: rho, measured to q = 65521 and modelled at 131071.])
