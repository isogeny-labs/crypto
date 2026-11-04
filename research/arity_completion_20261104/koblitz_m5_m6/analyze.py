#!/usr/bin/env python3
"""Analysis for KOBLITZ-M5-M6-20261104 (stdlib only; always exits 0).

Usage: analyze.py [RAW_DIR] [--grid grid.tsv] [--json ANALYSIS.json] [--tsv table.tsv]

RAW_DIR (default ./raw) is searched recursively for the cell records written
by run_shard.sh: <family>__<curve>__m<M>__union-<S>__<engine>__<probes>.json,
either the n83_msweep_cell JSON (schema n83-msweep-cell/v2) or a
process_cap / process_error / skipped_stopping_rule stub, each with an
optional <record>.json.time sidecar ("wall_seconds peak_rss_kb").

It prints decided / undecided / capped per cell with seconds, the A/A and
negative-control checks, and the verdicts for K1 to K6 of protocol.json.
A capped cell is never given a cost.
"""
import glob, json, math, os, re, sys

PARITY_S = 188.0          # m = 4 root-table parity (n83_msweep, K1107, 175-188 s)
N_OF = {"k0n23": 23, "k1n29": 29, "k0n31": 31, "k0n37": 37, "k0n41": 41, "n83k1": 83}
NAME = re.compile(r"^(?P<fam>[a-zA-Z]+)__(?P<curve>[a-z0-9]+)__m(?P<m>\d+)__(?P<fb>[a-z]+-\d+)__(?P<eng>[a-z0-9-]+)__(?P<probes>public|planted)\.json$")


def variables(m, n):
    return (m - 1) * (3 * n * n + 4 * n) + n * (2 * m - 1)


def load(raw):
    cells = {}
    for path in sorted(glob.glob(os.path.join(raw, "**", "*.json"), recursive=True)):
        mt = NAME.match(os.path.basename(path))
        if not mt:
            continue
        try:
            rec = json.load(open(path))
        except Exception as e:  # unreadable record
            rec = {"status": "unreadable", "error": str(e)}
        c = mt.groupdict()
        c["m"] = int(c["m"]); c["S"] = int(c["fb"].split("-")[1]); c["n"] = N_OF.get(c["curve"])
        c["rec"] = rec; c["path"] = path
        c["wall"] = c["rss_kb"] = None
        try:
            parts = open(path + ".time").read().split()
            c["wall"], c["rss_kb"] = float(parts[-2]), int(parts[-1])
        except Exception:
            pass
        if c["wall"] is None and isinstance(rec.get("wall_seconds"), (int, float)):
            c["wall"] = float(rec["wall_seconds"])
        c["key"] = (c["fam"], c["curve"], c["m"], c["S"], c["eng"], c["probes"])
        cells[c["key"]] = c
    return cells


def classify(c):
    r = c["rec"]; st = r.get("status")
    if st == "process_cap": return "capped"
    if st == "skipped_stopping_rule": return "skipped_stopping_rule"
    if st in ("process_error", "unreadable", "error"): return "process_error"
    if st != "complete": return "process_error"
    out = r.get("outcomes", {}); trials = r.get("trials", 0)
    if out.get("invalid_model"): return "invalid_control"
    w, u = out.get("witness", 0), out.get("proved_unsat", 0)
    if trials and w + u == trials:
        if r.get("verified_relations", 0) != r.get("relations", 0):
            return "invalid_control"
        return "decided_sat" if u == 0 else ("decided_unsat" if w == 0 else "decided_mixed")
    return "undecided"


def is_decided(cls): return cls.startswith("decided")


def spq(c):
    """Seconds per decided query (mean over the cell's queries); None unless decided."""
    r = c["rec"]
    if not is_decided(c["cls"]): return None
    if "collection_seconds" in r and r.get("trials"):
        return r["collection_seconds"] / r["trials"]
    return r.get("seconds_per_trial")


def fmt(x, nd=2): return "-" if x is None else f"{x:.{nd}f}"


def main(argv):
    raw = "raw"; jpath = tpath = None
    a = argv[1:]
    i = 0
    while i < len(a):
        if a[i] == "--json": jpath = a[i + 1]; i += 2
        elif a[i] == "--tsv": tpath = a[i + 1]; i += 2
        elif a[i] == "--grid": i += 2          # grid is documentation here; cells are read from raw/
        else: raw = a[i]; i += 1
    cells = load(raw)
    for c in cells.values(): c["cls"] = classify(c)
    print(f"# KOBLITZ-M5-M6-20261104 analysis: {len(cells)} cell records under {raw}")
    if not cells: print("no records found; every claim is not_evaluable")

    hdr = ["family", "curve", "n", "m", "S", "engine", "probes", "class", "trials", "decided", "verified", "sec/query", "median", "wall_s", "cap_s", "conflicts", "variables", "rss_kb"]
    print("\t".join(hdr))
    rows = []
    order = lambda c: (c["fam"], c["n"] or 0, c["m"], c["S"], c["eng"], c["probes"])
    for c in sorted(cells.values(), key=order):
        r = c["rec"]; cls = c["cls"]
        dec = r.get("decided_fraction")
        row = [c["fam"], c["curve"], c["n"], c["m"], c["S"], c["eng"], c["probes"], cls,
               r.get("trials", "-"), "-" if dec is None else f"{dec:.2f}", r.get("verified_relations", "-"),
               fmt(spq(c), 3), fmt(r.get("median_seconds_per_trial"), 3), fmt(c["wall"], 1),
               r.get("cap_seconds", "-"), r.get("engine_work", "-"),
               variables(c["m"], c["n"]) if c["n"] else "-", c["rss_kb"] if c["rss_kb"] is not None else "-"]
        rows.append(row); print("\t".join(str(x) for x in row))
    if tpath:
        with open(tpath, "w") as f:
            f.write("\t".join(hdr) + "\n")
            for row in rows: f.write("\t".join(str(x) for x in row) + "\n")

    get = lambda fam, curve, m, S, probes, eng="chain-s3-orbit": cells.get((fam, curve, m, S, eng, probes))
    A_CURVES = ["k0n23", "k1n29", "k0n31", "k0n37", "k0n41"]
    verdicts = {}
    notes = []

    # ---- controls -------------------------------------------------------
    print("\n# controls")
    bad_planted = set()      # (curve, m, S) whose planted control missed
    n_planted_done = n_planted_ok = 0
    for fam in ("koblitzA", "liftB"):
        for c in cells.values():
            if c["fam"] == fam and c["probes"] == "planted" and c["cls"] not in ("capped", "skipped_stopping_rule", "process_error"):
                r = c["rec"]; n_planted_done += 1
                ok = r.get("trials") and r.get("verified_relations") == r.get("trials") and r.get("outcomes", {}).get("witness") == r.get("trials")
                if ok: n_planted_ok += 1
                else:
                    bad_planted.add((c["curve"], c["m"], c["S"]))
                    print(f"POSITIVE CONTROL MISS {c['curve']} m={c['m']} S={c['S']}: class={c['cls']} outcomes={r.get('outcomes')} verified={r.get('verified_relations')}")
    print(f"planted cells completed: {n_planted_done}, recovered 4/4: {n_planted_ok}")
    for fam in ("koblitzA", "liftB"):
        for c in cells.values():
            if c["fam"] == fam and c["probes"] == "public" and (c["curve"], c["m"], c["S"]) not in bad_planted:
                p = get(fam, c["curve"], c["m"], c["S"], "planted")
                if is_decided(c["cls"]) and (p is None or p["cls"] in ("capped", "skipped_stopping_rule")):
                    notes.append(f"public cell {c['curve']} m={c['m']} S={c['S']} decided without a completed planted control")

    aa_bad = 0
    for k, c in sorted(cells.items(), key=lambda kv: order(kv[1])):
        if c["fam"] != "aa": continue
        o = cells.get(("koblitzA",) + k[1:])
        if o is None: continue
        r1, r2 = o["rec"], c["rec"]
        if o["cls"] == "skipped_stopping_rule" or c["cls"] == "skipped_stopping_rule":
            continue
        if o["cls"] == "capped" or c["cls"] == "capped":
            tag = "agree(both capped)" if o["cls"] == c["cls"] else "NOISE FLAG (one capped, one not)"
            if o["cls"] != c["cls"]: aa_bad += 1
        else:
            same = all(r1.get(f) == r2.get(f) for f in ("outcomes", "relations", "verified_relations", "base_points", "columns", "trials"))
            tag = "agree" if same else "DISAGREE (invalid pair)"
            if not same: aa_bad += 1
        ratio = (spq(c) / spq(o)) if spq(o) and spq(c) else None
        print(f"A/A {c['curve']} m={c['m']} S={c['S']}: {o['cls']} vs {c['cls']} {tag}; seconds ratio {fmt(ratio, 3)}")
    print(f"A/A pairs with disagreement or noise flag: {aa_bad}")

    neg_undecided = {}
    for c in sorted((c for c in cells.values() if c["fam"] == "negative"), key=order):
        print(f"NEG m=2 {c['curve']} {c['eng']}: {c['cls']} sec/query={fmt(spq(c), 3)}")
        if c["eng"] == "chain-s3-orbit": neg_undecided[c["curve"]] = not is_decided(c["cls"])
    n83neg = cells.get(("negative", "n83k1", 2, 2, "chain-s3", "public"))
    if n83neg and spq(n83neg): print(f"n = 83 chain-s3 m=2 union:2: {spq(n83neg):.2f} s/query against the n83_msweep record 17.33 s (ratio {spq(n83neg)/17.3292:.2f}; diagnostic, different host)")

    # ---- K1 -------------------------------------------------------------
    k1 = [get("koblitzA", "k0n23", m, 2, "public") for m in (5, 6)]
    if any(x is None or x["cls"] in ("skipped_stopping_rule",) for x in k1): v = "not_evaluable"
    elif all(is_decided(x["cls"]) for x in k1): v = "supported"
    else: v = "falsified"
    if v == "supported" and any((("k0n23", m, 2) in bad_planted) for m in (5, 6)): v = "not_evaluable"
    verdicts["K1"] = (v, ", ".join(f"m={m}: {x['cls'] if x else 'missing'} {fmt(spq(x) if x else None, 2)} s/q" for m, x in zip((5, 6), k1)))

    # ---- K2 -------------------------------------------------------------
    m6 = {(cv, S): get("koblitzA", cv, 6, S, "public") for cv in A_CURVES for S in (2, 3)}
    tail = [m6[(cv, S)] for cv in ("k0n37", "k0n41") for S in (2, 3)]
    measured = [x for x in tail if x is not None and x["cls"] != "skipped_stopping_rule"]
    slope_notes, slope_ok = [], True
    for S in (2, 3):
        pts = [(N_OF[cv], spq(m6[(cv, S)])) for cv in A_CURVES if m6[(cv, S)] is not None and spq(m6[(cv, S)])]
        for (n1, s1), (n2, s2) in zip(pts, pts[1:]):
            sl = (math.log2(s2) - math.log2(s1)) / (n2 - n1)
            slope_notes.append(f"S={S} n {n1}->{n2}: log2-slope {sl:.3f}/n")
            if sl <= 0: slope_ok = False
    neg37 = neg_undecided.get("k0n37") or neg_undecided.get("k0n41")
    if not measured: v = "not_evaluable"
    elif any(x["cls"] not in ("capped",) for x in measured): v = "falsified"
    elif not slope_ok: v = "falsified"
    elif len(measured) < 4: v = "supported_partial"
    else: v = "supported"
    if neg37 and v.startswith("supported"): notes.append("K2: m=2 orbit negative control undecided at n=37/41; capped cells there are uninformative")
    verdicts["K2"] = (v, f"{len(measured)}/4 tail cells measured, " + ", ".join(f"{x['curve']} S={x['S']}: {x['cls']}" for x in measured) + ("; " + "; ".join(slope_notes) if slope_notes else "; no two decided points for a slope"))

    # ---- K3 -------------------------------------------------------------
    if n_planted_done == 0: v = "not_evaluable"
    else: v = "falsified" if bad_planted else "supported"
    verdicts["K3"] = (v, f"{n_planted_ok}/{n_planted_done} completed planted cells recovered 4/4; cells that capped are unmeasured")

    # ---- K4 -------------------------------------------------------------
    b = [get("liftB", "n83k1", m, S, "public") for m in (5, 6) for S in (2, 3)]
    if any(x is None for x in b): v = "not_evaluable"
    elif any(is_decided(x["cls"]) for x in b): v = "falsified"
    elif all(x["cls"] in ("capped", "undecided") for x in b): v = "supported"
    else: v = "not_evaluable"
    if v == "supported" and cells.get(("negative", "n83k1", 2, 2, "chain-s3-orbit", "public")) and not is_decided(cells[("negative", "n83k1", 2, 2, "chain-s3-orbit", "public")]["cls"]):
        notes.append("K4: the n = 83 m=2 orbit negative control is undecided at 1800 s; K4 is supported but weak (the orbit engine does not decide even m = 2 there)")
    verdicts["K4"] = (v, ", ".join(f"m={x['m']} S={x['S']}: {x['cls']}" if x else "missing" for x in b))

    # ---- K5 -------------------------------------------------------------
    cand = [c for c in cells.values() if c["probes"] == "public" and c["fam"] in ("koblitzA", "liftB") and c["n"] and c["n"] >= 31]
    dec = [c for c in cand if is_decided(c["cls"])]
    under = [c for c in dec if spq(c) is not None and spq(c) < PARITY_S]
    if not cand: v = "not_evaluable"
    elif under: v = "falsified"
    else: v = "supported"
    detail = f"{len(dec)} decided public cells at n >= 31 of {len(cand)} recorded"
    if v == "supported" and not dec: detail += " (vacuous: none decided)"
    if under: detail += "; under 188 s: " + ", ".join(f"{c['curve']} m={c['m']} S={c['S']} {spq(c):.1f} s ({spq(c)/PARITY_S:.2f} x parity)" for c in under)
    verdicts["K5"] = (v, detail)

    # ---- K6 kill criterion ---------------------------------------------
    if verdicts["K3"][0] == "falsified": v = "not_evaluable_instrument_invalid"
    elif verdicts["K4"][0] == "supported" and verdicts["K5"][0] == "supported": v = "kill_high_arity_koblitz_line"
    elif "not_evaluable" in (verdicts["K4"][0], verdicts["K5"][0]): v = "not_evaluable"
    else: v = "line_stays_open"
    verdicts["K6"] = (v, f"K4={verdicts['K4'][0]}, K5={verdicts['K5'][0]}, K3={verdicts['K3'][0]}")

    print("\n# verdicts")
    for k in ("K1", "K2", "K3", "K4", "K5", "K6"): print(f"{k}: {verdicts[k][0]} | {verdicts[k][1]}")

    # ---- descriptive comparisons ---------------------------------------
    print("\n# C-m5-vs-m6 (seconds per decided query ratio m6/m5; outcomes otherwise)")
    for cv in A_CURVES:
        for S in (2, 3):
            x, y = get("koblitzA", cv, 5, S, "public"), get("koblitzA", cv, 6, S, "public")
            if x is None and y is None: continue
            s5, s6 = (spq(x) if x else None), (spq(y) if y else None)
            print(f"{cv} S={S}: m5={x['cls'] if x else 'missing'} m6={y['cls'] if y else 'missing'}" + (f" ratio {s6/s5:.2f}" if s5 and s6 else ""))
    print("\n# decided cells: distance from the 188 s m = 4 root-table parity point")
    any_dec = False
    for c in sorted(cells.values(), key=order):
        if c["probes"] == "public" and c["fam"] in ("koblitzA", "liftB") and is_decided(c["cls"]):
            any_dec = True; print(f"{c['curve']} m={c['m']} S={c['S']}: {spq(c):.2f} s/query = {spq(c)/PARITY_S:.3f} x 188 s")
    if not any_dec: print("no decided public cell")
    print("\n# C-lift-n83: the 180 s record is table.tsv of research/n83_msweep_20261024; compare the class column above (capped at 180 s there).")
    for n in notes: print("NOTE:", n)

    if jpath:
        out = {"verdicts": {k: {"verdict": v[0], "detail": v[1]} for k, v in verdicts.items()},
               "cells": [{"family": c["fam"], "curve": c["curve"], "m": c["m"], "S": c["S"], "engine": c["eng"], "probes": c["probes"],
                          "class": c["cls"], "seconds_per_decided_query": spq(c), "wall_seconds": c["wall"], "peak_rss_kb": c["rss_kb"],
                          "conflicts": c["rec"].get("engine_work"), "variables": variables(c["m"], c["n"]) if c["n"] else None,
                          "path": os.path.relpath(c["path"])} for c in sorted(cells.values(), key=order)],
               "notes": notes, "parity_seconds": PARITY_S}
        with open(jpath, "w") as f: json.dump(out, f, indent=1, sort_keys=True)
    return 0


if __name__ == "__main__":
    try:
        main(sys.argv)
    except Exception as e:           # the analysis never fails the pipeline
        print(f"analyze.py internal error: {type(e).__name__}: {e}")
    sys.exit(0)
