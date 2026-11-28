#!/usr/bin/env python3
"""Pilot summary: every grid arm against the incumbent (paired, ecbench compare) at w = 0, 0.1, 1, and the model's ranking.
usage: python3 pilot/analyse.py <ecbench> <pilot sessions dir>  (cwd = study dir) -> pilot/pilot_summary.json, pilot/PILOT.md table"""
import json, subprocess, sys, math
sys.path.insert(0, ".")
import rule, make_specs
B, D = sys.argv[1], sys.argv[2]
out = {}
lines = ["| row | arm | model C0/sqrt(r) | S w=0 | ratio vs incumbent w=0 [95% lin-t; boot] | w=0.1 | w=1 |", "|:--|:--|--:|--:|:--|:--|:--|"]
for row in make_specs.ROWS:
    spec = json.load(open(f"pilot/specs/{row}.json")); r = spec["workloads"]["curves"][0]["r"]
    s1 = json.load(open(f"{make_specs.SPLIT}/{row}-r1.json"))
    ki = make_specs.INCUMBENT_K[row]; inc = f"k{ki}n{make_specs.producer_N(s1, ki)}"
    kc, Nc = rule.pick(r); pick = f"k{kc}n{Nc}"
    out[row] = {"incumbent": inc, "rule": pick, "arms": {}}
    for a in spec["arms"][1:]:
        name = a["name"]; k, N = int(name[1]), int(name.split("n")[1])
        e = {"model_c0": rule.c0(N, k, r) / math.sqrt(r)}
        for w in ("0", "0.1", "1"):
            if name == inc:
                e[w] = {"s": None, "ratio": 1.0, "ci": [1.0, 1.0], "boot": [1.0, 1.0]}; continue
            j = json.loads(subprocess.run([B, "compare", "--dir", f"{D}/{row}", "--a", inc, "--b", name, "--weight", w,
                                           "--resamples", "4000", "--seed", "20261118", "--json"], capture_output=True, text=True, check=True).stdout)
            c = j["curves"][0]
            e[w] = {"s": j["b"]["mean_s"], "s_inc": j["a"]["mean_s"], "ratio": c["ratio_b_over_a"], "ci": c["ci95"], "boot": c["ci95_bootstrap"]}
        out[row]["arms"][name] = e
    arms = sorted(out[row]["arms"].items(), key=lambda kv: kv[1]["0"]["ratio"])
    out[row]["empirical_best_w0"] = arms[0][0]
    for name, e in sorted(out[row]["arms"].items(), key=lambda kv: kv[1]["model_c0"]):
        tag = (" **rule**" if name == pick else "") + (" (incumbent)" if name == inc else "")
        f = lambda w: f"{e[w]['ratio']:.3f}"
        lines.append(f"| {row} | {name}{tag} | {e['model_c0']:.2f} | {(e['0']['s'] or out[row]['arms'][next(n for n in out[row]['arms'] if n != inc)]['0']['s_inc']):.3f} | {e['0']['ratio']:.3f} [{e['0']['ci'][0]:.3f}, {e['0']['ci'][1]:.3f}; {e['0']['boot'][0]:.3f}, {e['0']['boot'][1]:.3f}] | {f('0.1')} | {f('1')} |")
json.dump(out, open("pilot/pilot_summary.json", "w"), separators=(",", ":"))
open("pilot/pilot_table.md", "w").write("\n".join(lines) + "\n")
for row, v in out.items():
    p = v["arms"][v["rule"]]
    print(row, "inc", v["incumbent"], "rule", v["rule"], "emp.best", v["empirical_best_w0"], " rule/inc:", *(f"w{w}={p[w]['ratio']:.3f}[{p[w]['ci'][0]:.3f},{p[w]['ci'][1]:.3f}]" for w in ("0", "0.1", "1")))
