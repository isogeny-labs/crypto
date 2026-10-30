#!/usr/bin/env python3
"""Validator recompute for benchmarks/20261018-k4-v4-line (E19 against E24).

Independent of the producing session: stdlib only, reads the committed raw run records
experiments/27_quartic_pipeline_d4/p*s*.json (E19) and experiments/29_quartic_pipeline_v4/p*s*.json
(E24), recomputes every number the README row (benchmarks/README.md, the 20261018-k4-v4-line row),
the note research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md §9.6 and §9.14, the
study research/glv_k4_pipeline_d4_20261017/ (README.md, REPORT.typ, summary.json) state for these
two experiments, and writes validator-report.json next to this script.

Tolerance rule: a claim printed with k digits after the decimal point of its mantissa (or as an
integer) agrees when |claimed - recomputed| <= half a unit in its last printed digit; rel_tol is that
half unit divided by max(|claimed|, |recomputed|), which is exactly the gate's comparison.  Values
copied from summary.json are machine output and must agree to rel_tol 1e-9.

    python3 benchmarks/20261018-k4-v4-line/validate.py
"""
import glob, hashlib, json, math, os, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.environ.get("VALIDATE_REPO") or os.path.normpath(os.path.join(HERE, "..", ".."))
E19_DIR = "experiments/27_quartic_pipeline_d4"
E24_DIR = "experiments/29_quartic_pipeline_v4"
STUDY = "research/glv_k4_pipeline_d4_20261017"
NOTE = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md"
README_ROW = "benchmarks/README.md row 20261018-k4-v4-line"


def sha(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def load(d):
    files = sorted(glob.glob(os.path.join(REPO, d, "p*s*.json")))
    rows = []
    for f in files:
        rows += json.load(open(f))
    return files, rows


e19_files, E19 = load(E19_DIR)
e24_files, E24 = load(E24_DIR)
row = {("e19", r["p"], r["seed"]): r for r in E19}
row.update({("e24", r["p"], r["seed"]): r for r in E24})


def y(r):
    return r["residuals"] / r["folded"]["relations"]


def fit(points):
    pts = [(math.log2(x), math.log2(v)) for x, v in points]
    mx = sum(a for a, _ in pts) / len(pts)
    my = sum(b for _, b in pts) / len(pts)
    return sum((a - mx) * (b - my) for a, b in pts) / sum((a - mx) ** 2 for a, _ in pts)


def handover(rows):
    """Section 11.16's model as the note states it: p* = y (cols/p) C sqrt(h) / (S_rho (1 - r)),
    n* = p*^4 / h; y pooled, the rest means over curves; S_rho = rho / sqrt(n), r = LA / rho."""
    m = lambda f: sum(f(r) for r in rows) / len(rows)
    yy = sum(r["residuals"] for r in rows) / sum(r["folded"]["relations"] for r in rows)
    cols_p = m(lambda r: r["folded"]["columns"] / r["p"])
    c = m(lambda r: r["call"]["mean"])
    h = m(lambda r: r["cofactor"])
    s_rho = m(lambda r: r["rho_matched_muls"] / math.sqrt(r["n"]))
    r_la = m(lambda r: r["la_fp_muls"] / r["rho_matched_muls"])
    ps = yy * cols_p * c * math.sqrt(h) / (s_rho * (1 - r_la))
    return dict(y=yy, cols_p=cols_p, c=c, h=h, s_rho=s_rho, r=r_la, log2n=math.log2(ps ** 4 / h))


def band(r, g):
    """S / rho with unpriced native work at g GAE per operation (one per Macaulay-matrix entry for
    the index calculus, one per step for the walk; a GAE is the curve's measured F_p multiplications
    per affine addition), as AGENTS.md section 5a and the protocol (note section 9.13) state."""
    G = r["fp_muls_per_add"]
    return (r["total_muls"] + g * G * r["macaulay_entries"]) / (r["rho_matched_muls"] + g * G * r["rho_matched_steps"])


def ic_at(r, g):
    return r["total_muls"] + g * r["fp_muls_per_add"] * r["macaulay_entries"]


def rho_at(r, g):
    return r["rho_matched_muls"] + g * r["fp_muls_per_add"] * r["rho_matched_steps"]


def solves_share(r):
    return 100.0 * (r["weil_muls"] + r["solve_muls"]) / r["total_muls"]


# ---------------------------------------------------------------- recomputed quantities
V = {}
for k, r in row.items():
    V[k + ("S",)] = r["total_muls"]
    V[k + ("rho",)] = r["rho_matched_muls"]
    V[k + ("ratio",)] = r["ratio"]
    V[k + ("y",)] = y(r)
    V[k + ("cmed",)] = r["call"]["median"]
    V[k + ("targets",)] = r["residuals"]
    V[k + ("unsolved",)] = r["unsolved"]
    V[k + ("cols",)] = r["folded"]["columns"]
    V[k + ("base",)] = r["base_points"]
    V[k + ("log2n",)] = r["bits"]
    V[k + ("share",)] = solves_share(r)
pairs = {}
for (e, p, s), r in row.items():
    if e == "e19" and ("e24", p, s) in row:
        pairs[(p, s)] = r["ratio"] / row[("e24", p, s)]["ratio"]
anypair = [a["ratio"] / b["ratio"] for a in E19 for b in E24 if a["p"] == b["p"]]
ps_common = sorted({p for p, _ in pairs})
bysize = {p: statistics.mean(r["ratio"] for r in E19 if r["p"] == p) / statistics.mean(r["ratio"] for r in E24 if r["p"] == p)
          for p in ps_common}
H19, H24 = handover(E19), handover(E24)
H19L = handover([r for r in E19 if r["p"] == max(x["p"] for x in E19)])
H24L = handover([r for r in E24 if r["p"] == max(x["p"] for x in E24)])
g212 = {(r["p"], r["seed"]): r["ratio"] / (0.584 * 1.3e9 / r["p"]) for r in E19}
srho_mean = lambda rows, p: statistics.mean(r["rho_matched_muls"] / math.sqrt(r["n"]) for r in rows if r["p"] == p)

# ---------------------------------------------------------------- claims
HEAD, EXPL, BOUNDS = [], [], []


def parse(printed):
    s = printed.replace(",", "").replace("−", "-")
    mant, _, ex = s.partition("e")
    dec = len(mant.split(".")[1]) if "." in mant else 0
    return float(s), 0.5 * 10 ** ((int(ex) if ex else 0) - dec)


N19 = NOTE + " §9.19"
ERRATUM = "additive erratum, note §9.19 (2026-10-21); the earlier text is left as written and §9.19 is the record"
# Claims the note's §9.19 (and, for two of them, the study REPORT's errata section) corrected: name ->
# the corrected value as printed there.  The claim is checked against the corrected value; the value as
# first written is kept in the report under "superseded", with whether it agreed with the raw data.
ERRATA = {
    "E24 p=521 seed=1 rho": "1.17e7",
    "E19/E24 p=101 seed=2": "170",
    "E24 kept rows re-added to O": "392",
    "E24 unsolved share per curve, most (percent)": "0.033",
    "E19 G2.12 measured/registered, p=53 seed 1": "0.494",
    "E19 S_rho per sqrt n at p=53 (G2.12 note)": "243",
}
ERRATA_ALSO_IN_REPORT = {"E19/E24 p=101 seed=2", "E19 G2.12 measured/registered, p=53 seed 1"}


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
        superseded = {"printed": printed, "source": where, "agrees_with_raw_data": _agree(oc, ohalf, value, bound),
                      "superseded_by": ERRATUM + (" and the study REPORT.typ's errata section" if name in ERRATA_ALSO_IN_REPORT else "")}
        printed = ERRATA[name]
        where = N19 + (" and " + ST + " errata" if name in ERRATA_ALSO_IN_REPORT else "") + " (supersedes " + where + ")"
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


R, N6, N14, SR, ST = README_ROW, NOTE + " §9.6", NOTE + " §9.14", STUDY + "/README.md", STUDY + "/REPORT.typ"
e19max_c = max(r["call"]["median"] for r in E19)
e24max_c = max(r["call"]["median"] for r in E24)
r19 = [r["ratio"] for r in E19]
r24 = [r["ratio"] for r in E24]
# README row -------------------------------------------------------------
claim("E19 curves", R, "10", len(E19), "number of E19 run records")
claim("E19 smallest p", R, "53", min(r["p"] for r in E19), "min p over E19 records")
claim("E19 largest p", R, "269", max(r["p"] for r in E19), "max p over E19 records")
claim("E24 curves", R, "12", len(E24), "number of E24 run records")
claim("E24 smallest p", R, "101", min(r["p"] for r in E24), "min p over E24 records")
claim("E24 largest p", R, "521", max(r["p"] for r in E24), "max p over E24 records")
claim("negation walks per curve (min over runs)", R, "16", min(sum(w["fold"] == "Negation" for w in r["rho"]) for r in E19 + E24), "count of fold = Negation walks in each run, minimum")
claim("negation walks per curve (max over runs)", R, "16", max(sum(w["fold"] == "Negation" for w in r["rho"]) for r in E19 + E24), "count of fold = Negation walks in each run, maximum")
claim("E19 per-call C (median, largest)", R, "1.41e9", e19max_c, "max over E19 curves of the per-call C median (p = 269)")
claim("E24 per-call C (median, largest)", R, "1.66e6", e24max_c, "max over E24 curves of the per-call C median (p = 521)")
claim("E19 S/rho min", R, "2.6e6", min(r19), "min over E19 curves of total_muls / rho_matched_muls (16 negation walks)")
claim("E19 S/rho max", R, "8.8e6", max(r19), "max over E19 curves")
claim("E24 S/rho min", R, "1.3e4", min(r24), "min over E24 curves")
claim("E24 S/rho max", R, "1.7e5", max(r24), "max over E24 curves")
claim("E19/E24 at matched p, min", R, "35", min(pairs.values()), "min over matched (p, seed) of (E19 S/rho) / (E24 S/rho)")
claim("E19/E24 at matched p, max", R, "240", max(pairs.values()), "max over matched (p, seed)")
claim("E24 slope of S/rho in n", R, "-0.25", fit([(r["n"], r["ratio"]) for r in E24]), "least-squares slope of log2(S/rho) on log2 n, 12 E24 curves")
claim("E19 handover log2 n*", R, "116.7", H19["log2n"], "section 11.16 model from E19's own constants, every curve pooled")
claim("E24 handover log2 n*", R, "90.1", H24["log2n"], "section 11.16 model from E24's own constants, every curve pooled")
claim("E24 still slower than rho by at least", R, "1.3e4", min(r24), "min E24 S/rho")
# Code constants the row states (src/cryptanalysis/klein_v4.rs) --------------------------
import re
KV4 = os.path.join(REPO, "src/cryptanalysis/klein_v4.rs")
_src = open(KV4).read()
_w = [int(x) for x in re.search(r"pub const WEIGHTS: \[u8; 4\] = \[([0-9, ]+)\];", _src).group(1).split(",")]
_d = int(re.search(r"pub const EQUATION_WDEG: u16 = (\d+);", _src).group(1))
claim("V4 solve weights (sorted, as one number 2211)", R, "2211", int("".join(str(x) for x in sorted(_w, reverse=True))), "klein_v4.rs WEIGHTS, digits in descending order")
claim("V4 solve Macaulay degree", R, "11", 4 * _d - sum(_w) + 1, "sum of the four equations' weighted degrees - sum of weights + 1, from klein_v4.rs (EQUATION_WDEG, WEIGHTS)")
claim("V4 solve quotient (weighted Bezout)", R, "64", _d ** 4 / math.prod(_w), "EQUATION_WDEG^4 / prod(WEIGHTS), from klein_v4.rs")
# Note section 9.6, E19 table -------------------------------------------------
T19 = [  # p, seed, log2 n, cols, targets, y, unsolved, C median, S, rho, S/rho  (as printed)
    (53, 1, "20.9", "15", "1498", "93.6", "2", "1.37e9", "2.41e12", "3.40e5", "7.08e6"),
    (53, 2, "20.9", "13", "1926", "137.6", "5", "1.37e9", "3.00e12", "3.43e5", "8.75e6"),
    (101, 1, "24.6", "23", "2608", "108.7", "0", "1.39e9", "3.91e12", "6.18e5", "6.33e6"),
    (101, 2, "24.6", "23", "2701", "112.5", "0", "1.39e9", "4.01e12", "6.86e5", "5.85e6"),
    (149, 1, "26.9", "37", "3525", "92.8", "0", "1.40e9", "5.26e12", "1.29e6", "4.09e6"),
    (149, 2, "26.9", "41", "2868", "68.3", "0", "1.40e9", "4.30e12", "1.26e6", "3.40e6"),
    (197, 1, "28.5", "48", "7061", "144.1", "0", "1.41e9", "1.04e13", "2.11e6", "4.91e6"),
    (197, 2, "28.5", "53", "4306", "79.7", "0", "1.41e9", "6.29e12", "2.22e6", "2.83e6"),
    (269, 1, "30.3", "58", "11975", "203.0", "0", "1.41e9", "1.74e13", "3.17e6", "5.51e6"),
    (269, 2, "30.3", "67", "6200", "91.2", "0", "1.41e9", "9.01e12", "3.50e6", "2.57e6"),
]
for p, s, *vals in T19:
    for f, v in zip(("log2n", "cols", "targets", "y", "unsolved", "cmed", "S", "rho", "ratio"), vals):
        claim(f"E19 p={p} seed={s} {f}", N6 + " table", v, V[("e19", p, s, f)], f"E19 record p{p}s{s}: {f}")
claim("E19 walks, all correct", N6, "320", sum(len(r["rho"]) for r in E19) if all(w["correct"] for r in E19 for w in r["rho"]) else -1, "walks in the E19 records, every one correct (else -1)")
claim("E19 pooled y (G2.10)", N6 + " G2.10", "115.1", sum(r["residuals"] for r in E19) / sum(r["folded"]["relations"] for r in E19), "sum targets / sum relations over E19")
claim("E19 C median, smallest (G2.11)", N6 + " G2.11", "1.37e9", min(r["call"]["median"] for r in E19), "min over E19 curves of per-call C median")
claim("E19 C median, largest (G2.11)", N6 + " G2.11", "1.41e9", e19max_c, "max over E19 curves")
claim("E19 unsolved targets (G2.11)", N6 + " G2.11", "7", sum(r["unsolved"] for r in E19), "sum unsolved over E19")
claim("E19 targets (G2.11)", N6 + " G2.11", "44668", sum(r["residuals"] for r in E19), "sum targets over E19")
for p, v in zip((53, 101, 149, 197, 269), ("0.55", "0.81", "0.73", "1.00", "1.43")):
    claim(f"E19 G2.12 measured/registered, mean at p={p}", N6 + " G2.12", v,
          statistics.mean(g212[(p, s)] for s in (1, 2)), "mean over the two curves of (S/rho) / (0.584 * 1.3e9 / p)")
claim("E19 G2.12 measured/registered, p=53 seed 1", N6 + " G2.12 and " + ST, "0.495", g212[(53, 1)], "(S/rho) / (0.584 * 1.3e9 / 53) for p53s1")
claim("E19 slope of S/rho in n (G2.12)", N6 + " G2.12", "-0.13", fit([(r["n"], r["ratio"]) for r in E19]), "least-squares slope, 10 E19 curves")
claim("E19 S_rho per sqrt n at p=53 (G2.12 note)", N6 + " G2.12", "242", srho_mean(E19, 53), "mean over the two p = 53 curves of rho_matched_muls / sqrt(n), the definition that gives the 92 at p = 269 (seed 1 alone is 241.9, but seed 1 alone at p = 269 is 87.5)", cls="explanatory")
claim("E19 S_rho per sqrt n at p=269 (G2.12 note)", N6 + " G2.12", "92", srho_mean(E19, 269), "mean over the two p = 269 curves of rho_matched_muls / sqrt(n)", cls="explanatory")
claim("E19 non-solve share of S, largest (G2.12)", N6 + " G2.12", "0.001", max(100 - solves_share(r) for r in E19), "max over E19 curves of 100 * (S - Weil - solves) / S, percent")
claim("E19 handover log2 n* (G2.13)", N6 + " G2.13", "116.7", H19["log2n"], "every curve pooled")
claim("E19 handover log2 n*, p=269 alone (G2.13)", N6 + " G2.13", "120.0", H19L["log2n"], "the two p = 269 curves")
claim("E19 band: largest S/rho change across 0..1 GAE", N6, "1.5", max(band(r, 1) / band(r, 0) for r in E19), "max over E19 curves of (S/rho at 1 GAE) / (S/rho at 0); the note says 'at most 1.5x'", bound="le")
# Note section 9.14, E24 table ------------------------------------------------
T24 = [  # p, seed, log2 n, base, cols, targets, y, unsolved, C median, solves share %, S, rho, S/rho
    (101, 1, "24.6", "36", "9", "77960", "7796", "9", "1.62e6", "99.5", "1.28e11", "7.6e5", "1.69e5"),
    (101, 2, "24.6", "52", "13", "15387", "1099", "5", "1.62e6", "99.5", "2.55e10", "7.4e5", "3.43e4"),
    (149, 1, "26.9", "64", "16", "31225", "1837", "4", "1.63e6", "99.5", "5.20e10", "1.37e6", "3.80e4"),
    (149, 2, "26.9", "60", "15", "45343", "2834", "2", "1.63e6", "99.5", "7.51e10", "1.36e6", "5.54e4"),
    (197, 1, "28.5", "76", "19", "142361", "7118", "10", "1.65e6", "99.4", "2.37e11", "1.70e6", "1.40e5"),
    (197, 2, "28.5", "84", "21", "74345", "3379", "7", "1.64e6", "99.4", "1.24e11", "2.27e6", "5.44e4"),
    (269, 1, "30.3", "132", "33", "44281", "1302", "0", "1.65e6", "99.4", "7.38e10", "3.21e6", "2.30e4"),
    (269, 2, "30.3", "148", "37", "50040", "1317", "1", "1.65e6", "99.4", "8.37e10", "3.58e6", "2.34e4"),
    (397, 1, "32.5", "208", "52", "66981", "1264", "1", "1.66e6", "99.3", "1.13e11", "8.61e6", "1.31e4"),
    (397, 2, "32.5", "188", "47", "89424", "1863", "2", "1.66e6", "99.3", "1.50e11", "9.48e6", "1.58e4"),
    (521, 1, "34.1", "240", "60", "131952", "2163", "1", "1.66e6", "99.3", "2.21e11", "1.18e7", "1.88e4"),
    (521, 2, "34.1", "232", "58", "142487", "2415", "1", "1.66e6", "99.3", "2.39e11", "1.11e7", "2.15e4"),
]
for p, s, *vals in T24:
    for f, v in zip(("log2n", "base", "cols", "targets", "y", "unsolved", "cmed", "share", "S", "rho", "ratio"), vals):
        claim(f"E24 p={p} seed={s} {f}", N14 + " table", v, V[("e24", p, s, f)], f"E24 record p{p}s{s}: {f}")
claim("E24 kept rows re-added to O", N14, "381", sum(r["rows_readded"] for r in E24) if all(r["rows_readd_failures"] == 0 for r in E24) else -1,
      "sum of rows_readded over the 12 E24 records (rows_readd_failures = 0 on every record; rows_readded = folded.relations on every record)")
claim("E24 walks, all correct", N14, "384", sum(len(r["rho"]) for r in E24) if all(w["correct"] for r in E24 for w in r["rho"]) else -1, "walks in the E24 records, every one correct (else -1)")
claim("E24 unsolved per curve, least", N14, "0", min(r["unsolved"] for r in E24), "min unsolved over E24 curves")
claim("E24 unsolved per curve, most", N14, "10", max(r["unsolved"] for r in E24), "max unsolved over E24 curves")
claim("E24 unsolved share per curve, most (percent)", N14 + " and G2.28", "0.012", max(100 * r["unsolved"] / r["residuals"] for r in E24), "max over E24 curves of 100 * unsolved / targets; the note says '<= 0.012 %' per curve (pooled it is 0.0047 %)", bound="le")
claim("E24 slope of S/rho in n (G2.30)", N14 + " G2.30", "-0.25", fit([(r["n"], r["ratio"]) for r in E24]), "least-squares slope, 12 E24 curves")
claim("E24 solves share, least (G2.30)", N14 + " G2.30", "99.3", min(solves_share(r) for r in E24), "min over E24 curves of the Weil + solve share of S, percent")
claim("E24 solves share, most (G2.30)", N14 + " G2.30", "99.5", max(solves_share(r) for r in E24), "max over E24 curves")
claim("E24 C median, smallest (G2.28)", N14 + " G2.28", "1.62e6", min(r["call"]["median"] for r in E24), "min over E24 curves")
claim("E24 C median, largest (G2.28)", N14 + " G2.28", "1.66e6", e24max_c, "max over E24 curves")
for f, v in (("y", "2326"), ("cols_p", "0.114"), ("c", "1.66e6"), ("s_rho", "110.9"), ("r", "0.072"), ("log2n", "90.1")):
    claim(f"E24 handover constant {f} (all curves)", N14 + " G2.31", v, H24[f], "section 11.16 model, every E24 curve pooled")
for f, v in (("s_rho", "84.2"), ("r", "0.090"), ("log2n", "91.7")):
    claim(f"E24 handover constant {f} (p=521 only)", N14 + " G2.31", v, H24L[f], "section 11.16 model, the two p = 521 curves")
claim("E24 band: largest S/rho rise at 1 GAE", N14, "7", max(band(r, 1) / band(r, 0) for r in E24), "max over E24 curves of (S/rho at 1 GAE) / (S/rho at 0); the note says 'up to 7x'", bound="le")
for (p, s), v in zip(((101, 1), (101, 2), (149, 1), (149, 2), (197, 1), (197, 2), (269, 1), (269, 2)),
                     ("37", "171", "108", "61", "35", "52", "240", "110")):
    claim(f"E19/E24 p={p} seed={s}", N14 + " G2.29 and " + ST, v, pairs[(p, s)], "(E19 S/rho) / (E24 S/rho), same p and seed")
for p, v in zip((101, 149, 197, 269), ("60", "80", "40", "174")):
    claim(f"E19/E24 by size p={p}", N14 + " G2.29", v, bysize[p], "mean E19 S/rho over the two curves / mean E24 S/rho over the two curves")
claim("E19/E24 any pairing, least (bound 20)", N14, "20", min(anypair), "min over every E19 x E24 pair at the same p; the note says any pairing stays in [20, 240]", bound="ge")
claim("E19/E24 any pairing, most (bound 240)", N14, "240", max(anypair), "max over every E19 x E24 pair at the same p", bound="le")
claim("E24 best row S/rho (p=397 seed 1)", N14, "1.3e4", V[("e24", 397, 1, "ratio")], "S/rho of p397s1, the minimum over E24")
claim("E24 largest log2 n (scope)", N14, "34.1", max(r["bits"] for r in E24), "max bits over E24")
claim("E19 per-call C, 'What it says'", N14, "1.4e9", e19max_c, "max E19 per-call C median")
# Note §9.19 and the study REPORT.typ errata section (2026-10-21): the values they state -------
RE = ST + " errata"
claim("§9.19: E24 p=521 seed 1 matched rho, four digits", N19, "1.1749e7", V[("e24", 521, 1, "rho")], "rho_matched_muls of p521s1")
claim("§9.19: E24 p=521 seed 1 S/rho (unchanged)", N19, "1.88e4", V[("e24", 521, 1, "ratio")], "ratio of p521s1")
claim("§9.19 and REPORT errata: E19/E24 p=101 seed 2, one decimal", N19 + " and " + RE, "170.4", pairs[(101, 2)], "(E19 S/rho) / (E24 S/rho) at p101s2")
claim("§9.19 and REPORT errata: the written 171 from rounded inputs", N19 + " and " + RE, "171", 5.85e6 / 3.43e4, "5.85e6 / 3.43e4, the two printed values the original 171 was divided from", cls="explanatory")
claim("§9.19: E19/E24 range at 0 GAE, least", N19 + " and " + RE, "35", min(pairs.values()), "min over matched (p, seed), 0 GAE")
claim("§9.19: E19/E24 range at 0 GAE, most", N19 + " and " + RE, "240", max(pairs.values()), "max over matched (p, seed), 0 GAE")
claim("§9.19: E24 curves with rows re-added", N19, "12", sum(1 for r in E24 if r["rows_readded"] > 0 and r["rows_readd_failures"] == 0), "E24 records with rows re-added and no failure")
claim("§9.19: E24 rows re-add failures", N19, "0", sum(r["rows_readd_failures"] for r in E24), "sum of rows_readd_failures")
claim("§9.19: E24 p=101 seed 2 unsolved", N19, "5", V[("e24", 101, 2, "unsolved")], "unsolved of p101s2")
claim("§9.19: E24 p=101 seed 2 targets", N19, "15387", V[("e24", 101, 2, "targets")], "targets of p101s2")
claim("§9.19: E24 p=149 seed 1 unsolved share, percent", N19, "0.0128", 100 * V[("e24", 149, 1, "unsolved")] / V[("e24", 149, 1, "targets")], "100 * unsolved / targets of p149s1")
claim("§9.19: E24 unsolved share pooled, percent", N19, "0.0047", 100 * sum(r["unsolved"] for r in E24) / sum(r["residuals"] for r in E24), "100 * sum unsolved / sum targets over E24")
claim("§9.19 and REPORT errata: E19 G2.12 p=53 seed 1, five decimals", N19 + " and " + RE, "0.49448", g212[(53, 1)], "(S/rho) / (0.584 * 1.3e9 / 53) for p53s1")
claim("§9.19: E19 S_rho per sqrt n, p=53 seed 1 alone", N19, "241.9", V[("e19", 53, 1, "rho")] / math.sqrt(row[("e19", 53, 1)]["n"]), "rho_matched_muls / sqrt(n) of p53s1", cls="explanatory")
claim("§9.19: E19 S_rho per sqrt n, p=269 seed 1 alone", N19, "87.5", V[("e19", 269, 1, "rho")] / math.sqrt(row[("e19", 269, 1)]["n"]), "rho_matched_muls / sqrt(n) of p269s1", cls="explanatory")
claim("§9.19: E19 S_rho per sqrt n, p=269 two-curve mean", N19, "92", srho_mean(E19, 269), "mean over the two p = 269 curves", cls="explanatory")
claim("§9.19 and REPORT errata: GAE in F_p multiplications, least over records", N19 + " and " + RE, "97", min(r["fp_muls_per_add"] for r in E19 + E24), "fp_muls_per_add, min over the 22 records")
claim("§9.19 and REPORT errata: GAE in F_p multiplications, most over records", N19 + " and " + RE, "97", max(r["fp_muls_per_add"] for r in E19 + E24), "fp_muls_per_add, max over the 22 records")
for g, lo, hi in (("0.1", "19", "132"), ("1", "6.5", "45")):
    vals = [band(row[("e19", p, s)], float(g)) / band(row[("e24", p, s)], float(g)) for (p, s) in pairs]
    claim(f"§9.19 and REPORT errata: E19/E24 at {g} GAE, least", N19 + " and " + RE, lo, min(vals), f"min over matched (p, seed) of (E19 S/rho) / (E24 S/rho), one GAE per Macaulay entry and per walk step at {g} GAE")
    claim(f"§9.19 and REPORT errata: E19/E24 at {g} GAE, most", N19 + " and " + RE, hi, max(vals), f"max over matched (p, seed), {g} GAE")
claim("§9.19: E19 runs replayed", N19, "10", len(E19), "E19 records (every one replayed field-identical: replay-certificate.json)")
claim("§9.19: E24 runs replayed", N19, "12", len(E24), "E24 records (every one replayed field-identical: replay-certificate.json)")
# Study README / REPORT (numbers not already above) -------------------------------
for p, (c19, c24) in {101: ("1.39e9", "1.62e6"), 149: ("1.40e9", "1.63e6"), 269: ("1.41e9", "1.65e6"),
                      397: (None, "1.66e6"), 521: (None, "1.66e6")}.items():
    for e, c in (("e19", c19), ("e24", c24)):
        if c:
            for s in (1, 2):
                claim(f"{e.upper()} C median p={p} seed={s} (REPORT per-p column)", ST + " table", c, V[(e, p, s, "cmed")], "per-call C median; the REPORT prints one value for both curves of p")
claim("E19 C median p=197 (REPORT per-p column), seed 1", ST + " table", "1.41e9", V[("e19", 197, 1, "cmed")], "per-call C median")
claim("E19 C median p=197 (REPORT per-p column), seed 2", ST + " table", "1.41e9", V[("e19", 197, 2, "cmed")], "per-call C median")
claim("E24 C median p=197 (REPORT per-p column), seed 1", ST + " table", "1.65e6", V[("e24", 197, 1, "cmed")], "per-call C median (seed 2 is the note's 1.64e6)")
# summary.json (machine output, exact) ----------------------------------------
S = json.load(open(os.path.join(REPO, STUDY, "summary.json")))
SJ = STUDY + "/summary.json"
claim("summary e19_curves", SJ, str(S["e19_curves"]), len(E19), "count", exact=True)
claim("summary e24_curves", SJ, str(S["e24_curves"]), len(E24), "count", exact=True)
claim("summary e19_handover_log2_n", SJ, repr(S["e19_handover_log2_n"]), H19["log2n"], "handover, every E19 curve", exact=True)
claim("summary e24_handover_log2_n", SJ, repr(S["e24_handover_log2_n"]), H24["log2n"], "handover, every E24 curve", exact=True)
claim("summary e24_s_over_rho_all min", SJ, repr(S["e24_s_over_rho_all"][0]), min(r24), "min E24 S/rho", exact=True)
claim("summary e24_s_over_rho_all max", SJ, repr(S["e24_s_over_rho_all"][1]), max(r24), "max E24 S/rho", exact=True)
for sr in S["rows"]:
    p = sr["p"]
    for key, e, f in (("e19_s_over_rho", "e19", "ratio"), ("e24_s_over_rho", "e24", "ratio"), ("e19_c_median", "e19", "cmed"), ("e24_c_median", "e24", "cmed")):
        if sr[key] is not None:
            vals = [V[(e, p, s, f)] for s in (1, 2) if (e, p, s) in row]
            claim(f"summary p={p} {key} min", SJ, repr(sr[key][0]), min(vals), f"min over {e} curves at p", exact=True)
            claim(f"summary p={p} {key} max", SJ, repr(sr[key][1]), max(vals), f"max over {e} curves at p", exact=True)
    if sr["e19_over_e24_by_seed"] is not None:
        vals = [pairs[(p, s)] for s in (1, 2)]
        claim(f"summary p={p} e19_over_e24_by_seed min", SJ, repr(sr["e19_over_e24_by_seed"][0]), min(vals), "by seed", exact=True)
        claim(f"summary p={p} e19_over_e24_by_seed max", SJ, repr(sr["e19_over_e24_by_seed"][1]), max(vals), "by seed", exact=True)

# ---------------------------------------------------------------- non-numeric checks
checks = {
    "every E19/E24 record: folded.solved and folded.correct (log equals planted d)": all(r["folded"]["solved"] and r["folded"]["correct"] for r in E19 + E24),
    "every E19/E24 walk correct": all(w["correct"] for r in E19 + E24 for w in r["rho"]),
    "check_failures = 0, rows_readd_failures = 0, off_base = 0 on every record": all(r["check_failures"] == 0 and r["rows_readd_failures"] == 0 and r["off_base"] == 0 for r in E19 + E24),
    "rows_readded = folded.relations on every record": all(r["rows_readded"] == r["folded"]["relations"] for r in E19 + E24),
    "h = 4 on every record": all(r["cofactor"] == 4 for r in E19 + E24),
    "E19/E24 any pairing within [20, 240]": 20 <= min(anypair) and max(anypair) <= 240,
    "rho below the index calculus on every curve at 0, 0.1 and 1 GAE": all(band(r, g) > 1 for r in E19 + E24 for g in (0, 0.1, 1)),
    "E24 below E19 at every matched (p, seed) at 0, 0.1 and 1 GAE": all(band(row[("e19", p, s)], g) / band(row[("e24", p, s)], g) > 1 for (p, s) in pairs for g in (0, 0.1, 1)),
    "16 unfolded (fold = None) walks per record": all(sum(w["fold"] == "None" for w in r["rho"]) == 16 for r in E19 + E24),
    "rho_matched_muls equals the mean of the 16 negation walks' fp_muls on every record": all(
        abs(r["rho_matched_muls"] - statistics.mean(w["fp_muls"] for w in r["rho"] if w["fold"] == "Negation")) <= 1e-9 * r["rho_matched_muls"] for r in E19 + E24),
    "ratio = total_muls / rho_matched_muls on every record": all(abs(r["ratio"] - r["total_muls"] / r["rho_matched_muls"]) <= 1e-12 * r["ratio"] for r in E19 + E24),
    "total_muls = sum of the charged phases on every record": all(r["total_muls"] == sum(r[k] for k in ("precompute_muls", "base_muls", "generation_muls", "weil_muls", "solve_muls", "lift_muls", "check_muls", "la_fp_muls")) for r in E19 + E24),
    "la_fp_muls = 16 * folded.la_ops on every record": all(r["la_fp_muls"] == 16 * r["folded"]["la_ops"] for r in E19 + E24),
}

band_rows = []
for e, rows in (("e19", E19), ("e24", E24)):
    for r in sorted(rows, key=lambda r: (r["p"], r["seed"])):
        band_rows.append({"experiment": e, "p": r["p"], "seed": r["seed"], "gae_fp_muls": r["fp_muls_per_add"],
                          "macaulay_entries": r["macaulay_entries"], "rho_steps_mean": r["rho_matched_steps"],
                          "ic":{g: ic_at(r, float(g)) for g in ("0", "0.1", "1")},
                          "rho": {g: rho_at(r, float(g)) for g in ("0", "0.1", "1")},
                          "s_over_rho": {g: band(r, float(g)) for g in ("0", "0.1", "1")}})
pair_band = [{"p": p, "seed": s, "e19_over_e24": {g: band(row[("e19", p, s)], float(g)) / band(row[("e24", p, s)], float(g)) for g in ("0", "0.1", "1")}}
             for (p, s) in sorted(pairs)]
med = lambda rows, f, g: statistics.median(f(r, float(g)) for r in rows)
gae_band = {g: {"ic": med(E24, ic_at, g), "rho": med(E24, rho_at, g)} for g in ("0", "0.1", "1")}

RHO_ASSESSMENT = {'kind': 'negation', 'protocol': 'note section 9.5 item 6 (E19) and section 9.13 (E24): the whole run priced in F_p multiplications against the matched negation walk on the same curve, 16 negation and 16 unfolded walks per curve; ratio = total_muls / mean fp_muls of the 16 negation walks (set-up included).', 'matches_protocol': "every record carries 16 negation and 16 unfolded walks, all correct; rho_matched_muls equals the mean of the negation walks' fp_muls and ratio = total_muls / rho_matched_muls on every record (checks above).", 'strongest_available': 'on all 22 curves j(E) is outside F_{p^2} (independent Python check: j^(p^2) != j; the generators also reject j in F_{p^2}), so no Frobenius-type endomorphism is defined over F_{p^4} and Aut(E) = {+-1}: negation is the strongest fold. Negation walks take 0.71-1.16 sqrt(n) steps (16-walk means over the 22 curves), against sqrt(pi n / 4) = 0.886 sqrt(n).', 'same_unit_and_process': 'the walks (rho4_folded) run in the same process after the index calculus (collect_and_solve), on the same instance, with the same Fp4/Curve4 counters.', 'unpriced_native_work': "Read from src/cryptanalysis/quartic_folds.rs, klein_v4.rs, d4_weighted_s5.rs and gaudry_quartic.rs: the index calculus charges F_p multiplications only (the Fp4 counter: an F_{p^4} multiplication 19, an inversion 40; the solve's F_p multiplications counted directly; the matrix at 16 per mod-n multiplication). Unpriced on the index calculus: the Macaulay matrices' entry traffic and additions (emitted: macaulay_entries, rows x cols at each call's last degree), the solve's F_p inversions (inv_mod per pivot or eigenvector; count not emitted, at most one per pivot row, far fewer than the entries), hash and table work in the lift-back, F_p additions. Unpriced on the walks: per step the hash-table lookup and insert, the partition hashes (two with the look-ahead) and the class comparisons, F_p additions (emitted: steps). The protocol's band prices one GAE per Macaulay entry and one per walk step, GAE = the curve's measured F_p multiplications per affine addition (fp_muls_per_add = 97.0 on every record). Native work is unpriced on both arms, so the 0, 0.1 and 1 GAE values differ.", 'limitations': ['16 walks per curve: the matched-walk mean carries roughly 13 % sampling error (coefficient of variation about 0.5 / sqrt(16)); per-curve S/rho inherits it.', "The walk's mean includes its set-up (32 random multiples), which dominates at small n (S_rho per sqrt n 243 at p = 53 against 92 at p = 269 for E19); this makes rho dearer at small n and the IC/rho ratio lower there.", "The E19 -> E24 factor (35-240x) is the 0 GAE value; at 1 GAE per native operation it is 6.5-45x (E24's Macaulay entries per call weigh more against its smaller C); E24 stays below E19 at every matched (p, seed) at every price.", "The handovers (2^116.7, 2^90.1) are section 11.16's model extrapolated with theory exponents from measured constants, not measured crossings."]}

raw = [{"path": os.path.relpath(f, HERE), "sha256": sha(f)} for f in e19_files + e24_files]
raw.append({"path": os.path.relpath(os.path.join(REPO, SJ), HERE), "sha256": sha(os.path.join(REPO, SJ))})
code_pins = [{"path": os.path.relpath(KV4, HERE), "sha256": sha(KV4), "why": "weights, equation degree and Macaulay bound the row states"}]
CLAIM_TEXT = [os.path.join(REPO, q) for q in (NOTE, STUDY + "/REPORT.typ", STUDY + "/REPORT.pdf", STUDY + "/README.md", STUDY + "/summary.json", "benchmarks/20261018-k4-v4-line/REPORT.pdf")]
claim_text_pins = [{"path": os.path.relpath(q, HERE), "sha256": sha(q)} for q in CLAIM_TEXT]
mism = [h for h in HEAD + BOUNDS + EXPL if not h["agrees"]]
GATING_OK = all(h["agrees"] for h in HEAD + BOUNDS)
rep = {
    "schema": "validator-report/v1",
    "status": "pass" if GATING_OK and all(checks.values()) else "fail",
    "independent": True,
    "validator": "validator subagent (review-adversarial policy), independent of the session that produced E19/E24; recompute in this stdlib script, which shares no code with examples/glv_invariant_experiment_tables.rs",
    "date": "2026-10-21",
    "row": "benchmarks/README.md, row 20261018-k4-v4-line",
    "script": "validate.py",
    "tolerance_rule": "a printed claim agrees when |claimed - recomputed| <= half a unit in its last printed digit (integers: 0.5); rel_tol = that half unit / max(|claimed|, |recomputed|), the gate's comparison. summary.json values are machine output: rel_tol 1e-9.",
    "raw_data": raw,
    "code_pins": code_pins,
    "claim_text_pins": claim_text_pins,
    "claim_text_note": "the claims were read from these files at these hashes. The note's §9.19 and the REPORT errata (REPORT.typ, both REPORT.pdf copies) were read from the working tree on 2026-10-21 and then committed unchanged in 911674b04 (same hashes); re-run this script if any of these files changes",
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
    "preregistration": {'E19 (G2.9-G2.13, note section 9.5)': 'registered in 99e0a521b (2026-10-17 15:11 UTC) before the E19 code 4e5810598 (15:45 UTC)', 'E24 (G2.27-G2.31, note section 9.13)': 'registered in 9896027ed (2026-10-18 00:39 UTC) before the E24 code 854acdbc6 (00:51 UTC)', 'how': 'git log -S on the note for the prediction ids, read by the validator on 2026-10-21'},
    "gae_band_rows": band_rows,
    "gae_band_e19_over_e24": pair_band,
    "gae_band_summary": {"what": "median over the 12 E24 curves of S_IC(g) and of the matched negation walk S_rho(g), F_p multiplications, g GAE per unpriced native operation", "values": gae_band},
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
