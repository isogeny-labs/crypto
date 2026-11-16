"""R7.5 recount for the R6.7 erratum.  Reads the frozen m2_torsion.json (and the
m3 records for the other table rows) and counts, per pricing, how many per-curve
bootstrap 95% CIs contain the law value.  Computes nothing new: it only compares
recorded 'point' and 'ci95' fields with the law, and writes recount_v2.json."""
import json, sys
PR = ["unpriced", "priced_as_run", "priced_essential"]

def rows(n): return json.load(open(n))["rows"]

def count(rs, key, law):
    out = {}
    for pr in PR:
        inside = []
        for r in rs:
            b = r[key][pr]; lo, hi = b["ci95"]
            inside.append(lo <= law <= hi)
        out[pr] = inside
    return out

def label(r):
    return f"bits{r['bits']}/seed{r['seed']}/p={r['instance']['p']}"

res = {}
m2t = rows("m2_torsion.json")
c = count(m2t, "ratio_c_over_b_size_matched", 1.0)
print("(c)/(b) size-matched, m=2, torsion, law 1, n curves =", len(m2t))
print(f"{'curve':28s} {'point(unp)':>10s} {'unpriced CI':>20s} {'asrun CI':>20s} {'essential CI':>20s}")
for i, r in enumerate(m2t):
    x = r["ratio_c_over_b_size_matched"]
    def f(pr):
        b = x[pr]; return f"[{b['ci95'][0]:.3f},{b['ci95'][1]:.3f}]" + ("*" if c[pr][i] else " ")
    print(f"{label(r):28s} {x['unpriced']['point']:10.3f} {f('unpriced'):>20s} {f('priced_as_run'):>20s} {f('priced_essential'):>20s}")
print("(* = CI contains 1)")
for pr in PR:
    print(f"  {pr:17s}: {sum(c[pr])} of {len(m2t)} CIs contain 1")
pts = {pr: [r["ratio_c_over_b_size_matched"][pr]["point"] for r in m2t] for pr in PR}
for pr in PR:
    print(f"  {pr:17s}: point range {min(pts[pr]):.3f} to {max(pts[pr]):.3f}")
print("  max |priced as run - unpriced| (points):", round(max(abs(a-b) for a, b in zip(pts['priced_as_run'], pts['unpriced'])), 4))
print("  max |priced essential - unpriced| (points):", round(max(abs(a-b) for a, b in zip(pts['priced_essential'], pts['unpriced'])), 4))
print("  curves whose unpriced CI excludes 1:", [label(r) for r, ok in zip(m2t, c['unpriced']) if not ok])
print("  curves whose as-run CI excludes 1:", [label(r) for r, ok in zip(m2t, c['priced_as_run']) if not ok])
print("  curves whose essential CI excludes 1:", [label(r) for r, ok in zip(m2t, c['priced_essential']) if not ok])
res["m2_cb"] = {"n": len(m2t), "contains_1": {pr: sum(c[pr]) for pr in PR},
                "per_curve": [{"curve": label(r), "point": {pr: r["ratio_c_over_b_size_matched"][pr]["point"] for pr in PR},
                               "ci95": {pr: r["ratio_c_over_b_size_matched"][pr]["ci95"] for pr in PR},
                               "contains_1": {pr: c[pr][i] for pr in PR}} for i, r in enumerate(m2t)]}

# cross-check the other rows of the table (law containment) so the erratum states what is NOT affected
m2p = rows("m2_prime.json")
m3 = rows("m3_9_10.json") + rows("m3_11.json")
print("\ncross-check of the other table rows")
for name, rs, key, law in [("B/A m=3", m3, "ratio_B_over_A_size_matched", 3.0), ("C/A m=3", m3, "ratio_C_over_A_size_matched", 1.0),
                           ("B/C m=3", m3, "ratio_B_over_C", 3.0), ("(b)/(a) m=2", m2t + m2p, "ratio_b_over_a", 1/3)]:
    cc = count(rs, key, law)
    print(f"  {name:14s} n={len(rs):2d} " + "  ".join(f"{pr}: {sum(cc[pr])}" for pr in PR))
    res[name] = {"n": len(rs), **{pr: sum(cc[pr]) for pr in PR}}
json.dump(res, open("recount_v2.json", "w"), indent=1)

# additional checks of two neighbouring statements (RESULTS.md and report.typ), same records
print("\nadditional checks (statements next to the miscount)")
sh2 = [r["arm_ab"]["inversion_share_of_priced_as_run"] for r in m2t + m2p] + [r["arm_c"]["inversion_share_of_priced_as_run"] for r in m2t]
sh3 = [r[a]["inversion_share_of_priced_as_run"] for r in m3 for a in ("arm_A", "arm_B", "arm_C")]
print(f"  inversion share of priced-as-run cost: m=3 {min(sh3):.3f} to {max(sh3):.3f} (27 arm-curves); m=2 {min(sh2):.3f} to {max(sh2):.3f} (arms (a)/(b) on 30 curves, arm (c) on 18 torsion curves)")
mx3 = {}
for k in ["ratio_B_over_A", "ratio_B_over_A_size_matched", "ratio_B_over_C", "ratio_C_over_A", "ratio_C_over_A_size_matched"]:
    mx3[k] = max(abs(r[k][pr]["point"] - r[k]["unpriced"]["point"]) for r in m3 for pr in ("priced_as_run", "priced_essential"))
    print(f"  m=3 max |priced point - unpriced point|, {k}: {mx3[k]:.4f}")
print(f"  m=3 max over all five ratios: {max(mx3.values()):.4f}  (RESULTS.md / report.typ say 'no more than 0.011')")
mx2 = max(abs(r["ratio_c_over_b_size_matched"][pr]["point"] - r["ratio_c_over_b_size_matched"]["unpriced"]["point"]) for r in m2t for pr in ("priced_as_run", "priced_essential"))
print(f"  m=2 max |priced - unpriced| for (c)/(b): {mx2:.4f}  (statement: <= 0.024)")
res["extra"] = {"share_m3": [min(sh3), max(sh3)], "share_m2": [min(sh2), max(sh2)], "m3_max_pricing_shift": mx3, "m2_cb_max_pricing_shift": mx2}
json.dump(res, open("recount_v2.json", "w"), indent=1)
