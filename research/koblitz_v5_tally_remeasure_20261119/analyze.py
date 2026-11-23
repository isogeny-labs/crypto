#!/usr/bin/env python3
"""Frozen analysis (PROTOCOL.md section 3). Reads the 8 sessions, runs ecbench band/compare, checks the v5 tally on every
record, writes tables/ and results.json. Usage: ECBENCH=<pinned binary> python3 analyze.py [SESS_DIR]"""
import json, os, subprocess, sys, math, statistics, hashlib
HERE = os.path.dirname(os.path.abspath(__file__))
SESS = sys.argv[1] if len(sys.argv) > 1 else "/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/v5t/sessions"
EC = os.environ["ECBENCH"]
T = os.path.join(HERE, "tables"); os.makedirs(T, exist_ok=True)
W = ["0", "0.1", "1"]
NAMES = ["a0-n13", "a1-n17", "a0-n19", "a0-n23", "a1-n23", "a1-n29", "a0-n31", "a0-n37"]
TALLY = "partition_hashes_uncharged"
def ec(*a):
    return subprocess.run([EC, *a], capture_output=True, text=True, check=True).stdout
def cmp(d, a, b, w):
    j = json.loads(ec("compare", "--dir", d, "--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261002", "--json"))
    o = j["ops"]
    his = [x[1] for x in (o.get("ci95"), o.get("ci95_bootstrap")) if x]
    los = [x[0] for x in (o.get("ci95"), o.get("ci95_bootstrap")) if x]
    return {"ratio": o["ratio_b_over_a"], "lo": min(los), "hi": max(his), "pairs": o["pairs"], "status": o.get("status"), "raw": o}
res = {"sessions": {}, "curves": {}}
for name in NAMES:
    d = os.path.join(SESS, name)
    if not os.path.exists(os.path.join(d, "session.json")):
        res["curves"][name] = {"missing": True}; continue
    rec = os.path.join(d, "records.jsonl")
    ses = json.load(open(os.path.join(d, "session.json")))
    res["sessions"][name] = {"records_sha256": hashlib.sha256(open(rec, "rb").read()).hexdigest(), "session": ses.get("session_id"),
                             "binary_sha256": ses.get("binary_sha256")}
    recs = [json.loads(l) for l in open(rec)]
    meas = [r for r in recs if not r.get("warmup")]
    cur = meas[0]["workload"]["curve"]; r_ = cur["r"]; A = cur["automorphisms_available"]
    ideal = math.sqrt(math.pi * r_ / (2 * A))
    # R1: soundness and the tally check on every v5 record (warm-ups included)
    v5 = [r for r in recs if r["arm"].startswith("rho-v5")]
    tally_ok = sum(1 for r in v5 if r["counters"].get(TALLY) == r["counters"].get("steps") and TALLY in r["cost"]["unpriced"])
    unverified = sum(1 for r in meas if r["outcome"]["status"] != "verified" or not r["outcome"].get("matches_target"))
    bj = os.path.join(T, f"band-{name}.json")
    open(os.path.join(T, f"band-{name}.txt"), "w").write(ec("band", "--dir", d, "--ranks", "--json", bj))
    band = {row["arm"]: row for row in json.load(open(bj))["rows"]}
    arms = {}
    for a in band:
        rr = [r for r in meas if r["arm"] == a]
        steps = [r["counters"].get("steps", r["counters"].get("walk_steps")) for r in rr]
        arms[a] = {"runs": band[a]["runs"], "verified": band[a]["verified"], "mean_S": dict(zip(W, band[a]["mean_s"])),
                   "median_steps_over_ideal": statistics.median(steps) / ideal,
                   "mean_setup_gae": statistics.mean(sum(p["gae"] for p in r["phases"] if p["name"] == "setup") for r in rr),
                   "mean_total_gae": statistics.mean(r["cost"]["total_gae"] for r in rr)}
    C = {"slug": cur["slug"], "a": int(name[1]), "n": int(name.split("-n")[1]), "r": r_, "log2_r": math.log2(r_), "A": A,
         "ideal_steps": ideal, "v5_records": len(v5), "v5_tally_equals_steps": tally_ok, "unverified_measured": unverified,
         "arms": arms, "ratios": {}}
    for a, b in [("rho-strong", "rho-v5"), ("rho-strong", "rho-strong-aa"), ("rho-v5", "rho-v5-aa")]:
        C["ratios"][f"{b}/{a}"] = {w: cmp(d, a, b, w) for w in W}
    for w in W:
        q = C["ratios"]["rho-v5/rho-strong"][w]
        q["order"] = "v5 below strong" if q["hi"] < 1 else ("v5 above strong" if q["lo"] > 1 else "unresolved")
    res["curves"][name] = C
# crossover per weight: smallest log2 r with v5/strong >= 1, by linear interpolation of ln(ratio) in log2 r between adjacent measured
# curves (sorted by log2 r); "not reached" if every measured ratio is below 1.
pts = sorted((C["log2_r"], name) for name, C in res["curves"].items() if not C.get("missing"))
cross = {}
for w in W:
    seq = [(x, res["curves"][nm]["ratios"]["rho-v5/rho-strong"][w]["ratio"]) for x, nm in pts]
    hit = None
    for (x0, y0), (x1, y1) in zip(seq, seq[1:]):
        if y0 < 1 <= y1:
            l0, l1 = math.log(y0), math.log(y1); hit = x0 + (0 - l0) * (x1 - x0) / (l1 - l0); break
    if hit is None and seq and seq[0][1] >= 1: hit = seq[0][0]
    cross[w] = {"log2_r": hit, "note": None if hit is not None else f"not reached: v5/strong < 1 at every measured curve up to log2 r {seq[-1][0]:.2f}"}
res["crossover_v5_over_strong_eq_1"] = cross
json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1, default=str)
with open(os.path.join(T, "v5_over_strong.md"), "w") as f:
    f.write("| curve | slug | log2 r | v5/strong w=0 | w=0.1 | w=1 | A/A strong (w=0) | A/A v5 (w=0) | tally==steps |\n|:--|:--|--:|:--|:--|:--|:--|:--|:--|\n")
    for x, nm in pts:
        C = res["curves"][nm]; q = C["ratios"]["rho-v5/rho-strong"]
        f.write(f"| {nm} | `{C['slug']}` | {x:.2f} | " + " | ".join(f"{q[w]['ratio']:.3f} [{q[w]['lo']:.3f}, {q[w]['hi']:.3f}]" for w in W)
                + f" | {C['ratios']['rho-strong-aa/rho-strong']['0']['ratio']:.3f} | {C['ratios']['rho-v5-aa/rho-v5']['0']['ratio']:.3f} | {C['v5_tally_equals_steps']}/{C['v5_records']} |\n")
print("wrote results.json", [nm for _, nm in pts])
