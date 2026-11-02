#!/usr/bin/env python3
"""Validator recompute for benchmarks/20261024-e31-chain-solve (E31: the P2-chain solve on the V4 line).

Independent of the producing session: stdlib only, shares no code with examples/e31_tables.rs or
examples/e31_stage_a.rs.  It reads the committed raw records

  experiments/31_solve_cost/stage_a.json            (Stage A: per-call counters, three solvers)
  experiments/31_solve_cost/e2e_{chain,chain-sparsest}_p*s*.json   (end to end, both arms)
  experiments/30_gls_v4/p*s*.json                   (the E25 baseline records)

and recomputes every number that research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md
section 9.23 (and the E31 commit message 04a33428b9) states.  The two section 9.23 tables are parsed
straight out of the note, so the claimed values are not hand-transcribed; prose claims are listed
below with their quoted source.  Writes validator-report.json next to this script.

Tolerance rule (as benchmarks/20261018-k4-gls-v4/validate.py): a claim printed with k digits after
the decimal point of its mantissa (or as an integer) agrees when |claimed - recomputed| <= half a
unit in its last printed digit; rel_tol is that half unit / max(|claimed|, |recomputed|), the gate's
comparison.  Claims printed with '≈' are 'approximate': they agree within 10 % (stated per claim).
Ranges 'a–b' are checked as min and max over the stated population.  Medians are the Stage A
harness's definition, s[n // 2] of the sorted list (upper median of 1,000 values); the conventional
median is reported alongside and decides nothing.

    python3 benchmarks/20261024-e31-chain-solve/validate.py
"""
import glob, hashlib, json, math, os, re, statistics, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.environ.get("VALIDATE_REPO") or os.path.normpath(os.path.join(HERE, "..", ".."))
E31 = "experiments/31_solve_cost"
E25 = "experiments/30_gls_v4"
NOTE = "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md"
S923 = NOTE + " §9.23"
COMMIT = "commit 04a33428b9 message"
CURVES = [(p, s) for p in (101, 149, 197, 269, 397) for s in (1, 2)]
SOLVERS = {"A": "chain", "B": "chain-sparsest"}
GS = (0.0, 0.1, 1.0)


def rp(rel):
    return os.path.join(REPO, rel)


def sha(rel):
    return hashlib.sha256(open(rp(rel), "rb").read()).hexdigest()


def load(rel):
    return json.load(open(rp(rel)))


def umed(v):
    s = sorted(v)
    return s[len(s) // 2]


def mean(v):
    return sum(v) / len(v) if v else float("nan")


# ---------------------------------------------------------------- raw data
STAGE_A = {(c["p"], c["seed"]): c for c in load(f"{E31}/stage_a.json")}
E25R, E31R = {}, {"A": {}, "B": {}}
for p, s in CURVES:
    for r in load(f"{E25}/p{p}s{s}.json"):
        E25R[(p, s, r["arm"])] = r
    for k, name in SOLVERS.items():
        for r in load(f"{E31}/e2e_{name}_p{p}s{s}.json"):
            E31R[k][(p, s, r["arm"])] = r
ROWS = [(p, s, a) for p, s in CURVES for a in ("A", "B")]

RAW = sorted(
    [f"{E31}/stage_a.json"]
    + [f"{E31}/e2e_{n}_p{p}s{s}.json" for n in SOLVERS.values() for p, s in CURVES]
    + [f"{E25}/p{p}s{s}.json" for p, s in CURVES]
)

# ---------------------------------------------------------------- claim text
NOTE_TEXT = open(rp(NOTE), encoding="utf-8").read()
m = re.search(r"^### 9\.23 .*?(?=^### |\Z)", NOTE_TEXT, re.S | re.M)
SEC = m.group(0)
SUP = str.maketrans("⁰¹²³⁴⁵⁶⁷⁸⁹⁻", "0123456789-")


def num(tok):
    """'`1,617,013`' -> 1617013; '`6.76·10¹⁰`' -> 6.76e10 with its half unit."""
    t = tok.strip().strip("`").replace(",", "").strip()
    if "·10" in t:
        mant, ex = t.split("·10")
        e = int(ex.translate(SUP))
        dec = len(mant.split(".")[1]) if "." in mant else 0
        return float(mant) * 10 ** e, 0.5 * 10 ** (e - dec)
    dec = len(t.split(".")[1]) if "." in t else 0
    return float(t), 0.5 * 10 ** (-dec)


def table_after(marker):
    i = SEC.index(marker)
    lines = SEC[i:].splitlines()
    out, started = [], False
    for ln in lines:
        if ln.startswith("|"):
            started = True
            cells = [c.strip() for c in ln.strip().strip("|").split("|")]
            if set("".join(cells)) <= set(":- "):
                continue
            out.append(cells)
        elif started:
            break
    return out[0], out[1:]


HEADLINE, CHECKS, DISAGREE = [], {}, []


def claim(name, source, printed, claimed, recomputed, half=None, rel=None, definition="", cls="headline"):
    if rel is None:
        if half is None:
            _, half = num(printed)
        den = max(abs(claimed), abs(recomputed)) or 1.0
        rel = half / den
        ok = abs(claimed - recomputed) <= half * (1 + 1e-12)
    else:
        den = max(abs(claimed), abs(recomputed)) or 1.0
        ok = abs(claimed - recomputed) <= rel * den
    rec = {"name": name, "source": source, "printed": printed, "claimed": claimed,
           "recomputed": recomputed, "rel_tol": rel, "agrees": bool(ok), "definition": definition,
           "class": cls}
    HEADLINE.append(rec)
    if not ok:
        DISAGREE.append(rec)
    return ok


def check(name, ok):
    CHECKS[name] = bool(ok)
    return ok


# ================================================================ Stage A
SA = {}
for (p, s), c in STAGE_A.items():
    ex, ca, cb = c["arms"]
    assert (ex["solver"], ca["solver"], cb["solver"]) == ("macaulay", "chain", "chain-sparsest")
    n = c["calls"]
    for a in (ex, ca, cb):
        for k in ("call", "echelon", "normal_forms", "charpoly", "roots", "eigenvectors", "other", "wall_us"):
            assert len(a[k]) == n, (p, s, a["solver"], k)
    # A call fell back iff it cost more than the existing solver on the same call (a fallback pays
    # the chain attempt and then the whole existing solve); its count must equal arms[].fallbacks.
    fbA = [ca["call"][i] > ex["call"][i] for i in range(n)]
    fbB = [cb["call"][i] > ex["call"][i] for i in range(n)]
    clean = [i for i in range(n) if not fbA[i]]
    fbi = [i for i in range(n) if fbA[i]]
    SA[(p, s)] = dict(
        n=n,
        cmed=[umed(a["call"]) for a in (ex, ca, cb)],
        cmed_conv=[statistics.median(a["call"]) for a in (ex, ca, cb)],
        cmean=[mean(a["call"]) for a in (ex, ca, cb)],
        nf=[mean(a["normal_forms"]) for a in (ex, ca, cb)],
        cp=[mean(a["charpoly"]) for a in (ex, ca, cb)],
        eig=[mean(a["eigenvectors"]) for a in (ex, ca, cb)],
        ech=[mean(a["echelon"]) for a in (ex, ca, cb)],
        wall=[umed(a["wall_us"]) for a in (ex, ca, cb)],
        fb=[ex["fallbacks"], ca["fallbacks"], cb["fallbacks"]],
        fb_derived=[sum(fbA), sum(fbB)],
        same_fb_calls=fbA == fbB,
        nf_clean=[mean([ca["normal_forms"][i] for i in clean]), mean([cb["normal_forms"][i] for i in clean])],
        nf_clean_ex=mean([ex["normal_forms"][i] for i in clean]),
        ech_share_A=mean(ca["echelon"]) / mean(ca["call"]),
        cp_share_A=mean(ca["charpoly"]) / mean(ca["call"]),
        ech_share_A_clean=mean([ca["echelon"][i] for i in clean]) / mean([ca["call"][i] for i in clean]),
        cp_share_A_clean=mean([ca["charpoly"][i] for i in clean]) / mean([ca["call"][i] for i in clean]),
        fb_ex_cost=mean([ex["call"][i] for i in fbi]) if fbi else None,
        fb_overhead=(sum(ca["call"][i] for i in fbi) / sum(ex["call"][i] for i in fbi) - 1) if fbi else None,
        fb_share_B=sum(cb["call"][i] for i in fbi) / sum(cb["call"]),
        # Per-call identities that pin the chain path to the existing echelon on clean calls.
        clean_echelon_A_eq_existing=all(ca["echelon"][i] == ex["echelon"][i] for i in clean),
        clean_other_eq=all(ca["other"][i] == ex["other"][i] == cb["other"][i] for i in clean),
        clean_roots_eq=all(ca["roots"][i] == ex["roots"][i] == cb["roots"][i] for i in clean),
        clean_echelon_B_lt_A=all(cb["echelon"][i] < ca["echelon"][i] for i in clean),
        cp_eig_A_eq_B=all(ca["charpoly"][i] == cb["charpoly"][i] and ca["eigenvectors"][i] == cb["eigenvectors"][i] for i in range(n)),
        fb_echelon_gt=all(ca["echelon"][i] > ex["echelon"][i] for i in fbi),
        clean_call_lt=all(ca["call"][i] < ex["call"][i] and cb["call"][i] < ca["call"][i] for i in clean),
        summaries_consistent=all(
            sm["call_median"] == umed(a["call"]) and sm["fallbacks"] == a["fallbacks"]
            and abs(sm["call_mean"] - mean(a["call"])) < 1e-3 * max(1, sm["call_mean"])
            for sm, a in zip(c["summaries"], (ex, ca, cb))),
        agree=c["agree"], qa=c["quotient_agree"], qc=c["quotient_checked"],
        solved=[ex["solved"], ca["solved"], cb["solved"]],
    )

check("Stage A: ten curves, 1,000 calls each, three solvers on the same calls",
      sorted(SA) == sorted(CURVES) and all(v["n"] == 1000 for v in SA.values()))
check("Stage A: derived fallback calls (A's call above the existing call) number arms[].fallbacks, A and B on the same calls",
      all(v["fb_derived"] == [v["fb"][1], v["fb"][2]] and v["same_fb_calls"] and v["fb"][0] == 0 for v in SA.values()))
check("Stage A: on every clean call A's echelon count equals the existing solver's (same code path and count)",
      all(v["clean_echelon_A_eq_existing"] for v in SA.values()))
check("Stage A: on every clean call Weil + triangularise + checks and the root finder cost the same in all three solvers",
      all(v["clean_other_eq"] and v["clean_roots_eq"] for v in SA.values()))
check("Stage A: A and B charge identical charpoly and eigenvector counts on every call (unique normal forms)",
      all(v["cp_eig_A_eq_B"] for v in SA.values()))
check("Stage A: on every clean call B's echelon is below A's, and C_B < C_A < C_existing",
      all(v["clean_echelon_B_lt_A"] and v["clean_call_lt"] for v in SA.values()))
check("Stage A: on every fallback call A's echelon count exceeds the existing one (both runs charged)",
      all(v["fb_echelon_gt"] for v in SA.values()))
check("Stage A: summaries[] agree with the per-call arrays", all(v["summaries_consistent"] for v in SA.values()))
check("Stage A: solved counts equal across the three solvers on every curve",
      all(len(set(v["solved"])) == 1 for v in SA.values()))

# The note's Stage A table, cell by cell.
hdr, body = table_after("**Stage A** (per call")
for cells in body:
    p, s = int(cells[0]), int(cells[1])
    v = SA[(p, s)]
    src = f"{S923} Stage A table, p = {p} seed {s}"
    e, a, b = v["cmed"]
    vals = [
        ("C median existing", e), ("C median A", a), ("C median B", b),
        ("existing / A", e / a), ("existing / B", e / b),
        ("NF existing / A", v["nf"][0] / v["nf"][1]),
        ("charpoly existing / A", v["cp"][0] / v["cp"][1]),
        ("eigvec existing / A", v["eig"][0] / v["eig"][1]),
        ("echelon A / B", v["ech"][1] / v["ech"][2]),
        ("fallbacks A", v["fb"][1]),
    ]
    for (nm, rec), tok in zip(vals, cells[2:12]):
        c_, h_ = num(tok)
        claim(f"Stage A p={p} s={s}: {nm}", src, tok.strip("`"), c_, rec, half=h_,
              definition="medians s[n//2] of the per-call arrays; phase ratios are ratios of per-call means over all 1,000 calls, fallbacks charged")
    walls = [w.strip() for w in cells[12].split("/")]
    for nm, tok, rec in zip(("wall median existing", "wall median A", "wall median B"), walls, v["wall"]):
        c_, h_ = num(tok)
        claim(f"Stage A p={p} s={s}: {nm} (us)", src, tok.strip("`"), c_, rec, half=h_,
              definition="s[n//2] of wall_us; a practicality note, host-dependent", cls="practicality")

# Stage A prose and verdict ranges.
rng = lambda f: (min(f(v) for v in SA.values()), max(f(v) for v in SA.values()))


def claim_range(name, src, lo_p, hi_p, f_vals, scale=1.0, definition="", cls="headline"):
    lo, hi = min(f_vals), max(f_vals)
    c_lo, h_lo = num(lo_p)
    c_hi, h_hi = num(hi_p)
    claim(name + ", least", src, lo_p, c_lo * scale, lo, half=h_lo * scale, definition=definition, cls=cls)
    claim(name + ", most", src, hi_p, c_hi * scale, hi, half=h_hi * scale, definition=definition, cls=cls)


tot_calls = sum(v["n"] for v in SA.values())
claim("Stage A calls, total", S923 + " 'on all 10,000 calls each'", "10,000", 10000, tot_calls, half=0.5)
claim("Stage A calls where A agrees (solutions, degree, outcome, decomposition set)", S923, "10,000", 10000, sum(v["agree"][0] for v in SA.values()), half=0.5)
claim("Stage A calls where B agrees", S923, "10,000", 10000, sum(v["agree"][1] for v in SA.values()), half=0.5)
claim("Stage A calls the existing solver does not refuse (quotient check applies)", S923, "9,996", 9996, sum(v["qc"][0] for v in SA.values()), half=0.5)
claim("Stage A: M_P2 and charpoly identical, A", S923, "9,996", 9996, sum(v["qa"][0] for v in SA.values()), half=0.5)
claim("Stage A: M_P2 and charpoly identical, B", S923, "9,996", 9996, sum(v["qa"][1] for v in SA.values()), half=0.5)
check("Stage A: quotient checks apply to the same calls for A and B and all agree",
      all(v["qc"][0] == v["qc"][1] == v["qa"][0] == v["qa"][1] for v in SA.values()))
claim_range("A's normal forms on calls without fallback (10^3)", S923 + " 'A's normal forms cost 65.6–66.8·10³'",
            "65.6", "66.8", [v["nf_clean"][0] / 1e3 for v in SA.values()],
            definition="per-curve mean of A's nf_muls over calls whose A cost is not above the existing cost")
claim_range("B's normal forms on calls without fallback (10^3)", S923 + " 'and B's 31.6–32.2·10³'",
            "31.6", "32.2", [v["nf_clean"][1] / 1e3 for v in SA.values()])
claim("A's echelon share of mean C, all calls (≈70 %)", S923, "≈ 70 %", 0.70,
      mean([v["ech_share_A"] for v in SA.values()]), rel=0.10, cls="approximate",
      definition="mean over curves of mean(echelon_A)/mean(C_A); approximate claim, 10 % tolerance")
claim("A's charpoly share of mean C, all calls (≈15 %)", S923, "≈ 15 %", 0.15,
      mean([v["cp_share_A"] for v in SA.values()]), rel=0.10, cls="approximate")
claim("B's echelon per call (≈170·10³)", S923, "≈ 170·10³", 170e3, mean([v["ech"][2] for v in SA.values()]), rel=0.10, cls="approximate")
claim("B's charpoly per call (≈94·10³)", S923, "≈ 94·10³", 94e3, mean([v["cp"][2] for v in SA.values()]), rel=0.10, cls="approximate")
fbc = [v["fb_ex_cost"] / 1e6 for v in SA.values() if v["fb_ex_cost"] is not None]
claim_range("existing solver's cost on the fallback calls, all of them (10^6)", S923 + " 'Those calls cost the existing solver 4.5–4.8·10⁶ each'",
            "4.5", "4.8", fbc, cls="secondary",
            definition="per-curve mean of the existing call cost over all of Stage A's fallback calls, the existing solver's own refusals included")
fbc_rerun, refused_cost = [], []
for k, c in STAGE_A.items():
    ex, ca = c["arms"][0], c["arms"][1]
    fbi = [i for i in range(1000) if ca["call"][i] > ex["call"][i]]
    rr = [ex["call"][i] for i in fbi if ex["charpoly"][i] > 0]
    refused_cost += [ex["call"][i] for i in fbi if ex["charpoly"][i] == 0]
    if rr:
        fbc_rerun.append(mean(rr) / 1e6)
claim_range("existing solver's cost on the fallback calls it does not refuse (10^6)", S923 + " 'Those calls cost the existing solver 4.5–4.8·10⁶ each'",
            "4.5", "4.8", fbc_rerun, cls="secondary",
            definition="per-curve mean of the existing call cost over Stage A fallback calls on which the existing solver reaches its characteristic polynomial (its reruns)")
REFUSED = {"calls": len(refused_cost), "existing_cost_min": min(refused_cost) if refused_cost else None,
           "existing_cost_max": max(refused_cost) if refused_cost else None}
claim("chain attempt overhead on fallback calls (≈13 %)", S923, "≈ 13 %", 0.13,
      sum(sum(STAGE_A[k]["arms"][1]["call"][i] for i in range(1000) if STAGE_A[k]["arms"][1]["call"][i] > STAGE_A[k]["arms"][0]["call"][i]) for k in STAGE_A)
      / sum(sum(STAGE_A[k]["arms"][0]["call"][i] for i in range(1000) if STAGE_A[k]["arms"][1]["call"][i] > STAGE_A[k]["arms"][0]["call"][i]) for k in STAGE_A) - 1,
      rel=0.10, cls="approximate", definition="pooled over all Stage A fallback calls: sum C_A / sum C_existing - 1")
claim("fallback share of B's mean C at p = 101, most (≈16 %)", S923 + " Next levers", "≈ 16 %", 0.16,
      max(SA[(101, 1)]["fb_share_B"], SA[(101, 2)]["fb_share_B"]), rel=0.10, cls="approximate")
claim("fallback cost each (≈4.6·10⁶)", S923 + " Next levers", "≈ 4.6·10⁶", 4.6e6,
      mean([STAGE_A[k]["arms"][2]["call"][i] for k in STAGE_A for i in range(1000)
            if STAGE_A[k]["arms"][1]["call"][i] > STAGE_A[k]["arms"][0]["call"][i]]), rel=0.10, cls="approximate",
      definition="mean C_B over all Stage A fallback calls")
claim_range("C median existing (10^6)", S923 + " 'cuts one call from 1.62–1.66·10⁶'", "1.62", "1.66", [v["cmed"][0] / 1e6 for v in SA.values()])
claim_range("C median B (10^5)", S923 + " 'to 3.04–3.33·10⁵'", "3.04", "3.33", [v["cmed"][2] / 1e5 for v in SA.values()])
claim_range("C median A (10^5)", COMMIT + " '6.01-6.34e5 (A'", "6.01", "6.34", [v["cmed"][1] / 1e5 for v in SA.values()])
claim_range("G2.86 NF existing / A", S923 + " G2.86", "9.2", "12.4", [v["nf"][0] / v["nf"][1] for v in SA.values()])
claim("G2.86: curves with NF existing / A in the band [12, 40]", S923 + " G2.86 'in band on one curve of ten'", "1", 1,
      sum(12 <= v["nf"][0] / v["nf"][1] <= 40 for v in SA.values()), half=0.5)
claim("G2.86 NF existing / A on calls without fallback (≈13×)", S923 + " G2.86", "≈ 13", 13.0,
      mean([v["nf_clean_ex"] / v["nf_clean"][0] for v in SA.values()]), rel=0.10, cls="approximate")
claim_range("G2.87 charpoly existing / A", S923 + " G2.87", "2.82", "2.89", [v["cp"][0] / v["cp"][1] for v in SA.values()])
claim_range("G2.88 eigenvectors existing / A", S923 + " G2.88", "15.4", "30.0", [v["eig"][0] / v["eig"][1] for v in SA.values()])
claim_range("G2.89 C median existing / A", S923 + " G2.89", "2.62", "2.69", [v["cmed"][0] / v["cmed"][1] for v in SA.values()])
claim_range("C median existing / B", COMMIT + " '(B, sparsest pivots, 4.99-5.32x)'", "4.99", "5.32", [v["cmed"][0] / v["cmed"][2] for v in SA.values()])
claim_range("G2.90 Stage A fallbacks per curve (%)", S923 + " G2.90", "0.1", "1.2", [100 * v["fb"][1] / v["n"] for v in SA.values()])
claim("G2.90 Stage A fallbacks p = 101 s1 (%)", S923 + " G2.90 '(1.2 %, 1.1 % at p = 101)'", "1.2", 1.2, 100 * SA[(101, 1)]["fb"][1] / 1000)
claim("G2.90 Stage A fallbacks p = 101 s2 (%)", S923 + " G2.90", "1.1", 1.1, 100 * SA[(101, 2)]["fb"][1] / 1000)
claim_range("G2.91 wall median existing / A", S923 + " G2.91", "1.56", "1.67", [v["wall"][0] / v["wall"][1] for v in SA.values()], cls="practicality")
claim_range("G2.92 echelon A / B", S923 + " G2.92", "2.52", "2.65", [v["ech"][1] / v["ech"][2] for v in SA.values()])
claim_range("G2.92 B's C median below A's (%)", S923 + " G2.92 '49–50 % below A's'", "49", "50",
            [100 * (1 - v["cmed"][2] / v["cmed"][1]) for v in SA.values()], cls="secondary",
            definition="100 (1 - C median B / C median A) per curve")
check("G2.92 rule: B's C median >= 2 % below A's on every curve (B carried to end to end)",
      all(v["cmed"][2] <= 0.98 * v["cmed"][1] for v in SA.values()))
# Title / task-statement summaries: a one-decimal number that is the rounding of at least one
# per-curve value (it lies inside the range at its printed precision); the median is reported.
def summary_claim(label, val, vals, what):
    lo, hi = min(vals), max(vals)
    near = min(vals, key=lambda x: abs(x - val))
    claim(f"section title '{label}'", S923 + " title", str(val), val, near, half=0.05, cls="summary",
          definition=f"{what}: range {lo:.3f}–{hi:.3f}, median {statistics.median(vals):.3f}; recomputed = the value nearest the printed one")


summary_claim("2.6× (A) per call", 2.6, [v["cmed"][0] / v["cmed"][1] for v in SA.values()], "per-call C median ratio existing / A over the ten curves")
summary_claim("5.1× (B) per call", 5.1, [v["cmed"][0] / v["cmed"][2] for v in SA.values()], "per-call C median ratio existing / B over the ten curves")

# ================================================================ end to end
def rho_walks(r, fold):
    w = [x for x in r["rho"] if x["fold"] == fold]
    return w


E2E = {}
for key in ROWS:
    b = E25R[key]
    w = rho_walks(b, "Gls")
    rho_m = mean([x["fp_muls"] for x in w])
    rho_s = mean([x["steps"] for x in w])
    gae = b["fp_muls_per_add"]
    d = {"E25": b, "A": E31R["A"][key], "B": E31R["B"][key], "rho_muls": rho_m, "rho_steps": rho_s, "gae": gae, "n_gls": len(w)}
    for k in ("E25", "A", "B"):
        r = d[k]
        d[k + "_S"] = {g: r["total_muls"] + g * gae * r["macaulay_entries"] for g in GS}
        d[k + "_share"] = (r["weil_muls"] + r["solve_muls"]) / r["total_muls"]
    d["rho"] = {g: rho_m + g * gae * rho_s for g in GS}
    E2E[key] = d


def parts_sum(r):
    return (r["precompute_muls"] + r["base_muls"] + r["generation_muls"] + r["weil_muls"] + r["solve_muls"]
            + r["lift_muls"] + r["check_muls"] + r["la_fp_muls"])


def leaves(x, pre=""):
    if isinstance(x, dict):
        for k, v in x.items():
            yield from leaves(v, f"{pre}.{k}" if pre else k)
    elif isinstance(x, list):
        for i, v in enumerate(x):
            yield from leaves(v, f"{pre}[{i}]")
    else:
        yield pre, x


ALLOWED = {"call", "solve_muls", "total_muls", "ratio", "macaulay_entries", "solve_ms", "wall_ms"}
leaf_report = []
for key in ROWS:
    b = dict(leaves(E25R[key]))
    for k in ("A", "B"):
        r = dict(leaves(E31R[k][key]))
        extra = sorted(set(r) - set(b))
        missing = sorted(set(b) - set(r))
        diff = sorted(x for x in set(r) & set(b) if r[x] != b[x])
        bad = [x for x in diff if x.split(".")[0].split("[")[0] not in ALLOWED]
        leaf_report.append({"row": f"p={key[0]} s={key[1]} arm {key[2]} solver {k}", "leaves": len(r),
                            "differing": len(diff), "unexpected": bad, "added": extra, "missing": missing})
check("end to end: 40 records, each reproducing its E25 record leaf for leaf except the solve's cost, entries, timings and solver tags",
      len(leaf_report) == 40 and all(not x["unexpected"] and not x["missing"] and x["added"] == ["solver", "solver_fallbacks"] for x in leaf_report))
check("end to end: solver tags 'chain' / 'chain-sparsest' on every E31 record",
      all(E31R[k][key]["solver"] == SOLVERS[k] for k in SOLVERS for key in ROWS))
check("end to end: fallbacks = E25 reruns + unsolved on all 40 records",
      all(E31R[k][key]["solver_fallbacks"] == E25R[key]["reruns"] + E25R[key]["unsolved"] for k in SOLVERS for key in ROWS))
claim("fallbacks at p = 101 seed 1, arm A", S923 + " '488 = 467 + 21'", "488", 488, E31R["A"][(101, 1, "A")]["solver_fallbacks"], half=0.5)
claim("E25 reruns at p = 101 seed 1, arm A", S923, "467", 467, E25R[(101, 1, "A")]["reruns"], half=0.5)
claim("E25 unsolved at p = 101 seed 1, arm A", S923, "21", 21, E25R[(101, 1, "A")]["unsolved"], half=0.5)
check("end to end: fallback calls are charged — macaulay_entries rises over E25 exactly where fallbacks occurred, by at least one 393 x 448 matrix each",
      all(E31R[k][key]["macaulay_entries"] - E25R[key]["macaulay_entries"] >= 393 * 448 * E31R[k][key]["solver_fallbacks"] for k in SOLVERS for key in ROWS))
check("end to end: total_muls = sum of the charged phases on all 60 records (E25 and E31)",
      all(parts_sum(r) == r["total_muls"] for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])))
check("end to end: la_fp_muls = 16 * folded.la_ops on all 60 records",
      all(r["la_fp_muls"] == 16 * r["folded"]["la_ops"] for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])))
check("end to end: solve_muls + weil_muls = calls * mean C on all 60 records",
      all(abs(r["solve_muls"] + r["weil_muls"] - r["call"]["calls"] * r["call"]["mean"]) <= 1e-6 * r["total_muls"]
          for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])))
check("end to end: every log recovered (folded.solved and folded.correct) and every walk correct, 60 records",
      all(r["folded"]["correct"] and r["folded"].get("solved", True) and all(x["correct"] for x in r["rho"])
          for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])))
check("end to end: every kept row re-added, no check failures, no off-base decompositions, 60 records",
      all(r["rows_readded"] == r["folded"]["relations"] and r["rows_readd_failures"] == 0 and r["check_failures"] == 0 and r["off_base"] == 0
          for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])) )
check("matched walk: 16 <±psi> (Gls) walks per record, matched_fold = Gls, rho_matched_muls/steps = their means (recomputed from the walk list)",
      all(d["n_gls"] == 16 and all(r["matched_fold"] == "Gls" and abs(r["rho_matched_muls"] - d["rho_muls"]) < 1e-9 * d["rho_muls"]
                                    and abs(r["rho_matched_steps"] - d["rho_steps"]) < 1e-9 * d["rho_steps"]
                                    for r in (d["E25"], d["A"], d["B"])) for d in E2E.values()))
check("ratio = total_muls / rho_matched_muls on all 60 records",
      all(abs(r["ratio"] - r["total_muls"] / r["rho_matched_muls"]) <= 1e-9 * r["ratio"]
          for key in ROWS for r in (E25R[key], E31R["A"][key], E31R["B"][key])))
check("GAE = 97.0 F_p multiplications per affine addition on every record", all(d["gae"] == 97.0 for d in E2E.values()))

# The note's end-to-end table, cell by cell.
hdr, body = table_after("**End to end** (E25's ten curves")
for cells in body:
    p, s, arm = int(cells[0]), int(cells[1]), cells[2]
    d = E2E[(p, s, arm)]
    src = f"{S923} end-to-end table, p = {p} seed {s} arm {arm}"
    SE, SAx, SBx = d["E25_S"][0.0], d["A_S"][0.0], d["B_S"][0.0]
    vals = [("S E25", SE), ("S A", SAx), ("S B", SBx), ("E25 / A", SE / SAx), ("E25 / B", SE / SBx),
            ("S/rho E25", SE / d["rho_muls"]), ("S/rho A", SAx / d["rho_muls"]), ("S/rho B", SBx / d["rho_muls"])]
    for (nm, rec), tok in zip(vals, cells[3:11]):
        c_, h_ = num(tok)
        claim(f"e2e p={p} s={s} arm {arm}: {nm}", src, tok.strip("`"), c_, rec, half=h_,
              definition="S = total_muls (F_p multiplications, every phase, cold); rho = mean fp_muls of the record's 16 <±psi> walks")

# End-to-end prose and band.
def rows_range(f):
    v = [f(E2E[k]) for k in ROWS]
    return v


claim_range("G2.93 S_E25 / S_A at 0 GAE", S923 + " §5a band / G2.93", "2.50", "2.56", rows_range(lambda d: d["E25_S"][0] / d["A_S"][0]))
claim_range("S_E25 / S_A at 0.1 GAE", S923 + " §5a band / G2.94", "1.39", "1.43", rows_range(lambda d: d["E25_S"][0.1] / d["A_S"][0.1]))
claim_range("S_E25 / S_A at 1 GAE", S923 + " §5a band / G2.94", "1.034", "1.053", rows_range(lambda d: d["E25_S"][1] / d["A_S"][1]))
claim_range("S_E25 / S_B at 0 GAE", S923 + " §5a band", "4.53", "4.76", rows_range(lambda d: d["E25_S"][0] / d["B_S"][0]))
claim_range("S_E25 / S_B at 0.1 GAE", S923 + " §5a band", "1.59", "1.64", rows_range(lambda d: d["E25_S"][0.1] / d["B_S"][0.1]))
claim_range("S_E25 / S_B at 1 GAE", S923 + " §5a band", "1.051", "1.071", rows_range(lambda d: d["E25_S"][1] / d["B_S"][1]))
claim_range("solve share of S, solver A (%)", S923 + " 'The solves are now 98.4–98.9 % of S (A)'", "98.4", "98.9",
            rows_range(lambda d: 100 * d["A_share"]), definition="(weil_muls + solve_muls) / total_muls, i.e. sum of C / S")
claim_range("solve share of S, solver B (%)", S923 + " 'and 97.0–97.9 % (B)'", "97.0", "97.9", rows_range(lambda d: 100 * d["B_share"]))
claim_range("E25 solve share of S, (Weil + solve) / S (%)", NOTE + " §9.22 'the solve is 99.0–99.4 % of S' (quoting the red team)", "99.0", "99.4",
            rows_range(lambda d: 100 * d["E25_share"]), cls="secondary", definition="(weil_muls + solve_muls) / total_muls of the E25 records")
claim_range("E25 solve share of S, solve_muls / S (%)", NOTE + " §9.22 'the solve is 99.0–99.4 % of S' (quoting the red team)", "99.0", "99.4",
            rows_range(lambda d: 100 * d["E25"]["solve_muls"] / d["E25"]["total_muls"]), cls="secondary",
            definition="solve_muls / total_muls of the E25 records (the solve without the Weil restriction)")
fb_pct = [100 * E31R[k][key]["solver_fallbacks"] / E31R[k][key]["residuals"] for k in SOLVERS for key in ROWS]
claim_range("G2.90 end-to-end fallbacks per run (%)", S923 + " G2.90 '0.2–1.1 % per end-to-end run' (also 'Next levers')", "0.2", "1.1", fb_pct,
            cls="secondary", definition="solver_fallbacks / residuals per record (each residual is one solver call)")
check("§5a: S_A < S_E25 and S_B < S_E25 on every row at 0, 0.1 and 1 GAE",
      all(d["A_S"][g] < d["E25_S"][g] and d["B_S"][g] < d["E25_S"][g] for d in E2E.values() for g in GS))
check("§5a: S_B < S_A on every row at 0, 0.1 and 1 GAE (under E25's native-operation model, which does not see B's pivot-selection scans)",
      all(d["B_S"][g] < d["A_S"][g] for d in E2E.values() for g in GS))
check("§5a: the matched <±psi> rho is below every row of E25, A and B at 0, 0.1 and 1 GAE",
      all(d["rho"][g] < d[k + "_S"][g] for d in E2E.values() for g in GS for k in ("E25", "A", "B")))
check("G2.93: 'the gap to the matched rho shrinks by the same factor' (identical walks, so (S/rho)_E25/(S/rho)_X = S_E25/S_X exactly)",
      all(d["E25"]["rho"] == d["A"]["rho"] == d["B"]["rho"] for d in E2E.values()))
srb = rows_range(lambda d: d["B_S"][0] / d["rho_muls"])
claim_range("distance remaining, solver B rows: S/rho", S923 + " 'every row is still 2.39·10³–1.52·10⁵× slower'", "2.39·10³", "1.52·10⁵", srb,
            definition="over the 20 solver-B rows (B is the claimed new baseline); solver-A rows reach "
                       f"{max(rows_range(lambda d: d['A_S'][0] / d['rho_muls'])):.3e}")
best = min(ROWS, key=lambda k: E2E[k]["B_S"][0] / E2E[k]["rho_muls"])
check("the best solver-B row is p = 397 seed 1, arm B", best == (397, 1, "B"))
claim("E25's best S/rho", S923 + " 'E25's best was 1.13·10⁴×'", "1.13·10⁴", 1.13e4, min(rows_range(lambda d: d["E25_S"][0] / d["rho_muls"])), half=0.005e4)
claim("B at 1 GAE: improvement over E25, least (%)", S923 + " 'at 1 GAE per Macaulay entry it is 5–7 %'", "5", 5.0,
      100 * (min(rows_range(lambda d: d["E25_S"][1] / d["B_S"][1])) - 1), half=0.5)
claim("B at 1 GAE: improvement over E25, most (%)", S923, "7", 7.0, 100 * (max(rows_range(lambda d: d["E25_S"][1] / d["B_S"][1])) - 1), half=0.5)
for label, val, k in (("S by 2.5× (A)", 2.5, "A"), ("S by 4.7× (B)", 4.7, "B")):
    summary_claim(label, val, rows_range(lambda d: d["E25_S"][0] / d[k + "_S"][0]), f"S_E25 / S_{k} at 0 GAE over the 20 rows")

# ================================================================ section 9.22 step-1 profile table (secondary)
PROFILE = {(r["p"], r["seed"]): r for r in load(f"{E31}/profile_p269_p397.json")}
RAW.append(f"{E31}/profile_p269_p397.json")
m22 = re.search(r"^### 9\.22 .*?(?=^### |\Z)", NOTE_TEXT, re.S | re.M).group(0)
i22 = m22.index("| phase, per call (mean)")
prof_lines = [ln for ln in m22[i22:].splitlines() if ln.startswith("|")]
PHASE_KEY = {"Weil restriction": "weil", "triangularise": "triangularise", "echelon": "echelon", "normal forms": "normal_forms",
             "characteristic polynomial": "charpoly", "rational roots": "roots", "eigenvectors": "eigenvectors",
             "solution checks": "verify", "lift-back": "lift_back"}
prof_cols = [(269, 1), (269, 2), (397, 1), (397, 2)]
for ln in prof_lines[2:]:
    cells = [c.strip() for c in ln.strip().strip("|").split("|")]
    key = next((v for k, v in PHASE_KEY.items() if cells[0].startswith(k) or cells[0].startswith(k.split()[0]) and k == cells[0][:len(k)]), None)
    if key is None:
        continue
    for (p, s), tok in zip(prof_cols, cells[1:5]):
        ph = PROFILE[(p, s)]["phases"]
        c_, h_ = num(tok)
        claim(f"§9.22 profile p={p} s={s}: {cells[0]} per call", NOTE + " §9.22 profile table", tok.strip("`"), c_,
              ph[key] / ph["calls"], half=h_, cls="secondary", definition="phase sum / calls in profile_p269_p397.json")

# ================================================================ the registered predictions (G2.85-G2.94), recomputed
sa_vals = list(SA.values())
inb = lambda x, lo, hi: lo <= x <= hi
V = {}
V["G2.85"] = ("holds" if all(v["agree"] == [v["n"], v["n"]] and v["qa"] == v["qc"] for v in sa_vals)
              and all(not x["unexpected"] for x in leaf_report) else "falsified")
V["G2.86"] = "holds" if all(inb(v["nf"][0] / v["nf"][1], 12, 40) for v in sa_vals) else "falsified low" if all(v["nf"][0] / v["nf"][1] <= 40 for v in sa_vals) else "falsified"
V["G2.87"] = "holds" if all(inb(v["cp"][0] / v["cp"][1], 1.8, 3.2) for v in sa_vals) else "falsified"
V["G2.88"] = "holds" if all(inb(v["eig"][0] / v["eig"][1], 15, 80) for v in sa_vals) else "falsified"
V["G2.89"] = ("holds" if all(inb(v["cmed"][0] / v["cmed"][1], 2.2, 3.0) for v in sa_vals) else "falsified") + \
             ("; kill reached" if any(v["cmed"][0] / v["cmed"][1] < 1.5 for v in sa_vals) else "; kill not reached")
sa_over = sorted({p for (p, s), v in SA.items() if v["fb"][1] / v["n"] > 0.01})
e2e_over = sorted({key[0] for k in SOLVERS for key in ROWS if E31R[k][key]["solver_fallbacks"] / E31R[k][key]["residuals"] > 0.01})
V["G2.90"] = "holds" if not sa_over and not e2e_over else f"falsified at p in {sorted(set(sa_over) | set(e2e_over))}, holds at the others"
V["G2.91"] = "holds" if all(inb(v["wall"][0] / v["wall"][1], 1.2, 2.0) for v in sa_vals) else "falsified"
V["G2.92"] = "holds" if all(inb(v["ech"][1] / v["ech"][2], 1.0, 1.6) for v in sa_vals) else "falsified high" if all(v["ech"][1] / v["ech"][2] > 1.6 for v in sa_vals) else "falsified"
V["G2.93"] = "holds" if all(not x["unexpected"] for x in leaf_report) and all(inb(d["E25_S"][0] / d["A_S"][0], 2.1, 2.9) for d in E2E.values()) else "falsified"
V["G2.94"] = "holds" if all(inb(d["E25_S"][0.1] / d["A_S"][0.1], 1.3, 1.6) and inb(d["E25_S"][1] / d["A_S"][1], 1.03, 1.10)
                             and all(d["A_S"][g] < d["E25_S"][g] for g in GS) for d in E2E.values()) else "falsified"
NOTE_VERDICTS = {"G2.85": "holds", "G2.86": "falsified low", "G2.87": "holds", "G2.88": "holds", "G2.89": "holds; kill not reached",
                 "G2.90": "falsified at p in [101], holds at the others", "G2.91": "holds", "G2.92": "falsified high",
                 "G2.93": "holds", "G2.94": "holds"}
for g, want in NOTE_VERDICTS.items():
    check(f"{g}: recomputed verdict '{V[g]}' equals the note's verdict", V[g] == want)
check("G2.90: Stage A fallbacks above 1 % only at p = 101, end-to-end runs above 1 % only at p = 101 (all four)",
      sa_over == [101] and e2e_over == [101]
      and sum(1 for k in SOLVERS for key in ROWS if key[0] == 101 and E31R[k][key]["solver_fallbacks"] / E31R[k][key]["residuals"] > 0.01) == 8)

# ================================================================ validator assessment: B's unlisted native work (§5a)
# echelon_by_lead_with(.., sparsest = true) counts the nonzeros of every row before eliminating
# (one read per Macaulay entry of every matrix the chain attempt builds) and recounts each updated
# row's tail; neither pass is emitted.  A lower bound on the first pass, exact from the records:
# on a clean call the chain attempt's degree-11 matrix is E25's matrix, so the clean calls' entries
# are E25's entries minus the fallback calls' (final-degree) entries = 2 E_E25 - E_E31.  Priced
# like an entry write (one native operation per entry read), as the band prices the matrix.
B_NNZ = []
for key in ROWS:
    d = E2E[key]
    lb = 2 * d["E25"]["macaulay_entries"] - d["B"]["macaulay_entries"]
    sb = {g: d["B_S"][g] + g * d["gae"] * lb for g in GS}
    B_NNZ.append({"p": key[0], "seed": key[1], "arm": key[2], "nnz_pass_lower_bound": lb,
                  "S_B_with_pass": {str(g): sb[g] for g in GS},
                  "S_E25_over_S_B_with_pass": {str(g): d["E25_S"][g] / sb[g] for g in GS},
                  "S_A_over_S_B_with_pass": {str(g): d["A_S"][g] / sb[g] for g in GS},
                  "B_below_E25": {str(g): sb[g] < d["E25_S"][g] for g in GS},
                  "B_below_A": {str(g): sb[g] < d["A_S"][g] for g in GS}})
B_NNZ_SUMMARY = {
    "what": "validator assessment, not a claim of the note: S_B(g) with B's pivot-selection nonzero-count pass priced at one native operation per Macaulay entry read, "
            "lower bound 2 E_E25 - E_E31 entries per record (the per-updated-row recounts are not included: not emitted, not bounded here)",
    "rows_B_below_E25": {str(g): sum(r["B_below_E25"][str(g)] for r in B_NNZ) for g in GS},
    "rows_B_below_A": {str(g): sum(r["B_below_A"][str(g)] for r in B_NNZ) for g in GS},
    "S_E25_over_S_B_with_pass_range": {str(g): [min(r["S_E25_over_S_B_with_pass"][str(g)] for r in B_NNZ),
                                                max(r["S_E25_over_S_B_with_pass"][str(g)] for r in B_NNZ)] for g in GS},
    "rows": B_NNZ,
}

# ================================================================ band rows and summary
band_rows = []
for key in ROWS:
    d = E2E[key]
    band_rows.append({
        "p": key[0], "seed": key[1], "arm": key[2], "gae_fp_muls": d["gae"],
        "rho_psi_muls_mean": d["rho_muls"], "rho_psi_steps_mean": d["rho_steps"],
        "macaulay_entries": {k: d[k]["macaulay_entries"] for k in ("E25", "A", "B")},
        "S": {k: {str(g): d[k + "_S"][g] for g in GS} for k in ("E25", "A", "B")},
        "rho_psi": {str(g): d["rho"][g] for g in GS},
        "S_E25_over_S": {k: {str(g): d["E25_S"][g] / d[k + "_S"][g] for g in GS} for k in ("A", "B")},
        "S_A_over_S_B": {str(g): d["A_S"][g] / d["B_S"][g] for g in GS},
        "S_over_rho": {k: {str(g): d[k + "_S"][g] / d["rho"][g] for g in GS} for k in ("E25", "A", "B")},
    })
med = lambda xs: statistics.median(xs)
summary_rows = [E2E[(p, s, "B")] for p, s in CURVES]
gae_summary = {str(g): {"ic": med([d["B_S"][g] for d in summary_rows]), "rho": med([d["rho"][g] for d in summary_rows])} for g in GS}
nnz_by_key = {(r["p"], r["seed"], r["arm"]): r for r in B_NNZ}
gae_summary_with_pass = {str(g): {"ic": med([nnz_by_key[(p, s, "B")]["S_B_with_pass"][str(g)] for p, s in CURVES]),
                                  "rho": med([E2E[(p, s, "B")]["rho"][g] for p, s in CURVES])} for g in GS}
check("the matched rho is below S_B with B's nonzero-count pass priced, every row, every weight",
      all(E2E[(r["p"], r["seed"], r["arm"])]["rho"][g] < r["S_B_with_pass"][str(g)] for r in B_NNZ for g in GS))

stage_a_rows = []
for (p, s), v in sorted(SA.items()):
    stage_a_rows.append({"p": p, "seed": s, "C_median": dict(zip(("existing", "A", "B"), v["cmed"])),
                         "C_median_conventional": dict(zip(("existing", "A", "B"), v["cmed_conv"])),
                         "C_mean": dict(zip(("existing", "A", "B"), v["cmean"])),
                         "existing_over_A": v["cmed"][0] / v["cmed"][1], "existing_over_B": v["cmed"][0] / v["cmed"][2],
                         "fallbacks": v["fb"][1], "fallback_share_of_B_C": v["fb_share_B"],
                         "nf_clean_A": v["nf_clean"][0], "nf_clean_B": v["nf_clean"][1],
                         "wall_median_us": dict(zip(("existing", "A", "B"), v["wall"]))})

# ================================================================ code-read and protocol assessments (validator's reading, 2026-10-24)
n_clean = sum(v["n"] - v["fb"][1] for v in SA.values())
n_fb = sum(v["fb"][1] for v in SA.values())
COUNTING = {
    "verdict": "honest: every F_p multiplication the chain solve performs is counted in the existing solver's unit; no uncounted per-curve or per-call precomputation; fallbacks charged in full",
    "read_at": "src at 04a33428b9 = a8f52422d5 (git diff empty over src/, Cargo.*, the two examples)",
    "findings": [
        "klein_v4.rs:1137-1215 build_echelon: the existing row construction, triangularise (same function, counted) and echelon_by_lead_with(rows, p, &mut ech_muls, false, sparsest); for A this is exactly the existing echelon_by_lead (gaudry_quartic.rs:1099-1101 delegates with sparsest = false). "
        f"Per-call identity verified on all {n_clean} clean Stage A calls: echelon_A == echelon_existing, and Weil + triangularise + checks and roots equal in all three solvers.",
        "klein_v4.rs:1515-1612 chain_quotient: left-looking normal forms of the 12 chain-end targets, one counted multiplication per nonzero right of each substituted (normalised) pivot row (line 1585 / 1589); the structural reachability DFS is index work, as in the existing NormalForms4::ensure.",
        "klein_v4.rs:1404-1439 det_small, 1444-1496 kernel_small, 1616-1641 relation_at, 1646-1697 charpoly_by_chains: every mm(...) is followed by a counter increment; each inversion is charged inv_charge(p) = 2 * bitlen(p - 1) = 2 ceil(log2 p) (14 at p = 101, 18 at p = 397). The existing solver charges none of its echelon, triangularise or extraction inversions, so this is conservative against the chain solve.",
        "klein_v4.rs:1765-1808 eigen extraction: the lambda powers are counted, and the inversion plus 4 multiplications are charged (the existing extract_points charges neither); the solution check uses the same eval, counted in verify_muls.",
        "klein_v4.rs:1843-1870 fallback: the RNG is restored to its entry state, solve_weighted_traced is rerun, and all seven phase counters of the chain attempt are added back; weil_muls is kept once (the system is not rebuilt). quartic_folds.rs:2030 call_muls = st.total_muls -> solve_muls -> total_muls (1279-1290, 1367-1374); entries doubled for a fallback (2036). "
        f"Stage A: {n_fb} fallbacks, each costing more than the existing call on the same input (check above).",
        "No statics, thread-locals, caches or once-cells in the E31 section (lines 1119-1944); solve_weighted_chain takes only (comps, d, p, rng, st, sparsest). No per-curve precomputation exists to be charged.",
        "chain_quotient_agrees (1906-1944) counts into a scratch variable, but it is a diagnostic called only by e31_stage_a and the unit test, outside every counted call and outside S.",
        "Additions and negations are uncounted, as in the existing solver: the unit is F_p multiplications (inversions as 2 ceil(log2 p) multiplications in the new phases only).",
    ],
    "native_work": [
        "B (sparsest = true) adds native work the band does not see: gaudry_quartic.rs:1130-1136 counts the nonzeros of every row of every built matrix (one read per Macaulay entry) and 1181-1185 recounts each updated row's tail; neither is emitted. "
        "Priced at the band's own per-entry rate, the first pass alone (lower bound 2 E_E25 - E_E31 entries, exact for the clean calls) puts S_B above S_E25 on all 20 rows at 0.1 GAE (S_E25/S_B 0.885-0.899) and at 1 GAE (0.542-0.544); see b_native_work_assessment. "
        "Under AGENTS.md section 5a the order of B against E25 and against A therefore holds at 0 GAE only and is unresolved across the band until the pass (and the echelon's dense/sparse traffic) is emitted. B's sparser pivots may also save native traffic in the echelon's sparse path (not emitted either). "
        "Wall clock (practicality note): Stage A wall median existing/B 1.53-1.74 and B/A 0.96-1.04, i.e. on this host B's halving of A's counted multiplications buys no wall time over A.",
        "A builds and eliminates the identical matrix with the identical code and adds no pass over the matrix (the registered model's 'unchanged by A' holds for the Macaulay entries). Its new phases change per-phase native traffic that the model emits for no arm: the left-looking normal forms re-read a pivot-row tail once per target reaching it (profile: reached_sum ~1,020 per call against reached_union ~240), "
        "while the existing normal forms read each reached tail twice and run a dense 64-wide axpy per nonzero non-standard entry (~15.1e3 per call; ~9.7e5 element updates of which ~8.5e5 are the counted multiplications). The validator's rough estimate puts the two NF phases' uncounted traffic at the same order (~1-2e5 per call); not emitted, so not a measurement.",
        "Consequence for the band: at 1 GAE A's margin over E25 is 3.4-5.3 % of S_E25(1), about 7-10e3 native operations per call, smaller than either arm's unpriced per-phase traffic; the 1-GAE ordering (G2.94) therefore holds under the registered model and is not resolved beyond it. At 0.1 GAE the margin (~1e5 per call) is of the order of that traffic's uncertainty for A and is exceeded by B's full nonzero-count pass. Wall clock (practicality, host-dependent) supports A and B below E25 in practice: existing/A 1.56-1.67 and existing/B 1.53-1.74 recorded, 1.31-1.62 and 1.28-1.74 in the validator's loaded replay.",
    ],
}
PREREG = {
    "E31 (G2.85-G2.94, note section 9.22)": "registered in c1c5378059 (2026-10-24 04:04:15 UTC), after the step-1 profile a2de57077a (04:03:32) and before the solver code f6ab6b7253 (04:19:36), the Stage A harness a8f52422d5 (04:20:51) and the results 04a33428b9 (05:41:09). Section 9.22 at c1c5378059 and at 14fcda1621 differ only by one trailing blank line, so the bands were not moved after the data.",
    "deviations": "the registration says det R(x) is interpolated from 65 points; the code evaluates det R(x) - x^64 at 64 points (degree <= 63), which is equivalent. B was registered as exploratory and its section 5a band was not pre-registered; the registered band model ('one GAE per Macaulay entry, unchanged by A') was written for A.",
    "how": "git log / git show of the note, diff of the section 9.22 text at both commits, read by the validator on 2026-10-24",
}
RHO_ASSESS = {
    "kind": "negation_frobenius",
    "protocol": "note section 9.15 (E25, registered before code) as inherited by section 9.22/9.23: on each GLS curve the matched walk is the <±psi> walk (psi the twisted p^2-power Frobenius, psi^2 = -1), 16 <±psi>, 16 negation and 16 unfolded walks per (p, seed), all in the same process as the index calculus; S/rho = total_muls / mean fp_muls of the 16 <±psi> walks.",
    "matches_protocol": "every E31 record carries the identical 48 walks of its E25 record (leaf comparison), matched_fold = Gls, rho_matched_muls / steps equal the means recomputed from the walk list; all walks correct",
    "strongest_available": "the E31 instances are E25's (identical n, group order, cofactor and base points); the E25 validator checked independently that j(E) is in F_p2 but not F_p on all ten curves, so psi is the only Frobenius-type endomorphism over F_p4 and Aut(E) = {±1}: <±psi> is the largest fold (benchmarks/20261018-k4-gls-v4/replay-certificate.json instance_checks)",
    "same_unit": "F_p multiplications on both arms (the same Fp4 counters: an F_p4 multiplication 19, an inversion 40; the solve's F_p multiplications counted directly; Wiedemann at 16 per multiplication mod n); GAE = 97.0 F_p multiplications per affine addition, measured per record",
    "same_binary_or_process": "examples/quartic_pipeline_gls --solver {chain|chain-sparsest}: one process per (p, seed) runs arm A then arm B, each with its 48 walks on the same instance and counters",
    "band": "S(g) = total_muls + g * 97 * macaulay_entries (IC), S_rho(g) = rho_matched_muls + g * 97 * rho_matched_steps; g = 0, 0.1, 1. The matched walk is below every row of E25, A and B at every weight by 10^3-10^6x; it stays so with B's nonzero-count pass priced too, and no unpriced IC traffic can reverse it (more native work only raises S_IC). The walk's own unpriced work (hash lookups/inserts, partition hashes) is priced at one GAE per step.",
    "order_holds_across_band": True,
}
LIMITS = [
    "Scope: E25's ten GLS twists over F_p4, p = 101-397 (n = 2^22.6-2^30.5), the degree-11 V4 solve, both arms; operation counts in F_p multiplications. Nothing here is an asymptotic or exponent claim (class engineering: a constant on the dominant phase). E24's curves share the solver; only the unit test covers two of them.",
    "Out of sample (validator's own runs, replay/agreement_comparison.json): three new GLS curves (113,4), (229,3), (353,3) agree on all 3,000 Stage A calls; end to end on (113,4) every non-solve field matches the existing solver and S falls 2.49-2.50x (A) and 4.48-4.51x (B), just below the in-sample B range 4.53-4.76x; the per-call ratio existing/B drifts down with p (5.29 at p = 113 to 4.96 at p = 353). Extrapolation beyond p = 397 is not measured.",
    "Thread count: the end-to-end target stream depends on RAYON_NUM_THREADS (4 here and on the recording host); a different count gives a different but equally valid stream. Stage A is one-threaded.",
    "Host: one 4-vCPU Xeon @ 2.80 GHz container, shared with other jobs during the replay (load average up to ~11); wall-clock fields differ from the record (replay Stage A existing/A wall ratio 1.31-1.62 against the recorded 1.56-1.67, outside the printed G2.91 range on 8 of 10 curves but inside the registered band [1.2, 2.0]) and are a practicality note only.",
    "Section 5a: the band uses E25's native-operation model (one per Macaulay entry, one per walk step), which counts no arm's per-phase scans; B's pivot-selection pass is an unlisted counter specific to B (see counting_honesty.native_work), so B's ranking over E25 and A beyond 0 GAE is unresolved; A's 1-GAE margin is below the model's resolution. E25's convention charges a rerun's entries at its final degree only; E31 inherits it and charges a fallback's extra matrix at that size too.",
    "Medians: Stage A medians are the harness's s[n // 2] (upper median of 1,000); the conventional median differs by at most 35.5 multiplications on any curve and solver and decides nothing.",
    "Labelling: the E31 JSON records carry 'experiment': 'e25' (the driver's fixed tag); they are distinguished only by the 'solver' field and the file name.",
    "Stage A records no per-call fallback flag; fallback calls are identified as those where A's call costs more than the existing call, which reproduces arms[].fallbacks exactly on every curve.",
]
VREPORT = {
    "id": "VAL-20261024-24ef8b",
    "task_id": None,
    "task_note": "dispatched directly by the main session on branch claude/glv-endomorphism-index-calculus-pe7ige at 04a33428b9; no TASK card. The snapshot validated is the committed tree (04a33428b9, data unchanged at 14fcda1621).",
    "run_ids": ["experiments/31_solve_cost/stage_a.json", "experiments/31_solve_cost/e2e_chain_p*s*.json (20 arm records)",
                "experiments/31_solve_cost/e2e_chain-sparsest_p*s*.json (20 arm records)", "experiments/30_gls_v4/p*s*.json (E25 baseline, 20 arm records)"],
    "artifact_checks": ["all raw files present and sha256-pinned in raw_data; binary_commit.txt = a8f52422d5; build inputs at HEAD identical to it",
                        "replay certificate: see ../replay-certificate.json"],
    "metric_recomputations": "every cell of both section 9.23 tables and every stated range recomputed from the raw JSON (headline, secondary); see headline, errata",
    "control_checks": ["positive/agreement: Stage A 10,000 calls x2 and the 40 end-to-end records reproduce the existing solver's outputs; the validator's own 3,000 calls and 4 end-to-end arm records on new curves agree",
                       "A/A: the existing solver run through the E31 dispatch reproduces E25 p = 101 s2 exactly",
                       "negative control: the unit test's one-coefficient perturbation changes every non-empty solution set (unit_test.log)"],
    "heuristic_validation_checks": "not applicable (no heuristic distribution is validated)",
    "cost_model_checks": ["unit declared: F_p multiplications, inversions in the new phases at 2 ceil(log2 p)", "every phase in S, cold, fallbacks charged",
                          "section 5a band at 0, 0.1, 1 GAE (97 F_p multiplications) on both arms and the matched rho; B's band ranking qualified (native_work)",
                          "memory: not reported by the producer (the 393 x 448 u16 matrix per call is unchanged by A and B)"],
    "proof_architecture_checks": "not applicable",
    "verdict": "passed",
    "verdict_note": "the receipts are admissible: replay identical, counts honest, outputs identical, numbers recomputed. Not a speedup or promotion decision. Errata to the prose are listed under 'errata'; B's section 5a ranking beyond 0 GAE is unresolved (native_work).",
    "limitations": "see limitations",
    "artifact_paths": ["benchmarks/20261024-e31-chain-solve/VALIDATION.json", "benchmarks/20261024-e31-chain-solve/replay-certificate.json",
                       "benchmarks/20261024-e31-chain-solve/validator-report.json", "benchmarks/20261024-e31-chain-solve/validate.py",
                       "benchmarks/20261024-e31-chain-solve/replay/"],
    "review_attestation": {
        "joints_owned": ["counting honesty", "replay", "agreement", "recomputation and section 5a band", "limitations"],
        "sources_read": ["agents/validator.md (crypto-autoresearcher)", "AGENTS.md (standing rules, success criterion, sections 2-6)", "docs/VALIDATION_GATE.md",
                         "benchmarks/20261018-k4-gls-v4/{VALIDATION.json, replay-certificate.json, validator-report.json, validate.py}",
                         "research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md sections 9.22-9.23",
                         "src/cryptanalysis/klein_v4.rs (1-1117 solver core, 1119-2122 E31)", "src/cryptanalysis/gaudry_quartic.rs (1020-1360)",
                         "src/cryptanalysis/quartic_folds.rs (collect_and_solve 1225-1420, v4_decompose 1933-2044, generate_gls_klein_instance head, E31 diff)",
                         "examples/e31_stage_a.rs", "examples/quartic_pipeline_gls.rs (E31 diff)", "experiments/31_solve_cost/* (JSON, logs, tables.md, run_e2e.sh, profile_p269_p397.json)",
                         "experiments/30_gls_v4/p*s*.json", "scripts/validation_gate.py (evidence checks)",
                         "research/e31_chain_solve_20261024/REPORT.typ lines 150-222 and README.md (producer's report, read as claim text after it landed mid-session, not as evidence)"],
        "read_sibling_reports": False,
        "verdict": "passed (receipts admissible; prose errata and a section 5a qualification for B)",
    },
}

# ================================================================ write
DECIDING = {"headline", "approximate", "summary", "practicality"}
deciding_dis = [d for d in DISAGREE if d["class"] in DECIDING]
errata = [d for d in DISAGREE if d["class"] not in DECIDING]
status = "pass" if not deciding_dis and all(CHECKS.values()) else "fail"
pins = [{"path": os.path.relpath(rp(x), HERE), "sha256": sha(x)} for x in RAW]
claim_pins = [{"path": os.path.relpath(rp(x), HERE), "sha256": sha(x)} for x in (NOTE, f"{E31}/tables.md")]
out = {
    "schema": "validator-report/v1",
    "status": status,
    "independent": True,
    "validator": "validator subagent (review-adversarial policy), independent of the session that produced E31; "
                 "recompute in this stdlib script, which shares no code with examples/e31_tables.rs or examples/e31_stage_a.rs",
    "date": "2026-10-24",
    "row": "E31 (GLV note §9.23): the P2-chain solve (A) and chain with sparsest pivots (B) on E25's ten GLS curves, both arms; no benchmarks/README.md row exists yet",
    "script": "validate.py",
    "tolerance_rule": "a printed claim agrees when |claimed - recomputed| <= half a unit in its last printed digit (integers 0.5); "
                      "rel_tol = that half unit / max(|claimed|, |recomputed|); '≈' claims within 10 % (class approximate); "
                      "title summaries: the one-decimal value within 0.05 of the median over curves/rows (class summary)",
    "raw_data": pins,
    "claim_text_pins": claim_pins,
    "status_rule": "pass iff every claim of class headline, approximate, summary or practicality agrees at its printed precision and every check holds "
                   "(the checks include the recomputed G2.85-G2.94 verdicts). Class 'secondary' = supporting prose figures outside the two section 9.23 "
                   "tables and the main claims; they are recomputed and listed, a disagreement is reported under 'errata' and needs an additive erratum, "
                   "and it does not decide the status because no table cell, headline range or registered verdict depends on it.",
    "headline_count": sum(1 for h in HEADLINE if h["class"] in DECIDING),
    "headline_disagreements": len(deciding_dis),
    "secondary_count": sum(1 for h in HEADLINE if h["class"] not in DECIDING),
    "errata_count": len(errata),
    "headline": [h for h in HEADLINE if h["class"] in DECIDING],
    "secondary": [h for h in HEADLINE if h["class"] not in DECIDING],
    "checks": CHECKS,
    "disagreements": deciding_dis,
    "errata": errata,
    "existing_solver_refusals_among_stage_a_fallbacks": REFUSED,
    "verdicts_recomputed": V,
    "leaf_comparison_e31_vs_e25": leaf_report,
    "stage_a_rows": stage_a_rows,
    "gae_band_rows": band_rows,
    "b_native_work_assessment": B_NNZ_SUMMARY,
    "counting_honesty": COUNTING,
    "preregistration": PREREG,
    "rho_reference_assessment": RHO_ASSESS,
    "limitations": LIMITS,
    "validation_report": VREPORT,
    "gae_band_summary": {"what": "median over the 10 curves of solver B, arm B (the GLS-fold arm, E25's benchmark arm): S_IC(g) and the matched <±psi> walk's S_rho(g), F_p multiplications, g GAE (97 F_p multiplications) per unpriced native operation (one per Macaulay entry, one per walk step)",
                         "values": gae_summary,
                         "values_with_B_nonzero_count_pass": gae_summary_with_pass,
                         "note": "'values' use the registered native-operation model (entries only); 'values_with_B_nonzero_count_pass' add B's pivot-selection pass at one native operation per entry read (lower bound 2 E_E25 - E_E31); VALIDATION.json carries the latter"},
}
if os.environ.get("VALIDATE_NO_WRITE") != "1":
    with open(os.path.join(HERE, "validator-report.json"), "w") as f:
        json.dump(out, f, indent=1, ensure_ascii=False)
        f.write("\n")
print(f"status {status}: {len(HEADLINE)} claims ({len(HEADLINE) - len([h for h in HEADLINE if h['class'] not in DECIDING])} deciding), "
      f"{len(deciding_dis)} deciding disagreements, {len(errata)} errata, {sum(CHECKS.values())}/{len(CHECKS)} checks hold")
for d in deciding_dis:
    print("  DISAGREES:", d["name"], "| printed", d["printed"], "| recomputed", d["recomputed"], "| source", d["source"])
for d in errata:
    print("  ERRATUM:", d["name"], "| printed", d["printed"], "| recomputed", round(d["recomputed"], 4), "| source", d["source"])
for k, v in CHECKS.items():
    if not v:
        print("  CHECK FAILS:", k)
sys.exit(0 if status == "pass" else 1)
