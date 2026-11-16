#!/usr/bin/env python3
"""VALIDATOR check of every number in RESULTS.md's table and prose ranges, noverify.txt, and the winners counts
against the validator's recompute (recompute.json).  usage: results_check.py <frozen study dir> <recompute.json> <out.json>"""
import json, sys, os, re
FZ, RC, OUT = sys.argv[1:4]
rc = json.load(open(RC)); txt = open(os.path.join(FZ, "RESULTS.md")).read()
checks = []
def chk(name, claimed, recomputed, ndp):
    ok = round(recomputed, ndp) == round(claimed, ndp) or abs(recomputed - claimed) < 0.5 * 10 ** -ndp + 1e-12
    checks.append(dict(name=name, claimed=claimed, recomputed=recomputed, decimals=ndp, ok=bool(ok)))
# table rows: | 24 | 1024 | 1.146 [1.138, 1.152] | 1.054 [...] | 0.748 [...] | 1.282 | 158.0, 149.4 |
tab = {(t["bits"], t["T"]): t for t in rc["results_table"]}
summ = {(s["bits"], s["T"], s["h"], s["vs"]): s for s in rc["summary_5a"] if s["cand"] == "icbest" and s["cI"] == 10}
for m in re.finditer(r"^\| (\d+) \| (\d+) \| ([\d.]+) \[([\d.]+), ([\d.]+)\] \| ([\d.]+) \[([\d.]+), ([\d.]+)\] \| ([\d.]+) \[([\d.]+), ([\d.]+)\] \| ([\d.]+) \| ([\d.]+), ([\d.]+) \|$", txt, re.M):
    b, T = int(m.group(1)), int(m.group(2)); v = [float(x) for x in m.groups()[2:]]
    for j, h in enumerate((0, 0.1, 1)):
        s = summ[(b, T, h, "rhobest")]
        for k, f in enumerate(("median", "lo", "hi")): chk(f"table {b}/{T} h={h} {f}", v[3 * j + k], s[f], 3)
    chk(f"table {b}/{T} icbest/bsgs2 h=0", v[9], summ[(b, T, 0, "bsgs2")]["median"], 3)
    chk(f"table {b}/{T} GAE/t icbest h=0.1", v[10], summ[(b, T, 0.1, "rhobest")]["ic"], 1)
    chk(f"table {b}/{T} GAE/t rhobest h=0.1", v[11], summ[(b, T, 0.1, "rhobest")]["ref"], 1)
r5, r4 = rc["ranges_5a"], rc["ranges_v4"]
for h, (a, b) in (("0", (1.06, 1.16)), ("0.1", (1.00, 1.05)), ("1", (0.71, 0.80))):
    lo, hi = r5[f"icbest/rhobest I=10M h={float(h) if h != '0' and h != '1' else int(h)}"]
    chk(f"I=10M h={h} min", a, lo, 2); chk(f"I=10M h={h} max", b, hi, 2)
chk("v4 rule I=10M min (all h)", 1.05, r4["icbest/rhobest I=10M all h"][0], 2)
chk("v4 rule I=10M max (all h)", 2.02, r4["icbest/rhobest I=10M all h"][1], 2)
chk("icbest/bsgs2 min (all cI, h)", 1.11, r5["icbest/bsgs2 all"][0], 2)
chk("icbest/bsgs2 max (all cI, h)", 1.47, r5["icbest/bsgs2 all"][1], 2)
nv = rc["noverify_ranges"]
for h, (a, b) in (("0", (1.12, 1.21)), ("0.1", (1.00, 1.07)), ("1", (0.71, 0.73))):
    chk(f"noverify I=10M h={h} min", a, nv[h][0], 2); chk(f"noverify I=10M h={h} max", b, nv[h][1], 2)
# noverify.txt
nvt = open(os.path.join(FZ, "results/holdout/noverify.txt")).read()
for m in re.finditer(r"^\| (\d+) \| (\d+) \| ([\d.]+) / ([\d.]+) / ([\d.]+) \|$", nvt, re.M):
    key = f"{m.group(1)}/{m.group(2)}"
    for j in range(3): chk(f"noverify.txt {key} h#{j}", float(m.group(3 + j)), rc["noverify_I10M"][key][j], 3)
# prose counts
w = rc["winners_I10M_5a"]
def cnt(h, part, key): return w[h][part].get(key, 0)
chk("h=0 lo>1 cells (dearer in all 9)", 9, r5["icbest/rhobest I=10M h=0 lo>1 cells"], 0)
chk("h=0.1 lo>1 cells (dearer in 8 of 9)", 8, r5["icbest/rhobest I=10M h=0.1 lo>1 cells"], 0)
chk("h=1 hi<1 cells (cheaper in all 9)", 9, r5["icbest/rhobest I=10M h=1 hi<1 cells"], 0)
chk("rho2xks wins h=0 (198)", 198, cnt("0", "rho_arm", "rho2xks"), 0)
chk("rho2xks wins h=0.1 (198)", 198, cnt("0.1", "rho_arm", "rho2xks"), 0)
chk("rho2xks wins h=1 (198: 'in all 198 configurations; rhodpu never')", 198, cnt("1", "rho_arm", "rho2xks"), 0)
for h, pairs in (("0", ((0.05, 126), (0.1, 69))), ("0.1", ((0.1, 111), (0.05, 63))), ("1", ((0.25, 147), (0.5, 36)))):
    for kp, c in pairs: chk(f"best kappa h={h} kappa={kp}", c, cnt(h, "rho2xks_kappa", str(kp)), 0)
chk("ic5rw wins h=0 (198)", 198, cnt("0", "ic_arm", "ic5rw"), 0)
chk("ic5rw wins h=0.1 (197)", 197, cnt("0.1", "ic_arm", "ic5rw"), 0)
chk("ic5w wins h=1 (100)", 100, cnt("1", "ic_arm", "ic5w"), 0)
chk("ic5 wins h=1 (98)", 98, cnt("1", "ic_arm", "ic5"), 0)
bad = [c for c in checks if not c["ok"]]
json.dump(dict(checks=len(checks), mismatches=len(bad), mismatching=bad, all=checks), open(OUT, "w"), indent=1)
print(len(checks), "checks;", len(bad), "mismatches"); [print(" ", b) for b in bad]
