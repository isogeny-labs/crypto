import json, math, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from genus_table import genus_row
from split_law import splitfrac, mc_count, MC_TRIALS
rows = [json.loads(l) for l in open("raw/split_law.jsonl")]
ok = True
def rep(name, v):
    global ok
    ok &= v
    print("GATE", name, "PASS" if v else "FAIL")
rep("1_TABLE", all(genus_row(g)["h"] == g+1 and genus_row(g)["genus_D"] == 2*g+1 and genus_row(g)["dim_P"] == g
                   and genus_row(g)["dominant"] == ("true" if g <= 5 else "false") for g in range(1, 7)))
mc = {(r["d"], r["seed"]): r for r in rows if r["kind"] == "mc"}
rep("2_POS_d1", all(mc[(1, s)]["frac_mc_dec"] == 1.0 for s in (20261116, 70261116)))
p2 = float(splitfrac(101, 2)); T = MC_TRIALS
sig = math.sqrt(p2*(1-p2)/T)
for s in (20261116, 70261116):
    z = (mc[(2, s)]["frac_mc_dec"]-p2)/sig
    print("  d=2 seed", s, "z=%.3f" % z)
rep("2_POS_d2_3sigma", all(abs(mc[(2, s)]["frac_mc_dec"]-p2) <= 3*sig for s in (20261116, 70261116)))
aa = True
for d in (1, 2, 3):
    f = float(splitfrac(101, d)); tol = 4*math.sqrt(f*(1-f)/T)
    gap = abs(mc[(d, 20261116)]["frac_mc_dec"]-mc[(d, 70261116)]["frac_mc_dec"])
    print("  AA d=%d gap=%.6f tol=%.6f" % (d, gap, tol)); aa &= gap <= tol
rep("3_AA", aa)
mono = all(all(splitfrac(p, d) > splitfrac(p, e) for d, e in zip(range(1, 11), range(2, 12))) for p in (101, 1009, 10007))
rep("4_MONOTONE", mono)
det = all(mc_count(101, d, 20261116, 20000) == mc_count(101, d, 20261116, 20000) for d in (1, 2, 3))
# also reproduce frozen-cell exact count for d=2
det &= mc_count(101, 2, 20261116, MC_TRIALS) == mc[(2, 20261116)]["n_complete"]
rep("5_DETERMINISM", det)
print("ALL", "PASS" if ok else "FAIL")
