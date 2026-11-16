# Validator's independent recomputation of the holdout tables and the section-5a ranking.
import json, sys, re
import numpy as np
from collections import defaultdict
EV = sys.argv[1]
rows = [json.loads(l) for l in open(f"{EV}/results/holdout/rows.jsonl")]
INST = "/home/user/crypto/research/ic_e222_m221_20261106/results/instances_vsrho"
CI, HP, ARMS = (3, 10, 30), (0, 0.1, 1), ("rho", "rho2", "bsgs2")
# --- design checks
cells_expected = {(22,1024),(24,1024),(24,4096),(24,16384),(25,4096),(25,16384)}
cfg = defaultdict(lambda: defaultdict(dict))
probs = []
for r in rows:
    k = (r["pb"], r["T"], r["idx"], r["seed"])
    inst = json.load(open(f"{INST}/W{r['pb']}-{r['idx']}.json"))
    if int(inst["subgroup_order"]) != r["n"] or int(inst["field"]["modulus"]) != r["p"]: probs.append(("inst", k))
    exp_argv = [r["arm"], f"/home/user/crypto/research/mt_fieldops_20261107/../ic_e222_m221_20261106/results/instances_vsrho/W{r['pb']}-{r['idx']}.json", str(r["T"]), str(r["seed"]), str(r["param"])] + ([str(r["g"])] if r["arm"]=="ic4" else [])
    if r["argv"] != exp_argv: probs.append(("argv", k, r["argv"]))
    if r["targets"] != r["T"] or r["ok"] != r["targets"]: probs.append(("ok", k))
    if r["arm"] == "ic4" and (r["param"], r["g"]) != (1.0, 2.0): probs.append(("frozen", k))
    if r["param"] in cfg[k][r["arm"]]: probs.append(("dup", k, r["arm"], r["param"]))
    cfg[k][r["arm"]][r["param"]] = r
print("rows", len(rows), "configs", len(cfg), "problems", probs[:5], len(probs))
assert {k[:2] for k in cfg} == cells_expected
assert all(sorted(k[2] for k in cfg if k[:2]==c) == sorted(list(range(1,12))*2) for c in cells_expected)
assert all(set(v["ic4"])=={1.0} and set(v["rho"])=={0.5,0.7,1.0} and set(v["rho2"])=={0.5,0.7,1.0} and set(v["bsgs2"])=={0.7,1.0,1.4} for v in cfg.values())
def gae(r, ci, h): return ((ci*r["I"] + r["M"] + r["S"])/(ci + 3) + h*(r["H"] + r["L"])) / r["targets"]
def bootmed(x, B, seed):
    rng = np.random.default_rng(seed); x = np.asarray(x)
    m = np.median(x[rng.integers(0, len(x), (B, len(x)))], axis=1)
    return np.quantile(m, 0.025), np.quantile(m, 0.975)
mine = {}
for ci in CI:
    for c in sorted(cells_expected):
        ks = sorted(k for k in cfg if k[:2]==c)
        for h in HP:
            ic = np.array([gae(cfg[k]["ic4"][1.0], ci, h) for k in ks])
            for a in ARMS:
                ref = np.array([min(gae(r, ci, h) for r in cfg[k][a].values()) for k in ks])
                rat = ic/ref
                bs = [bootmed(rat, 20000, s) for s in (11, 22, 33)]
                mine[(ci, c, h, a)] = dict(med=float(np.median(rat)), lo=min(b[0] for b in bs), hi=max(b[1] for b in bs),
                    lo1=bs[0][0], hi1=bs[0][1], wins=int((rat<1).sum()), icmed=float(np.median(ic)), refmed=float(np.median(ref)))
# --- compare with producer summary.json
summ = json.load(open(f"{EV}/results/holdout/summary.json"))
dmed = dic = 0; dbound = []
for s in summ:
    m = mine[(s["cI"], (s["bits"], s["T"]), s["h"], s["vs"])]
    dmed = max(dmed, abs(m["med"]-s["median"])); dic = max(dic, abs(m["icmed"]-s["ic"]), abs(m["refmed"]-s["ref"]))
    if m["wins"] != s["wins"]: print("WINS DIFF", s)
    dbound.append(max(abs(m["lo1"]-s["lo"]), abs(m["hi1"]-s["hi"])))
print("summary entries", len(summ), "max |median diff|", dmed, "max |GAE median diff|", dic, "max |bound diff| (my seed 11 vs theirs)", max(dbound))
# --- compare with analysis.txt printed table (rounded)
txt = open(f"{EV}/results/holdout/analysis.txt").read()
nchk = nbad = 0; ci = None
for line in txt.splitlines():
    mm = re.match(r"## I = (\d+) M", line)
    if mm: ci = int(mm.group(1)); continue
    if line.startswith("| 2"):
        f = [x.strip() for x in line.strip("|").split("|")]
        c = (int(f[0]), int(f[1])); h = float(f[2]); h = 0 if h == 0 else (1 if h == 1 else 0.1)
        exp = [f"{mine[(ci,c,h,'rho')]['icmed']:.1f}"] + [f"{mine[(ci,c,h,a)]['refmed']:.1f}" for a in ARMS] + [f"{mine[(ci,c,h,a)]['med']:.3f}" for a in ARMS]
        got = f[3:7] + [x.split()[0] for x in f[7:10]]
        nchk += 1
        if exp != got: nbad += 1; print("TABLE DIFF", ci, c, h, exp, got)
print("analysis.txt table lines checked", nchk, "differing", nbad)
# --- section 5a ranking with my bounds (union over 3 bootstrap seeds = conservative)
def rank(ci, c, a, key_lo="lo", key_hi="hi"):
    ss = [mine[(ci, c, h, a)] for h in HP]
    if all(s[key_hi] < 1 for s in ss): return "cheaper"
    if all(s[key_lo] > 1 for s in ss): return "dearer"
    return "unranked"
for a in ARMS:
    for ci in CI:
        print(f"5a I={ci}M vs {a}: " + "; ".join(f"{c[0]}b/T{c[1]}: {rank(ci,c,a)}" + ("" if rank(ci,c,a)==rank(ci,c,a,'lo1','hi1') else "(seed-sensitive)") for c in sorted(cells_expected)))
# borderline intervals
print("intervals within 0.01 of 1:")
for k, m in sorted(mine.items()):
    if m["lo"] < 1 < m["hi"] or abs(m["hi"]-1) < 0.01 or abs(m["lo"]-1) < 0.01: print("  ", k, f"{m['med']:.3f} [{m['lo']:.3f}, {m['hi']:.3f}]")
# ranges quoted in RESULTS.md
def rng(a, hs, cis=CI):
    v = [mine[(ci, c, h, a)]["med"] for ci in cis for c in cells_expected for h in hs]; return round(min(v),3), round(max(v),3)
for a in ARMS:
    print(a, "median ratio range h=0:", rng(a,(0,)), "h<=0.1:", rng(a,(0,0.1)), "all h:", rng(a,HP))
json.dump({repr(k): v for k, v in mine.items()}, open(sys.argv[2], "w"), indent=0)
