"""Layout only: formats recorded values from results/run.json into README/REPORT tables."""
import json, os
here = os.path.dirname(os.path.abspath(__file__))
R = json.load(open(os.path.join(here, "..", "results", "run.json")))
ORD = ["msb", "lsb", "sift_msb", "sift_lsb"]
rows = {(r["n"], r["set"]): r for r in R["rows"]}
cur = {c["n"]: c for c in R["curves"]}
hdr = ["n", "p", "set", "|X|", "nodes msb / lsb / sift_msb / sift_lsb", "ratio to RAND mean (relsd), same order", "INT nodes (4 orders)"]
lines = []
for n in R["header"]["ns"]:
    for s in ("X2", "XE"):
        r = rows[(n, s)]
        nodes = " / ".join(str(r["orders"][o]["nodes"]) for o in ORD)
        rat = " / ".join("%.3f (%.3f)" % (r["vs_rand"][o]["ratio_x_over_mean_rand"], r["vs_rand"][o]["rand_sd"] / r["vs_rand"][o]["rand_mean"]) for o in ORD)
        it = rows[(n, "INT")]
        ints = " / ".join(str(it["orders"][o]["nodes"]) for o in ORD) if s == "X2" else ""
        lines.append([str(n), str(cur[n]["p"]), "X2" if s == "X2" else "X_E", str(r["size"]), nodes, rat, ints])
md = "| " + " | ".join(h.replace("|X|", "\\|X\\|") for h in hdr) + " |\n|" + "---|" * len(hdr) + "\n" + "\n".join("| " + " | ".join(l) + " |" for l in lines) + "\n"
open(os.path.join(here, "table.md"), "w").write(md)
def esc(s): return s.replace("|", "\\|").replace("#", "\\#")
ty = "#table(columns: (auto, auto, auto, auto, 1fr, 1.5fr, auto), align: (right, right, left, right, left, left, left), stroke: 0.4pt, inset: 3pt,\n  " + ", ".join("[*%s*]" % esc(h) for h in hdr) + ",\n"
for l in lines:
    ty += "  " + ", ".join("[%s]" % esc(c) for c in l) + ",\n"
ty += ")\n"
open(os.path.join(here, "table.typ"), "w").write(ty)

rl = ["| n | set | RAND mean nodes msb / lsb / sift_msb / sift_lsb (10 sets) |", "|---|---|---|"]
for n in R["header"]["ns"]:
    for s_ in ("X2", "XE"):
        r = rows[(n, s_)]
        rl.append("| %d | %s | %s |" % (n, "X2" if s_ == "X2" else "X_E", " / ".join("%.1f" % r["vs_rand"][o]["rand_mean"] for o in ORD)))
open(os.path.join(here, "table_rand.md"), "w").write("\n".join(rl) + "\n")
