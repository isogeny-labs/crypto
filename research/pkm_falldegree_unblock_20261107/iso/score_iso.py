#!/usr/bin/env python3
"""Committed scorer for Amendment R4 (draft): the 2-isogeny arm at P1 (edits E4.4, E4.5).

Ports, unchanged, the frozen scorer's semantics (examples/prime_fall_degree_tools.rs at f13a690df0):
  d_f(row)          closure d_F: `last_fall_degree` exact, else from the last closure c: `>` c+1 if it was not censored,
                    `>=` c if it was;
  lower_median(v)   value at index (n-1)//2 after sorting; censored ('>=') if any censored value is at or below it.
Per draw (t, target_index), the bounds are combined in this order:
  1. closure (A-m2-iso-p1*): exact d_F, or lower bound L;
  2. F4 (DA-m2-iso-p1): refuted at solving degree D gives an upper bound U = D (G2: d_F <= D);
  3. CAP-iso-m2-tT-kK-cC (capped F4 at C = D - 1): completed, not refuted -> L = max(L, C + 1); refuted -> U = min(U, C);
  4. X-iso-m2-tT-kK-cC (Amendment 7 B bracket): the same rule;
  exact when L == U; else '>=' L. Decomposable draws (certificate: solutions > 0) are excluded from every median.
Readings (E4.4) follow AMENDMENT_R4_ISOGENY_P1.md section 5 exactly.
    python3 score_iso.py RUNS_DIR OUT.json     (RUNS_DIR holds the job's runs/*.jsonl; prints the readings)
    python3 score_iso.py --selftest
"""
import glob, json, os, re, sys

P0_MEDIANS = {1: 3, 2: 5, 3: 5, 4: 5, 5: 5, 6: 5, 7: 5, 8: 5}  # admitted, results/R9/analysis/pfdA_summary.json
T_ALL, T_P1, T_P2 = range(1, 14), range(1, 9), range(9, 14)


def d_f(r):
    if r.get("last_fall_degree") is not None:
        return (r["last_fall_degree"], "")
    last = r["closures"][-1]
    return (last["c"] + 1, ">") if last.get("censored") is None else (last["c"], ">=")


def lower_median(v):
    if not v:
        return None
    s = sorted(v, key=lambda x: x[0])
    med = s[(len(s) - 1) // 2][0]
    cens = any(c and x <= med for x, c in s)
    return (med, ">=" if cens else "")


def rows_of(path):
    for line in open(path):
        if line.startswith("{"):
            r = json.loads(line)
            if "x_r" in r and r.get("kind") == "isogeny" and r.get("m") == 2 and r.get("control") == "tower":
                yield r


def f4_done(r):
    return not r.get("timed_out") and not r.get("oversize")


def score(runs):
    draws = {}
    def d(t, k):
        return draws.setdefault((t, k), {"t": t, "k": k, "L": None, "U": None, "exact": None, "src": [], "D": None,
                                         "closure": None, "decomposable": None})
    for p in sorted(glob.glob(os.path.join(runs, "A-m2-iso-p1*.jsonl"))):
        for r in rows_of(p):
            x = d(r["t"], r["target_index"])
            v, c = d_f(r)
            x["closure"] = (v, c)
            if c == "":
                x["exact"] = v
            else:
                x["L"] = max(x["L"] or 0, v)  # (c+1, '>') and (c, '>=') are both lower bounds d_F >= v
            x["src"].append(os.path.basename(p))
    for p in sorted(glob.glob(os.path.join(runs, "DA-m2-iso-p1.jsonl"))):
        for r in rows_of(p):
            x = d(r["t"], r["target_index"])
            if f4_done(r) and r.get("inconsistent"):
                x["D"] = r["solving_degree_max"]
                x["U"] = r["solving_degree_max"] if x["U"] is None else min(x["U"], r["solving_degree_max"])
    for pat in ("CAP-iso-m2-t*-k*-c*.jsonl", "X-iso-m2-t*-k*-c*.jsonl"):
        for p in sorted(glob.glob(os.path.join(runs, pat))):
            m = re.match(r"(CAP|X)-iso-m2-t(\d+)-k(\d+)-c(\d+)\.jsonl$", os.path.basename(p))
            if not m:
                continue
            cap = int(m.group(4))
            for r in rows_of(p):
                x = d(r["t"], r["target_index"])
                if not f4_done(r):
                    continue
                if r.get("inconsistent"):
                    x["U"] = cap if x["U"] is None else min(x["U"], cap)
                else:
                    x["L"] = max(x["L"] or 0, cap + 1)
                x["src"].append(os.path.basename(p))
    for p in sorted(glob.glob(os.path.join(runs, "CERT-m2-iso-p1.jsonl")) + glob.glob(os.path.join(runs, "*.cert.jsonl"))):
        for line in open(p):
            c = json.loads(line)
            x = d(c["t"], c["target_index"])
            x["decomposable"] = bool(x["decomposable"]) or c["solutions"] > 0
    out = []
    for (t, k), x in sorted(draws.items()):
        if x["exact"] is None and x["L"] is not None and x["U"] is not None and x["L"] == x["U"]:
            x["exact"] = x["L"]
        x["dF"] = (x["exact"], "") if x["exact"] is not None else ((x["L"], ">=") if x["L"] is not None else None)
        # G2 (registered for m = 2, t <= 10): exact closure d_F <= D.
        x["g2"] = None if (x["closure"] is None or x["closure"][1] or x["D"] is None or t > 10) else x["closure"][0] <= x["D"]
        out.append(x)
    return out


def readings(draws):
    med, n_ge6, n_dec = {}, {}, {}
    for t in T_ALL:
        ds = [x for x in draws if x["t"] == t]
        n_dec[t] = sum(1 for x in ds if x["decomposable"])
        keep = [x["dF"] for x in ds if not x["decomposable"] and x["dF"] is not None]
        med[t] = lower_median(keep)
        n_ge6[t] = sum(1 for v, c in keep if v >= 6)
    ex = lambda m: m is not None and m[1] == ""
    # P-ISO-1
    if any(ex(med[t]) and med[t][0] != P0_MEDIANS[t] for t in T_P1):
        p1 = "refuted"
    elif all(ex(med[t]) and med[t][0] == P0_MEDIANS[t] for t in T_P1):
        p1 = "holds"
    else:
        p1 = "undecided"
    # P-ISO-2 per t
    per = {}
    for t in T_P2:
        m = med[t]
        if m is None:
            per[t] = "undecided"
        elif (ex(m) and m[0] > 5) or (m[1] == ">=" and m[0] >= 6):
            per[t] = "grows"
        elif ex(m) and m[0] < 5:
            per[t] = "below 5"
        elif ex(m) and m[0] == 5:
            per[t] = "flat at 5"
        else:
            per[t] = "undecided"
    vals = set(per.values())
    p2 = ("grows" if "grows" in vals else "below 5" if "below 5" in vals
          else "flat at 5" if vals == {"flat at 5"} else "undecided")
    g2 = [x for x in draws if x["g2"] is not None]
    return {"medians": {t: (None if med[t] is None else f"{med[t][1]}{med[t][0]}") for t in T_ALL},
            "draws_dF_ge_6": n_ge6, "decomposable_draws": n_dec,
            "P_ISO_1": p1, "P_ISO_2_per_t": per, "P_ISO_2": p2,
            "G2": {"checked": len(g2), "violations": sum(1 for x in g2 if not x["g2"])}}


def _selftest():
    import tempfile
    tmp = tempfile.mkdtemp()
    def row(t, k, **kw):
        r = {"x_r": 1000 + 10 * t + k, "kind": "isogeny", "m": 2, "control": "tower", "t": t, "target_index": k}
        r.update(kw)
        return json.dumps(r)
    A = [row(t, k, last_fall_degree=P0_MEDIANS.get(t, 5), closures=[{"c": 5, "censored": None}])
         for t in range(1, 11) for k in (0, 1)]
    # t = 11: exact 5 by closure; t = 12: last closure c = 5 finished without 1 -> (6, '>'), i.e. d_F >= 6
    A += [row(11, k, last_fall_degree=5, closures=[]) for k in (0, 1)]
    A += [row(12, k, last_fall_degree=None, closures=[{"c": 5, "censored": None}]) for k in (0, 1)]
    A += [row(13, k, last_fall_degree=None, closures=[{"c": 4, "censored": "budget"}]) for k in (0, 1)]
    open(os.path.join(tmp, "A-m2-iso-p1.jsonl"), "w").write("\n".join(A[:20]) + "\n")
    open(os.path.join(tmp, "A-m2-iso-p1-big.jsonl"), "w").write("\n".join(A[20:]) + "\n")
    DA = [row(t, k, inconsistent=True, solving_degree_max=(P0_MEDIANS.get(t, 5) if t <= 11 else 6), timed_out=False)
          for t in range(1, 14) for k in (0, 1)]
    open(os.path.join(tmp, "DA-m2-iso-p1.jsonl"), "w").write("\n".join(DA) + "\n")
    # t = 12: closure '>' 6 and DA U = 6 -> exact 6 (CAP at 5 not refuting agrees); t = 13 draw 0: closure censored at 4,
    # X cap 5 not refuting -> L = 6, DA U = 6 -> exact 6; draw 1 is decomposable and excluded.
    open(os.path.join(tmp, "CAP-iso-m2-t12-k0-c5.jsonl"), "w").write(row(12, 0, inconsistent=False, timed_out=False) + "\n")
    open(os.path.join(tmp, "CAP-iso-m2-t12-k1-c5.jsonl"), "w").write(row(12, 1, inconsistent=False, timed_out=False) + "\n")
    open(os.path.join(tmp, "X-iso-m2-t13-k0-c5.jsonl"), "w").write(row(13, 0, inconsistent=False, timed_out=False) + "\n")
    open(os.path.join(tmp, "CERT-m2-iso-p1.jsonl"), "w").write(
        json.dumps({"t": 13, "target_index": 1, "solutions": 2}) + "\n")
    rd = readings(score(tmp))
    want = {"P_ISO_1": "holds", "P_ISO_2": "grows",
            "per": {9: "flat at 5", 10: "flat at 5", 11: "flat at 5", 12: "grows", 13: "grows"},
            "med": {12: "6", 13: "6"}, "ge6": {12: 2, 13: 1}, "dec": {13: 1}, "g2": (20, 0)}
    got = (rd["P_ISO_1"], rd["P_ISO_2"], rd["P_ISO_2_per_t"], {t: rd["medians"][t] for t in (12, 13)},
           {t: rd["draws_dF_ge_6"][t] for t in (12, 13)}, {13: rd["decomposable_draws"][13]},
           (rd["G2"]["checked"], rd["G2"]["violations"]))
    exp = (want["P_ISO_1"], want["P_ISO_2"], want["per"], want["med"], want["ge6"], want["dec"], want["g2"])
    assert got == exp, f"selftest\n got {got}\nwant {exp}"
    # lower_median semantics of the frozen scorer
    assert lower_median([(6, ""), (6, ">=")]) == (6, ">=")
    assert lower_median([(5, ""), (6, ">=")]) == (5, "")
    assert lower_median([(5, ">="), (7, "")]) == (5, ">=")
    print("selftest: pass")


if __name__ == "__main__":
    if sys.argv[1:] == ["--selftest"]:
        _selftest()
    else:
        ds = score(sys.argv[1])
        rd = readings(ds)
        json.dump({"schema": "pfd.iso-score/v1", "draws": ds, "readings": rd}, open(sys.argv[2], "w"), indent=1, default=str)
        print(json.dumps(rd, indent=1))
