"""J3 (TASK-20261029-654f73): charged-work identity, own code.

Condition 8 record half: every dgbf and dgbf-edge record, warm-ups included,
against the dgb record of the same (curve, target, round): recovered log,
total_gae, every band-priced unpriced count, per phase (by name) adds, doubles,
scalar_mults, gae, native, and work_native except table_filter_bit_sets; the
three table_filter_bit_sets requirements. Bitop half: every record's bitop/v1
work is re-priced from its primitive counts with the validator's own reading
of docs/units/BITOPS.md (3271a25d02), and dgbf - dgb = 384 x
table_filter_bit_sets, phase by phase. Condition 10 for every IC record of
both sessions. Counted dgb/dgbf records equal the wall records of the same arm,
curve, target and round in the condition 8 fields (table_filter_bit_sets
included). Counted sessions are read through vcommon.load_counted.

usage: j3_work.py OUT.json
"""
import collections
import sys

import vcommon as v

PHASE_FIELDS = ("adds", "doubles", "scalar_mults", "gae")


def prices(b, w):
    fa = 13 * b
    fm = 2 * (b * b + 5 * b * (b - 1)) + 13 * b
    fi = 2 * b * 13 * b
    I, M, S, A = fi, fm, fm, fa
    return {
        "field_add": fa, "field_mul": fm, "field_sqr": fm, "field_inv": fi,
        "field_sqrt": 2 * b * fm, "legendre": fi,
        "group_add": I + 2 * M + S + 6 * A, "group_double": I + 2 * M + 2 * S + 8 * A,
        "modr_add": 13 * w, "modr_mul": 2 * (w * w + 5 * w * (w - 1)) + 13 * w,
        "modr_inv": 2 * w * 13 * w, "word_op": 64, "hash_probe": 4 * b,
    }


def reprice(r):
    wk = r["work"]
    b = wk["widths"]["field"]["bits"]
    w = wk["widths"]["r_bits"]
    if wk["widths"]["field"]["kind"] != "prime":
        raise SystemExit("STOP: non-prime field")
    P = prices(b, w)
    bad, tot = [], 0
    for ph in wk["phases"]:
        s = 0
        for prim, n in ph["prims"].items():
            if prim not in P:
                raise SystemExit(f"STOP: unpriced primitive {prim} in {r['record_id']}")
            s += P[prim] * n
        if s != int(ph["bitops"]):
            bad.append((ph["name"], s, ph["bitops"]))
        tot += s
    if tot != int(wk["bitops"]):
        bad.append(("total", tot, wk["bitops"]))
    return bad


def unpriced_counts(r):
    out = {}
    counters = r.get("counters") or {}
    for name in r["cost"]["unpriced"]:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            out[name] = counters[name]
            continue
        t, found = 0, False
        for p in r["phases"]:
            nat = p.get("native") or {}
            if name in nat:
                t += nat[name]; found = True
            elif base in nat:
                t += nat[base]; found = True
        out[name] = t if found else None
    return out


def cond8_fields(r, drop_tfbs):
    ph = {}
    for p in r["phases"]:
        wn = dict(p.get("work_native") or {})
        if drop_tfbs:
            wn.pop("table_filter_bit_sets", None)
        ph[p["name"]] = {**{f: p[f] for f in PHASE_FIELDS}, "native": p.get("native") or {}, "work_native": wn}
    return {"recovered": r["outcome"]["recovered"], "total_gae": r["cost"]["total_gae"],
            "unpriced": unpriced_counts(r), "phases": ph}


def diff(a, b, path=""):
    out = []
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append(f"{path}/{k}: missing on {'A' if k not in a else 'B'}")
            else:
                out += diff(a[k], b[k], f"{path}/{k}")
    elif a != b:
        out.append(f"{path}: {a!r} != {b!r}")
    return out


def all_by_key(records):
    d = collections.defaultdict(dict)
    for r in records:
        d[r["arm"]][v.key(r)] = r
    return d


def cond8(b, recs):
    arms = all_by_key(recs)
    res = {"matched": {}, "mismatches": [], "tfbs": {}, "bitops": {}}
    for arm in ("dgbf", "dgbf-edge", "dgb-aa"):
        n = 0
        for k, r in arms[arm].items():
            base = arms["dgb"][k]
            d = diff(cond8_fields(base, True), cond8_fields(r, True))
            n += 1
            if d:
                res["mismatches"].append({"arm": arm, "key": list(k), "record_id": r["record_id"],
                                          "dgb_record_id": base["record_id"], "diff": d[:5]})
        res["matched"][arm] = n
    # tfbs requirements
    absent = all(v.tfbs(r)[1] == [] for a in ("dgb", "dgb-aa") for r in arms[a].values())
    present = all(v.tfbs(r)[1] == ["oracle_setup"] for a in ("dgbf", "dgbf-edge") for r in arms[a].values())
    per_curve = collections.defaultdict(set)
    for a in ("dgbf", "dgbf-edge"):
        for k, r in arms[a].items():
            per_curve[k[0]].add(v.tfbs(r)[0])
    res["tfbs"] = {"absent_on_dgb_and_dgb_aa": absent, "present_once_in_oracle_setup_on_filtered": present,
                   "values_per_curve": {c: sorted(s) for c, s in per_curve.items()},
                   "one_value_per_curve": all(len(s) == 1 for s in per_curve.values())}
    # bitop identity, phase by phase
    bad_ident, bad_price, n = [], [], 0
    for r in recs:
        bp = reprice(r)
        if bp:
            bad_price.append({"record_id": r["record_id"], "bad": bp})
    for arm in ("dgbf", "dgbf-edge"):
        for k, r in arms[arm].items():
            base = arms["dgb"][k]
            t, _ = v.tfbs(r)
            n += 1
            dt = int(r["work"]["bitops"]) - int(base["work"]["bitops"])
            ph_r = {p["name"]: p for p in r["work"]["phases"]}
            ph_b = {p["name"]: p for p in base["work"]["phases"]}
            ok = dt == 384 * t and set(ph_r) == set(ph_b)
            for name in ph_r:
                d_ph = int(ph_r[name]["bitops"]) - int(ph_b[name]["bitops"])
                want = 384 * t if name == "oracle_setup" else 0
                ok &= d_ph == want
                dp = {p: ph_r[name]["prims"].get(p, 0) - ph_b[name]["prims"].get(p, 0)
                      for p in set(ph_r[name]["prims"]) | set(ph_b[name]["prims"])}
                dp = {p: x for p, x in dp.items() if x}
                ok &= dp == ({"word_op": 6 * t} if name == "oracle_setup" else {})
            if not ok:
                bad_ident.append({"arm": arm, "key": list(k), "record_id": r["record_id"], "delta": dt, "tfbs": t})
    meas = lambda a: sum(int(r["work"]["bitops"]) for r in arms[a].values() if not r["warmup"])
    res["bitops"] = {"records_repriced": len(recs), "reprice_mismatches": bad_price,
                     "identity_checked": n, "identity_failures": bad_ident,
                     "measured_sum_dgb": meas("dgb"), "measured_sum_dgbf": meas("dgbf"),
                     "measured_ratio_dgbf_over_dgb": meas("dgbf") / meas("dgb"),
                     "measured_difference": meas("dgbf") - meas("dgb")}
    return res


def cond10(recs):
    bad = []
    pmax, spmax, src = 0, 0, set()
    for r in recs:
        if r["method"]["id"] != "ic.pipeline":
            continue
        p = int(r["workload"]["curve"]["icv1"].split(":")[1].split("-")[1])
        fb = r["factor_base"]
        if "signed_points" in fb:
            sp = fb["signed_points"]; src.add("signed_points")
        else:
            sp = 2 * fb["abscissae"]; src.add("2*abscissae")
        pmax, spmax = max(pmax, p), max(spmax, sp)
        if p > 4294967295 or sp > 65536:
            bad.append(r["record_id"])
    return {"ic_records": sum(1 for r in recs if r["method"]["id"] == "ic.pipeline"),
            "p_max": pmax, "signed_points_max": spmax, "source": sorted(src), "failures": bad}


def counted_equals_wall(b, wall, counted):
    W, C = all_by_key(wall), all_by_key(counted)
    res = {}
    for arm in ("dgb", "dgbf"):
        mism, n = [], 0
        if set(W[arm]) != set(C[arm]):
            mism.append("key sets differ")
        for k in sorted(C[arm]):
            n += 1
            d = diff(cond8_fields(W[arm][k], False), cond8_fields(C[arm][k], False))
            if int(W[arm][k]["work"]["bitops"]) != int(C[arm][k]["work"]["bitops"]):
                d.append("work.bitops differ")
            if d:
                mism.append({"key": list(k), "wall_record": W[arm][k]["record_id"],
                             "counted_record": C[arm][k]["record_id"], "diff": d[:5]})
        res[arm] = {"matched": n, "mismatches": mism}
    res["equal"] = all(not res[a]["mismatches"] for a in ("dgb", "dgbf"))
    return res


def main():
    out = {}
    for b in (32, 24, 28):
        counted = v.load_counted(b)
        if b == 28:
            wall = v.load_wall_counts_only(28)  # descriptive session: counts only
        else:
            wall = v.load_wall(b)
            out[f"P{b}-wall-cond8"] = cond8(b, wall)
            out[f"P{b}-wall-cond10"] = cond10(wall)
        out[f"P{b}-counted-cond10"] = cond10(counted)
        out[f"P{b}-counted-equals-wall"] = counted_equals_wall(b, wall, counted)
        bp = [r["record_id"] for r in counted if reprice(r)]
        out[f"P{b}-counted-reprice-mismatches"] = bp
        cw = all_by_key(counted)
        out[f"P{b}-counted-bitop-identity-failures"] = [
            r["record_id"] for k, r in cw["dgbf"].items()
            if int(r["work"]["bitops"]) - int(cw["dgb"][k]["work"]["bitops"]) != 384 * v.tfbs(r)[0]]
    v.dump(out, sys.argv[1])
    for k, r in out.items():
        if k.endswith("cond8"):
            print(k, "matched", r["matched"], "mismatches", len(r["mismatches"]), r["tfbs"],
                  "reprice bad", len(r["bitops"]["reprice_mismatches"]), "identity", r["bitops"]["identity_checked"],
                  "fail", len(r["bitops"]["identity_failures"]), "ratio %.7f diff %d" % (r["bitops"]["measured_ratio_dgbf_over_dgb"], r["bitops"]["measured_difference"]))
        elif k.endswith("equals-wall"):
            print(k, {a: (r[a]["matched"], len(r[a]["mismatches"])) for a in ("dgb", "dgbf")}, "equal", r["equal"])
        else:
            print(k, r if not isinstance(r, dict) else {kk: vv for kk, vv in r.items()})


if __name__ == "__main__":
    main()
