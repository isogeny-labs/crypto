#!/usr/bin/env python3
"""Collect raw_part_a.jsonl + raw_part_b.json + validation.json into results.json and print tables."""
import json, os, math, collections

H = os.path.dirname(os.path.abspath(__file__))
A = [json.loads(x) for x in open(f"{H}/raw_part_a.jsonl")]
B = json.load(open(f"{H}/raw_part_b.json"))
V = json.load(open(f"{H}/validation.json"))


def slope(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def med(v):
    v = sorted(v)
    n = len(v)
    return None if n == 0 else (v[n // 2] if n % 2 else (v[n // 2 - 1] + v[n // 2]) / 2)


summary = {"part_a": {}, "part_b": {}}
t = collections.defaultdict(list)
for r in A:
    if r["kind"] == "time":
        t[(r["l"], r["class"], r["arm"])].append(r)
ls = sorted({k[0] for k in t})
arms = ["enum_scalar", "enum_bitsliced", "scan_scalar", "scan_bitsliced"]
print("PART A  (ns per target = min of 3 reps; units per target)")
for cls in ("planted", "random"):
    print(f"\nclass={cls}")
    print("| l | " + " | ".join(f"{a} ns | {a} units" for a in arms) + " | scan_bs/enum_bs (ns) | scan_bs/scan_scalar (ns) |")
    print("|" + "---|" * (3 + 2 * len(arms)))
    cell = {}
    for l in ls:
        row = {}
        for a in arms:
            rr = t[(l, cls, a)]
            row[a] = (min(x["ns_per_target"] for x in rr), rr[0]["units_per_target"], max(x["ns_per_target"] for x in rr))
        cell[l] = row
        print(f"| {l} | " + " | ".join(f"{row[a][0]:.0f} | {row[a][1]:.0f}" for a in arms)
              + f" | {row['scan_bitsliced'][0] / row['enum_bitsliced'][0]:.3f} | {row['scan_bitsliced'][0] / row['scan_scalar'][0]:.3f} |")
    sl = {}
    for a in arms:
        sl[a] = dict(units_bits_per_l=slope(ls, [math.log2(cell[l][a][1]) for l in ls]),
                     ns_bits_per_l=slope(ls, [math.log2(cell[l][a][0]) for l in ls]),
                     ns_bits_per_l_top3=slope(ls[-3:], [math.log2(cell[l][a][0]) for l in ls[-3:]]))
        print(f"slope {a}: units {sl[a]['units_bits_per_l']:.2f} bits/l, ns {sl[a]['ns_bits_per_l']:.2f} (top-3 sizes {sl[a]['ns_bits_per_l_top3']:.2f})")
    summary["part_a"][cls] = dict(
        cells={l: {a: dict(ns_per_target_min=cell[l][a][0], ns_per_target_max=cell[l][a][2], units_per_target=cell[l][a][1]) for a in arms} for l in ls},
        slopes=sl,
        ratio_scan_bs_over_enum_bs={l: cell[l]["scan_bitsliced"][0] / cell[l]["enum_bitsliced"][0] for l in ls},
        ratio_scan_bs_over_scan_scalar={l: cell[l]["scan_bitsliced"][0] / cell[l]["scan_scalar"][0] for l in ls},
        ratio_enum_bs_over_enum_scalar={l: cell[l]["enum_bitsliced"][0] / cell[l]["enum_scalar"][0] for l in ls})
print("\njoin (T=512, all targets):")
jn = {}
for l in ls:
    rr = t[(l, "all512", "join")]
    jn[l] = dict(ns_per_target_min=min(x["ns_per_target"] for x in rr), quadratic_solves_per_target=rr[0]["units_per_target"])
    print(l, jn[l])
summary["part_a"]["join_T512"] = jn
summary["part_a"]["checks"] = [r for r in A if r["kind"] == "check"]

print("\nPART B")
cd = [r for r in B if r["kind"] == "cdcl"]
pairs = [r for r in B if r["kind"] == "pair"]
inc = [r for r in B if r["kind"] == "incremental"]
lsb = sorted({r["l"] for r in cd})
print("| l | class | targets | E1 cadical153 conflicts (median) | E1 glucose4 | E1 minisat22 | E2 cadical153 fresh | E2 glucose4 fresh | traced decisions E1 | shared-prop fraction E1 |")
print("|---|---|---|---|---|---|---|---|---|---|")
pb = {}
for l in lsb:
    for cls in ("planted", "random"):
        rr = [r for r in cd if r["l"] == l and r["cls"] == cls]
        if not rr:
            continue
        d = dict(targets=len(rr),
                 E1_cadical153_conflicts_median=med([r["E1_cadical153"]["conflicts"] for r in rr]),
                 E1_glucose4_conflicts_median=med([r["E1_glucose4"]["conflicts"] for r in rr]),
                 E1_minisat22_conflicts_median=med([r["E1_minisat22"]["conflicts"] for r in rr]),
                 E2_cadical153_fresh_conflicts_median=med([r["E2_cadical153_fresh"]["conflicts"] for r in rr]),
                 E2_glucose4_fresh_conflicts_median=med([r["E2_glucose4_fresh"]["conflicts"] for r in rr]),
                 E1_traced_decisions_median=med([r["E1_trace"]["decisions"] for r in rr]),
                 E1_shared_prop_fraction_pooled=sum(r["E1_implied_shared_only"] for r in rr) / max(1, sum(r["E1_implied_full"] for r in rr)))
        pb[f"{l}/{cls}"] = d
        print(f"| {l} | {cls} | {d['targets']} | {d['E1_cadical153_conflicts_median']} | {d['E1_glucose4_conflicts_median']} | {d['E1_minisat22_conflicts_median']} | {d['E2_cadical153_fresh_conflicts_median']} | {d['E2_glucose4_fresh_conflicts_median']} | {d['E1_traced_decisions_median']} | {d['E1_shared_prop_fraction_pooled']:.3f} |")
summary["part_b"]["cells"] = pb
print("\n(b1) common decision prefix, all pairs in a cell")
print("| l | enc | pairs | median lcp (all vars) | max lcp | median fraction (all vars) | max fraction | median fraction (a,b vars only) |")
print("|---|---|---|---|---|---|---|---|")
b1 = {}
for l in lsb:
    for enc in ("E1", "E2"):
        pp = [r for r in pairs if r["l"] == l and r["enc"] == enc]
        if not pp:
            continue
        d = dict(pairs=len(pp), lcp_all_median=med([r["lcp_all"] for r in pp]), lcp_all_max=max(r["lcp_all"] for r in pp),
                 frac_all_median=med([r["frac_all"] for r in pp]), frac_all_max=max(r["frac_all"] for r in pp),
                 frac_ab_median=med([r["frac_ab"] for r in pp]),
                 frac_all_median_random_random=med([r["frac_all"] for r in pp if r["classes"] == "random-random"]),
                 frac_all_median_planted_planted=med([r["frac_all"] for r in pp if r["classes"] == "planted-planted"]))
        b1[f"{l}/{enc}"] = d
        print(f"| {l} | {enc} | {d['pairs']} | {d['lcp_all_median']} | {d['lcp_all_max']} | {d['frac_all_median']:.4f} | {d['frac_all_max']:.4f} | {d['frac_ab_median']:.4f} |")
summary["part_b"]["b1_prefix"] = b1
print("\n(b3) E2 incremental (state kept across the 16 targets) vs fresh, conflicts per target")
print("| l | solver | class | fresh median | incremental median | sum incremental / sum fresh |")
print("|---|---|---|---|---|---|")
b3 = {}
for l in lsb:
    for sv in ("cadical153", "glucose4"):
        for cls in ("planted", "random"):
            ii = [r for r in inc if r["l"] == l and r["solver"] == sv and r["cls"] == cls]
            ff = [r for r in cd if r["l"] == l and r["cls"] == cls]
            if not ii:
                continue
            fr = [r[f"E2_{sv}_fresh"]["conflicts"] for r in ff]
            ic = [r["conflicts"] for r in ii]
            d = dict(fresh_median=med(fr), incremental_median=med(ic), sum_ratio=sum(ic) / max(1, sum(fr)))
            b3[f"{l}/{sv}/{cls}"] = d
            print(f"| {l} | {sv} | {cls} | {d['fresh_median']} | {d['incremental_median']} | {d['sum_ratio']:.3f} |")
summary["part_b"]["b3_incremental"] = b3
# slopes for CDCL conflicts (random class)
for key in ("E1_cadical153_conflicts_median", "E2_cadical153_fresh_conflicts_median", "E1_minisat22_conflicts_median"):
    xs = [l for l in lsb if f"{l}/random" in pb and pb[f"{l}/random"][key] > 0]
    if len(xs) >= 3:
        s = slope(xs, [math.log2(pb[f"{l}/random"][key]) for l in xs])
        summary["part_b"][f"slope_random_{key}"] = s
        print(f"slope random {key}: {s:.2f} bits/l over l={xs}")
json.dump(dict(tag="[pcc:unverified] pilot, not claim data", summary=summary, validation=V, raw_part_a=A, raw_part_b=B),
          open(f"{H}/results.json", "w"), indent=1)
