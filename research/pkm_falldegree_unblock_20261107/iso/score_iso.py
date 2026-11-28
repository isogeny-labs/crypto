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
T_ALL, T_P1, T_P2 = range(1, 15), range(1, 9), range(9, 14)  # t = 14: Amendment 19, P-ISO-3 only
P1 = 2013265921


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


CONTROL = "tower"   # "null" for Amendment 21's isogeny null (set by --null)


def rows_of(path):
    for line in open(path):
        if line.startswith("{"):
            r = json.loads(line)
            if "x_r" in r and r.get("kind") == "isogeny" and r.get("m") == 2 and r.get("control") == CONTROL:
                yield r


def f4_done(r):
    return not r.get("timed_out") and not r.get("oversize")


def score(runs):
    """Per-draw bounds, with conflicts flagged (re-review edits R2-E1, R2-E2, R2-E4).

    Closure (A-m2-iso-p1*) gives an exact value or a lower bound Lc. F4 gives an interval [Lf, Uf]: a refuting uncapped
    run at D -> Uf = D; a completed capped run at C -> Uf = C if it refutes, Lf = C + 1 if not; a stopped run gives nothing.
    Conflicts (any one stops R4): L > U; a closure-exact value outside [Lf, Uf]; a closure lower bound above a refuting D
    (G2 in bound form); sources of one draw disagreeing on x_R or on the auxiliary curve.
    Admission: a draw enters a median only if it is refuted (closure exact, or some F4 refutation) or certified
    (CERT solutions == 0); a decomposable draw (solutions > 0) is excluded.
    """
    draws = {}
    def d(t, k):
        return draws.setdefault((t, k), {"t": t, "k": k, "closure": None, "Lf": None, "Uf": None, "D": None,
                                         "refuted": False, "solutions": None, "sources": [], "conflicts": []})
    def src(x, path, r, what):
        x["sources"].append({"file": os.path.basename(path), "what": what, "x_r": r.get("x_r"),
                             "aux": [r.get("tower", {}).get("aux_a"), r.get("tower", {}).get("aux_b")]})
        # Case H: every R4 row is at P1 and built with --aux-order mestre.
        if r.get("p") != P1 or r.get("tower", {}).get("aux_order_method") != "mestre":
            x["conflicts"].append(f"{os.path.basename(path)}: row not at P1 or not built with --aux-order mestre")
    for p in sorted(glob.glob(os.path.join(runs, "A-m2-iso-p1*.jsonl"))):
        for r in rows_of(p):
            x = d(r["t"], r["target_index"])
            x["closure"] = d_f(r)
            if x["closure"][1] == "":
                x["refuted"] = True
            src(x, p, r, "closure " + "".join(map(str, x["closure"][::-1])))
    for p in sorted(glob.glob(os.path.join(runs, "DA-m2-iso-p1*.jsonl"))):
        for r in rows_of(p):
            x = d(r["t"], r["target_index"])
            if f4_done(r) and r.get("inconsistent"):
                x["D"] = r["solving_degree_max"]
                x["Uf"] = x["D"] if x["Uf"] is None else min(x["Uf"], x["D"])
                x["refuted"] = True
                src(x, p, r, f"F4 refutes at D={x['D']}")
            else:
                src(x, p, r, "F4 stopped or not refuted: no bound")
    for pat in ("CAP-iso-m2-t*-k*-c*.jsonl", "CAPN-iso-m2-t*-k*-c*.jsonl", "X-iso-m2-t*-k*-c*.jsonl"):
        for p in sorted(glob.glob(os.path.join(runs, pat))):
            m = re.match(r"(CAP|CAPN|X)-iso-m2-t(\d+)-k(\d+)-c(\d+)\.jsonl$", os.path.basename(p))
            if not m:
                continue
            cap = int(m.group(4))
            for r in rows_of(p):
                x = d(r["t"], r["target_index"])
                if not f4_done(r):
                    src(x, p, r, f"cap {cap} stopped: no bound")
                    continue
                if r.get("inconsistent"):
                    x["Uf"] = cap if x["Uf"] is None else min(x["Uf"], cap)
                    x["refuted"] = True
                    src(x, p, r, f"cap {cap} refutes: <= {cap}")
                else:
                    x["Lf"] = max(x["Lf"] or 0, cap + 1)
                    src(x, p, r, f"cap {cap} does not refute: >= {cap + 1}")
    for p in sorted(glob.glob(os.path.join(runs, "CERT-m2-iso-p1.jsonl")) + glob.glob(os.path.join(runs, "*.cert.jsonl"))):
        for line in open(p):
            c = json.loads(line)
            x = d(c["t"], c["target_index"])
            x["solutions"] = c["solutions"] if x["solutions"] is None else max(x["solutions"], c["solutions"])
            x["sources"].append({"file": os.path.basename(p), "what": f"certificate solutions={c['solutions']}",
                                 "x_r": c.get("x_r"), "aux": None})
    out = []
    for (t, k), x in sorted(draws.items()):
        xs = {s["x_r"] for s in x["sources"] if s["x_r"] is not None}
        auxs = {tuple(s["aux"]) for s in x["sources"] if s["aux"] and s["aux"][0] is not None}
        if len(xs) > 1:
            x["conflicts"].append(f"sources disagree on x_R: {sorted(xs)}")
        if len(auxs) > 1:
            x["conflicts"].append(f"sources disagree on the auxiliary curve: {sorted(auxs)}")
        cv, cc = x["closure"] if x["closure"] else (None, None)
        L = max(v for v in (cv, x["Lf"]) if v is not None) if (cv is not None or x["Lf"] is not None) else None
        U = x["Uf"]
        if cc == "":
            if (x["Lf"] is not None and cv < x["Lf"]) or (U is not None and cv > U):
                x["conflicts"].append(f"closure exact {cv} outside F4 interval [{x['Lf']}, {U}]")
            L = U = cv
        elif cc and x["D"] is not None and cv > x["D"]:
            x["conflicts"].append(f"G2 (bound form): closure >= {cv} above refuting D = {x['D']}")
        if L is not None and U is not None and L > U:
            x["conflicts"].append(f"empty interval [{L}, {U}]")
        x["dF"] = None if L is None else ((L, "") if U is not None and L == U else (L, ">="))
        x["decomposable"] = x["solutions"] is not None and x["solutions"] > 0
        # Case F: a refuted system has no solution on the grid (verify.py's cross-check); both at once is a conflict.
        if x["decomposable"] and x["refuted"]:
            x["conflicts"].append(f"refuted, but the certificate finds {x['solutions']} V x V solutions")
        x["admitted"] = not x["decomposable"] and x["dF"] is not None and (x["refuted"] or x["solutions"] == 0)
        # G2 (registered for m = 2, t <= 10): exact closure d_F <= D.
        x["g2"] = None if (cc != "" or x["D"] is None or t > 10) else cv <= x["D"]
        out.append(x)
    return out


def readings(draws):
    conflicts = [(x["t"], x["k"], c) for x in draws for c in x["conflicts"]]
    med, n_ge6, n_dec, complete = {}, {}, {}, {}
    for t in T_ALL:
        ds = {x["k"]: x for x in draws if x["t"] == t}
        n_dec[t] = sum(1 for x in ds.values() if x["decomposable"])
        keep = [x["dF"] for x in ds.values() if x["admitted"]]
        # R2-E3: both draws are needed, unless a draw is certified decomposable.
        complete[t] = all((k in ds) and (ds[k]["admitted"] or ds[k]["decomposable"]) for k in (0, 1)) and keep != []
        med[t] = lower_median(keep)
        n_ge6[t] = sum(1 for v, c in keep if v >= 6)
    ex = lambda t: complete[t] and med[t] is not None and med[t][1] == ""
    if any(ex(t) and med[t][0] != P0_MEDIANS[t] for t in T_P1):
        p1 = "refuted"
    elif all(ex(t) and med[t][0] == P0_MEDIANS[t] for t in T_P1):
        p1 = "holds"
    else:
        p1 = "undecided"
    per = {}
    for t in T_P2:
        m = med[t]
        if not complete[t] or m is None:
            per[t] = "undecided"
        elif (m[1] == "" and m[0] > 5) or (m[1] == ">=" and m[0] >= 6):
            per[t] = "grows"
        elif m[1] == "" and m[0] < 5:
            per[t] = "below 5"
        elif m[1] == "" and m[0] == 5:
            per[t] = "flat at 5"
        else:
            per[t] = "undecided"
    vals = set(per.values())
    p2 = ("grows" if "grows" in vals else "below 5" if "below 5" in vals
          else "flat at 5" if vals == {"flat at 5"} else "undecided")
    m14 = med.get(14)
    if not complete.get(14) or m14 is None:
        p3 = "undecided"
    elif (m14[1] == "" and m14[0] > 5) or (m14[1] == ">=" and m14[0] >= 6):
        p3 = "grows"
    elif m14[1] == "" and m14[0] < 5:
        p3 = "below 5"
    elif m14[1] == "" and m14[0] == 5:
        p3 = "flat at 5"
    else:
        p3 = "undecided"
    g2 = [x for x in draws if x["g2"] is not None]
    g2v = sum(1 for x in g2 if not x["g2"])
    stop = bool(conflicts) or g2v > 0
    return {"STOP": stop, "conflicts": conflicts,
            "medians": {t: (None if med[t] is None else f"{med[t][1]}{med[t][0]}") for t in T_ALL},
            "cell_complete": complete, "draws_dF_ge_6": n_ge6, "decomposable_draws": n_dec,
            "P_ISO_1": "STOPPED" if stop else p1, "P_ISO_2_per_t": per, "P_ISO_2": "STOPPED" if stop else p2,
            "P_ISO_3_t14": "STOPPED" if stop else p3,
            "G2": {"checked": len(g2), "violations": g2v}}


def _selftest():
    import tempfile
    tmp = tempfile.mkdtemp()
    def row(t, k, **kw):
        r = {"x_r": 1000 + 10 * t + k, "kind": "isogeny", "m": 2, "control": "tower", "t": t, "target_index": k,
             "p": P1, "tower": {"aux_a": 1, "aux_b": 2, "aux_order_method": "mestre"}}
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
          for t in range(1, 14) for k in (0, 1) if (t, k) != (13, 1)]
    DA.append(row(13, 1, inconsistent=False, timed_out=True))   # draw 1 at t = 13 is decomposable: F4 cannot refute it
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


def _selftest_review_cases():
    """The re-review's cases A-E (REVIEW_R3_R4.md, re-review of R4 revision 2), each with the required outcome."""
    import tempfile
    def base(t=13, k=0, **kw):
        r = {"x_r": 777, "kind": "isogeny", "m": 2, "control": "tower", "t": t, "target_index": k,
             "p": P1, "tower": {"aux_a": 1, "aux_b": 2, "aux_order_method": "mestre"}}
        r.update(kw)
        return r
    def run(files):
        tmp = tempfile.mkdtemp()
        for name, rs in files.items():
            open(os.path.join(tmp, name), "w").write("".join(json.dumps(r) + "\n" for r in rs))
        return readings(score(tmp))
    cert = lambda sols, x_r=777: [{"t": 13, "target_index": k, "x_r": x_r, "solutions": sols} for k in (0, 1)]
    # A: closure >= 8 but uncapped F4 refutes at D = 6 -> conflict, STOP.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=None, closures=[{"c": 8, "censored": "budget"}]) for k in (0, 1)],
              "DA-m2-iso-p1.jsonl": [base(k=k, inconsistent=True, solving_degree_max=6, timed_out=False) for k in (0, 1)]})
    assert rd["STOP"] and rd["P_ISO_2"] == "STOPPED", ("A", rd["conflicts"])
    # B: censored >= 6, no refutation and no certificate -> not admitted -> undecided (not 'grows').
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=None, closures=[{"c": 5, "censored": None}]) for k in (0, 1)]})
    assert not rd["STOP"] and rd["P_ISO_2_per_t"][13] == "undecided", ("B", rd)
    #    ... and with a solutions == 0 certificate it is admitted and reads 'grows'.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=None, closures=[{"c": 5, "censored": None}]) for k in (0, 1)],
              "CERT-m2-iso-p1.jsonl": cert(0)})
    assert rd["P_ISO_2_per_t"][13] == "grows", ("B2", rd)
    # C: one draw missing -> the cell is incomplete -> undecided, not 'flat at 5'.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=0, last_fall_degree=5, closures=[])]})
    assert rd["P_ISO_2_per_t"][13] == "undecided" and not rd["cell_complete"][13], ("C", rd)
    # D: closure exact 5 but a completed cap-5 run does not refute (>= 6) -> conflict, STOP.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=5, closures=[]) for k in (0, 1)],
              "CAP-iso-m2-t13-k0-c5.jsonl": [base(k=0, inconsistent=False, timed_out=False)]})
    assert rd["STOP"], ("D", rd["conflicts"])
    # E: sources of one draw with different x_R -> conflict, STOP.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=None, closures=[{"c": 5, "censored": None}]) for k in (0, 1)],
              "DA-m2-iso-p1.jsonl": [base(k=k, x_r=999, inconsistent=True, solving_degree_max=6, timed_out=False) for k in (0, 1)]})
    assert rd["STOP"] and any("x_R" in c[2] for c in rd["conflicts"]), ("E", rd["conflicts"])
    # F: closure refutes (exact 5) on both draws, but the certificate finds solutions on draw 1 -> conflict, STOP.
    rd = run({"A-m2-iso-p1-big.jsonl": [base(k=k, last_fall_degree=5, closures=[]) for k in (0, 1)],
              "CERT-m2-iso-p1.jsonl": [{"t": 13, "target_index": 0, "x_r": 777, "solutions": 0},
                                       {"t": 13, "target_index": 1, "x_r": 777, "solutions": 2}]})
    assert rd["STOP"] and any("certificate finds" in c[2] for c in rd["conflicts"]), ("F", rd["conflicts"])
    # H: a row at the wrong prime, or without the mestre flag -> conflict, STOP.
    bad = base(k=0, last_fall_degree=5, closures=[]); bad["p"] = 786433
    rd = run({"A-m2-iso-p1-big.jsonl": [bad, base(k=1, last_fall_degree=5, closures=[])]})
    assert rd["STOP"], ("H1", rd["conflicts"])
    bad = base(k=0, last_fall_degree=5, closures=[]); bad["tower"] = {"aux_a": 1, "aux_b": 2}
    rd = run({"A-m2-iso-p1-big.jsonl": [bad, base(k=1, last_fall_degree=5, closures=[])]})
    assert rd["STOP"], ("H2", rd["conflicts"])
    print("selftest (review cases A-F, H): pass")


def null_reading(tower_json, null_runs):
    """Amendment 21, P-ISO-N: per t = 1..13, the isogeny tower median (from a committed score_iso.json) against the
    isogeny null median (lower median, both draws, the same bound rules; null draws are not certified: a null system
    is not a summation system). 'shape-driven' if every t with both medians exact has them equal and at least 8 such t
    exist; 'Semaev-specific' if some t has both exact and different, or one exact and the other a bound above it;
    'undecided' otherwise."""
    global CONTROL
    tw = json.load(open(tower_json))["readings"]["medians"]
    CONTROL = "null"
    ds = score(null_runs)
    out, eq, diff = {}, 0, 0
    for t in range(1, 14):
        keep = [x["dF"] for x in ds if x["t"] == t and x["dF"] is not None and not x["conflicts"]]
        both = len([x for x in ds if x["t"] == t]) == 2 and len(keep) == 2
        m = lower_median(keep) if both else None
        nm = None if m is None else f"{m[1]}{m[0]}"
        tm = tw.get(str(t), tw.get(t))
        out[t] = {"tower": tm, "null": nm}
        if nm and tm:
            ex_t, ex_n = not tm.startswith(">"), not nm.startswith(">")
            vt, vn = int(tm.lstrip(">=")), int(nm.lstrip(">="))
            if ex_t and ex_n:
                eq += vt == vn
                diff += vt != vn
            elif (ex_t and not ex_n and vn > vt) or (ex_n and not ex_t and vt > vn):
                diff += 1
    conflicts = [(x["t"], x["k"], c) for x in ds for c in x["conflicts"]]
    reading = ("STOPPED" if conflicts else "Semaev-specific" if diff else "shape-driven" if eq >= 8 else "undecided")
    return {"per_t": out, "exact_equal": eq, "differ": diff, "conflicts": conflicts, "P_ISO_N": reading}


if __name__ == "__main__":
    if sys.argv[1:2] == ["--null"]:
        r = null_reading(sys.argv[2], sys.argv[3])
        json.dump(r, open(sys.argv[4], "w"), indent=1)
        print(json.dumps(r, indent=1))
        sys.exit(2 if r["P_ISO_N"] == "STOPPED" else 0)
    if sys.argv[1:] == ["--selftest"]:
        _selftest()
        _selftest_review_cases()
    else:
        ds = score(sys.argv[1])
        rd = readings(ds)
        json.dump({"schema": "pfd.iso-score/v1", "draws": ds, "readings": rd}, open(sys.argv[2], "w"), indent=1, default=str)
        print(json.dumps(rd, indent=1))
        sys.exit(2 if rd["STOP"] else 0)   # F-R4-1: a STOP is a non-zero exit
