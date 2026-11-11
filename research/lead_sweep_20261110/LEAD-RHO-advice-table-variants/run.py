import json, random, sys, math, statistics
SMOKE = "--smoke" in sys.argv
if SMOKE: N, T, W, REPS, L = 4093, 64, 60, 2, 16
else: N, T, W, REPS, L = 262139, 1024, 2500, 8, 64   # N prime
FRACS = (0.65, 0.75)
CAP = 40 * (N // T)

def make_walk(kind, rng):
    if kind == "group":   # r-adding walk in Z_N (additive group model of E(F_p)); steps jump by s[h(x)]
        s = [rng.randrange(1, N) for _ in range(16)]
        return lambda x: (x + s[((x * 2654435761) >> 7) & 15]) % N
    f = [rng.randrange(N) for _ in range(N)]   # random mapping (positive control: heavy in-degree basins)
    return lambda x: f[x]

def pilot_counts(step, rng):
    cnt = {}
    for _ in range(max(1, T // L)):
        x = rng.randrange(N)
        for _ in range(L):
            x = step(x); cnt[x] = cnt.get(x, 0) + 1
    return cnt

def table_for(arm, step, rng, frac):
    tsel = int(frac * T)
    if arm == "random":
        return set(rng.sample(range(N), T))
    cnt = pilot_counts(step, rng)
    pts = list(cnt)
    if arm == "reselect": # top-visited pilot points
        pts.sort(key=lambda p: -cnt[p]); sel = pts[:tsel]
    else:                 # null: random pilot-visited points (same pilot cost, no selection)
        rng.shuffle(pts); sel = pts[:tsel]
    tab = set(sel)
    while len(tab) < T: tab.add(rng.randrange(N))
    return tab

def mean_steps(step, tab, starts):
    tot = 0
    for x in starts:
        n = 0
        while x not in tab and n < CAP:
            x = step(x); n += 1
        tot += n
    return tot / len(starts)

def run_arm(kind, arm, frac, seed):
    rng = random.Random(seed); step = make_walk(kind, rng)
    tab = table_for(arm, step, rng, frac)
    base = table_for("random", step, random.Random(seed + 7), frac)
    st = random.Random(seed + 13); starts = [st.randrange(N) for _ in range(W)]
    return mean_steps(step, tab, starts), mean_steps(step, base, starts)

def summarize(r):
    m = statistics.mean(r); sd = statistics.stdev(r) if len(r) > 1 else 0.0
    h = 2.365 * sd / math.sqrt(len(r)) if len(r) > 1 else float("inf")
    return {"mean": m, "lo": m - h, "hi": m + h}

out = {"params": {"N": N, "T": T, "walks": W, "reps": REPS, "pilot_walk_len": L, "fracs": FRACS, "cap": CAP, "smoke": SMOKE},
       "seeds": "20261110*1000+rep*10+arm_index"}
res = {}
for kind in ("group", "control_map"):
    for arm in ("aa", "null", "reselect"):
        for frac in (FRACS if arm != "aa" else (0.75,)):
            rs = []
            for rep in range(REPS):
                seed = 20261110000 + rep * 10 + (0 if kind == "group" else 5)
                a, b = run_arm(kind, "random" if arm == "aa" else arm, frac, seed)
                rs.append(a / b)
            res[f"{kind}/{arm}/{frac}"] = summarize(rs)
out["ratio_vs_random_table"] = res
gains = {f: 1 - res[f"group/reselect/{f}"]["mean"] for f in FRACS}
lo_gain = {f: 1 - res[f"group/reselect/{f}"]["hi"] for f in FRACS}
out["gain_group"] = gains
pos = 1 - res["control_map/reselect/0.75"]["hi"]
aa_ok = all(0.9 <= res[f"{k}/aa/0.75"]["lo"] and res[f"{k}/aa/0.75"]["hi"] <= 1.1 or SMOKE for k in ("group", "control_map"))
out["controls"] = {"positive_control_gain_lo": pos, "aa_ok": aa_ok}
if SMOKE: v = "inconclusive"
elif not aa_ok or pos <= 0.03: v = "inconclusive"
elif all(g <= 0.02 for g in gains.values()) and all(lo_gain[f] <= 0.03 for f in FRACS): v = "consistent_with_prediction"
elif any(lo_gain[f] > 0.03 for f in FRACS): v = "falsified"
else: v = "inconclusive"
out["verdict"] = v
print(json.dumps(out))
