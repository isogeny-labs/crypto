#!/usr/bin/env python3
"""F1: lab genus_ladder, genus 3, 2LP base constant cc = 1 against cc = 2.

Python 3 standard library only.  Reads `genus_ladder run` JSONL files and
nothing else.  Subcommands:

  python3 f1.py gates   RAWDIR OUT.json      correctness gates G1..G8 (PROTOCOL section 7)
  python3 f1.py analyze RAWDIR OUT.json OUT.md decision statistics (PROTOCOL sections 6, 8)
  python3 f1.py selftest                       unit tests (SPEC section 6)

File naming in RAWDIR (PROTOCOL section 9):
  p{P}_cc{CC}_main.jsonl    e.g. p8209_cc2.0_main.jsonl, p8209_cc1.0_main.jsonl
  p{P}_cc2.0_rerun.jsonl    the determinism A/A process
Unit (the instrument's own, research/genus_ladder_20261114/PROTOCOL.md section 4):
  FPM = cantor_mul + I*cantor_inv + rel_mul + kappa*la_mul,
  bands (I, kappa) in {lo (1, 1), mid (10, 9), hi (30, 9)}; mid is primary.
  Rho: FPM = cantor_mul + I*cantor_inv.
All figures [pcc:unverified].
"""
import json
import math
import os
import random
import re
import sys
from collections import defaultdict

G = 3
PRIMES = (8209, 16411, 32771)
CURVES = {8209: (6, 18, 23), 16411: (8, 27, 43), 32771: (8, 16, 31)}
SEEDS = tuple(range(1, 9))
CC_PRIMARY = (1.0, 2.0)
CC_SECONDARY = (0.5, 1.5, 3.0)
BANDS = {"lo": (1, 1), "mid": (10, 9), "hi": (30, 9)}
IC_ARM = "C_double_lp+pre"
RHO_ARM = "R_rho_neg"
BOOT = 4000
BOOT_SEED = 20261117
REF_TRIALS_C6_S1 = 42975          # scale study, (8209, c6, s1), cc = 2 [pcc:unverified]
REF_MEAN_TRIALS_8209 = 43964      # scale study mean over (8209, 3 curves, seeds 1,2), rounded
NONCOUNT = {"wall_ns", "wall_s", "search_wall_s"}
AA_MAX = 0.05                     # gate G9: |G_AA_mid| at most this at every prime
HERE = os.path.dirname(os.path.abspath(__file__))
REF_INSTANCES = os.path.join(HERE, "ref", "instances_L03.jsonl")

FNAME = re.compile(r"^p(\d+)_cc(\d+\.\d+)_(main|rerun)\.jsonl$")


# ---------------------------------------------------------------- loading
def load_dir(rawdir):
    """Return {(p, cc, tag): [records]} for every file matching FNAME."""
    out = {}
    for name in sorted(os.listdir(rawdir)):
        m = FNAME.match(name)
        if not m:
            continue
        key = (int(m.group(1)), float(m.group(2)), m.group(3))
        with open(os.path.join(rawdir, name)) as fh:
            out[key] = [json.loads(l) for l in fh if l.strip()]
    return out


def runs(recs, arm):
    """{(curve_seed, seed): record} for kind == run and the given arm."""
    d = {}
    for r in recs:
        if r.get("kind") == "run" and r.get("arm") == arm:
            k = (r["curve_seed"], r["seed"])
            if k in d:
                raise ValueError(f"duplicate run {k} arm {arm}")
            d[k] = r
    return d


def nb_expected(p, cc):
    return min(math.ceil(cc * float(p) ** (1.0 - 1.0 / G)), p)


def fpm_ic(r, band):
    i, kappa = BANDS[band]
    return r["cantor_mul"] + i * r["cantor_inv"] + r["rel_mul"] + kappa * r["la_mul"]


def fpm_rel(r, band):
    i, _ = BANDS[band]
    return r["cantor_mul"] + i * r["cantor_inv"] + r["rel_mul"]


def fpm_la(r, band):
    _, kappa = BANDS[band]
    return kappa * r["la_mul"]


def fpm_rho(r, band):
    i, _ = BANDS[band]
    return r["cantor_mul"] + i * r["cantor_inv"]


def strip(r):
    return {k: v for k, v in r.items() if k not in NONCOUNT}


# ---------------------------------------------------------------- gates
def gates(data, expect_primes=PRIMES, expect_cc=CC_PRIMARY, ref_path=REF_INSTANCES,
          curves=CURVES, seeds=SEEDS, check_repro=True):
    """Every gate returns (pass, detail).  Missing required file = fail."""
    res = {}
    ref = {}
    with open(ref_path) as fh:
        for l in fh:
            if l.strip():
                d = json.loads(l)
                ref[(d["p"], d["curve_seed"])] = d
    # G0 presence: every required (p, cc, main) file exists, and (p, 2.0, rerun)
    missing = [(p, cc, "main") for p in expect_primes for cc in expect_cc
               if (p, cc, "main") not in data]
    missing += [(p, 2.0, "rerun") for p in expect_primes if (p, 2.0, "rerun") not in data]
    res["G0_files_present"] = (not missing, {"missing": [list(m) for m in missing]})

    # G1 instances identical to L03 / scale-study instances (f, order, h, r)
    bad = []
    for (p, cc, tag), recs in data.items():
        inst = sorted((r for r in recs if r.get("kind") == "instance"),
                      key=lambda r: r["curve_seed"])
        got = [r["curve_seed"] for r in inst]
        if tuple(got) != tuple(curves[p]):
            bad.append({"file": [p, cc, tag], "curve_seeds": got})
            continue
        for r in inst:
            e = ref[(p, r["curve_seed"])]
            for k in ("f", "order", "h", "r", "g"):
                if r[k] != e[k]:
                    bad.append({"file": [p, cc, tag], "curve": r["curve_seed"], "field": k})
    res["G1_instances"] = (not bad, {"bad": bad})

    # G2 the flag is honoured: config cc, fb_cols == ceil(cc p^(2/3)) on every IC run
    bad = []
    for (p, cc, tag), recs in data.items():
        cfg = [r for r in recs if r.get("kind") == "config"]
        if len(cfg) != 1 or abs(cfg[0]["cc"] - cc) > 1e-12 or cfg[0]["threads"] != 1:
            bad.append({"file": [p, cc, tag], "why": "config"})
        for k, r in runs(recs, IC_ARM).items():
            if r["fb_cols"] != nb_expected(p, cc):
                bad.append({"file": [p, cc, tag], "run": list(k), "fb_cols": r["fb_cols"],
                            "expected": nb_expected(p, cc)})
    res["G2_cc_honoured"] = (not bad, {"bad": bad[:20], "n_bad": len(bad)})

    # G3 completeness and correctness: every expected run present, certified, ok,
    # not exhausted; no skip rows; a done row closes every file.
    bad = []
    for (p, cc, tag), recs in data.items():
        want = {(c, s) for c in curves[p] for s in seeds}
        arms = [IC_ARM] + ([RHO_ARM] if tag == "main" and cc in CC_PRIMARY else [])
        for arm in arms:
            d = runs(recs, arm)
            if set(d) != want:
                bad.append({"file": [p, cc, tag], "arm": arm, "why": "run set",
                            "n": len(d)})
            for k, r in d.items():
                if not (r.get("certified") is True and r.get("ok") is True
                        and r.get("exhausted", False) is False):
                    bad.append({"file": [p, cc, tag], "arm": arm, "run": list(k)})
        if any(r.get("kind") == "skip" for r in recs):
            bad.append({"file": [p, cc, tag], "why": "skip row"})
        if not recs or recs[-1].get("kind") != "done":
            bad.append({"file": [p, cc, tag], "why": "no done row"})
    res["G3_certified_complete"] = (not bad, {"bad": bad[:20], "n_bad": len(bad)})

    # G4 determinism A/A: the rerun process equals the main cc = 2 process, count for count
    bad = []
    for p in expect_primes:
        a, b = data.get((p, 2.0, "main")), data.get((p, 2.0, "rerun"))
        if a is None or b is None:
            bad.append({"p": p, "why": "missing"})
            continue
        ra, rb = runs(a, IC_ARM), runs(b, IC_ARM)
        if set(ra) != set(rb):
            bad.append({"p": p, "why": "run set"})
            continue
        for k in ra:
            if strip(ra[k]) != strip(rb[k]):
                bad.append({"p": p, "run": list(k)})
    res["G4_determinism"] = (not bad, {"bad": bad[:20], "n_bad": len(bad)})

    # G5 negative control: --cc does not touch rho; rho rows equal in the cc = 1 and cc = 2 files
    bad = []
    for p in expect_primes:
        a, b = data.get((p, 2.0, "main")), data.get((p, 1.0, "main"))
        if a is None or b is None:
            bad.append({"p": p, "why": "missing"})
            continue
        ra, rb = runs(a, RHO_ARM), runs(b, RHO_ARM)
        if set(ra) != set(rb) or any(strip(ra[k]) != strip(rb[k]) for k in ra):
            bad.append({"p": p})
    res["G5_rho_independent_of_cc"] = (not bad, {"bad": bad})

    # G6 reproduction of the scale study's cc = 2 counts at 8209 (different build)
    if check_repro:
        d = runs(data.get((8209, 2.0, "main"), []), IC_ARM)
        ok, det = True, {}
        try:
            det["c6_s1_trials"] = d[(6, 1)]["trials"]
            sub = [d[(c, s)]["trials"] for c in CURVES[8209] for s in (1, 2)]
            det["mean_trials_s12"] = sum(sub) / len(sub)
            ok = (det["c6_s1_trials"] == REF_TRIALS_C6_S1
                  and abs(det["mean_trials_s12"] - REF_MEAN_TRIALS_8209) <= 0.5)
        except KeyError as e:
            ok, det["missing"] = False, str(e)
        res["G6_scale_reproduction"] = (ok, det)

    # G7 positive control (aggregate): cc = 1 needs more trials and less la_mul than cc = 2
    bad = []
    for p in expect_primes:
        a, b = data.get((p, 1.0, "main")), data.get((p, 2.0, "main"))
        if a is None or b is None:
            bad.append({"p": p, "why": "missing"})
            continue
        ra, rb = runs(a, IC_ARM), runs(b, IC_ARM)
        ks = sorted(set(ra) & set(rb))
        if not ks:
            bad.append({"p": p, "why": "no pairs"})
            continue
        t1 = sum(ra[k]["trials"] for k in ks)
        t2 = sum(rb[k]["trials"] for k in ks)
        l1 = sum(ra[k]["la_mul"] for k in ks)
        l2 = sum(rb[k]["la_mul"] for k in ks)
        if not (t1 > t2 and l1 < l2):
            bad.append({"p": p, "trials": [t1, t2], "la_mul": [l1, l2]})
    res["G7_tradeoff_visible"] = (not bad, {"bad": bad})

    # G8 accounting identity: group_ops == setup_ops + walk_ops + verify_ops on every IC run
    bad = 0
    for recs in data.values():
        for r in runs(recs, IC_ARM).values():
            if r["group_ops"] != r["setup_ops"] + r["walk_ops"] + r["verify_ops"]:
                bad += 1
    res["G8_ops_identity"] = (bad == 0, {"n_bad": bad})

    # G9 A/A noise floor: |G_AA| (band mid, point; even seeds against odd seeds within the
    # cc = 2 main file, matched by curve) <= AA_MAX at every prime
    bad = []
    for p in expect_primes:
        a = data.get((p, 2.0, "main"))
        if a is None:
            bad.append({"p": p, "why": "missing"})
            continue
        b = runs(a, IC_ARM)
        odd = {k: v for k, v in b.items() if k[1] % 2 == 1}
        even = {(k[0], k[1] - 1): v for k, v in b.items() if k[1] % 2 == 0}
        ks = sorted(set(odd) & set(even))
        if not ks:
            bad.append({"p": p, "why": "no pairs"})
            continue
        g_aa = gain(even, odd, "mid")(ks)
        if abs(g_aa) > AA_MAX:
            bad.append({"p": p, "G_AA_mid": g_aa})
    res["G9_aa_noise_floor"] = (not bad, {"bad": bad})

    allpass = all(v[0] for v in res.values())
    return {"all_pass": allpass, "gates": {k: {"pass": v[0], "detail": v[1]}
                                           for k, v in res.items()}}


# ---------------------------------------------------------------- statistics
def q_index(n, q):
    return int(q * (n - 1) + 0.5)


def hboot(keys, stat, nboot=BOOT, seed=BOOT_SEED):
    """Hierarchical bootstrap over keys (curve, seed): resample curves with
    replacement, then seeds within each drawn curve with replacement.
    stat(list_of_keys) -> float.  Returns (point, lo, hi)."""
    by = defaultdict(list)
    for k in keys:
        by[k[0]].append(k)
    curves = sorted(by)
    rng = random.Random(seed)
    vals = []
    for _ in range(nboot):
        samp = []
        for _ in curves:
            v = by[rng.choice(curves)]
            samp.extend(rng.choice(v) for _ in v)
        vals.append(stat(samp))
    vals.sort()
    return stat(list(keys)), vals[q_index(nboot, 0.025)], vals[q_index(nboot, 0.975)]


def gain(ra, rb, band):
    """G = 1 - mean FPM(a) / mean FPM(b) over the same keys (paired)."""
    def st(ks):
        return 1.0 - sum(fpm_ic(ra[k], band) for k in ks) / sum(fpm_ic(rb[k], band) for k in ks)
    return st


def analyze(data):
    out = {"per_prime": {}, "decision": None}
    for p in PRIMES:
        pp = {}
        main = {cc: runs(data[(p, cc, "main")], IC_ARM)
                for cc in CC_PRIMARY + CC_SECONDARY if (p, cc, "main") in data}
        rho = runs(data[(p, 2.0, "main")], RHO_ARM)
        keys = sorted(set(main[1.0]) & set(main[2.0]))
        # primary: G(cc=1 vs cc=2), all bands
        for band in BANDS:
            pt, lo, hi = hboot(keys, gain(main[1.0], main[2.0], band))
            wins = sum(fpm_ic(main[1.0][k], band) < fpm_ic(main[2.0][k], band) for k in keys)
            pp[f"G_{band}"] = {"point": pt, "lo": lo, "hi": hi, "pairs": len(keys),
                               "pairs_cc1_cheaper": wins}
        # A/A: seed halves within cc = 2 (unpaired; odd seeds vs even seeds, matched by curve)
        odd = {k: v for k, v in main[2.0].items() if k[1] % 2 == 1}
        even = {(k[0], k[1] - 1): v for k, v in main[2.0].items() if k[1] % 2 == 0}
        ak = sorted(set(odd) & set(even))
        for band in ("mid",):
            pt, lo, hi = hboot(ak, gain(even, odd, band))
            pp["AA_mid"] = {"point": pt, "lo": lo, "hi": hi, "pairs": len(ak)}
        # per-cc means, shares and rho ratios
        cells = {}
        for cc, d in sorted(main.items()):
            ks = sorted(set(d) & set(rho))
            n = len(ks)
            c = {"cc": cc, "nb": nb_expected(p, cc), "runs": n}
            for f in ("trials", "relations", "core_rows", "core_cols", "attempts",
                      "cantor_mul", "cantor_inv", "rel_mul", "la_mul", "la_add"):
                c[f"mean_{f}"] = sum(d[k][f] for k in ks) / n
            c["max_attempts"] = max(d[k]["attempts"] for k in ks)
            for band in BANDS:
                m = sum(fpm_ic(d[k], band) for k in ks) / n
                c[f"fpm_{band}"] = m
                c[f"rel_share_{band}"] = sum(fpm_rel(d[k], band) for k in ks) / n / m
                c[f"la_share_{band}"] = sum(fpm_la(d[k], band) for k in ks) / n / m

                def rr(kk, d=d, band=band):
                    return (sum(fpm_ic(d[k], band) for k in kk)
                            / sum(fpm_rho(rho[k], band) for k in kk))
                pt, lo, hi = hboot(ks, rr)
                c[f"ic_over_rho_{band}"] = {"point": pt, "lo": lo, "hi": hi}
            if cc != 2.0:
                pt, lo, hi = hboot(sorted(set(d) & set(main[2.0])), gain(d, main[2.0], "mid"))
                c["G_mid_vs_cc2"] = {"point": pt, "lo": lo, "hi": hi}
            cells[str(cc)] = c
        pp["cells"] = cells
        pp["rho_fpm_mid"] = sum(fpm_rho(rho[k], "mid") for k in rho) / len(rho)
        sec = [cc for cc in main]
        pp["argmin_cc_mid_secondary"] = min(sec, key=lambda cc: cells[str(cc)]["fpm_mid"])
        pp["secondary_complete"] = all(cc in main for cc in CC_SECONDARY)
        out["per_prime"][str(p)] = pp
    out["decision"] = decide(out["per_prime"])
    pp = out["per_prime"]
    out["decision"]["band_robust"] = all(pp[str(p)][f"G_{b}"]["lo"] > 0.0
                                         for p in PRIMES for b in BANDS)
    out["decision"]["sign_flip_at_lo"] = [p for p in PRIMES if pp[str(p)]["G_lo"]["point"] < 0.0]
    return out


SUPPORT_G, KILL_G = 0.15, 0.05


def decide(per_prime):
    """PROTOCOL section 8, applied to G_mid (cc = 1 against cc = 2)."""
    g = {int(p): v["G_mid"] for p, v in per_prime.items()}
    if all(g[p]["point"] >= SUPPORT_G and g[p]["lo"] > 0.0 for p in PRIMES):
        verdict = "supported"
    elif g[32771]["point"] < KILL_G:
        verdict = "killed"
    else:
        verdict = "inconclusive"
    return {"verdict": verdict, "rule": "supported: G_mid >= 0.15 and 95% lo > 0 at 8209, "
            "16411 and 32771; killed: G_mid(32771) < 0.05; inconclusive otherwise"}


def tables(an, gt):
    L = ["# F1 tables (generated by f1.py analyze) [pcc:unverified]", "",
         f"Gates all pass: **{gt['all_pass']}**. Verdict (PROTOCOL section 8): "
         f"**{an['decision']['verdict']}**. Band-robust (95% lo > 0 at lo, mid, hi, every prime): "
         f"{an['decision'].get('band_robust')}; G_lo point < 0 at: {an['decision'].get('sign_flip_at_lo')}.", "",
         "## Primary: G = 1 - FPM(cc=1)/FPM(cc=2), paired, 3 curves x 8 seeds", "",
         "| p | band | G | 95% lo | 95% hi | pairs with cc=1 cheaper |", "|---|---|---|---|---|---|"]
    for p, pp in an["per_prime"].items():
        for band in BANDS:
            x = pp[f"G_{band}"]
            L.append(f"| {p} | {band} | {x['point']:.4f} | {x['lo']:.4f} | {x['hi']:.4f} | "
                     f"{x['pairs_cc1_cheaper']}/{x['pairs']} |")
    L += ["", "## A/A (cc = 2, even seeds against odd seeds, band mid)", "",
          "| p | G_AA | 95% lo | 95% hi |", "|---|---|---|---|"]
    for p, pp in an["per_prime"].items():
        x = pp["AA_mid"]
        L.append(f"| {p} | {x['point']:.4f} | {x['lo']:.4f} | {x['hi']:.4f} |")
    L += ["", "## Per cc (means over 3 curves x 8 seeds)", "",
          "| p | cc | nb | trials | core_cols | max att. | FPM lo | FPM mid | FPM hi | "
          "rel share mid | LA share mid | IC/rho mid [95%] | G_mid vs cc=2 |",
          "|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for p, pp in an["per_prime"].items():
        for cc, c in pp["cells"].items():
            r = c["ic_over_rho_mid"]
            gm = c.get("G_mid_vs_cc2")
            gs = f"{gm['point']:.4f} [{gm['lo']:.4f}, {gm['hi']:.4f}]" if gm else "-"
            L.append(f"| {p} | {cc} | {c['nb']} | {c['mean_trials']:.0f} | "
                     f"{c['mean_core_cols']:.1f} | {c['max_attempts']} | {c['fpm_lo']:.4g} | "
                     f"{c['fpm_mid']:.4g} | {c['fpm_hi']:.4g} | {c['rel_share_mid']:.3f} | "
                     f"{c['la_share_mid']:.3f} | {r['point']:.3f} [{r['lo']:.3f}, {r['hi']:.3f}] "
                     f"| {gs} |")
    L += ["", "Secondary argmin over the cc run (same seeds select and evaluate: not decisive):",
          ""]
    for p, pp in an["per_prime"].items():
        L.append(f"- p = {p}: argmin cc = {pp['argmin_cc_mid_secondary']} "
                 f"(secondary sweep complete: {pp['secondary_complete']})")
    L += ["", "## Gates", "", "| gate | pass |", "|---|---|"]
    for k, v in gt["gates"].items():
        L.append(f"| {k} | {v['pass']} |")
    return "\n".join(L) + "\n"


# ---------------------------------------------------------------- self-test
def _fake_run(p, c, s, cc, arm=IC_ARM, scale=1.0):
    nb = nb_expected(p, cc)
    base = {"kind": "run", "family": "hyperelliptic", "p": p, "g": G, "curve_seed": c,
            "seed": s, "arm": arm, "certified": True, "ok": True, "wall_ns": s}
    if arm == RHO_ARM:
        base.update({"cantor_mul": 1000, "cantor_inv": 10, "group_ops": 5})
        return base
    base.update({"exhausted": False, "fb_cols": nb, "trials": int(1e6 / nb),
                 "cantor_mul": int(scale * 4000), "cantor_inv": 100, "rel_mul": 900,
                 "la_mul": int(scale * 1000), "la_add": 7, "relations": 10, "core_rows": 9,
                 "core_cols": 8, "attempts": 1, "group_ops": 6, "setup_ops": 1,
                 "walk_ops": 4, "verify_ops": 1})
    return base


def _fake_file(p, cc, tag, scale, rho=True):
    recs = [{"kind": "config", "cc": cc, "threads": 1}]
    with open(REF_INSTANCES) as fh:
        for l in fh:
            d = json.loads(l)
            if d["p"] == p:
                recs.append(d)
    for c in CURVES[p]:
        for s in SEEDS:
            recs.append(_fake_run(p, c, s, cc, scale=scale))
            if rho:
                recs.append(_fake_run(p, c, s, cc, arm=RHO_ARM))
    recs.append({"kind": "done", "wall_s": 1.0})
    return recs


def selftest():
    n = 0
    # T1 base-size formula (expected values computed independently, PROTOCOL table 4)
    exp = {(8209, 0.5): 204, (8209, 1.0): 407, (8209, 1.5): 611, (8209, 2.0): 814,
           (8209, 3.0): 1221, (16411, 1.0): 646, (16411, 2.0): 1292, (32771, 1.0): 1025,
           (32771, 2.0): 2049, (32771, 3.0): 3073}
    for k, v in exp.items():
        assert nb_expected(*k) == v, (k, nb_expected(*k), v)
        n += 1
    # T2 FPM bands on a known record
    r = {"cantor_mul": 100, "cantor_inv": 2, "rel_mul": 30, "la_mul": 5}
    assert fpm_ic(r, "lo") == 100 + 2 + 30 + 5
    assert fpm_ic(r, "mid") == 100 + 20 + 30 + 45
    assert fpm_ic(r, "hi") == 100 + 60 + 30 + 45
    assert fpm_rho(r, "mid") == 120
    n += 4
    # T3 positive control of the analysis: scale 0.6 at cc = 1 gives G_mid = 0.4 exactly
    data = {}
    for p in PRIMES:
        data[(p, 2.0, "main")] = _fake_file(p, 2.0, "main", 1.0)
        data[(p, 2.0, "rerun")] = _fake_file(p, 2.0, "rerun", 1.0, rho=False)
        data[(p, 1.0, "main")] = _fake_file(p, 1.0, "main", 0.6)
    an = analyze(data)
    for p in PRIMES:
        gm = an["per_prime"][str(p)]["G_mid"]
        num = 0.6 * 4000 + 1000 + 900 + 9 * 0.6 * 1000
        den = 4000 + 1000 + 900 + 9 * 1000
        assert abs(gm["point"] - (1 - num / den)) < 1e-12, gm
        assert abs(gm["lo"] - gm["point"]) < 1e-12 and abs(gm["hi"] - gm["point"]) < 1e-12
        assert abs(an["per_prime"][str(p)]["AA_mid"]["point"]) < 1e-12
        n += 3
    assert an["decision"]["verdict"] == "supported"
    n += 1
    # T4 decision branches
    mk = lambda pt, lo: {"point": pt, "lo": lo, "hi": pt + 0.01}
    pp = {str(p): {"G_mid": mk(0.30, 0.20)} for p in PRIMES}
    assert decide(pp)["verdict"] == "supported"
    pp["16411"]["G_mid"] = mk(0.14, 0.10)
    assert decide(pp)["verdict"] == "inconclusive"
    pp["16411"]["G_mid"] = mk(0.30, -0.01)
    assert decide(pp)["verdict"] == "inconclusive"
    pp["32771"]["G_mid"] = mk(0.049, 0.01)
    assert decide(pp)["verdict"] == "killed"
    pp = {str(p): {"G_mid": mk(0.30, 0.20)} for p in PRIMES}
    pp["32771"]["G_mid"] = mk(0.05, 0.01)
    assert decide(pp)["verdict"] == "inconclusive"
    n += 5
    # T5 gates on the fake set pass except G6 (fake trials) and G7 (fake la_mul scale 0.6 < 1
    # with trials higher at cc = 1: passes) -> G6 must fail, all others pass
    gt = gates(data)
    for k, v in gt["gates"].items():
        if k == "G6_scale_reproduction":
            assert v["pass"] is False, k
        else:
            assert v["pass"] is True, (k, v)
        n += 1
    # T6 gates catch faults: wrong fb_cols, uncertified run, non-deterministic rerun,
    # rho differing between cc files, wrong instance
    import copy
    bad = copy.deepcopy(data)
    next(r for r in bad[(8209, 1.0, "main")] if r.get("kind") == "run")["fb_cols"] = 1
    assert gates(bad)["gates"]["G2_cc_honoured"]["pass"] is False
    bad = copy.deepcopy(data)
    next(r for r in bad[(16411, 2.0, "main")] if r.get("kind") == "run")["certified"] = False
    assert gates(bad)["gates"]["G3_certified_complete"]["pass"] is False
    bad = copy.deepcopy(data)
    next(r for r in bad[(32771, 2.0, "rerun")] if r.get("kind") == "run")["trials"] += 1
    assert gates(bad)["gates"]["G4_determinism"]["pass"] is False
    bad = copy.deepcopy(data)
    next(r for r in bad[(32771, 1.0, "main")] if r.get("arm") == RHO_ARM)["cantor_mul"] += 1
    assert gates(bad)["gates"]["G5_rho_independent_of_cc"]["pass"] is False
    bad = copy.deepcopy(data)
    next(r for r in bad[(8209, 1.0, "main")] if r.get("kind") == "instance")["order"] += 1
    assert gates(bad)["gates"]["G1_instances"]["pass"] is False
    bad = copy.deepcopy(data)
    del bad[(8209, 2.0, "rerun")]
    assert gates(bad)["gates"]["G0_files_present"]["pass"] is False
    bad = copy.deepcopy(data)
    for r in bad[(16411, 2.0, "main")]:
        if r.get("arm") == IC_ARM and r["seed"] % 2 == 0:
            r["cantor_mul"] *= 2
    assert gates(bad)["gates"]["G9_aa_noise_floor"]["pass"] is False
    n += 7
    # T7 bootstrap is deterministic and brackets the point on a noisy set
    rng = random.Random(1)
    a = {(c, s): {"cantor_mul": 1000 + rng.randint(0, 50), "cantor_inv": 0, "rel_mul": 0,
                  "la_mul": 0} for c in (1, 2, 3) for s in SEEDS}
    b = {k: {"cantor_mul": 2000 + rng.randint(0, 50), "cantor_inv": 0, "rel_mul": 0,
             "la_mul": 0} for k in a}
    x1 = hboot(sorted(a), gain(a, b, "mid"))
    x2 = hboot(sorted(a), gain(a, b, "mid"))
    assert x1 == x2 and x1[1] <= x1[0] <= x1[2] and 0.45 < x1[0] < 0.52
    n += 1
    print(f"selftest: {n} checks passed")


def main(argv):
    if len(argv) >= 2 and argv[1] == "selftest":
        selftest()
        return 0
    if len(argv) == 4 and argv[1] == "gates":
        gt = gates(load_dir(argv[2]))
        with open(argv[3], "w") as fh:
            json.dump(gt, fh, indent=1, sort_keys=True)
        print(json.dumps({k: v["pass"] for k, v in gt["gates"].items()}))
        print("all_pass", gt["all_pass"])
        return 0 if gt["all_pass"] else 1
    if len(argv) == 5 and argv[1] == "analyze":
        data = load_dir(argv[2])
        gt = gates(data)
        an = analyze(data)
        an["gates_all_pass"] = gt["all_pass"]
        if not gt["all_pass"]:
            an["decision"]["verdict_if_gates_passed"] = an["decision"]["verdict"]
            an["decision"]["verdict"] = "void"
            an["decision"]["rule"] = "a gate failed (PROTOCOL section 7)"
        with open(argv[3], "w") as fh:
            json.dump(an, fh, indent=1, sort_keys=True)
        with open(argv[4], "w") as fh:
            fh.write(tables(an, gt))
        print("verdict", an["decision"]["verdict"])
        return 0
    print(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
