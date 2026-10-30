#!/usr/bin/env python3
"""Validator recompute for benchmarks/20261018-k4-gls-v4 (E25: the GLS fold on the V4 line).

Independent of the producing session: stdlib only, reads the committed raw run records
experiments/30_gls_v4/p*s*.json (both arms per file) and experiments/30_gls_v4/stage_a.json,
recomputes every number the README row (benchmarks/README.md, the 20261018-k4-gls-v4 row), the note
research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.16 and the study
research/glv_gls_v4_20261018/ (README.md, REPORT.typ, summary.json) state for E25, and writes
validator-report.json next to this script.  E26 (the addendum in the study) is not part of this row.

Tolerance rule: a claim printed with k digits after the decimal point of its mantissa (or as an
integer) agrees when |claimed - recomputed| <= half a unit in its last printed digit; rel_tol is that
half unit divided by max(|claimed|, |recomputed|), which is exactly the gate's comparison.  Values
copied from summary.json are machine output and must agree to rel_tol 1e-9.

    python3 benchmarks/20261018-k4-gls-v4/validate.py
"""
import glob, hashlib, json, math, os, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.environ.get("VALIDATE_REPO") or os.path.normpath(os.path.join(HERE, "..", ".."))
E25_DIR = "experiments/30_gls_v4"
STUDY = "research/glv_gls_v4_20261018"
NOTE = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.16"
R = "benchmarks/README.md row 20261018-k4-gls-v4"
SR, ST, SJ = STUDY + "/README.md", STUDY + "/REPORT.typ", STUDY + "/summary.json"


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


files = sorted(glob.glob(os.path.join(REPO, E25_DIR, "p*s*.json")))
ROWS = []
for f in files:
    ROWS += json.load(open(f))
STAGE_A_PATH = os.path.join(REPO, E25_DIR, "stage_a.json")
STAGE_A = json.load(open(STAGE_A_PATH))
A = {(r["p"], r["seed"]): r for r in ROWS if r["arm"] == "A"}
B = {(r["p"], r["seed"]): r for r in ROWS if r["arm"] == "B"}
KEYS = sorted(B, key=lambda k: (B[k]["bits"], k[1]))


def y(r):
    return r["residuals"] / r["folded"]["relations"]


def fit(points):
    pts = [(math.log2(x), math.log2(v)) for x, v in points]
    mx = sum(a for a, _ in pts) / len(pts)
    my = sum(b for _, b in pts) / len(pts)
    return sum((a - mx) * (b - my) for a, b in pts) / sum((a - mx) ** 2 for a, _ in pts)


def handover(rows):
    """Section 11.16's model as the note states it: p* = y (cols/p) C sqrt(h) / (S_rho (1 - r)),
    n* = p*^4 / h; y pooled, the rest means over curves; S_rho = matched rho / sqrt(n), r = LA / rho."""
    m = lambda f: sum(f(r) for r in rows) / len(rows)
    yy = sum(r["residuals"] for r in rows) / sum(r["folded"]["relations"] for r in rows)
    cols_p = m(lambda r: r["folded"]["columns"] / r["p"])
    c = m(lambda r: r["call"]["mean"])
    h = m(lambda r: r["cofactor"])
    s_rho = m(lambda r: r["rho_matched_muls"] / math.sqrt(r["n"]))
    r_la = m(lambda r: r["la_fp_muls"] / r["rho_matched_muls"])
    ps = yy * cols_p * c * math.sqrt(h) / (s_rho * (1 - r_la))
    return dict(y=yy, cols_p=cols_p, c=c, h=h, s_rho=s_rho, r=r_la, log2n=math.log2(ps ** 4 / h))


def ic_at(r, g):
    return r["total_muls"] + g * r["fp_muls_per_add"] * r["macaulay_entries"]


def walk_at(r, g, kind):
    """The walk's F_p multiplications plus g GAE per step; kind 'matched' (<±psi>) or 'negation'."""
    muls = r["rho_matched_muls"] if kind == "matched" else r["rho_negation_muls"]
    steps = r["rho_matched_steps"] if kind == "matched" else r["rho_negation_steps"]
    return muls + g * r["fp_muls_per_add"] * steps


def improvement(k, g=0.0):
    ra, rb = A[k], B[k]
    return (ic_at(ra, g) / walk_at(ra, g, "negation")) / (ic_at(rb, g) / walk_at(rb, g, "matched"))


sab = {k: A[k]["total_muls"] / B[k]["total_muls"] for k in KEYS}
imp = {k: improvement(k) for k in KEYS}
steps = {k: B[k]["rho_negation_steps"] / B[k]["rho_matched_steps"] for k in KEYS}
yrat = {k: y(B[k]) / y(A[k]) for k in KEYS}
HA, HB = handover(list(A.values())), handover(list(B.values()))
pmax = max(k[0] for k in KEYS)
HAL, HBL = handover([A[k] for k in KEYS if k[0] == pmax]), handover([B[k] for k in KEYS if k[0] == pmax])
pyA = sum(r["residuals"] for r in A.values()) / sum(r["folded"]["relations"] for r in A.values())
pyB = sum(r["residuals"] for r in B.values()) / sum(r["folded"]["relations"] for r in B.values())

HEAD, EXPL, BOUNDS = [], [], []


def parse(printed):
    s = printed.replace(",", "").replace("−", "-")
    mant, _, ex = s.partition("e")
    dec = len(mant.split(".")[1]) if "." in mant else 0
    return float(s), 0.5 * 10 ** ((int(ex) if ex else 0) - dec)


N19 = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.19"
ERRATUM = "additive erratum, note §9.19 (2026-10-21); the earlier text is left as written and §9.19 is the record"
# Claims the note's §9.19 corrected: name -> (the corrected value as printed there, its bound or None).
# The claim is checked against the corrected value; the value as first written is kept under "superseded".
ERRATA = {
    "G2.36 <±psi> walk steps below n ~ 2^25, most": ("4057", None),
}


def _agree(c, half, value, bound):
    if bound == "le":
        return value <= c + half * (1 + 1e-12)
    if bound == "ge":
        return value >= c - half * (1 + 1e-12)
    return abs(c - value) <= half * (1 + 1e-12)


def claim(name, where, printed, value, definition, exact=False, cls="headline", bound=None):
    """cls: 'headline' (a reported result; decides the status) or 'explanatory' (a remark inside an
    explanation; checked and reported, does not decide the status).  bound: None (equality at printed
    precision), 'le' (the claim says value <= printed) or 'ge'."""
    superseded = None
    if name in ERRATA:
        oc, ohalf = parse(printed)
        superseded = {"printed": printed, "source": where, "bound": bound, "agrees_with_raw_data": _agree(oc, ohalf, value, bound),
                      "superseded_by": ERRATUM}
        printed, bound = ERRATA[name]
        where = N19 + " (supersedes " + where + ")"
    c, half = parse(printed)
    if exact:
        half = 1e-9 * max(abs(c), abs(value))
    scale = max(abs(c), abs(value), 1e-300)
    agree = _agree(c, half, value, bound)
    item = {"name": name, "source": where, "printed": printed, "claimed": c, "recomputed": value,
            "rel_tol": 1e-9 if exact else half / scale * (1 + 1e-12), "agrees": agree, "definition": definition}
    if superseded:
        item["superseded"] = superseded
    if bound:
        item["bound"] = {"le": "recomputed <= claimed (+ half a printed unit)", "ge": "recomputed >= claimed (- half a printed unit)"}[bound]
        item.pop("rel_tol")
    item["class"] = cls
    (EXPL if cls == "explanatory" else BOUNDS if bound else HEAD).append(item)


hi = [k for k in KEYS if k[0] >= 197]
sbr = [B[k]["ratio"] for k in KEYS]
# README row -------------------------------------------------------------------
claim("curves", R, "10", len(KEYS), "curves with both arms")
claim("smallest p", R, "101", min(k[0] for k in KEYS), "min p")
claim("largest p", R, "397", max(k[0] for k in KEYS), "max p")
claim("h, least over arms and curves", R, "16", min(r["cofactor"] for r in ROWS), "min cofactor over all 20 records")
claim("h, most over arms and curves", R, "16", max(r["cofactor"] for r in ROWS), "max cofactor over all 20 records")
for fold in ("Gls", "Negation", "None"):
    claim(f"{fold} walks per curve and arm, least", R, "16", min(sum(w["fold"] == fold for w in r["rho"]) for r in ROWS), f"min over records of the fold = {fold} walk count")
    claim(f"{fold} walks per curve and arm, most", R, "16", max(sum(w["fold"] == fold for w in r["rho"]) for r in ROWS), f"max over records")
claim("points per column, arm A, least", R, "4", min(A[k]["points_per_column"] for k in KEYS), "min over curves of arm A points_per_column")
claim("points per column, arm A, most", R, "4", max(A[k]["points_per_column"] for k in KEYS), "max over curves")
claim("points per column, arm B, least", R, "8", min(B[k]["points_per_column"] for k in KEYS), "min over curves of arm B points_per_column")
claim("points per column, arm B, most", R, "8", max(B[k]["points_per_column"] for k in KEYS), "max over curves")
claim("S_A/S_B least", R, "1.35", min(sab.values()), "min over curves of arm A total_muls / arm B total_muls")
claim("S_A/S_B most", R, "2.67", max(sab.values()), "max over curves")
claim("S_A/S_B median", R, "1.86", statistics.median(sab.values()), "median over the 10 curves")
claim("improvement at p >= 197, least", R, "1.26", min(imp[k] for k in hi), "(S_A / rho_negation) / (S_B / rho_<±psi>), min over the six curves at p >= 197")
claim("improvement at p >= 197, most", R, "1.97", max(imp[k] for k in hi), "max over the six curves at p >= 197")
claim("S_B/rho least", R, "1.1e4", min(sbr), "min over curves of arm B total_muls / mean <±psi> walk fp_muls")
claim("S_B/rho most", R, "2.7e5", max(sbr), "max over curves")
claim("handover arm A log2 n*", R, "93.2", HA["log2n"], "section 11.16 model from arm A's constants against the <±psi> walk, every curve pooled")
claim("handover arm B log2 n*", R, "88.0", HB["log2n"], "same from arm B's constants")
# Note section 9.16: Stage A -------------------------------------------------------
ok_rows = [r for r in STAGE_A if "error" not in r]
nonempty = [r for r in ok_rows if r["base_points"] > 0]
with_logs = [r for r in ok_rows if r.get("distinct_column_logs") is not None]
outside = [r for r in ok_rows if not (0.3 <= r["density"] <= 0.7)]
claim("Stage A curves", NOTE + " Stage A", "56", len(ok_rows), "stage_a.json rows without an error")
claim("Stage A points per column under <V4, psi>, least (non-empty bases)", NOTE + " Stage A", "8.00", min(r["points_per_column_psi"] for r in nonempty), "min over non-empty bases")
claim("Stage A points per column under <V4, psi>, most (non-empty bases)", NOTE + " Stage A", "8.00", max(r["points_per_column_psi"] for r in nonempty), "max over non-empty bases")
claim("Stage A points per column under V4, least", NOTE + " Stage A", "4.00", min(r["points_per_column_v4"] for r in nonempty), "min over non-empty bases")
claim("Stage A points per column under V4, most", NOTE + " Stage A", "4.00", max(r["points_per_column_v4"] for r in nonempty), "max over non-empty bases")
claim("Stage A curves with nu^2 not in F_p", NOTE + " Stage A", "56", sum(not r["nu2_in_fp"] for r in ok_rows), "count of nu2_in_fp = false")
claim("Stage A largest p with logarithms computed", NOTE + " Stage A", "149", max(r["p"] for r in with_logs), "max p where distinct_column_logs is recorded")
claim("Stage A density pooled", NOTE + " Stage A and " + ST, "0.49", sum(r["base_points"] for r in ok_rows) / sum(r["eligible"] for r in ok_rows), "sum base_points / sum eligible over the 56 curves")
claim("Stage A density pooled at p >= 149", NOTE + " Stage A", "0.52", sum(r["base_points"] for r in ok_rows if r["p"] >= 149) / sum(r["eligible"] for r in ok_rows if r["p"] >= 149), "sum base_points / sum eligible over curves at p >= 149")
claim("Stage A density per curve, least", NOTE + " Stage A and " + ST, "0", min(r["density"] for r in ok_rows), "min density")
claim("Stage A density per curve, most", NOTE + " Stage A and " + ST, "0.75", max(r["density"] for r in ok_rows), "max density")
claim("Stage A curves outside [0.3, 0.7]", NOTE + " Stage A and " + ST, "14", len(outside), "count of curves with density outside [0.3, 0.7]")
claim("Stage A outside curves at p > 113", NOTE + " Stage A", "1", sum(r["p"] > 113 for r in outside), "'all but one at p <= 113'")
claim("Stage A curves with no base", NOTE + " Stage A", "2", sum(r["base_points"] == 0 for r in ok_rows), "count of base_points = 0 (both at p = 41)")
orb = [r["eligible"] / 8 for r in ok_rows if r["p"] <= 113]
claim("Stage A eligible orbits of 8 at p <= 113, least", NOTE + " Stage A", "3", min(orb), "min over the p <= 113 curves of eligible / 8: the sentence 'where a base is 3-13 orbits of 8 points that lift together' read as the candidate line holding 3-13 orbits (the literal reading, base_points / 8, gives 0-9 and does not match)", cls="explanatory")
claim("Stage A eligible orbits of 8 at p <= 113, most", NOTE + " Stage A", "13", max(orb), "max over the p <= 113 curves of eligible / 8", cls="explanatory")
claim("Stage A h, least", NOTE + " Stage A", "16", min(r["cofactor"] for r in ok_rows), "min cofactor")
claim("Stage A h, most", NOTE + " Stage A", "16", max(r["cofactor"] for r in ok_rows), "max cofactor")
# Note section 9.16: end to end -------------------------------------------------------
claim("walks, all correct", NOTE + " and " + ST, "960", sum(len(r["rho"]) for r in ROWS) if all(w["correct"] for r in ROWS for w in r["rho"]) else -1, "walks in the 20 records, every one correct (else -1)")
claim("unsolved targets", NOTE, "198", sum(r["unsolved"] for r in ROWS), "sum unsolved over both arms")
claim("targets", NOTE, "1.15e6", sum(r["residuals"] for r in ROWS), "sum targets over both arms")
claim("unsolved share, percent", NOTE, "0.017", 100 * sum(r["unsolved"] for r in ROWS) / sum(r["residuals"] for r in ROWS), "pooled")
claim("C median, smallest", NOTE, "1.62e6", min(r["call"]["median"] for r in ROWS), "min over the 20 records")
claim("C median, largest", NOTE, "1.66e6", max(r["call"]["median"] for r in ROWS), "max over the 20 records")
TAB = [  # p, seed, log2 n, cols A, cols B, y A, y B, S_A, S_B, S_A/S_B, neg/psi steps, S_A/rho neg, S_B/rho psi, improvement
    (101, 1, "22.6", "10", "5", "3731", "3528", "6.76e10", "3.48e10", "1.94", "1.03", "1.53e5", "6.77e4", "2.26"),
    (101, 2, "22.6", "14", "7", "1001", "888", "2.47e10", "1.17e10", "2.11", "1.49", "6.02e4", "2.93e4", "2.05"),
    (149, 1, "24.9", "10", "5", "32013", "22006", "5.83e11", "2.19e11", "2.67", "1.09", "8.31e5", "2.65e5", "3.13"),
    (149, 2, "24.9", "14", "7", "4039", "4272", "1.01e11", "5.67e10", "1.77", "0.96", "1.58e5", "7.01e4", "2.25"),
    (197, 1, "26.5", "22", "11", "2218", "3150", "8.53e10", "6.32e10", "1.35", "1.52", "7.71e4", "6.11e4", "1.26"),
    (197, 2, "26.5", "30", "15", "846", "969", "4.39e10", "2.59e10", "1.69", "1.62", "3.97e4", "2.62e4", "1.51"),
    (269, 1, "28.3", "30", "15", "2645", "2950", "1.37e11", "7.88e10", "1.74", "1.61", "7.45e4", "4.84e4", "1.54"),
    (269, 2, "28.3", "40", "20", "744", "718", "5.10e10", "2.52e10", "2.02", "2.08", "2.24e4", "1.56e4", "1.44"),
    (397, 1, "30.5", "60", "30", "700", "800", "7.19e10", "4.17e10", "1.72", "1.36", "1.99e4", "1.13e4", "1.76"),
    (397, 2, "30.5", "48", "24", "1600", "1349", "1.31e11", "5.65e10", "2.32", "1.67", "3.38e4", "1.71e4", "1.97"),
]
for p, s, *v in TAB:
    k = (p, s)
    vals = (B[k]["bits"], A[k]["folded"]["columns"], B[k]["folded"]["columns"], y(A[k]), y(B[k]), A[k]["total_muls"], B[k]["total_muls"],
            sab[k], steps[k], A[k]["total_muls"] / A[k]["rho_negation_muls"], B[k]["ratio"], imp[k])
    names = ("log2 n", "cols A", "cols B", "y A", "y B", "S_A", "S_B", "S_A/S_B", "neg/psi steps", "S_A/rho neg", "S_B/rho psi", "improvement")
    for nm, pv, val in zip(names, v, vals):
        claim(f"p={p} seed={s} {nm}", NOTE + " table" + (" and " + ST + " table" if nm in ("log2 n", "S_A/S_B", "neg/psi steps", "S_A/rho neg", "S_B/rho psi", "improvement") else ""), pv, val, f"record p{p}s{s}: {nm}")
for f, v in (("y", "2474"), ("cols_p", "0.060"), ("c", "1.66e6"), ("h", "16"), ("s_rho", "121.7"), ("r", "0.045"), ("log2n", "88.0")):
    claim(f"handover arm B constant {f} (all curves)", NOTE + " G2.37", v, HB[f], "section 11.16 model, arm B, every curve pooled")
claim("handover arm B log2 n*, p=397 only", NOTE + " G2.37 and " + ST, "85.7", HBL["log2n"], "the two p = 397 curves")
claim("handover arm A log2 n* (all curves)", NOTE, "93.2", HA["log2n"], "arm A against the <±psi> walk, every curve pooled")
claim("handover arm A log2 n*, p=397 only", NOTE, "91.1", HAL["log2n"], "the two p = 397 curves")
claim("slope of S/rho in n, arm B", NOTE + " and " + ST, "-0.29", fit([(B[k]["n"], B[k]["ratio"]) for k in KEYS]), "least-squares slope of log2(S_B/rho_psi) on log2 n")
claim("slope of S/rho in n, arm A", NOTE + " and " + ST, "-0.30", fit([(A[k]["n"], A[k]["ratio"]) for k in KEYS]), "least-squares slope of log2(S_A/rho_psi) on log2 n (the tables tool's arm A ratio)")
claim("G2.33 cols A/B, least", NOTE + " G2.33", "2.00", min(A[k]["folded"]["columns"] / B[k]["folded"]["columns"] for k in KEYS), "min over curves")
claim("G2.33 cols A/B, most", NOTE + " G2.33", "2.00", max(A[k]["folded"]["columns"] / B[k]["folded"]["columns"] for k in KEYS), "max over curves")
claim("G2.34 y_B/y_A, least", NOTE + " G2.34 and " + ST, "0.69", min(yrat.values()), "min over curves")
claim("G2.34 y_B/y_A, most", NOTE + " G2.34 and " + ST, "1.42", max(yrat.values()), "max over curves")
claim("G2.34 curves outside [0.8, 1.25]", NOTE + " G2.34", "2", sum(not (0.8 <= v <= 1.25) for v in yrat.values()), "count")
claim("G2.34 pooled y B", NOTE + " G2.34", "2474", pyB, "pooled over arm B")
claim("G2.34 pooled y A", NOTE + " G2.34", "2707", pyA, "pooled over arm A")
claim("G2.34 pooled y B / y A", NOTE + " G2.34 and " + ST, "0.91", pyB / pyA, "ratio of pooled yields")
claim("G2.35 S_A/S_B least", NOTE + " G2.35 and " + ST, "1.35", min(sab.values()), "min")
claim("G2.35 S_A/S_B most", NOTE + " G2.35 and " + ST, "2.67", max(sab.values()), "max")
claim("G2.35 S_A/S_B median", NOTE + " G2.35 and " + ST, "1.86", statistics.median(sab.values()), "median")
claim("G2.35 curves in [1.6, 2.4]", NOTE + " G2.35 and " + ST, "8", sum(1.6 <= v <= 2.4 for v in sab.values()), "count of 10")
claim("G2.36 neg/psi steps, mean", NOTE + " G2.36 and " + ST, "1.44", statistics.mean(steps.values()), "mean over curves of negation steps / <±psi> steps (16-walk means)")
claim("G2.36 improvement, least", NOTE + " G2.36", "1.26", min(imp.values()), "min over the 10 curves")
claim("G2.36 improvement, most", NOTE + " G2.36", "3.13", max(imp.values()), "max over the 10 curves")
claim("G2.36 improvement, median", NOTE + " G2.36", "1.87", statistics.median(imp.values()), "median over the 10 curves")
claim("G2.36 improvement in [1.1, 2.0] at p >= 197 (count)", NOTE + " G2.36", "6", sum(1.1 <= imp[k] <= 2.0 for k in hi), "count of six")
claim("G2.36 improvement above 2.0 at p <= 149 (count)", NOTE + " G2.36", "4", sum(imp[k] > 2.0 for k in KEYS if k[0] <= 149), "count of four")
claim("G2.36 <±psi> walk F_p mults at p=101 seed 1", NOTE + " G2.36", "5.1e5", B[(101, 1)]["rho_matched_muls"], "mean of 16 <±psi> walks; printed '5.1 against 4.4e5'", cls="explanatory")
claim("G2.36 negation walk F_p mults at p=101 seed 1", NOTE + " G2.36", "4.4e5", B[(101, 1)]["rho_negation_muls"], "mean of 16 negation walks", cls="explanatory")
claim("G2.36 <±psi> walk steps below n ~ 2^25, most", NOTE + " G2.36", "3000", max(B[k]["rho_matched_steps"] for k in KEYS if k[0] <= 149), "max over the curves below n = 2^25 (p = 101 and 149) of the mean <±psi> walk steps; the remark 'below n ~ 2^25 a walk is <= 3,000 steps' (it holds at p = 101: 2,007 and 1,192; at p = 149, n = 2^24.9, the means are 4,057 and 3,965)", cls="explanatory", bound="le")
# Note §9.19 (2026-10-21): the corrected G2.36 remark and the limitations it records for E25 --------
for k, v in (((149, 1), "4057"), ((149, 2), "3965"), ((101, 1), "2007"), ((101, 2), "1192")):
    claim(f"§9.19: <±psi> walk mean steps p={k[0]} seed={k[1]}", N19, v, B[k]["rho_matched_steps"], "mean steps of the 16 <±psi> walks", cls="explanatory")
claim("§9.19: log2 n at p=149", N19, "24.9", B[(149, 1)]["bits"], "bits of the p = 149 curves", cls="explanatory")
claim("§9.19: log2 n at p=101", N19, "22.6", B[(101, 1)]["bits"], "bits of the p = 101 curves", cls="explanatory")
_ps = [B[k]["rho_matched_muls"] / B[k]["rho_matched_steps"] for k in KEYS if k[0] == 397]
_ng = [B[k]["rho_negation_muls"] / B[k]["rho_negation_steps"] for k in KEYS if k[0] == 397]
claim("§9.19: <±psi> walk F_p mults per step at p=397, least", N19, "152", min(_ps), "min over the two p = 397 curves of mean fp_muls / mean steps (set-up included)", cls="explanatory")
claim("§9.19: <±psi> walk F_p mults per step at p=397, most", N19, "154", max(_ps), "max over the two p = 397 curves", cls="explanatory")
claim("§9.19: negation walk F_p mults per step at p=397", N19, "109", statistics.mean(_ng), "mean over the two p = 397 curves (109.8 and 108.9 individually)", cls="explanatory")
claim("§9.19: improvement above 2.0 at p <= 149 (verdict unchanged)", N19, "4", sum(imp[k] > 2.0 for k in KEYS if k[0] <= 149), "count of four", cls="explanatory")
claim("best S_B/rho (p=397 seed 1)", NOTE + " and " + ST, "1.13e4", B[(397, 1)]["ratio"], "S_B / rho_psi at p397s1, the minimum")
claim("largest log2 n (scope)", NOTE + " and " + ST, "30.5", max(B[k]["bits"] for k in KEYS), "max bits")
# summary.json (machine output, exact) ------------------------------------------------
S = json.load(open(os.path.join(REPO, SJ)))
claim("summary curves", SJ, str(S["curves"]), len(KEYS), "count", exact=True)
claim("summary handover_log2_n_a", SJ, repr(S["handover_log2_n_a"]), HA["log2n"], "arm A handover", exact=True)
claim("summary handover_log2_n_b", SJ, repr(S["handover_log2_n_b"]), HB["log2n"], "arm B handover", exact=True)
for sr in S["rows"]:
    k = (sr["p"], sr["seed"])
    claim(f"summary p={k[0]} seed={k[1]} s_a_over_s_b", SJ, repr(sr["s_a_over_s_b"]), sab[k], "exact", exact=True)
    claim(f"summary p={k[0]} seed={k[1]} s_a_over_rho_negation", SJ, repr(sr["s_a_over_rho_negation"]), A[k]["total_muls"] / A[k]["rho_negation_muls"], "exact", exact=True)
    claim(f"summary p={k[0]} seed={k[1]} s_b_over_rho_psi", SJ, repr(sr["s_b_over_rho_psi"]), B[k]["ratio"], "exact", exact=True)
    claim(f"summary p={k[0]} seed={k[1]} negation_over_psi_steps", SJ, repr(sr["negation_over_psi_steps"]), steps[k], "exact", exact=True)
    claim(f"summary p={k[0]} seed={k[1]} log2_n", SJ, repr(sr["log2_n"]), B[k]["bits"], "exact", exact=True)

# ---------------------------------------------------------------- non-numeric checks
G = ("0", "0.1", "1")
checks = {
    "every record: folded.solved and folded.correct (log equals planted d), both arms": all(r["folded"]["solved"] and r["folded"]["correct"] for r in ROWS),
    "every walk correct": all(w["correct"] for r in ROWS for w in r["rho"]),
    "check_failures = 0, rows_readd_failures = 0, off_base = 0 on every record": all(r["check_failures"] == 0 and r["rows_readd_failures"] == 0 and r["off_base"] == 0 for r in ROWS),
    "rows_readded = folded.relations on every record": all(r["rows_readded"] == r["folded"]["relations"] for r in ROWS),
    "both arms of a curve share the instance (n, #E, h, base points)": all(A[k][f] == B[k][f] for k in KEYS for f in ("n", "group_order", "cofactor", "base_points")),
    "both arms of a curve ran identical walks (same seeds and trajectories)": all(A[k]["rho"] == B[k]["rho"] for k in KEYS),
    "matched_fold = Gls on every record": all(r.get("matched_fold") == "Gls" for r in ROWS),
    "rho_matched_muls = mean fp_muls of the 16 Gls walks on every record": all(abs(r["rho_matched_muls"] - statistics.mean(w["fp_muls"] for w in r["rho"] if w["fold"] == "Gls")) <= 1e-9 * r["rho_matched_muls"] for r in ROWS),
    "rho_negation_muls = mean fp_muls of the 16 negation walks on every record": all(abs(r["rho_negation_muls"] - statistics.mean(w["fp_muls"] for w in r["rho"] if w["fold"] == "Negation")) <= 1e-9 * r["rho_negation_muls"] for r in ROWS),
    "ratio = total_muls / rho_matched_muls on every record": all(abs(r["ratio"] - r["total_muls"] / r["rho_matched_muls"]) <= 1e-12 * r["ratio"] for r in ROWS),
    "total_muls = sum of the charged phases on every record": all(r["total_muls"] == sum(r[k] for k in ("precompute_muls", "base_muls", "generation_muls", "weil_muls", "solve_muls", "lift_muls", "check_muls", "la_fp_muls")) for r in ROWS),
    "la_fp_muls = 16 * folded.la_ops on every record": all(r["la_fp_muls"] == 16 * r["folded"]["la_ops"] for r in ROWS),
    "rho (<±psi> walk) below the index calculus on every record of both arms at 0, 0.1 and 1 GAE": all(ic_at(r, float(g)) > walk_at(r, float(g), "matched") for r in ROWS for g in G),
    "rho (negation walk) below the index calculus on every record of both arms at 0, 0.1 and 1 GAE": all(ic_at(r, float(g)) > walk_at(r, float(g), "negation") for r in ROWS for g in G),
    "arm B below arm A on every curve at 0, 0.1 and 1 GAE": all(ic_at(A[k], float(g)) > ic_at(B[k], float(g)) for k in KEYS for g in G),
    "improvement > 1 on every curve at 0, 0.1 and 1 GAE": all(improvement(k, float(g)) > 1 for k in KEYS for g in G),
    "Stage A: points in both arms identical (no error rows)": len(ok_rows) == len(STAGE_A),
    "Stage A: distinct column logs = columns under <V4, psi> wherever computed": all(r["distinct_column_logs"] == r["columns_psi"] for r in with_logs),
    "Stage A: the curves without a base are at p = 41": all(r["p"] == 41 for r in ok_rows if r["base_points"] == 0),
    "the two G2.35 outside curves are the two G2.34 outside curves": {k for k in KEYS if not (1.6 <= sab[k] <= 2.4)} == {k for k in KEYS if not (0.8 <= yrat[k] <= 1.25)},
}

band_rows = []
for k in KEYS:
    for arm, r in (("A", A[k]), ("B", B[k])):
        band_rows.append({"p": k[0], "seed": k[1], "arm": arm, "gae_fp_muls": r["fp_muls_per_add"],
                          "macaulay_entries": r["macaulay_entries"],
                          "rho_psi_steps_mean": r["rho_matched_steps"], "rho_negation_steps_mean": r["rho_negation_steps"],
                          "ic": {g: ic_at(r, float(g)) for g in G},
                          "rho_psi": {g: walk_at(r, float(g), "matched") for g in G},
                          "rho_negation": {g: walk_at(r, float(g), "negation") for g in G},
                          "s_over_rho_psi": {g: ic_at(r, float(g)) / walk_at(r, float(g), "matched") for g in G}})
pair_band = [{"p": k[0], "seed": k[1], "s_a_over_s_b": {g: ic_at(A[k], float(g)) / ic_at(B[k], float(g)) for g in G},
              "improvement": {g: improvement(k, float(g)) for g in G}} for k in KEYS]
med = lambda f: statistics.median(f(B[k]) for k in KEYS)
gae_band = {g: {"ic": med(lambda r: ic_at(r, float(g))), "rho": med(lambda r: walk_at(r, float(g), "matched"))} for g in G}

RHO_ASSESSMENT = {'kind': 'negation_frobenius', 'protocol': 'note section 9.15 (registered before code): on each GLS curve the matched walk is the <+-psi> walk, psi the GLS endomorphism (the twisted p^2-power Frobenius, psi^2 = -1), with 16 <+-psi>, 16 negation and 16 unfolded walks per curve; S_B / rho = total_muls / mean fp_muls of the 16 <+-psi> walks (set-up and class representatives included); the improvement compares arm A against the negation walk with arm B against the <+-psi> walk.', 'matches_protocol': "every record carries 16 Gls, 16 negation and 16 unfolded walks, all correct, matched_fold = Gls; rho_matched_muls equals the mean of the Gls walks' fp_muls; both arms of a curve ran identical walks (checks above).", 'strongest_available': 'on all 10 curves j(E) is in F_{p^2} but not in F_p (independent Python check: j^(p^2) = j, j^p != j), so the only Frobenius-type endomorphism over F_{p^4} is psi from the p^2-power map (verified independently: psi^2(G) = -G, psi(G) = [lambda]G, lambda^2 = -1 mod n); Aut(E) = {+-1} (j != 0, 1728); <+-psi> (classes of 4) is the largest fold, no order-8 fold from a p-power map exists. <+-psi> walks take 0.47-0.79 sqrt(n) steps (16-walk means over the 10 curves; 0.54-0.61 at p = 397), against sqrt(pi n / 8) = 0.627 sqrt(n).', 'same_unit_and_process': "one process per (p, seed) runs arm A, arm B and inside each arm's run the 16 + 16 + 16 walks (rho4_folded) on the same instance with the same Fp4/Curve4 counters.", 'unpriced_native_work': "Read from src/cryptanalysis/quartic_folds.rs, klein_v4.rs, d4_weighted_s5.rs and gaudry_quartic.rs: the index calculus charges F_p multiplications only (the Fp4 counter: an F_{p^4} multiplication 19, an inversion 40; the solve's F_p multiplications counted directly; the matrix at 16 per mod-n multiplication). Unpriced on the index calculus: the Macaulay matrices' entry traffic and additions (emitted: macaulay_entries, rows x cols at each call's last degree), the solve's F_p inversions (inv_mod per pivot or eigenvector; count not emitted, at most one per pivot row, far fewer than the entries), hash and table work in the lift-back, F_p additions. Unpriced on the walks: per step the hash-table lookup and insert, the partition hashes (two with the look-ahead) and the class comparisons, F_p additions (emitted: steps). The protocol's band prices one GAE per Macaulay entry and one per walk step, GAE = the curve's measured F_p multiplications per affine addition (fp_muls_per_add = 97.0 on every record). Native work is unpriced on both arms, so the 0, 0.1 and 1 GAE values differ.", 'limitations': ['16 walks per curve and fold: about 13 % sampling error on each walk mean; the per-curve improvement, a ratio of two walk means and two IC totals, carries it twice.', 'The <+-psi> walk applies psi to both coordinates on every step (152-154 F_p multiplications a step at p = 397, against 109 for the negation walk); comparing psi(x) first and mapping y only when needed would save an estimated 6 % (not measured). S_B / rho and the improvement are favourable to the index calculus by about that much.', "Below n ~ 2^25 the walks' set-up dominates and the <+-psi> walk costs more F_p multiplications than the negation walk, which inflates the improvement at p <= 149; the README row quotes the improvement at p >= 197 only.", "The two arms share one seeded RNG for target draws and Wiedemann; the target streams coincide until arm B's first Wiedemann attempt and then diverge, so the comparison is paired on a prefix of each stream only (code reading of collect_and_solve and Arm::attempt). This adds noise to S_A / S_B per curve, not a bias.", "The handovers (2^93.2, 2^88.0) are section 11.16's model extrapolated with theory exponents from measured constants, not measured crossings."]}

raw = [{"path": os.path.relpath(f, HERE), "sha256": sha(f)} for f in files + [STAGE_A_PATH, os.path.join(REPO, SJ)]]
CLAIM_TEXT = [os.path.join(REPO, q) for q in ("research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md", ST, SR, SJ)]
claim_text_pins = [{"path": os.path.relpath(q, HERE), "sha256": sha(q)} for q in CLAIM_TEXT]
CLAIM_TEXT_NOTE = ("the claims were read from these files at these hashes. The note's §9.19 was read from the working tree "
                   "on 2026-10-21 and then committed unchanged in 911674b04 (same hash); re-run this script if any of these "
                   "files changes. The §9.19 estimate that an abscissa-first <±psi> walk would save about 6 % is the validator's own "
                   "estimate, not a measurement, and is not checked here")
mism = [h for h in HEAD + BOUNDS + EXPL if not h["agrees"]]
GATING_OK = all(h["agrees"] for h in HEAD + BOUNDS)
rep = {
    "schema": "validator-report/v1",
    "status": "pass" if GATING_OK and all(checks.values()) else "fail",
    "independent": True,
    "validator": "validator subagent (review-adversarial policy), independent of the session that produced E25; recompute in this stdlib script, which shares no code with examples/glv_invariant_experiment_tables.rs",
    "date": "2026-10-21",
    "row": "benchmarks/README.md, row 20261018-k4-gls-v4",
    "script": "validate.py",
    "tolerance_rule": "a printed claim agrees when |claimed - recomputed| <= half a unit in its last printed digit (integers: 0.5); rel_tol = that half unit / max(|claimed|, |recomputed|), the gate's comparison. summary.json values are machine output: rel_tol 1e-9.",
    "raw_data": raw,
    "claim_text_pins": claim_text_pins,
    "claim_text_note": CLAIM_TEXT_NOTE,
    "superseded_claims": [h["name"] for h in HEAD + BOUNDS + EXPL if "superseded" in h],
    "status_rule": "pass iff every headline number agrees at its printed precision, every headline bound holds, and every check holds; explanatory remarks (numbers inside an explanation of a mechanism, not a reported result) are checked and listed but do not decide the status",
    "headline_count": len(HEAD),
    "headline_disagreements": sum(not h["agrees"] for h in HEAD),
    "headline": HEAD,
    "bounds": BOUNDS,
    "explanatory": EXPL,
    "checks": checks,
    "disagreements": [{"class": h["class"] + (" bound" if "bound" in h else ""), "name": h["name"], "source": h["source"], "printed": h["printed"], "recomputed": h["recomputed"], "definition": h["definition"]} for h in mism],
    "rho_reference_assessment": RHO_ASSESSMENT,
    "preregistration": {'E25 (G2.32-G2.37, note section 9.15)': 'registered in 40b2973f4 (2026-10-18 06:23 UTC) before the E25 code and Stage A 46202dc97 (07:03 UTC); the bands in the committed note equal those at 40b2973f4', 'how': 'git log -S on the note for the prediction ids and git show 40b2973f4 of the note, read by the validator on 2026-10-21'},
    "gae_band_rows": band_rows,
    "gae_band_arms": pair_band,
    "gae_band_summary": {"what": "median over the 10 curves of arm B's S_IC(g) and of the matched <±psi> walk's S_rho(g), F_p multiplications, g GAE per unpriced native operation", "values": gae_band},
}
out = os.environ.get("VALIDATE_OUT") or os.path.join(HERE, "validator-report.json")
json.dump(rep, open(out, "w"), indent=1, ensure_ascii=False)
open(out, "a").write("\n")
print(f"{out}: status {rep['status']}; {len(HEAD)} headline numbers, {len(BOUNDS)} bounds, {len(EXPL)} explanatory; {len(mism)} disagree; checks {sum(checks.values())}/{len(checks)}")
for h in mism:
    print(f"  DISAGREES ({h['class']}{' bound' if 'bound' in h else ''})  {h['name']}: printed {h['printed']} recomputed {h['recomputed']!r}  [{h['source']}]")
for k, v in checks.items():
    if not v:
        print("  CHECK FAILS", k)
sys.exit(0 if rep["status"] == "pass" else 1)
