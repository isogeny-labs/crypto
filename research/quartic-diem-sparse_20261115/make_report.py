#!/usr/bin/env python3
"""Builds REPORT.md and REPORT.typ from RESULTS.json (numbers are pulled, not typed). Pure Python.
usage: python3 make_report.py [RESULTS.json]"""
import json, math, re, sys

R = json.load(open(sys.argv[1] if len(sys.argv) > 1 else "RESULTS.json"))
cells = R["cells"]
dist = R["distance"]
fits = R["fits"]
P = R["predictions"]


def cell(q, c=2.0, tag="main"):
    return next((x for x in cells if x["q"] == q and x["c"] == c and x["tag"] == tag and x["n_ok"]), None)


def drow(q, c=2.0, tag="main"):
    return next((x for x in dist if x["q"] == q and x["c"] == c and x["tag"] == tag), None)


def g(v, p=3):
    if v is None:
        return "-"
    if isinstance(v, list):
        return "%.*g [%.*g, %.*g]" % (p, v[0], p, v[1], p, v[2])
    return "%.*g" % (p, v)


def where(f, lo=0.95, hi=1.15):
    if not f:
        return "n/a"
    if f[2] < lo:
        return "below"
    if f[1] > hi:
        return "above"
    if f[1] >= lo and f[2] <= hi:
        return "inside"
    return "straddling the edge of"


def fit(k, p=2):
    f = fits.get(k)
    return "%.*f [%.*f, %.*f]" % (p, f[0], p, f[1], p, f[2]) if f else "n/a"


qs = sorted({x["q"] for x in cells})
qbig = [q for q in qs if q >= 16381]
kap = R["rho_kappa"]
blocks = []


def H(t, lvl=2):
    blocks.append(("h", lvl, t))


def Pp(t):
    blocks.append(("p", t))


def T(head, rows, align=None):
    blocks.append(("t", head, rows))


def Fg(path, cap):
    blocks.append(("f", path, cap))


# ------------------------------------------------------------------ content
blocks.append(("title", "Generic plane quartics: Diem index calculus with a c sqrt(q) factor base, double large primes and sparse linear algebra"))
Pp("2026-11-15. Study `research/quartic-diem-sparse_20261115/`, plan frozen in `PLAN.md` before the first measured run. Local development check at small parameters on one shared 4-CPU VM, not a lab benchmark. Every number is `[pcc:unverified]`. No improvement is claimed; gaps to the matched rho are distances. Repo HEAD at run time `%s`." % R["git"])

H("Question")
Pp("The generic plane-quartic Diem tool (`examples/quartic_ic.rs`, study `tian_n3_quartic_route_20261024`) beats rho when native work is free and loses at 0.1 and 1 GAE per native operation, with dense linear algebra said to grow like q^1.4. The N4 study (`quartic_n4_la_20261021`) reached exponent 1.045 on the GHS-weak family. Does the generic quartic reach the same place when the same pipeline (c q^(1/2) factor base, double large primes, rows of at most four points plus the hyperplane unknown, sparse solver) is built on it?")

H("Answer in brief")
c2 = {q: cell(q) for q in qs}
d2 = {q: drow(q) for q in qs}
b1 = qbig[0] if qbig else qs[-1]
bl = qs[-1]
ok_all = sum(x["n_ok"] for x in cells if x["tag"] == "main" and x["c"] >= 2.0)
n_all = sum(x["n"] for x in cells if x["tag"] == "main" and x["c"] >= 2.0)
Pp("Every IC unit with c >= 2 verified `kG = Q` in the prime-order subgroup (%d of %d primary units; c = 1 never produced a solvable system: the pair graph stays subcritical). The tool is dense because of its **solver** (Gauss-Jordan on a b x b system with b = c sqrt(q), about 0.6 b^3 mod-ell operations), not because of its factor-base size, which is already c sqrt(q). Replacing the solver by Wiedemann does **not** bring the linear algebra to q^1.0: the mean row weight after the large primes are eliminated grows like q^%s, so Wiedemann costs about 6 w b^2 and scales like q^%s (dense: q^%s), and on the measured range the dense solver still uses fewer operations (dense/sparse operation ratio %s at q = %d)." % (ok_all, n_all, fit("wfb_main_c2"), fit("LAops_wied_c2"), fit("LAops_dense_c2"), "%.2f" % next(a["LA_dense_total"] / a["LA_sparse_total"] for a in R["ab"] if a["q"] == max(x["q"] for x in R["ab"])), max(x["q"] for x in R["ab"])))
Pp("Preregistered predictions (details below): P1 (LA share of S below 20%% at q >= 2^14) %s; P2 (exponent in [0.95, 1.15]) %s; P3 (S/rho below 1 at all three prices for q >= 2^14) %s." % tuple("REFUTED" if P[k]["holds"] is False else "holds" for k in ("P1_LA_share", "P2_exponent", "P3_S_over_rho")))
rows = []
for q in qs:
    d = d2[q]
    if not d:
        continue
    rows.append([str(q), g(d["S_over_rho_model_w0"], 2), g(d["S_over_rho_model_wcal"], 2) if "S_over_rho_model_wcal" in d else "-", g(d["S_over_rho_model_wcal10"], 2) if "S_over_rho_model_wcal10" in d else "-",
                 g(d["S_over_rho_model_w0.1"], 3), g(d["S_over_rho_model_w1"], 3), g(d.get("breakeven_w_vs_rho_model"), 2)])
Pp("Distance to the matched rho, c = 2, primary arm (S_IC / S_rho with rho modelled from measured calibration, 95% bootstrap over instances; the same table with measured rho is in `RESULTS.md`):")
T(["q", "w = 0", "w = w_cal", "w = 10 w_cal", "w = 0.1", "w = 1", "break-even w"], rows)
ext = []
for w in ("0.1", "1"):
    fi, fr = fits.get("ic_main_c2_w" + w), fits.get("rho_model_w" + w)
    dd = drow(bl)
    if fi and fr and dd and dd["S_over_rho_model_w" + w] and fi[0] > 0 and fr[0] > fi[0]:
        rr = dd["S_over_rho_model_w" + w][0]
        qs_ = bl * rr ** (1.0 / (fr[0] - fi[0]))
        ext.append("w = %s: ratio %.2f at q = %d, parity at about 2^%.0f" % (w, rr, bl, math.log2(qs_)))
Pp("Extrapolated parity (beyond the measured range, not a measurement; fitted slopes %s for IC and %s for modelled rho at w = 0.1, starting from the ratio at q = %d): %s." % (fit("ic_main_c2_w0.1"), fit("rho_model_w0.1"), bl, "; ".join(ext) if ext else "not reached by the fitted slopes"))
Pp("At zero-priced native work and at the calibrated price of a mod-ell operation (w_cal, about 2e-4 to 5e-4 GAE here) the IC is far below rho; at the band price 0.1 GAE the ratio is %s at q = %d (%s at q = %d) and at 1 GAE it is %s (%s), so the order changes inside the band and by the section 5a rule the ranking is **unresolved**, not a win. The price at which IC and rho tie (break-even w) is the distance to read: about %s GAE per native operation at q = %d. The ratios are not monotone in q because each instance has its own ell (cofactor 3 to 61), which moves its rho by factors of several; three instances per cell cannot average that out." % (g(d2[bl]["S_over_rho_model_w0.1"], 3), bl, g(d2[qs[0]]["S_over_rho_model_w0.1"], 3), qs[0], g(d2[bl]["S_over_rho_model_w1"], 3), "from " + g(d2[qs[0]]["S_over_rho_model_w1"], 3) + " at q = %d" % qs[0], g(d2[bl]["breakeven_w_vs_rho_model"], 2) if d2.get(bl) else "n/a", bl))

H("Why the generic tool is dense (task a)")
Pp("Read from `examples/quartic_ic.rs`, function `run_ic`, and checked by the A/B run below:")
Pp("1. Factor-base size. `b = ceil(bmult/100 * sqrt q)`: the tool already uses a c sqrt(q) base (c = 1 first, +1 after a failure). Not the cause. In this study c = 1 failed at every q (the graph of residual-point pairs is subcritical: about q/4 edges on q vertices) and c = 2 is the smallest base that works.")
Pp("2. Solver. The relations go to `nullspace`, a dense Gauss-Jordan over `Vec<Vec<u64>>` of width b + 1 with all cycle rows (about 2b + 30). It costs about 0.6 b^3 counted mod-ell operations, i.e. q^1.5 at b = c sqrt(q); measured exponent of the dense operation count on q >= 2^12: %s. This is the dense linear algebra of the earlier report." % fit("LAops_dense_c2"))
Pp("3. Relation shape. The tool builds rows from tree-path potentials of the contracted graph, so a row is a sum of path vectors, not a line relation: after the large primes are removed the mean row weight is %s at q = %d and %s at q = %d (raw rows: at most 4 points plus h, weight at most 5). The weight grows like q^%s. Rows are sparse only before the large primes are eliminated, and then the columns number about q." % (g(c2[qs[0]]["w_fb_mean"], 3), qs[0], g(c2[qs[-1]]["w_fb_mean"], 3), qs[-1], fit("wfb_main_c2")))
Pp("4. A bookkeeping flaw found while building: a line through three or four base points is enumerated once per pair of them and gave identical rows, so the tool's stopping count of independent cycles was inflated (in a q = 1021 debugging run 141 counted cycles left 83 nonempty rows after elimination). The new code keeps each line once.")

H("Method")
Pp("Arms (same instance, same group arithmetic, same counters, one process): **IC-sparse**, factor base of b = ceil(c sqrt q) random rational points plus the pinned base point O; pairs of base points in random order; each line gives the two residual points from a quadratic and a square-root table; rows `x_i + x_j + x_X + x_Y - h = 0` with large primes as extra columns; large-prime columns removed by minimum-weight elimination (`diem_quartic::eliminate_lp`); sequential Wiedemann on the normal equations mod ell (`diem_quartic::wiedemann_solve`, 8 held-out rows); large-prime logs by propagation over the edge graph; the base tool's target stage; result checked `kG = Q`. **IC-dense A/B**: the same eliminated rows solved by the base tool's `nullspace`; its kernel must equal the Wiedemann solution (agreement flag, checked on every unit with b <= 800). **rho / rhoneg**: the base tool's distinguished-point rho and its negation-map variant on the same subgroup.")
Pp("Cells: q in {%s}, three instances (curve seeds 7000 to 7002) per q, c in {1, 2, 4} plus an A/A repeat of c = 2 with a second IC seed; primary configuration c = 2. Rho measured at q = 1021 (20+20 runs per instance), 4093 (5+5), 16381 (1+1) and 65521 (plain only: 1 run on instances 0 and 1, 15 on instance 2); above that rho is modelled as kappa sqrt(ell) steps with kappa = %.3f the pooled mean of steps/sqrt(ell) over %d verified plain runs (textbook sqrt(pi/2) = %.3f) and the per-step cost of the plain runs at q = %d (%.3f GAE and %.2f native operations per step). Negation-map rho: kappa %.3f over %d runs but two group operations per step, so plain rho is the cheaper reference and is the one used." % (", ".join(map(str, qs)), kap["pooled_mean_steps_over_sqrt_ell"], kap["n_runs"], kap["ideal_sqrt_pi_over_2"], R["rho_model"]["from_q"], R["rho_model"]["gae_per_step_cI3"], R["rho_model"]["nat_per_step"], kap["neg_pooled"], kap["neg_n"]))
Pp("Cost unit: `S(w) = (M + cI I) / (gM + cI gI) + w (NAT + LA)` in GAE, cI = 3, with M, I the F_q multiplications and inversions of the whole unit, gM, gI those of one group addition measured on the instance, NAT the hash probes, table lookups, union-find steps and elimination entries, LA the mod-ell multiply-adds of elimination, Wiedemann and Berlekamp-Massey. rho uses the same formula. w in {0, 0.1, 1} (the band) and the calibrated w_cal = time of one mod-ell multiply-add / time of one group addition on the same curve and host (best-of-N on the shared VM, not isolated): %s." % "; ".join("q = %d: %.2e" % (c["q"], c["w_cal_mont"]) for c in R["calibration"]))

H("Results")
H("IC-sparse, c = 2 (verified units, means over three instances)", 3)
rows = []
for q in qs:
    c = c2[q]
    if not c:
        continue
    rows.append([str(q), str(c["b"]), "%d/%d" % (c["n_ok"], c["n"]), "%.0f" % c["pairs"], "%.0f" % c["rows_fb"], "%.1f" % c["w_fb_mean"], "%.2e" % c["LA_elim"], "%.2e" % c["LA_wied"],
                 ("%.2e" % c["LA_dense"]) if c.get("LA_dense") is not None else "-", "%.0f" % c["adds_target"], "%.0f" % c["S_w0_cI3"], "%.3g" % c["S_w0.1_cI3"], "%.3g" % c["S_w1_cI3"], "%.3f" % c["LAshare_w0.1"]])
T(["q", "b", "ok", "pairs", "rows", "weight", "LA elim", "LA Wiedemann", "LA dense", "target adds", "S w=0", "S w=0.1", "S w=1", "LA share w=0.1"], rows)
H("Matched rho", 3)
rows = []
for r in R["rho"]:
    rows.append([str(r["q"]), r["kind"], str(r["n"]), "%.0f" % r["mean_steps"], "%.2f" % r["kappa"], "%.3g" % r["S_w0"], "%.3g" % r["S_w0.1"], "%.3g" % r["S_w1"]])
T(["q", "arm", "runs", "mean steps", "steps/sqrt(ell)", "S w=0", "S w=0.1", "S w=1"], rows)
rows = []
for q in qs:
    rm = next((x for x in R["rho_model_by_q"] if x["q"] == q), None)
    if rm:
        rows.append([str(q), "%.3g" % rm["mean_ell"], "%.3g" % rm["S_w0"], "%.3g" % rm["S_w0.1"], "%.3g" % rm["S_w1"]])
Pp("Modelled rho on the instances of the IC cells (mean over the three instances of kappa sqrt(ell) steps at the measured per-step cost):")
T(["q", "mean ell", "S w=0", "S w=0.1", "S w=1"], rows)
rm_ = R["rho_model"]
_small = [r for r in R["rho"] if r["kind"] == "rho" and r["q"] <= 16381]
small_n = sum(r["n"] for r in _small)
small_kappa = sum(r["kappa"] * r["n"] for r in _small) / small_n
rl = R["rho_runs_large_q"]
byq = {}
for x in rl:
    byq.setdefault((x["q"], x["inst"]), []).append(x)
per = "; ".join("q = %d, instance %d: %d run(s), mean %.2f (cofactor %d)" % (q, i, len(v), sum(t["kappa"] for t in v) / len(v), v[0]["cof"]) for (q, i), v in sorted(byq.items()))
first3 = [x for x in rl if x["q"] == rm_["from_q"]][:3]
dsens = {q: drow(q) for q in qs}
sens = "; ".join("q = %d: %s instead of %s" % (q, g(dsens[q]["S_over_rho_kalt_w0.1"], 3), g(dsens[q]["S_over_rho_model_w0.1"], 3)) for q in qs if q >= 65521 and dsens[q])
Pp("**Check of the rho model.** The modelled rho uses the pooled kappa = %.2f (%d verified plain runs). Plain-rho runs at the larger sizes, steps/sqrt(ell) per instance: %s. The first three runs at q = %d (one per instance) were low (%s); %d further runs on instance 2 brought the mean over all %d runs at that size to %.2f, against the pooled %.2f, so the low start was chance and the pooled value is not contradicted at q = %d (it is not tested above that). Sensitivity: with kappa taken at the largest measured q the ratio S_IC/S_rho at w = 0.1 (c = 2, primary arm) is %s. Caveat: at q = %d, %d of the %d runs sit on one instance, so the measured-rho columns at that q rest mostly on one subgroup; above q = %d rho is modelled." % (
    R["rho_kappa"]["pooled_mean_steps_over_sqrt_ell"], R["rho_kappa"]["n_runs"], per, rm_["from_q"], ", ".join("%.2f" % x["kappa"] for x in first3), rm_["kappa_alt_n"] - 3, rm_["kappa_alt_n"], rm_["kappa_alt_largest_measured_q"], R["rho_kappa"]["pooled_mean_steps_over_sqrt_ell"], rm_["from_q"], sens, rm_["from_q"], sum(1 for x in rl if x["q"] == rm_["from_q"] and x["inst"] == 2), rm_["kappa_alt_n"], rm_["from_q"]))
H("Exponents on q >= 2^12 (log-log slope, 95% bootstrap over instances)", 3)
rows = [[k, "%.2f [%.2f, %.2f]" % (v[0], v[1], v[2]), str(v[3])] for k, v in fits.items() if v and (k.startswith("ic_main_c2") or k.startswith("LAops") or k.startswith("wfb") or k.startswith("pairs_main") or k.startswith("relGAE_main") or k.startswith("tgtadds_main"))]
T(["quantity", "slope", "n_q"], rows)
Pp("Reading: the pair-trial count grows like q^%s and the relation-phase M cost like q^%s; the target stage is nearly flat (q^%s, about 150 group additions); S at w = 0 is therefore dominated by the descent at small q (slope %s). At every priced w the cost is the linear algebra (LA share of S >= %.2f at q >= 16381)." % (fit("pairs_main_c2"), fit("relGAE_main_c2"), fit("tgtadds_main_c2"), fit("ic_main_c2_w0"), min(P["P1_LA_share"]["values"].values()) if P["P1_LA_share"]["values"] else float("nan")))
H("Dense A/B on the same rows (c = 2)", 3)
rows = [[str(a["q"]), str(a["b"]), "%.1f" % a["w_fb_mean"], "%.3g" % a["LA_sparse_total"], "%.3g" % a["LA_dense_total"], "%.2f" % (a["LA_dense_total"] / a["LA_sparse_total"])] for a in R["ab"]]
T(["q", "b", "row weight", "sparse LA ops", "dense LA ops", "dense / sparse"], rows)
agree = [x for c in cells for x in c.get("agree", []) if x is not None]
Pp("Dense and Wiedemann kernels agreed on %d of %d units with a dense run. With exponents %s (Wiedemann) against %s (dense) the two operation counts cross where dense/sparse reaches 1, an extrapolation beyond the measured range, near q of the order of 10^6." % (sum(1 for x in agree if x), len(agree), fit("LAops_wied_c2"), fit("LAops_dense_c2")))
H("A/A and controls", 3)
aa = R["aa"]
lo = min(min(a["ratio_w0.1"] for a in aa), 1)
hi = max(max(a["ratio_w0.1"] for a in aa), 1)
lo0 = min(a["ratio_w0"] for a in aa)
hi0 = max(a["ratio_w0"] for a in aa)
Pp("A/A (same instance, two IC seeds of c = 2): S(seed 1)/S(seed 2) lies in [%.2f, %.2f] at w = 0.1 and 1 (%d pairs) and in [%.2f, %.2f] at w = 0, so the IC-seed noise is far smaller than the gaps to rho reported above (the w = 0 spread is the descent, about 100 to 500 additions, which varies with the seed). Re-running the primary c = 2 cells at q = 4093 with a later build of the binary gave bit-identical counters (6 of 6 records). Rho calibration steps/sqrt(ell) = %.2f (criterion in the N4 study: 1.0 to 1.5)." % (lo, hi, len(aa), lo0, hi0, kap["pooled_mean_steps_over_sqrt_ell"]))

H("Preregistered predictions against what happened")
p1 = P["P1_LA_share"]
p2 = P["P2_exponent"]
p3 = P["P3_S_over_rho"]
p3c = P["P3_calibrated_variant"]
Pp("**P1: LA share of S below 20%% at q >= 2^14 (at w = 0.1 and at w = 1): %s.** The share is %s (w = 0.1) and %s (w = 1) at q = %s; at the calibrated price it is %s." % (
    "REFUTED" if p1["holds"] is False else "holds", g(p1["values"].get("q%d_w0.1" % qbig[0]), 3), g(p1["values"].get("q%d_w1" % qbig[0]), 3), qbig[0],
    ", ".join("%.2f at q = %d" % (cell(q)["LAshare_wcal"], q) for q in qbig if cell(q) and "LAshare_wcal" in cell(q))))
Pp("**P2: exponent of S in [0.95, 1.15] on q >= 2^12: %s.** w = 0: %s; w = 0.1 and 1: %s; w_cal: %s; 10 w_cal: %s. At w = 0 the cost is the relation phase plus a nearly constant descent (%s the range); at the band prices the slope is set by the linear algebra (%s the range); at w_cal it is %s the range and at 10 w_cal %s it." % (
    "REFUTED" if p2["holds"] is False else "holds", fit("ic_main_c2_w0"), fit("ic_main_c2_w0.1"), fit("ic_main_c2_wcal"), fit("ic_main_c2_wcal10"),
    where(fits.get("ic_main_c2_w0")), where(fits.get("ic_main_c2_w0.1")), where(fits.get("ic_main_c2_wcal")), where(fits.get("ic_main_c2_wcal10"))))
Pp("**P3: S/rho below 1 at all three prices for every q >= 2^14: %s** (w = 0: %s; w = 0.1: %s; w = 1: %s; rho modelled at q >= 65521 for the headline ratios). Variant at the calibrated price (order at w = 0, w_cal and 10 w_cal): %s." % (
    "REFUTED" if p3["holds"] is False else "holds", "holds" if p3["holds_by_w"]["w0"] else "fails", "holds" if p3["holds_by_w"]["w0.1"] else "fails", "holds" if p3["holds_by_w"]["w1"] else "fails",
    "holds at every q >= 2^14 measured (ranked at calibrated price, the band verdict still governs)" if p3c["holds"] else "does not hold at every q"))
Pp("The forecast written into `PLAN.md` before the runs said P2 would hold and P3 would hold at w = 0.1 but not at w = 1. It was wrong on both: the row weight growth (q^%s) pushed the linear-algebra exponent above 1.15, and rho on the actual subgroups (ell is the largest prime factor of the group order with cofactor at most 64) is several times cheaper than the nominal q^1.5, so at w = 0.1 the IC is above rho up to q = %d (ratio %s) and level with it at q = %d (ratio %s)." % (fit("wfb_main_c2"), qs[-2], g(d2[qs[-2]]["S_over_rho_model_w0.1"], 3), qs[-1], g(d2[qs[-1]]["S_over_rho_model_w0.1"], 3)))

H("Exploratory arms (PLAN Amendment 1, outside P1 to P3)")
rows = []
for q in qs:
    m = cell(q)
    for tag in ("bfs", "light"):
        x = cell(q, 2.0, tag)
        if m and x:
            rows.append([str(q), tag, "%d/%d" % (x["n_ok"], x["n"]), "%.1f" % x["w_fb_mean"], "%.2e" % (x["LA_elim"] + x["LA_wied"] + x["LA_bm"]), "%.2f" % ((x["LA_elim"] + x["LA_wied"] + x["LA_bm"]) / (m["LA_elim"] + m["LA_wied"] + m["LA_bm"])), "%.0f" % x["pairs"], "%.3g" % x["S_w0.1_cI3"], g(drow(q, 2.0, tag)["S_over_rho_model_w0.1"], 3) if drow(q, 2.0, tag) else "-"])
T(["q", "mode", "ok", "row weight", "LA ops", "LA ops / primary", "pairs", "S w=0.1", "S/rho w=0.1"], rows)
below = []
for tag in ("bfs", "light"):
    for q in qs:
        dr = drow(q, 2.0, tag)
        if dr:
            for w in ("0.1", "1"):
                v = dr["S_over_rho_model_w" + w]
                if v and v[0] < 1:
                    below.append("%s at q = %d, w = %s (%s)" % (tag, q, w, g(v, 3)))
expl_sentence = ("S/rho (modelled rho) stays above 1 for both modes at every q and price in the band except: " + "; ".join(below) + ".") if below else "Both stay above rho at w = 0.1 and 1 wherever they were run."
lq = [q for q in qs if cell(q, 2.0, "light") and cell(q)]
light_q = lq[-1] if lq else qs[-1]
if lq:
    xl, xm = cell(light_q, 2.0, "light"), cell(light_q)
    light_ratio = "%.2f" % ((xl["LA_elim"] + xl["LA_wied"] + xl["LA_bm"]) / (xm["LA_elim"] + xm["LA_wied"] + xm["LA_bm"]))
else:
    light_ratio = "n/a"

Pp("`bfs` takes breadth-first potentials from the base points as rows (no elimination); `light` collects 6b cycles and hands the lightest rows to Wiedemann after a coverage guard. %s `light` needs %s of the primary linear-algebra operations at q = %d (c = 2); the pre-written expectation was about 0.5. Weight exponents: primary q^%s, bfs q^%s, light q^%s." % (expl_sentence, light_ratio, light_q, fit("wfb_main_c2"), fit("wfb_bfs_c2"), fit("wfb_light_c2")))

H("Limitations and deviations from the task text")
Pp("* Sizes: q up to 2^17; three instances per cell, so intervals are wide and the ell of each instance (cofactor up to 64) moves the rho reference by a factor of several; the q = 131071 instances were run in parallel processes (see `run_q131071_parallel.sh`).\n* Rho is measured to q = 65521 only (1 run per instance at 16381; 17 runs at 65521, 15 of them on one instance); at q = 131071 it is modelled with the pooled kappa, and the model is also used for the headline ratios at 65521 (ratios against measured rho are in `RESULTS.md`).\n* The N4 graph code (`research/quartic_n4_la_20261021/ic.rs`) is tied to the GHS instance type and was not reused; the graph, breadth-first potentials and elimination here follow the base tool and `diem_quartic.rs`. The N4 rho records and tool do not apply to generic quartics (GHS instance type), so rho is the base tool's. The Pic^0 arithmetic is the base tool's two-cubic Mumford-like arithmetic (`examples/quartic_ic.rs`), the same for IC and rho, not `plane_quartic_pic.rs`.\n* Wiedemann is the sequential solver on the normal equations of `diem_quartic.rs`, not block Wiedemann; block Wiedemann does not change the operation count.\n* LA mod-ell operations are priced at w like other native operations (the convention of the Tian study and of this task). The N4 study prices them at a measured field-multiplication equivalent instead; the calibrated rows above bridge the two conventions (AGENTS.md section 5a, 2026-11-12).\n* w_cal is best-of-N on a shared VM; q = 131071 uses the value of the largest calibrated q. Hash probes are priced at the same weight as mod-ell operations.\n* Generic random smooth quartics with a prime-order subgroup of the Jacobian over F_q; no claim about structured, deployed or extension-field targets.")

H("Reproduce")
Pp("`cd research/quartic-diem-sparse_20261115 && cargo build --release -j2 --bin quartic_diem_sparse --target-dir ../../target && QDS_GIT=$(git rev-parse HEAD) ./run_ladder.sh ../../target/release/quartic_diem_sparse results/raw && QDS_GIT=$(git rev-parse HEAD) ./run_explore.sh ../../target/release/quartic_diem_sparse results/raw_explore && python3 summarize.py results . && python3 plot.py && python3 make_report.py` (smallest cell: `QDS_GIT=x ./quartic_diem_sparse run out.jsonl 4093 3 7 200 none 800`, a few seconds; the calibration is `quartic_diem_sparse calib <q> 7000`). At q = 131071 the instances ran in parallel processes (`run_q131071_parallel.sh`, `QDS_INST0`); the extra rho runs are `QDS_INST0=2 QDS_RHO_RUN0=1 ./quartic_diem_sparse run results/raw/rho_q65521_inst2_more.jsonl 65521 1 7 - plain 0 14` and one run per instance at 65521 with `QDS_INST0=i ... 65521 1 7 - plain 0 1`; the `light` arm at 131071 ran per instance with `QDS_ROWMODE=light QDS_LIGHTMULT=6 QDS_INST0=i`. Binary hashes are in `results/binary_sha256.txt` (several builds; counters of the primary cells are bit-identical across them where re-run). Unit tests: `cargo test --release --bin quartic_diem_sparse sparse_tests`.")

H("Sources read")
Pp("`research/quartic_n4_la_20261021/README.md`, `PROTOCOL.md`, `REPORT.typ` (head), `solver.rs`, `n4.rs`, `common.rs`, `summ.rs` (signatures and pricing lines; `ic.rs` not read in full); `research/tian_n3_quartic_route_20261024/REPORT.md` (items a and b), `PROTOCOL.md` (head); `examples/quartic_ic.rs` (group law, `run_ic`, `run_rho`, `build_instance`, `cmd_run`, `cmd_summarize`); `src/cryptanalysis/diem_quartic.rs` (header, sparse stage: `build_rows`, `eliminate_lp`, `wiedemann_solve`, cost accounting); `src/cryptanalysis/plane_quartic_pic.rs`, `pq_wiedemann.rs`, `large_prime_filter.rs`, `src/bin/quartic_ic.rs` (headers and signatures only; not used); `AGENTS.md` section 5a.")

Fg("figure.svg", "Cost, distance to rho, linear-algebra operations and row weight against q (c = 2). Dashed cost curves: rho, measured to q = 65521 and modelled at 131071.")


# ------------------------------------------------------------------ emitters
def esc_typ_plain(t):
    t = t.replace("\\", "\\\\")
    for ch in "#$@*_<>[]~`":
        t = t.replace(ch, "\\" + ch)
    return t


def typ_inline(t):
    parts = re.split(r"(`[^`]*`)", t)
    out = []
    for p_ in parts:
        if p_.startswith("`") and p_.endswith("`") and len(p_) > 1:
            out.append("`" + p_[1:-1] + "`")
        else:
            seg = re.split(r"(\*\*[^*]+\*\*)", p_)
            for s_ in seg:
                if s_.startswith("**") and s_.endswith("**") and len(s_) > 4:
                    out.append("*" + esc_typ_plain(s_[2:-2]) + "*")
                else:
                    out.append(esc_typ_plain(s_))
    return "".join(out)


md, ty = [], []
ty.append('#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n#set heading(numbering: none)\n#show raw: set text(size: 8.5pt)')
for b in blocks:
    k = b[0]
    if k == "title":
        md.append("# " + b[1] + "\n")
        ty.append("= " + typ_inline(b[1]))
    elif k == "h":
        md.append("#" * b[1] + " " + b[2] + "\n")
        ty.append("=" * b[1] + " " + typ_inline(b[2]))
    elif k == "p":
        md.append(b[1] + "\n")
        for para in b[1].split("\n"):
            if para.startswith("* "):
                ty.append("- " + typ_inline(para[2:]))
            else:
                ty.append(typ_inline(para) + "\n")
    elif k == "t":
        head, rows = b[1], b[2]
        md.append("| " + " | ".join(head) + " |\n|" + "|".join(["--:" if i else ":--" for i in range(len(head))]) + "|\n" + "\n".join("| " + " | ".join(r) + " |" for r in rows) + "\n")
        cells_ = "".join("[%s]," % typ_inline(h) for h in head) + "\n" + "\n".join("".join("[%s]," % typ_inline(c) for c in r) for r in rows)
        ty.append("#table(columns: %d, align: right, inset: 3pt, stroke: 0.4pt + gray,\n%s\n)\n" % (len(head), cells_))
    elif k == "f":
        md.append("![%s](%s)\n" % (b[2], b[1]))
        ty.append('#figure(image("%s", width: 100%%), caption: [%s])\n' % (b[1], typ_inline(b[2])))
open("REPORT.md", "w").write("\n".join(md))
open("REPORT.typ", "w").write("\n".join(ty))
print("wrote REPORT.md, REPORT.typ")
