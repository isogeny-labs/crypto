"""EXP-PKM-ODD-001 analysis (PROTOCOL.md §5). Frozen with the protocol.

Reads runs/*.jsonl (rows of examples/pkm_odd_tower_f4.rs), applies the
frozen validity checks and decision rule, writes results.json, and prints a
summary. Counted metrics only (field_ops); no clock is read.

    python3 analyze.py [--runs runs] [--out results.json]
"""
import argparse
import glob
import json
import math
import os
import statistics

ARMS = ["A3", "A5", "A23", "A32"]  # Kummer arms against the Kummer A2 reference
GAIN, LOSS = 0.5, 2.0


def load(runs):
    rows = []
    for f in sorted(glob.glob(os.path.join(runs, "*.jsonl"))):
        with open(f) as fh:
            rows += [json.loads(line) for line in fh if line.strip()]
    return rows


def arm_of(cell):
    # cell names: M2-A23-n4-tower, X2-A3-k3-tower, PL-..., AA-...
    return cell.split("-")[1]


def band(r):
    return "gain" if r <= GAIN else "loss" if r >= LOSS else "no_material_effect"


def cells(rows, prefix, control):
    """{arm: [(log2S, S, [W per draw], censored_any, cell)]} sorted by S."""
    out = {}
    by = {}
    for r in rows:
        if not r["cell"].startswith(prefix) or r["control"] != control or r["kind"] != "refute":
            continue
        by.setdefault(r["cell"], []).append(r)
    for cell, rs in by.items():
        rs.sort(key=lambda r: r["draw"])
        out.setdefault(arm_of(cell), []).append(
            (rs[0]["log2S"], rs[0]["S"], [r["field_ops"] for r in rs],
             any(r["censored"] for r in rs), cell))
    for a in out:
        out[a].sort()
    return out


def interp(ref, x):
    """log2-linear interpolation of the A2 reference (list of (log2S, log2W))."""
    for (x0, y0), (x1, y1) in zip(ref, ref[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0) if x1 > x0 else y0
    return None


def verdicts(cs, label):
    ref = [(x, math.log2(statistics.median(w))) for x, _, w, cens, _ in cs.get("A2", []) if not cens]
    res = {"reference_points": len(ref), "arms": {}}
    if len(ref) < 2:
        res["status"] = "no_reference"
        return res
    for arm in ARMS:
        pts = []
        for x, s, w, cens, cell in cs.get(arm, []):
            y = interp(ref, x)
            if cens or y is None:
                pts.append({"cell": cell, "S": s, "log2S": x, "censored": cens, "in_range": y is not None})
                continue
            rs = [wi / 2 ** y for wi in w]
            pts.append({"cell": cell, "S": s, "log2S": x, "r": rs, "r_median": statistics.median(rs),
                        "bands": [band(v) for v in rs], "censored": False, "in_range": True})
        done = [p for p in pts if "r" in p]
        if not done:
            res["arms"][arm] = {"verdict": "no_completed_cell", "points": pts}
            continue
        last = done[-1]
        bs = set(last["bands"])
        v = bs.pop() if len(bs) == 1 else "mixed"
        growing = None
        if v in ("gain", "loss") and len(done) >= 3:
            tail = [p["r_median"] for p in done[-3:]]
            growing = all(b <= a for a, b in zip(tail, tail[1:])) if v == "gain" else \
                all(b >= a for a, b in zip(tail, tail[1:]))
        res["arms"][arm] = {"verdict": v, "growing": growing, "at_S": last["S"],
                            "r_at_S": last["r"], "points": pts}
    return res


def slope(points):
    xs = [p[0] for p in points]
    ys = [p[1] for p in points]
    if len(xs) < 2:
        return None
    mx, my = statistics.mean(xs), statistics.mean(ys)
    den = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / den if den else None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--runs", default="runs")
    ap.add_argument("--out", default="results.json")
    a = ap.parse_args()
    allrows = load(a.runs)
    rows = [r for r in allrows if r.get("schema") == "pkm-odd-tower-f4.row/v1"]
    brows = [r for r in allrows if r.get("schema") == "pkm-algebraic-oracle.row/v1"]
    out = {"schema": "pkm-odd-tower-f4.results/v1", "rows": len(rows), "part_b_rows": len(brows)}

    # Validity (C1, C2, A/A).
    done = [r for r in rows if not r["censored"]]
    out["C1_agree"] = {"rows": len(done), "disagree": [r["cell"] for r in done if not r["agree"]]}
    pl = [r for r in rows if r["kind"] == "planted"]
    out["C2_planted"] = {"rows": len(pl), "bad": [r["cell"] for r in pl
                                                  if not (r["planted_ok"] and not r["inconsistent"] and not r["censored"])]}
    aa = {r["draw"]: r["field_ops"] for r in rows if r["cell"] == "AA-M2-A3-k5-tower"}
    ref = {r["draw"]: r["field_ops"] for r in rows if r["cell"] == "M2-A3-k5-tower"}
    out["AA"] = {"identical": bool(aa) and aa == ref, "aa": aa, "ref": ref}
    valid = not out["C1_agree"]["disagree"] and not out["C2_planted"]["bad"] and out["AA"]["identical"]
    out["valid"] = valid

    # Decision rule.
    for m in (2, 3):
        for control in ("tower", "null"):
            cs = cells(rows, f"M{m}-", control)
            out[f"m{m}_{control}"] = verdicts(cs, f"m{m}_{control}")
            out[f"m{m}_{control}_slopes"] = {
                arm: {"vs_log2S": slope([(x, math.log2(statistics.median(w))) for x, _, w, c, _ in v if not c])}
                for arm, v in cs.items()}
    # Attribution: tower and null verdicts agree?
    out["attribution"] = {}
    for m in (2, 3):
        for arm in ARMS:
            t = out[f"m{m}_tower"].get("arms", {}).get(arm, {}).get("verdict")
            n = out[f"m{m}_null"].get("arms", {}).get(arm, {}).get("verdict")
            out["attribution"][f"m{m}_{arm}"] = {"tower": t, "null": n,
                                                 "reading": "shape" if t == n else "elliptic-specific"}
    # Form: isogeny (I*) over Kummer (A*) for the same degree sequence, per draw (C6).
    form = {}
    for m in (2,):
        for control in ("tower", "null"):
            cs = cells(rows, f"M{m}-", control)
            for iso, kum in (("I2", "A2"), ("I3", "A3"), ("I23", "A23")):
                ka = {s: (w, c) for _, s, w, c, _ in cs.get(kum, [])}
                pts = []
                for x, s, w, c, cell in cs.get(iso, []):
                    if s not in ka or c or ka[s][1]:
                        pts.append({"S": s, "censored": True})
                        continue
                    rs = [a / b for a, b in zip(w, ka[s][0])]
                    pts.append({"S": s, "r": rs, "r_median": statistics.median(rs), "bands": [band(v) for v in rs]})
                done_pts = [p for p in pts if "r" in p]
                if done_pts:
                    bs = set(done_pts[-1]["bands"])
                    v = bs.pop() if len(bs) == 1 else "mixed"
                    form[f"m{m}_{control}_{iso}_over_{kum}"] = {"verdict": v, "at_S": done_pts[-1]["S"], "points": pts}
                else:
                    form[f"m{m}_{control}_{iso}_over_{kum}"] = {"verdict": "no_completed_cell", "points": pts}
    out["form"] = form
    # Order: A32 vs A23 at equal S (n = 4, 6), paired by draw.
    order = {}
    for n in (4, 6):
        a = {r["draw"]: r for r in rows if r["cell"] == f"M2-A23-n{n}-tower"}
        b = {r["draw"]: r for r in rows if r["cell"] == f"M2-A32-n{n}-tower"}
        if a and b and not any(r["censored"] for r in list(a.values()) + list(b.values())):
            rs = [b[d]["field_ops"] / a[d]["field_ops"] for d in sorted(a) if d in b]
            order[f"n{n}"] = {"S": a[0]["S"], "r_A32_over_A23": rs, "bands": [band(x) for x in rs],
                              "same_targets": all(a[d]["target_x"] == b[d]["target_x"] for d in a if d in b)}
    out["order"] = order
    # Positive control: one bit of |V| (A2 at t+1 read as if it were t).
    a2 = {x: statistics.median(w) for x, _, w, c, _ in cells(rows, "M2-", "tower").get("A2", []) if not c}
    xs = sorted(a2)
    pc = [{"t": round(x0), "ratio": a2[x1] / a2[x0], "detected": a2[x1] / a2[x0] >= LOSS}
          for x0, x1 in zip(xs, xs[1:])]
    out["positive_control_one_bit"] = {"pairs": pc, "all_detected": bool(pc) and all(p["detected"] for p in pc)}
    # Replication at P2: A3 vs A2 band at the largest common completed S.
    xrep = verdicts(cells(rows, "X2-", "tower"), "x2")
    out["replication"] = xrep
    for arm in ("A3", "A5"):
        p1 = out["m2_tower"].get("arms", {}).get(arm, {})
        p2 = xrep.get("arms", {}).get(arm, {})
        out.setdefault("replication_check", {})[arm] = {
            "P1": p1.get("verdict"), "P1_at_S": p1.get("at_S"),
            "P2": p2.get("verdict"), "P2_at_S": p2.get("at_S"),
            "replicated": p1.get("verdict") == p2.get("verdict")}

    # Part B: algebraic oracle vs enumeration.
    decided = [r for r in brows if not r["alg_undetermined"]]
    out["C7_part_b_agree"] = {"rows": len(brows), "decided": len(decided),
                              "disagree": [(r["cell"], r["index"]) for r in decided if not r["agree"]]}
    pb = {}
    by = {}
    for r in brows:
        key = (r["m"], r["form"], r["cell"].split("-")[2])  # B-M2-A3-k3 -> arm A3
        by.setdefault(key, {}).setdefault(r["cell"], []).append(r)
    for (m, form, arm), cellmap in sorted(by.items()):
        pts = []
        for cell, rs in sorted(cellmap.items(), key=lambda kv: kv[1][0]["S"]):
            cens = any(r["alg_undetermined"] for r in rs)
            ratios = [r["ratio_alg_over_enum"] for r in rs]
            pts.append({"cell": cell, "S": rs[0]["S"], "censored": cens, "ratios": ratios,
                        "median": statistics.median(ratios),
                        "alg_field_ops": [r["alg_field_ops"] for r in rs],
                        "enum_field_ops": [r["enum_field_ops"] for r in rs],
                        "relations": [r["enum_relations"] for r in rs]})
        done_pts = [p for p in pts if not p["censored"]]
        if not done_pts:
            pb[f"m{m}_{form}_{arm}"] = {"verdict": "no_completed_cell", "points": pts}
            continue
        last = done_pts[-1]
        if all(x <= GAIN for x in last["ratios"]):
            v = "advantage"
        elif all(x >= LOSS for x in last["ratios"]):
            v = "disadvantage"
        else:
            v = "parity"
        trend = "flat"
        if len(done_pts) >= 3:
            t = [p["median"] for p in done_pts[-3:]]
            if all(b < a for a, b in zip(t, t[1:])):
                trend = "closing"
            elif all(b > a for a, b in zip(t, t[1:])):
                trend = "widening"
        pb[f"m{m}_{form}_{arm}"] = {"verdict": v, "trend": trend, "at_S": last["S"],
                                    "ratios_at_S": last["ratios"], "points": pts}
    out["part_b"] = pb
    out["part_b_valid"] = not out["C7_part_b_agree"]["disagree"]

    with open(a.out, "w") as f:
        json.dump(out, f, indent=1)
    print(json.dumps({k: out[k] for k in ("rows", "valid", "C1_agree", "C2_planted", "AA")}, indent=1))
    for m in (2, 3):
        for arm in ARMS:
            v = out[f"m{m}_tower"].get("arms", {}).get(arm, {})
            print(f"m={m} {arm}: tower {v.get('verdict')} growing={v.get('growing')} at S={v.get('at_S')} "
                  f"r={[round(x, 3) for x in v.get('r_at_S', [])]}; null {out['attribution'][f'm{m}_{arm}']['null']}")
    print("order", json.dumps(order))
    print("part B valid", out["part_b_valid"], out["C7_part_b_agree"])
    for k, v in pb.items():
        print("partB", k, v["verdict"], v.get("trend"), "at S", v.get("at_S"),
              [f"{x:.3g}" for x in v.get("ratios_at_S", [])])
    for k, v in form.items():
        print("form", k, v["verdict"], "at S", v.get("at_S"))
    print("positive control (one bit)", out["positive_control_one_bit"]["all_detected"])
    print("replication", json.dumps(out.get("replication_check")))


if __name__ == "__main__":
    main()
