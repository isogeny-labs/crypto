#!/usr/bin/env python3
"""F2 analysis (SPEC.md section 12). Python 3 standard library only.

Decision run:  python3 -I analyze.py raw > raw/summary.json
Pilot (gates only, never prints criterion rates):
    python3 -I analyze.py pilot --census-prefix pilot_census_p --gate-prefix pilot_ge_p257_ \
        --tag pilot --gates-only > pilot/pilot_gates.json

Derived from L06 analyze.py (sha256 ef5edc54...61d); changes are listed in SPEC.md 12.
"""
import json, math, os, sys
from collections import Counter

args = sys.argv[1:]
RAW = args[0] if args and not args[0].startswith("--") else "raw"


def opt(name, default):
    return args[args.index(name) + 1] if name in args else default


CENSUS_PREFIX = opt("--census-prefix", "census_p")
GATE_PREFIX = opt("--gate-prefix", "gate_enum_p257_")
TAG = opt("--tag", "decision")
GATES_ONLY = "--gates-only" in args
AUX_PREFIX = opt("--aux-prefix", "")  # pilot only: prefix of gsp_reference.json, l2_lagrangians.json
L06_DIR = opt("--l06-dir", "/home/user/crypto/research/g2plus_leads_20261116/L06_odd_ell_isogeny_census")

PS = [257, 521, 1031, 2053, 4099]
LS = [3, 5, 7]
Z = 1.959964
STAGE_C_MAX_P = 1031
G1_EXPECTED = {"auto": 200, "B": 40, "C": 20}
# pilot draw ranges (PILOT.md): census draws per p; p = 257 also covers the gate-enum indices 0..199
PILOT_DRAWS = {257: 9453, 521: 9446, 1031: 11863, 2053: 10582, 4099: 11794}
if "--g1-counts" in args:  # pilot only: e.g. auto=200,B=40,C=20
    G1_EXPECTED = {k: int(v) for k, v in (x.split("=") for x in opt("--g1-counts", "").split(","))}

# ---------------- SplitMix64 and seeds (SPEC.md section 2) ----------------
M64 = (1 << 64) - 1
GAMMA = 0x9E3779B97F4A7C15
GINV = pow(GAMMA, -1, 1 << 64)
TAGS = {"l06": 0x4C06_0000_0000_0000, "decision": 0x4C26_0000_0000_0000,
        "pilot": 0x4C36_0000_0000_0000, "gate": 0x4C46_0000_0000_0000}
X_ORDER, X_G2, X_RES = 0xD1B5_4A32_D192_ED03, 0x8CB9_2BA7_2F3D_8DD7, 0x2545_F491_4F6C_DD1D


def curve_seed(tag, p, i):
    return TAGS[tag] ^ (p << 24) ^ i


class SM:
    def __init__(self, s):
        self.s = s

    def next(self):
        self.s = (self.s + GAMMA) & M64
        z = self.s
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
        return z ^ (z >> 31)

    def uniform(self, n):
        zone = M64 - (M64 % n)
        while True:
            x = self.next()
            if x < zone:
                return x % n


def curve_f(tag, p, i):
    r = SM(curve_seed(tag, p, i))
    return tuple(r.uniform(p) for _ in range(7))


def wilson(k, n):
    if n == 0:
        return [None, None]
    ph = k / n
    d = 1 + Z * Z / n
    center = (ph + Z * Z / (2 * n)) / d
    half = Z * math.sqrt(ph * (1 - ph) / n + Z * Z / (4 * n * n)) / d
    return [center - half, center + half]


def load_jsonl(path):
    rows, summ = [], None
    if not os.path.exists(path):
        return rows, summ
    for line in open(path):
        line = line.strip()
        if not line:
            continue
        o = json.loads(line)
        if o.get("type") == "run_summary":
            summ = o
        else:
            rows.append(o)
    return rows, summ


def rd(name):
    path = os.path.join(RAW, AUX_PREFIX + name)
    return json.load(open(path)) if os.path.exists(path) else None


census = {p: load_jsonl(os.path.join(RAW, f"{CENSUS_PREFIX}{p}.jsonl")) for p in PS}
gates = {}

# ---------------- G0 ----------------
tl = os.path.join(RAW, "test.log")
if os.path.exists(tl):
    txt = open(tl).read()
    bad = [l for l in txt.splitlines() if "FAILED" in l or "panicked" in l]
    npass = sum(1 for l in txt.splitlines() if l.startswith("test ") and l.endswith(" ok"))
    gates["G0"] = {"pass": ("test result: ok. 15 passed" in txt) and not bad,
                   "tests_ok": npass, "failed_lines": len(bad)}
else:
    gates["G0"] = {"pass": False, "note": "test.log missing"}

# ---------------- G1: cascade versus table enumeration at p = 257 ----------------
g1 = {"files": {}, "pass": True}
for force, want in G1_EXPECTED.items():
    rows, _ = load_jsonl(os.path.join(RAW, f"{GATE_PREFIX}{force}.jsonl"))
    st = Counter(r["status"] for r in rows)
    by = Counter(r["resolved_by"] for r in rows)
    ag = sum(1 for r in rows if r["agree"] is True)
    dis = sum(1 for r in rows if r["agree"] is False)
    ann = sum(1 for r in rows if r["gate_annihilation"] is not True and r["status"] == "ok")
    tags_ok = all(r.get("tag") == ("gate" if TAG == "decision" else TAG) for r in rows)
    path_ok = (force != "C" or by.get("n3", 0) == len(rows)) and \
              (force != "B" or by.get("bsgs", 0) == 0)
    ok = (len(rows) == want and st.get("ok", 0) == want and ag == want and dis == 0
          and ann == 0 and tags_ok and path_ok)
    g1["files"][force] = {"draws": len(rows), "expected": want, "status": dict(st),
                          "resolved_by": dict(by), "agree": ag, "disagree": dis,
                          "annihilation_failed": ann, "tag_ok": tags_ok, "path_ok": path_ok,
                          "pass": ok}
    g1["pass"] &= ok
gates["G1"] = g1

# ---------------- census-level gates ----------------
chi_rows = {p: [r for r in census[p][0] if r.get("status") in ("ok", "unresolved", "error")] for p in PS}
ok_rows = {p: [r for r in census[p][0] if r.get("status") == "ok"] for p in PS}
allok = [r for p in PS for r in ok_rows[p]]
gates["G2"] = {"checked": len(allok), "failed": sum(1 for r in allok if not r["gate_annihilation"])}
gates["G2"]["pass"] = len(allok) > 0 and gates["G2"]["failed"] == 0
gates["G3"] = {"checked": len(allok), "failed": sum(1 for r in allok if not r["gate_weil"])}
gates["G3"]["pass"] = len(allok) > 0 and gates["G3"]["failed"] == 0
g4_fail = g4_n = 0
for r in allok:  # G4 compares two tests of the criterion; it does not compute a rate
    for l in ("2", "3", "5", "7"):
        c = r["crit"][l]
        g4_n += 1
        if not ((c["fire"] == c["fire_fac"]) and ((not c["fire"]) or c["n_h"] == 2 ** c["n_pairs"])):
            g4_fail += 1
gsp = rd("gsp_reference.json")
gsp_g4 = {"samples_checked": 0, "failed": 0}
if gsp:
    for row in gsp["g3"]:
        gsp_g4["samples_checked"] += row["samples"]
        gsp_g4["failed"] += row["samples"] - row["g4_agree"]
gates["G4"] = {"census_checked": g4_n, "census_failed": g4_fail, "gsp": gsp_g4,
               "pass": g4_n > 0 and g4_fail == 0 and gsp is not None and gsp_g4["failed"] == 0}
g5_fail = sum(1 for r in allok if not r["gate_mod2"])
gates["G5"] = {"checked": len(allok), "failed": g5_fail, "pass": len(allok) > 0 and g5_fail == 0}
l2 = rd("l2_lagrangians.json")
if l2:
    ok6 = (l2["total"], l2["pair"], l2["other"]) == (135, 105, 30) and all(
        t["pair_rule"] == (t["stable_pair"] > 0) for t in l2["types"]) and len(l2["types"]) == 15
    gates["G6"] = {"total": l2["total"], "pair": l2["pair"], "other": l2["other"],
                   "pair_rule_mismatch": sum(1 for t in l2["types"] if t["pair_rule"] != (t["stable_pair"] > 0)),
                   "pass": ok6}
else:
    gates["G6"] = {"pass": False, "note": "missing"}
if gsp:
    zmax = max(abs(r["z"]) for r in gsp["g1"])
    bad_exact = sum(1 for r in gsp["g1"] if r["exact"] == 0 and r["fire"] > 0)
    gates["G7"] = {"similitude_failures": gsp["similitude_failures"], "max_abs_z": zmax,
                   "cells": len(gsp["g1"]), "exact0_fire_pos": bad_exact,
                   "pass": gsp["similitude_failures"] == 0 and zmax <= 4 and bad_exact == 0}
else:
    gates["G7"] = {"pass": False, "note": "missing"}

# G8: completeness of chi determination (no error anywhere; unresolved only possible for p > 1031)
g8 = {}
for p in PS:
    rows = chi_rows[p]
    st = Counter(r["status"] for r in rows)
    by = Counter(r["resolved_by"] for r in rows if r["status"] == "ok")
    summ = census[p][1]
    g8[str(p)] = {"chi_requested": len(rows), "ok": st.get("ok", 0), "unresolved": st.get("unresolved", 0),
                  "error": st.get("error", 0), "resolved_by": dict(by),
                  "run_completed": summ is not None}
g8_err = sum(v["error"] for v in g8.values())
g8_unres = {p: g8[str(p)]["unresolved"] for p in PS}
g8_unres_bad_p = [p for p in PS if g8_unres[p] > 0 and p <= STAGE_C_MAX_P]
gates["G8"] = {"per_p": g8, "error_total": g8_err,
               "pass": g8_err == 0 and not g8_unres_bad_p and all(v["run_completed"] for v in g8.values())
                       and all(v["chi_requested"] > 0 for v in g8.values()),
               "unresolved_flag_p": [p for p in PS if g8_unres[p] > 0]}

# G9: SplitMix64 stream disjointness over every stream of this run (census + G1 files)
def stream_check(rowsets):
    iv = []
    for tag, rows in rowsets:
        for r in rows:
            d = r.get("rng_draws")
            if d is None:
                continue
            s = curve_seed(tag, r["p"], r["i"])
            for key, x in (("coef", 0), ("order", X_ORDER), ("res", X_RES), ("g2", X_G2)):
                n = d.get(key, 0)
                if n > 0:
                    pos = ((s ^ x) * GINV) & M64
                    iv.append((pos + 1, pos + n, (tag, r["p"], r["i"], key, r.get("force"))))
    # identical (tag, p, i, key) appears in several G1 files (same seed, different force): those
    # are the SAME stream reused by design for the same curve; keep the longest once
    best = {}
    for a, b, k in iv:
        kk = (k[0], k[1], k[2], k[3])
        if kk not in best or b > best[kk][1]:
            best[kk] = (a, b, kk)
    iv = sorted(best.values())
    overlaps = 0
    wrap = 0
    for j in range(1, len(iv)):
        if iv[j][0] <= iv[j - 1][1]:
            overlaps += 1
    if iv and iv[-1][1] > M64:  # wraps around 2^64
        wrap = 1
        if (iv[-1][1] - (1 << 64)) >= iv[0][0]:
            overlaps += 1
    return {"streams": len(iv), "overlaps": overlaps, "wraps": wrap,
            "max_len": max((b - a + 1 for a, b, _ in iv), default=0)}


census_tag = TAG
gate_tag = "gate" if TAG == "decision" else TAG
rowsets = [(census_tag, census[p][0]) for p in PS]
for force in G1_EXPECTED:
    rowsets.append((gate_tag, load_jsonl(os.path.join(RAW, f"{GATE_PREFIX}{force}.jsonl"))[0]))
g9 = stream_check(rowsets)
# every census row's recorded f must equal the regenerated f (curve rule check)
fmis = 0
for p in PS:
    for r in census[p][0]:
        if "f" in r and tuple(r["f"]) != curve_f(census_tag, p, r["i"]):
            fmis += 1
g9["f_regeneration_mismatches"] = fmis
g9["pass"] = g9["streams"] > 0 and g9["overlaps"] == 0 and fmis == 0
gates["G9"] = g9

# G10: freshness against yesterday's L06 draws
def l06_draw_ranges():
    rng = {}
    for p in PS:
        _, s = load_jsonl(os.path.join(L06_DIR, "raw", f"census_p{p}.jsonl"))
        rng[p] = s["draws"] if s else None
    ge = {}
    for p in (101, 257):
        rows, _ = load_jsonl(os.path.join(L06_DIR, "raw", f"gate_enum_p{p}.jsonl"))
        ge[p] = (max(r["i"] for r in rows) + 1) if rows else None
    return rng, ge


g10 = {}
try:
    l06_census, l06_ge = l06_draw_ranges()
    old_seeds, old_f = set(), set()
    for p, n in l06_census.items():
        for i in range(n):
            old_seeds.add(curve_seed("l06", p, i))
            old_f.add((p, curve_f("l06", p, i)))
    for p, n in l06_ge.items():
        for i in range(n):
            old_seeds.add(curve_seed("l06", p, i))
            old_f.add((p, curve_f("l06", p, i)))
    # the coordinator's pilot (tag pilot), frozen ranges from PILOT.md: census draws 0..n-1 per p,
    # gate-enum p = 257 indices 0..199
    if TAG == "decision":
        for p, n in PILOT_DRAWS.items():
            for i in range(n):
                old_seeds.add(curve_seed("pilot", p, i))
                old_f.add((p, curve_f("pilot", p, i)))
    new_seed_hits = new_f_hits = 0
    dup_f = 0
    for tag, rows in rowsets:
        seen = set()
        for r in rows:
            if curve_seed(tag, r["p"], r["i"]) in old_seeds:
                new_seed_hits += 1
            f = tuple(r["f"]) if "f" in r else curve_f(tag, r["p"], r["i"])
            if (r["p"], f) in old_f:
                new_f_hits += 1
    for p in PS:
        fs = Counter(tuple(r["f"]) for r in census[p][0] if "f" in r)
        dup_f += sum(c - 1 for c in fs.values() if c > 1)
    g10 = {"pilot_draws_included": TAG == "decision", "l06_census_draws": {str(k): v for k, v in l06_census.items()},
           "l06_gate_enum_draws": {str(k): v for k, v in l06_ge.items()},
           "l06_curves": len(old_f), "seed_collisions": new_seed_hits,
           "f_collisions_with_l06": new_f_hits, "duplicate_f_within_run": dup_f,
           "pass": new_seed_hits == 0 and new_f_hits == 0 and all(v for v in l06_census.values())}
except Exception as e:  # missing L06 files: gate fails, reported
    g10 = {"pass": False, "note": f"exception: {e!r}"}
gates["G10"] = g10

core = ["G0", "G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G9", "G10"]
failed = [g for g in core if not gates[g]["pass"]]

if GATES_ONLY:
    # pilot mode: gates and instrument sanity only; no criterion rate is computed or printed
    json.dump({"mode": "gates-only", "raw": RAW, "tag": TAG, "gates": gates,
               "gate_failures": failed}, sys.stdout, indent=1)
    print()
    sys.exit(0)

# ---------------- populations (PROTOCOL.md section 2) ----------------
def dedup(rows):
    seen, out = set(), []
    for r in sorted(rows, key=lambda r: r["i"]):
        f = tuple(r["f"])
        if f in seen:
            continue
        seen.add(f)
        out.append(r)
    return out


def pops(p):
    ok = dedup(ok_rows[p])
    U = [r for r in ok if r["block"] == "U"]
    return {"ALL": U, "PRIME": [r for r in ok if r["prime"]],
            "C2": [r for r in U if r["c2"]], "ODD": [r for r in U if r["nJ"] % 2 == 1]}


P = {p: pops(p) for p in PS}
pooled = {k: [r for p in PS for r in P[p][k]] for k in ("ALL", "PRIME", "C2", "ODD")}
populations = {"per_p": {}, "pooled": {k: len(v) for k, v in pooled.items()}}
for p in PS:
    s = census[p][1] or {}
    populations["per_p"][str(p)] = {
        "draws": s.get("draws"), "stop_reason": s.get("stop_reason"),
        "u_block_squarefree_seen": s.get("u_count"), "status_counts": s.get("status_counts"),
        "unresolved_in_U": sum(1 for r in chi_rows[p] if r["status"] == "unresolved" and r["block"] == "U"),
        **{k: len(v) for k, v in P[p].items()}}


def stats(rows, l):
    n = len(rows)
    k = sum(1 for r in rows if r["crit"][str(l)]["fire"])
    cls = Counter(r["crit"][str(l)]["class"] for r in rows)
    fr = [r for r in rows if r["crit"][str(l)]["fire"]]
    nh = Counter(r["crit"][str(l)]["n_h"] for r in fr)
    ed = Counter(r["crit"][str(l)]["ext_deg"] for r in fr)
    return {"n": n, "fire": k, "fraction": (k / n if n else None), "wilson95": wilson(k, n),
            "class": {c: cls.get(c, 0) for c in ("fire", "none", "undetermined")},
            "n_h_hist_on_firing": {str(a): b for a, b in sorted(nh.items())},
            "ext_deg_hist_on_firing": {str(a): b for a, b in sorted(ed.items())}}


crit_out = {}
for l in LS:
    crit_out[str(l)] = {}
    for k in ("ALL", "PRIME", "C2"):
        crit_out[str(l)][k] = {"per_p": {str(p): stats(P[p][k], l) for p in PS}, "pooled": stats(pooled[k], l)}

a2 = {}
for k in ("ALL", "ODD"):
    def a2stats(rows):
        return {"n": len(rows),
                "fire_2": sum(1 for r in rows if r["crit"]["2"]["fire"]),
                "stable_pair_exists": sum(1 for r in rows if r["l2"]["stable_pair"] > 0),
                "pair_rule": sum(1 for r in rows if r["l2"]["pair_rule"]),
                "stable_other_ge2": sum(1 for r in rows if r["l2"]["stable_other"] >= 2),
                "stable_other_hist": {str(a): b for a, b in sorted(Counter(r["l2"]["stable_other"] for r in rows).items())},
                "stable_pair_hist": {str(a): b for a, b in sorted(Counter(r["l2"]["stable_pair"] for r in rows).items())}}
    a2[k] = {"per_p": {str(p): a2stats(P[p][k]) for p in PS}, "pooled": a2stats(pooled[k])}
o = a2["ODD"]["pooled"]
a2["predictions_on_ODD_pooled"] = {"a_fire_2_all": o["fire_2"] == o["n"],
                                   "b_no_pair_type": o["stable_pair_exists"] == 0,
                                   "c_other_ge2_all": o["stable_other_ge2"] == o["n"]}

# ---------------- S4 ----------------
ref = {(row["l"], row["m"]): row for row in gsp["g3"]} if gsp else {}
s4 = {"cells": [], "n_abs_z_gt_3": 0, "n_cells": 0}
for p in PS:
    for l in LS:
        m = p % l
        r = ref.get((l, m))
        for pk, rows, fkey, nkey in (("ALL", P[p]["ALL"], "fire", "samples"), ("PRIME", P[p]["PRIME"], "fire_nz1", "nz1")):
            n = len(rows)
            if n == 0 or r is None:
                continue
            k = sum(1 for rr in rows if rr["crit"][str(l)]["fire"])
            fo, fr = k / n, r[fkey] / r[nkey]
            den = math.sqrt(fo * (1 - fo) / n + fr * (1 - fr) / r[nkey])
            z = 0.0 if den == 0 else (fo - fr) / den
            s4["cells"].append({"p": p, "l": l, "m": m, "pop": pk, "n": n, "fire": k, "F_obs": fo, "F_ref": fr, "z": z})
            s4["n_cells"] += 1
            s4["n_abs_z_gt_3"] += abs(z) > 3

# ---------------- S5: l = 3 kernel field degree on firing PRIME curves ----------------
def pmul(a, b, l):
    out = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] = (out[i + j] + x * y) % l
    return out


def trim(a):
    a = list(a)
    while a and a[-1] == 0:
        a.pop()
    return a


def pmod(a, m, l):
    a = trim(a)
    inv = pow(m[-1], l - 2, l)
    while len(a) >= len(m):
        c = a[-1] * inv % l
        sh = len(a) - len(m)
        for k in range(len(m)):
            a[sh + k] = (a[sh + k] - c * m[k]) % l
        a = trim(a)
    return a


def pgcd_deg(a, b, l):
    a, b = trim(a), trim(b)
    while b:
        a, b = b, pmod(a, b, l)
    return len(a) - 1


def recip(h, q, l):
    d = len(h) - 1
    inv0 = pow(h[0], l - 2, l)
    return [h[d - i] * pow(q, d - i, l) * inv0 % l for i in range(d + 1)]


def ord_T(h, l):
    t, k = [0, 1], 1
    while trim(pmod(t, h, l)) != [1]:
        t = pmod(pmul(t, [0, 1], l), h, l)
        k += 1
        assert k <= 20000
    return k


def valid_h3(chi_mod, q):
    l = 3
    out = []
    for h0 in (1, 2):
        for h1 in range(3):
            for h2 in range(3):
                h = [h0, h1, h2, 1]
                hs = recip(h, q, l)
                if trim(pmul(h, hs, l)) == trim(chi_mod) and pgcd_deg(h, hs, l) == 0:
                    roots = sum(1 for x in range(3) if (h0 + h1 * x + h2 * x * x + x ** 3) % 3 == 0)
                    out.append({"h": h, "irreducible": roots == 0, "h0": h0, "ord": ord_T(h, l)})
    return out


s5 = {"per_p": {}, "pooled": Counter(), "by_p_mod_3": {"1": Counter(), "2": Counter()},
      "by_chi_mod3_squarefree": {"true": Counter(), "false": Counter()},
      "valid_h_types": Counter(), "recompute_mismatch": 0, "prediction_violations": 0, "n": 0}
for p in PS:
    c = Counter()
    for r in P[p]["PRIME"]:
        cr = r["crit"]["3"]
        if not cr["fire"]:
            continue
        s5["n"] += 1
        q = p % 3
        hs = valid_h3(cr["chi_mod_l"], q)
        e = min(h["ord"] for h in hs) if hs else None
        if e != cr["ext_deg"] or len(hs) != cr["n_h"]:
            s5["recompute_mismatch"] += 1
        ed = cr["ext_deg"]
        c[ed] += 1
        s5["pooled"][ed] += 1
        s5["by_p_mod_3"][str(q)][ed] += 1
        # squarefree test on chi mod 3 via gcd with derivative
        cm = cr["chi_mod_l"]
        der = [(k * cm[k]) % 3 for k in range(1, len(cm))]
        sqf = pgcd_deg(cm, der, 3) == 0
        s5["by_chi_mod3_squarefree"]["true" if sqf else "false"][ed] += 1
        types = "+".join(sorted("irr3" if h["irreducible"] else "red" for h in hs))
        s5["valid_h_types"][types] += 1
        # frozen predictions (PROTOCOL.md S5): q = 1 -> ext_deg in {13, 26}, 13 iff h(0) = 2;
        # q = 2 -> ext_deg 13, except chi = (T-1)^3 (T-2)^3 mod 3 -> ext_deg 3
        if q == 1:
            good = all(h["irreducible"] for h in hs) and ed == (13 if hs[0]["h0"] == 2 else 26)
        else:
            special = trim(cm) == trim(pmul(pmul(pmul([2, 1], [2, 1], 3), [2, 1], 3),
                                                pmul(pmul([1, 1], [1, 1], 3), [1, 1], 3), 3))
            good = (ed == 3) if special else (ed == 13)
        if not good:
            s5["prediction_violations"] += 1
    s5["per_p"][str(p)] = {str(a): b for a, b in sorted(c.items())}
s5["pooled"] = {str(a): b for a, b in sorted(s5["pooled"].items())}
for k in ("by_p_mod_3", "by_chi_mod3_squarefree"):
    s5[k] = {kk: {str(a): b for a, b in sorted(v.items())} for kk, v in s5[k].items()}
s5["valid_h_types"] = dict(s5["valid_h_types"])

# ---------------- verdict (PROTOCOL.md section 6) ----------------
fl = {}
for l in LS:
    st = crit_out[str(l)]["PRIME"]["pooled"]
    fl[str(l)] = {"n": st["n"], "fire": st["fire"], "F": st["fraction"], "lo": st["wilson95"][0], "hi": st["wilson95"][1]}
n_prime = len(pooled["PRIME"])


def rules_2_5():
    if n_prime < 300:
        return "inconclusive (sample)"
    if all(fl[str(l)]["F"] is not None and fl[str(l)]["F"] < 0.01 for l in LS):
        return "killed"
    if any(fl[str(l)]["lo"] is not None and fl[str(l)]["lo"] >= 0.01 for l in LS):
        return "supported"
    return "inconclusive"


verdict = "void (instrument)" if failed else rules_2_5()
out = {"gates": gates, "gate_failures": failed, "populations": populations, "criterion": crit_out,
       "A2": a2, "S4": s4, "S5_l3_ext_deg": s5, "pooled_prime_F_l": fl, "n_prime_pooled": n_prime,
       "verdict": verdict}
json.dump(out, sys.stdout, indent=1)
print()
