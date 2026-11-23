// W3b results report. Every number is read from the recorded files under
// data/ (written by `cargo run --release --example smith_isogeny`); nothing
// here computes a result.
#set page(paper: "a4", margin: (x: 1.5cm, y: 1.4cm), numbering: "1")
#set text(size: 8.8pt)
#set par(justify: true, leading: 0.55em)
#set heading(numbering: "1.")
#show heading: set text(size: 10pt)
#let S = json("data/summary.json")
#let CF = json("data/cost_fit.json")
#let f(x, n: 3) = str(calc.round(x, digits: n))
#let tbl(..args) = text(size: 7.2pt, table(inset: 2.6pt, stroke: 0.4pt + luma(170), ..args))
#let checks = ("t2_well_defined", "t3_homomorphism", "t4_kernel_s", "t4_outside_nonzero", "t5_order_nonzero", "t5_order_annihilated", "t6_group_order")
#let cnames = ("T2 well-defined", "T3 homomorphism", "T4 S ⊂ ker", "T4 ≠ 0 off S", "T5 Ψ(G) ≠ 0", "T5 ℓΨ(G) ~ 0", "T6 #J·Ψ ~ 0")
#let tot(e, c, k) = S.verify.map(v => v.engines.at(e).at(c).at(k)).sum()
#let pfs(t) = str(t.pass) + "/" + str(t.fail) + "/" + str(t.skip)

#align(center)[#text(size: 13.5pt, weight: "bold")[Smith's explicit isogeny ψ on divisor classes (W3b), and a toy Route S solve] \
`research/smith_isogeny_divisor_map_20261018` · branch `claude/smith-isogeny-divisor-map-20261018` · protocol `PROTOCOL.md` (frozen 2026-10-18, not edited) · synthetic toy fields only]

= Result in one paragraph

$psi = (pi_X)_* (pi_H)^*$ (Smith 2009, arXiv:0806.2995, §§5–6) was computed on Mumford divisors of
genus-3 curves H: y² = F(x), deg F = 7, as effective degree-6 divisors on the plane quartic C ≅ X that the same map's
canonical image defines, and classes were compared with two independent engines: W1's canonical `Pic^0` keys
(`plane_quartic_pic::PlaneQuartic::reduce_deg6`, `PicClass::key`) and the branch's own degree-6 adjoint reduction.
*No check failed on either engine* (pass/fail/skip below; a skip is an instance the engine could not decide and is not
counted as a pass). On every curve at q ≤ 13 the full image was enumerated: ker ψ ∩ $J_(H)$($F_(q)$) = S($F_(q)$) and
every fibre has \#S($F_(q)$) elements. Route S ran end to end at toy q: a planted ECDLP on E0/$F_(q^3)$ went through the
Joux–Vitse m = 3, r = 5 descent to H, through ψ to $J_(X)$($F_(q)$), was solved there on canonical keys, and the
recovered scalar was checked by scalar multiplication on E0. No security claim about any curve is made or implied.

#figure(image("reports/pipeline.svg", width: 100%), caption: [Route S as executed (T8). Module and function names under each box.])

= Instances and what was run

Master seed 20261018 (SplitMix64). Verification suite: q ∈ {7, 11, 13, 17, 19, 23}; per q the first 6 monic squarefree
degree-7 F (stream `random_curve7`, the `random_curve` scheme of `smith_prop24` with degree 7) having a rational (S, g)
(Props. 2 and 4); the first such map is used. Sample per curve (protocol §4): 30 T2 elements with two bases A, A′; 30 T3
pairs; all nonzero elements of S($F_(q)$) and 100 random elements outside it (T4); 3 elements of order ℓ for each odd
prime ℓ | \#$J_(H)$($F_(q)$) (T5); T6 on the T2 images. Engines see identical ψ-images.

#tbl(columns: 9,
  [*q*], [*curves scanned*], [*curves used*], [*T1 pass*], [*X hyperelliptic (T1a)*], [*hold-out pts / fail*], [*image enumerated (curves, elements)*], [*image = J/S, ker = S*], [*first-base retries*],
  ..S.verify.map(v => ([#v.q], [#v.curves_scanned], [#v.curves], [#v.t1_pass], [#v.x_hyperelliptic], [#v.holdout_points / #v.holdout_failures],
    [#v.image_curves, #v.image_elements], [#v.image_ok / #v.image_curves], [#v.image_first_try_errors])).flatten())

T1 passes means: quartic nullspace of dimension 1, hold-out of fresh image points (all on C), smoothness certificate
(two resultant frames) and W1's own certificate, \#C($F_(q^k)$) = \#H($F_(q^k)$) for k = 1, 2, 3 and equal
L-polynomials. "X hyperelliptic" is T1(a)'s separately reported outcome (images on a conic): no plane quartic, so T2–T6
do not apply to that curve. "First-base retries": elements whose image under the first base had a point of multiplicity
> 2 (which the bridge does not reduce) and were recomputed with another base (the class does not depend on it, T2).

= T2–T6: two engines side by side (pass / fail / skip, all q)

#tbl(columns: (auto, 1fr, 1fr),
  [*check*], [*W1 canonical keys*], [*degree-6 adjoint engine*],
  ..range(checks.len()).map(i => ([#cnames.at(i)],
    [#tot("w1", checks.at(i), "pass") / #tot("w1", checks.at(i), "fail") / #tot("w1", checks.at(i), "skip")],
    [#tot("deg6", checks.at(i), "pass") / #tot("deg6", checks.at(i), "fail") / #tot("deg6", checks.at(i), "skip")])).flatten())

#figure(image("reports/verify_tallies.svg", width: 100%), caption: [T2–T6 instances by outcome, summed over q ∈ {7, …, 23}
(per-q tallies in `data/summary.json`, per-curve rows in `data/verify_q*.jsonl`).])

W1 skips are images with a point of multiplicity > 2 (the bridge builds the cubics through $Z_(6)$ with tangency rows up to
multiplicity 2) and fall with q. Degree-6 engine skips are its own exceptional/overlap/frame failures (listed per curve in
the JSONL); its T6 skips are scalar multiplications that hit such a case. The degree-6 engine was validated by its
group-law test at p = 37 (associativity, commutativity, inverses, \#J·a = 0) and by agreeing with W1 on every instance
both decided; the protocol's T9(ii) class count at q = 7 for this engine was *not run*. Which arithmetic was kept: W1's
`PlaneQuartic` is the arithmetic (group law, keys, the T8 discrete log); the branch's adjoint `Pic` stays only as the
protocol's independent second engine.

*T1 on degree-8 models and T1(f).*
#tbl(columns: 7, [*q*], [*curves*], [*T1 pass*], [*X hyperelliptic*], [*twist maps (lc(s) non-square)*], [*$N_(1)$(X) = 2(q+1) − $N_(1)$(H)*], [*not applicable / disagree*],
  ..S.deg8.map(d => ([#d.q], [#d.curves], [#d.t1_pass], [#d.x_hyperelliptic], [#d.t1f.maps], [#d.t1f.agree], [#d.t1f.not_applicable / #d.t1f.disagree])).flatten())
T1(f) is checked at k = 1 only (`smith_prop24::x_point_count` exists for deg F = 8, k = 1; "not applicable" = a degenerate
fibre or $N_(1)$(H) = q + 1); k = 2, 3 of T1(f) were not run.

= T8: Route S end to end

Construction: `research/ghs_cubic_hyperelliptic_genus3_20261013/hyper_genus3.rs` (Joux–Vitse m = 3, r = 5, split arm
Q = (x − a)(x − b)), ported into `src/cryptanalysis/smith_route_s.rs`, candidates in its own order (census index of $r_(0)$,
then Q). Protocol filter: \#E0($F_(q^3)$) has a prime factor ℓ ≥ 5 (the largest is used; the old prototype's
\#E0 = 4ℓ / 8ℓ quota rule is recorded beside it). Per instance: descend, make h monic, Props. 2 and 4, then (where they
pass) the planted DLP: G of order ℓ, Q = kG (k from the seed), φ(G), φ(Q) by Cantor (homomorphism and order checked),
C fitted and T1-checked, Ψ(φG), Ψ(φQ) as W1 keys, brute-force search of k′ in ⟨Ψ(φG)⟩, then k′G = Q on E0.

#tbl(columns: 12,
  [*q*], [*candidates*], [*filter (ℓ ≥ 5)*], [*run*], [*not reached (1 h cap)*], [*Props. 2+4 pass*], [*DLP tried*], [*X quartic (T1)*], [*\#$J_(H)$ = \#E0*], [*Ψ(φG) ≠ 0, ℓΨ = 0*], [*k′G = Q verified*], [*k′ = k*],
  ..S.route_s.map(r => ([#r.q], [#r.split_candidates], [#r.filter_instances], [#r.instances_run], [#r.not_reached_time_cap], [#r.prop4_pass],
    [#r.dlp_attempted], [#r.quartic_t1_pass], [#r.n_jac_equals_order_e0], [#r.psi_g_ok], [#r.verified_on_e], [#r.k_equals_planted])).flatten())

#figure(image("reports/route_s_funnel.svg", width: 100%), caption: [T8 per q (`data/route_s_q*.jsonl`, one row per instance with
its skip reason).])

Every DLP attempt whose quotient X is a smooth plane quartic ended with the planted k recovered and verified on E0; the
others are skips with the recorded reason "images lie on a conic: X hyperelliptic" (Smith's quotient is then a
hyperelliptic Jacobian and there is no plane quartic for this route). Props. 2 and 4 pass per instance at 67.9% (q = 7), 82.5%
(11), 78.6% (13), 77.9% (17), 76.8% (19) and 75.8% (23) (table), against 18.6% predicted by Smith's Theorem 2 for a
random genus-3 curve. Not like for like: every H here has cycle type (3,3,1,1) and three rational tractable subgroups,
where a random curve has about one; at q ≥ 17 the instances are the first in order, not a sample. The DLP stage is brute
force on canonical keys; nothing here is an index calculus or a cost comparison with rho (that is W2b).

= T7: cost of ψ per divisor

#tbl(columns: 4, [*fit of $log_2$ y on $log_2$ q (#CF.mul.n points)*], [*slope*], [*95% interval*], [*intercept*],
  [$F_(q)$ multiplications], [#f(CF.mul.slope)], [[#f(CF.mul.slope_lo), #f(CF.mul.slope_hi)]], [#f(CF.mul.intercept)],
  [priced (inversions at measured time ratio)], [#f(CF.priced.slope)], [[#f(CF.priced.slope_lo), #f(CF.priced.slope_hi)]], [#f(CF.priced.intercept)],
  [wall time (ns)], [#f(CF.wall.slope)], [[#f(CF.wall.slope_lo), #f(CF.wall.slope_hi)]], [#f(CF.wall.intercept)])

#figure(image("reports/cost_vs_q.svg", width: 100%), caption: [T7: median over 200 generic degree-3 divisors per q,
q ∈ {7, …, 23, 101, 1009, 10007, 65537, 1000003, $2^31 − 1$} (`data/cost_q*.jsonl`, fit `data/cost_fit.json`).])

The slope is far below 1: consistent with the polylog model the protocol expected; the fit does not establish a polylog
law. Unpriced in these counts: factoring u over $F_(q)$ (`smith_prop24::factor_squarefree`), root finding in
$F_(q^2j)$, Frobenius-orbit grouping and closed-point normalisation (counted only through their field operations),
sorting and hashing. Wall times are from a shared host (load average above 80 during the runs), single thread, nice 10,
not pinned; counts are unaffected by load.

= Deviations from the frozen protocol (recorded; the protocol was not edited)

- JSONL rows are per curve (with per-check tallies) and per Route S instance, not one row per check instance.
- T1(f) at k = 1 only, on the degree-8 scan; T9(ii) for the degree-6 engine not run; T7 wall times not on pinned cores.
- T8 at q ≥ 17 ran under the 1 h cap; "not reached" counts the instances left, and the rows run are the first in the
  construction's order (lowest census index of $r_(0)$ first), not a uniform sample. q = 7, 11, 13 cover every instance
  (q = 13 was rerun with a 2 h cap after its first run hit 1 h at 53 173 of 56 784).
- A defect found during this study was fixed before the recorded runs: `Fld::sqrt`'s non-residue search did not
  terminate in some fields (e.g. q = 17, modulus x² + 3x + 4); it now enumerates field elements from z.

= Files

`src/cryptanalysis/smith_isogeny.rs` (ψ, quartic, both engines' bridge, checks), `src/cryptanalysis/smith_route_s.rs`
(T8), `examples/smith_isogeny.rs` (driver: `verify`, `deg8`, `route-s`, `cost`, `cost-fit`, `summarize`), `data/`,
`reports/plot.py`, `reports/diagram.py`. Depends on W1 (PR \#2227).
