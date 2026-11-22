#!/usr/bin/env python3
"""Tie the producer's instances and recovered logarithms to the validator's arithmetic.
For every dumped instance: f, G, T, t equal the validator's derivation (out/certs.json, jac.py).
For every dumped run: recovered == t, [recovered]G == T in jac.py (NAF chain), and the counted fields
equal the committed row at COMMIT (IC: group_ops rel_mul rel_add la_mul la_add trials relations;
Q: group_ops steps), and certified/ok equal the committed row's.
Usage: verify_dump.py REPO CERTS_JSON OUT_JSON DUMP.jsonl... [--commit C]"""
import gzip, json, os, subprocess, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from jac import Jac, random_f

args = sys.argv[1:]
COMMIT = "6213ecbc0da0ed4917ab75805258201d6629b477"
if "--commit" in args:
    i = args.index("--commit"); COMMIT = args[i + 1]; del args[i:i + 2]
REPO, CERTS, OUT, dumps = args[0], args[1], args[2], args[3:]
D = "research/genus4_followup_20261118"


def blob(path):
    raw = subprocess.run(["git", "-C", REPO, "show", f"{COMMIT}:{path}"], check=True, capture_output=True).stdout
    txt = gzip.decompress(raw).decode() if path.endswith(".gz") else raw.decode()
    return [json.loads(l) for l in txt.splitlines() if l.strip()]


tracked = subprocess.run(["git", "-C", REPO, "ls-tree", "-r", "--name-only", COMMIT], check=True,
                         capture_output=True).stdout.decode().splitlines()
committed = blob(f"{D}/b_wx.jsonl.gz") + blob(f"{D}/b_q.jsonl.gz") + blob(f"{D}/b_q.jsonl.r1.gz") + \
    [r for f in sorted(t for t in tracked if t.startswith("research/genus4_lean_rho_20261116/p16411") and t.endswith(".jsonl")) for r in blob(f)]
LBL = {"w": "C_double_lp+pre+coord+unitT", "x": "C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge(4,1.5)", "Q": "R_rho_plain_nowin"}
inst = {r["curve_seed"]: r for r in committed if r["kind"] == "instance" and r["p"] == 16411}
rowof = {}
for r in committed:
    if r["kind"] == "run" and r["p"] == 16411:
        rowof[(r["arm"], r["curve_seed"], r["target_seed"], r["seed"])] = r
cert = {(t["curve_seed"], t["target_seed"]): t for t in json.load(open(CERTS))["targets"]}
J = {}
out = {"instances": [], "runs": [], "commit": COMMIT}
for fn in dumps:
    for line in open(fn):
        if not line.strip():
            continue
        d = json.loads(line)
        cs, ts = d["curve_seed"], d["target_seed"]
        c = cert[(cs, ts)]
        if cs not in J:
            J[cs] = Jac(16411, random_f(16411, 4, cs))
        jac = J[cs]
        G = (tuple(c["G"][0]), tuple(c["G"][1]))
        T = (tuple(c["T"][0]), tuple(c["T"][1]))
        if d["kind"] == "instance":
            dg = (tuple(d["G"][0]), tuple(d["G"][1]))
            dt = (tuple(d["T"][0]), tuple(d["T"][1]))
            chk = {"f": d["f"] == random_f(16411, 4, cs), "G": dg == G, "T": dt == T, "t": d["t"] == c["t"],
                   "order_h_r": (d["order"], d["h"], d["r"]) == (inst[cs]["order"], inst[cs]["h"], inst[cs]["r"])}
            out["instances"].append({"curve_seed": cs, "target_seed": ts, "checks": chk, "pass": all(chk.values())})
        else:
            ref = rowof[(LBL[d["arm"]], cs, ts, d["seed"])]
            rec = d["recovered"]
            fields = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"] if d["arm"] in "wx" else ["group_ops", "steps"]
            chk = {"recovered_eq_t": rec == c["t"], "recovered_G_eq_T": rec is not None and jac.mul(G, rec) == T,
                   "counts_eq_committed_row": all(d[f] == ref[f] for f in fields),
                   "certified_ok_eq_row": (d["certified"], d["ok"]) == (ref["certified"], ref["ok"]) == (True, True)}
            out["runs"].append({"arm": d["arm"], "curve_seed": cs, "target_seed": ts, "seed": d["seed"], "recovered": rec,
                                "fields_compared": fields, "checks": chk, "pass": all(chk.values())})
out["n_instances"], out["n_runs"] = len(out["instances"]), len(out["runs"])
out["instances_pass"] = sum(x["pass"] for x in out["instances"])
out["runs_pass"] = sum(x["pass"] for x in out["runs"])
out["runs_by_arm"] = {a: sum(x["arm"] == a for x in out["runs"]) for a in "wxQ"}
out["all_pass"] = out["instances_pass"] == out["n_instances"] and out["runs_pass"] == out["n_runs"]
json.dump(out, open(OUT, "w"), indent=1)
print({k: v for k, v in out.items() if k not in ("instances", "runs")})
for x in out["instances"] + out["runs"]:
    if not x["pass"]:
        print("FAIL", x)
