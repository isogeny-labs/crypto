"""The filter's bit sets in the section 5a band (XP-20261029-packed-filter PROTOCOL.md section 5, APPROVAL.md E4).
For each record: S'_w = (priced_gae_w + w * table_filter_bit_sets) / sqrt(r), where priced_gae_w = total_gae - C + w * (U + n_C) is band.rs's
priced GAE at weight w (src/cryptanalysis/ecbench/band.rs at 3271a25d02), with U the sum of the record's unpriced-counter counts, C and n_C the GAE and the
operation count a pinned calibration put on native work, and table_filter_bit_sets the oracle_setup phase's work_native counter (0 on an arm without the filter).
Ratios sum(S'_w(B)) / sum(S'_w(A)) over the matched pairs (same curve, target and round; both measured, not warm-up, both verified; priced at w for both):
  wall session: dgb -> dgbf (A = dgb, B = dgbf)
  counted session: rho-neg -> dgb, rho-neg -> dgbf, and dgb -> dgbf
At each weight the same ratio without the bit-set term is printed beside the 'ecbench compare --weight w' band ratio of the same pair (ops.band.ratio_b_over_a),
as a fact (the tool does not judge it). A record with a pinned calibration (detail.calibration_pinned_units non-empty) would need the calibration table the
tool does not read: such records are counted and printed, and S_PRIME_EXACT is false when there is any.
It reads no wall figure. It decides nothing and exits 0 whatever the values are.

usage: sprime.py WALL_EKS COUNTED_EKS WALL_AN COUNTED_AN
Writes WALL_AN/sprime.json and COUNTED_AN/sprime.json. Exits 1 with a line beginning STOP: only when a file is absent or unreadable."""
import collections, json, math, os, sys

eks_w, eks_c, an_w, an_c = sys.argv[1:5]
WEIGHTS = [0.0, 0.1, 1.0]
WNAME = ["0", "0.1", "1"]
TFBS = "table_filter_bit_sets"


def stop(msg):
    print("STOP: " + msg)
    sys.exit(1)


def tf(x):
    return "true" if x else "false"


def key(r):
    w = r.get("workload") or {}
    return ((w.get("curve") or {}).get("slug"), w.get("target_index"), r.get("round"))


def unpriced_counts(r):
    out = {}
    counters = r.get("counters") or {}
    phases = r.get("phases") or []
    for name in (r.get("cost") or {}).get("unpriced") or []:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            out[name] = counters[name]
            continue
        total = 0
        found = False
        for p in phases:
            nat = p.get("native") or {}
            if name in nat:
                total += nat[name]
                found = True
            elif base in nat:
                total += nat[base]
                found = True
        if (not found) and base.startswith("solver_") and r.get("solver") is not None:
            out[name] = (r.get("solver") or {}).get("ops")
            continue
        out[name] = total if found else None
    return out


def priced_gae(r, w):
    """band.rs priced_gae with C = 0 and n_C = 0 (no pinned calibration); None for no recorded cost or an uncounted counter at w > 0."""
    g = (r.get("cost") or {}).get("total_gae")
    if g is None:
        return None
    counts = unpriced_counts(r)
    missing = any(v is None for v in counts.values())
    if missing and w > 0.0:
        return None
    u = sum(v for v in counts.values() if v is not None)
    return g + w * u


def tfbs(r):
    for p in r.get("phases") or []:
        if p.get("name") == "oracle_setup":
            return int((p.get("work_native") or {}).get(TFBS) or 0)
    return 0


def load(eks):
    p = os.path.join(eks, "session", "records.jsonl")
    try:
        return [json.loads(l) for l in open(p) if l.strip()]
    except Exception as e:
        stop("cannot read %s: %s" % (p, type(e).__name__))


def band_ratio(an, a, b):
    p = os.path.join(an, "compare-%s__%s-w0.json" % (a, b))
    try:
        d = json.load(open(p))
        return ((d.get("ops") or {}).get("band") or {}).get("ratio_b_over_a")
    except Exception:
        return None


def pair_ratios(recs, a, b, an):
    ra, rb = {}, {}
    for r in recs:
        if r.get("warmup") or (r.get("outcome") or {}).get("status") != "verified":
            continue
        if r.get("arm") == a:
            ra[key(r)] = r
        elif r.get("arm") == b:
            rb[key(r)] = r
    res = {"a": a, "b": b, "records_a": len(ra), "records_b": len(rb), "by_weight": {}}
    cb = band_ratio(an, a, b)
    for i, w in enumerate(WEIGHTS):
        sa = sb = sa0 = sb0 = 0.0
        n = 0
        percurve = collections.defaultdict(lambda: [0.0, 0.0, 0])
        for k in sorted(set(ra) & set(rb), key=lambda k: (str(k[0]), str(k[1]), str(k[2]))):
            pa, pb = priced_gae(ra[k], w), priced_gae(rb[k], w)
            if pa is None or pb is None:
                continue
            rr = ((ra[k].get("workload") or {}).get("curve") or {}).get("r")
            if not isinstance(rr, int) or rr <= 0:
                continue
            sq = math.sqrt(float(rr))
            xa = (pa + w * tfbs(ra[k])) / sq
            xb = (pb + w * tfbs(rb[k])) / sq
            sa += xa
            sb += xb
            sa0 += pa / sq
            sb0 += pb / sq
            n += 1
            c = percurve[k[0]]
            c[0] += xa
            c[1] += xb
            c[2] += 1
        row = {
            "pairs": n,
            "sum_S_prime_A_repr": repr(sa), "sum_S_prime_B_repr": repr(sb),
            "ratio_S_prime_B_over_A_repr": repr(sb / sa) if sa else None,
            "ratio_without_bit_set_term_repr": repr(sb0 / sa0) if sa0 else None,
            "compare_band_ratio_b_over_a_repr": repr(cb[i]) if isinstance(cb, list) and len(cb) == 3 else None,
            "per_curve_ratio_S_prime_B_over_A_repr": {str(c): [repr(v[1] / v[0]) if v[0] else None, v[2]] for c, v in sorted(percurve.items(), key=lambda kv: str(kv[0]))},
        }
        if isinstance(cb, list) and len(cb) == 3 and sa0:
            x, y = sb0 / sa0, cb[i]
            row["without_bit_set_term_equals_compare_band_rel_1e-9"] = abs(x - y) <= 1e-9 * max(abs(x), abs(y), 1e-300)
        else:
            row["without_bit_set_term_equals_compare_band_rel_1e-9"] = None
        res["by_weight"][WNAME[i]] = row
    return res


def session(eks, an, label, pairs):
    recs = load(eks)
    pinned = sum(1 for r in recs if ((r.get("detail") or {}).get("calibration_pinned_units") or []))
    missing_counts = sum(1 for r in recs if any(v is None for v in unpriced_counts(r).values()))
    out = {"label": label, "records": len(recs), "records_with_pinned_calibration": pinned,
           "records_with_an_unpriced_counter_without_a_count": missing_counts, "pairs": []}
    for a, b in pairs:
        out["pairs"].append(pair_ratios(recs, a, b, an))
    out["S_PRIME_EXACT"] = (pinned == 0)
    json.dump(out, open(os.path.join(an, "sprime.json"), "w"), indent=1, sort_keys=True)
    return out


for pth in (os.path.join(eks_w, "session", "records.jsonl"), os.path.join(eks_c, "session", "records.jsonl")):
    if not os.path.isfile(pth):
        stop("absent: " + pth)
w = session(eks_w, an_w, "wall", [("dgb", "dgbf")])
c = session(eks_c, an_c, "counted", [("rho-neg", "dgb"), ("rho-neg", "dgbf"), ("dgb", "dgbf")])
print("SPRIME_FACTS (mechanical facts; the executor does not interpret them)")
for label, o in (("WALL", w), ("COUNTED", c)):
    print("SPRIME_" + label + "_HEADER " + json.dumps({k: v for k, v in o.items() if k != "pairs"}, sort_keys=True))
    for pr in o["pairs"]:
        for wn in WNAME:
            row = pr["by_weight"][wn]
            print("SPRIME_%s %s->%s w=%s %s" % (label, pr["a"], pr["b"], wn, json.dumps({k: v for k, v in row.items() if k != "per_curve_ratio_S_prime_B_over_A_repr"}, sort_keys=True)))
            print("SPRIME_%s_PER_CURVE %s->%s w=%s %s" % (label, pr["a"], pr["b"], wn, json.dumps(row["per_curve_ratio_S_prime_B_over_A_repr"], sort_keys=True)))
print("S_PRIME_EXACT_WALL=" + tf(w["S_PRIME_EXACT"]))
print("S_PRIME_EXACT_COUNTED=" + tf(c["S_PRIME_EXACT"]))
print("SPRIME_OK")
