"""Format recorded run1 values as Markdown and Typst tables. Layout only."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
A = HERE.parent.parent / "run1" / "analysis"
FAM = ["n15l5", "n17l6"]
GROUPS = ["F-x", "F-xx", "F-tel64", "F-tel512"]
ARMS = ["a0_default", "a1_x_all", "a2_x1x2", "a3_e_vars", "a4_learned_activity"]
summ = {f: json.loads((A / f"summary_{f}.json").read_text()) for f in FAM}
l3 = {f: json.loads((A / f"l3_{f}.json").read_text()) for f in FAM}
tr = json.loads((A / "transfer.json").read_text())["size_transfer_n15_to_n17"]


def ci(v, c, p=3):
    return f"{v:.{p}f} [{c[0]:.{p}f}, {c[1]:.{p}f}]"


l1 = [["family", "split", "features", "dims", "test AUC [95% CI]", "permuted-label AUC", "test gain [95% CI]", "train gain", "exact-label ceiling"]]
l2 = [["family", "split", "train-optimal cutoff (conflicts)", "train gain", "test gain [95% CI]", "positive-control AUC", "positive-control gain"]]
for f in FAM:
    for s in ["even_odd", "aa_mod4"]:
        d = summ[f]["L1_L2"][s]
        for g in GROUPS:
            m = d[g]["model"]
            l1.append([f, s, g, str(d[g]["dims"]), ci(m["test_auc"], m["test_auc_ci95"]), f"{d[g]['negative_control_auc']:.3f}",
                       ci(m["test_gain"], m["test_gain_ci95"], 2), f"{m['train_gain']:.2f}", f"{d[g]['ceiling_gain_exact_labels']:.2f}"])
        c, pc = d["L2_cutoff"], d["positive_control"]
        l2.append([f, s, str(c["cutoff"]), f"{c['train_gain']:.2f}", ci(c["test_gain"], c["test_gain_ci95"], 2),
                   ci(pc["test_auc"], pc["test_auc_ci95"]), f"{pc['test_gain']:.2f}"])
l3t = [["family", "arm", "targets", "capped", "median conflicts", "sum / a0", "conflicts per sorted triple", "conflicts per pair"]]
for f in FAM:
    for a in ARMS:
        r = l3[f][a]
        lb = "≥ " if r["capped"] else ""
        l3t.append([f, a, str(r["targets"]), str(r["capped"]), f"{lb}{r['median_conflicts']}", f"{lb}{r['ratio_to_a0']:.3f}",
                    f"{lb}{r['conflicts_per_sorted_triple']:.3f}", f"{lb}{r['conflicts_per_pair']:.3f}"])
pop = [["family", "n", "l", "targets", "SAT", "SAT conflicts median [IQR]", "UNSAT conflicts median [IQR]", "UNSAT min–max"]]
for f in FAM:
    s = summ[f]
    pop.append([f, str(s["n"]), str(s["l"]), str(s["targets"]), str(s["sat"]),
                f"{s['sat_conflicts_median']} [{s['sat_conflicts_quartiles'][0]}, {s['sat_conflicts_quartiles'][1]}]",
                f"{s['unsat_conflicts_median']} [{s['unsat_conflicts_quartiles'][0]}, {s['unsat_conflicts_quartiles'][1]}]",
                f"{s['unsat_conflicts_min_max'][0]}–{s['unsat_conflicts_min_max'][1]}"])
trt = [["features", "trained on", "scored on", "AUC [95% CI]"]] + [[t["group"], "n15l5 (all 3000)", "n17l6 (all 1200)", ci(t["test_auc"], t["test_auc_ci95"])] for t in tr]

tables = {"pop": pop, "l1": l1, "l2": l2, "l3": l3t, "transfer": trt}
md = []
typ = []
for name, t in tables.items():
    md.append(f"<!-- table:{name} -->")
    md.append("| " + " | ".join(t[0]) + " |")
    md.append("|" + "|".join([":--"] * len(t[0])) + "|")
    md += ["| " + " | ".join(r) + " |" for r in t[1:]]
    md.append("")
    cells = ", ".join(f"[{c}]" for r in t for c in r)
    typ.append(f"#let table_{name} = table(columns: {len(t[0])}, stroke: 0.3pt, inset: 3pt, {cells})")
(HERE / "tables.md").write_text("\n".join(md))
(HERE / "tables.typ").write_text("\n".join(typ) + "\n")
print("ok")
