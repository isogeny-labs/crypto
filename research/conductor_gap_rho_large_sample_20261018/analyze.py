#!/usr/bin/env python3
"""Fixed analysis of research/conductor_gap_rho_large_sample_20261018 (PROTOCOL.md, "Analysis").

Reads recorded values only (records.jsonl of each session): cost.s per (arm, curve, target).
Wall time is never read. Writes results/summary.json and results/summary.md.
Ratios: mean S(curve) / mean S(crater of the same pair), bootstrap over targets (4000 draws, seed 1);
paired per-target log-ratio with a normal-approximation 95% interval; pooled ratios per tier.
"""
import json, math, os, random, re, statistics as st, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
SESS = os.path.join(HERE, "sessions")
B = 4000

def roles_of(spec):
    d = spec["description"]
    return [r.strip() for r in d.split("Curve roles, in order:")[1].rstrip(".").split(",")]

def load(name):
    d = os.path.join(SESS, name)
    spec = json.load(open(f"{d}/spec.json"))
    roles = roles_of(spec)
    key = lambda c: json.dumps(c, sort_keys=True)
    role_of = {key(c): roles[i] for i, c in enumerate(spec["workloads"]["curves"])}
    S = defaultdict(dict)       # (arm, role) -> target_index -> s
    status = defaultdict(int)
    for line in open(f"{d}/records.jsonl"):
        r = json.loads(line)
        if r["warmup"]:
            continue
        status[r["outcome"]["status"]] += 1
        if r["outcome"]["status"] != "verified":
            continue
        role = role_of[key(r["workload"]["curve_spec"])]
        S[(r["arm"], role)][r["workload"]["target_index"]] = r["cost"]["s"]
    return spec, roles, S, dict(status)

def boot_ratio(xs, ys, rng):
    out = []
    for _ in range(B):
        a = [rng.choice(xs) for _ in xs]; b = [rng.choice(ys) for _ in ys]
        out.append(st.mean(b) / st.mean(a))
    out.sort()
    return out[int(0.025 * B)], out[int(0.975 * B)]

def ratio_row(base, other, rng):
    """other/base: unpaired bootstrap of means and paired log-ratio on shared targets."""
    xs, ys = list(base.values()), list(other.values())
    r = st.mean(ys) / st.mean(xs)
    lo, hi = boot_ratio(xs, ys, rng)
    shared = sorted(set(base) & set(other))
    lr = [math.log(other[t] / base[t]) for t in shared]
    m = st.mean(lr); se = st.stdev(lr) / math.sqrt(len(lr)) if len(lr) > 1 else float("nan")
    return {"n_base": len(xs), "n_other": len(ys), "mean_base": st.mean(xs), "mean_other": st.mean(ys),
            "ratio": r, "ci95": [lo, hi], "paired_n": len(shared), "paired_geomean_ratio": math.exp(m),
            "paired_ci95": [math.exp(m - 1.96 * se), math.exp(m + 1.96 * se)], "excludes_1": not (lo <= 1 <= hi)}

def main():
    rng = random.Random(1)
    names, pending = [], []
    for n in sorted(os.listdir(SESS)):
        d = os.path.join(SESS, n)
        if not os.path.isfile(os.path.join(d, "session.json")):
            continue
        if json.load(open(os.path.join(d, "session.json"))).get("status") == "complete":
            names.append(n)
        else:
            pending.append(n)
    summary = {"sessions": {}, "pooled": {}, "pending_sessions": pending}
    md = ["# Results (operation counts only; S = charged group operations / sqrt(r))\n"]
    pooled = defaultdict(lambda: {"floor": [], "crater": [], "pairs": []})  # tier -> lists of per-target S
    for name in names:
        try:
            spec, roles, S, status = load(name)
        except FileNotFoundError:
            continue
        family = re.match(r"[a-z]\d+r?", name).group(0)
        tier = "lr" if family.endswith("r") else name[0]   # replication sessions are their own tier
        ses = {"label": spec["label"], "status_counts": status, "arms": {}}
        md.append(f"\n## {name}\n\n{spec['label']}\n\nrecord outcomes: {status}\n")
        arms = sorted({a for a, _ in S})
        for arm in arms:
            rows = {}
            # crater references: role 'crater' or 'pairK-crater'
            for role in roles:
                if role.endswith("crater"):
                    pref = role[:-len("crater")]
                    base = S.get((arm, role))
                    if not base:
                        continue
                    for other in roles:
                        if other == role or not other.startswith(pref) or not S.get((arm, other)):
                            continue
                        rows[f"{other} / {role}"] = ratio_row(base, S[(arm, other)], rng)
                        if arm == "rho-neg":
                            if "twin" in other:
                                orig = other.split("-twin")[0]
                                if S.get((arm, orig)):
                                    kind = "floortwin/floor" if "floor" in orig else "cratertwin/crater"
                                    pooled[tier + ":twin/original"]["pairs"].append((S[(arm, orig)], S[(arm, other)]))
                                    pooled[tier + ":" + kind]["pairs"].append((S[(arm, orig)], S[(arm, other)]))
                                if "floor" in other and S.get((arm, role + "-twin(u=5)")):
                                    pooled[tier + ":floortwin/cratertwin"]["pairs"].append((S[(arm, role + "-twin(u=5)")], S[(arm, other)]))
                            elif other.startswith(pref) and "floor" in other:
                                pooled[tier]["pairs"].append((base, S[(arm, other)]))
                                if tier in ("l", "lr"):
                                    pooled["l+lr"]["pairs"].append((base, S[(arm, other)]))
                                if family != tier:
                                    pooled[family]["pairs"].append((base, S[(arm, other)]))
            # positive control: rho-plain / rho-neg on the crater
            if arm == "rho-plain" and S.get(("rho-neg", "crater")):
                rows["rho-plain / rho-neg (crater, positive control, expect 1.414)"] = ratio_row(S[("rho-neg", "crater")], S[("rho-plain", "crater")], rng)
            if arm == "rho-neg-aa" and S.get(("rho-neg", "crater")):
                rows["rho-neg-aa / rho-neg (crater, A/A control, expect 1)"] = ratio_row(S[("rho-neg", "crater")], S[("rho-neg-aa", "crater")], rng)
            ses["arms"][arm] = rows
            md.append(f"\n### arm `{arm}`\n\n| comparison | n | mean S base | mean S other | ratio | 95% bootstrap | paired geo-mean ratio | paired 95% | excludes 1? |\n|---|---:|---:|---:|---:|---|---:|---|---|")
            for k, v in rows.items():
                md.append(f"| {k} | {v['n_base']}/{v['n_other']} | {v['mean_base']:.4f} | {v['mean_other']:.4f} | {v['ratio']:.4f} | [{v['ci95'][0]:.4f}, {v['ci95'][1]:.4f}] | {v['paired_geomean_ratio']:.4f} | [{v['paired_ci95'][0]:.4f}, {v['paired_ci95'][1]:.4f}] | {'yes' if v['excludes_1'] else 'no'} |")
        summary["sessions"][name] = ses
    # pooled floor/crater per tier (rho-neg): concatenate per-target S of all floors and all craters
    md.append(f"\nSessions not complete and not analysed: {pending}\n")
    md.append("\n## Pooled floor / crater, arm rho-neg, per tier (s = S1+S2, m = M1-M3, l = L frozen, lr = L replication) and per family; ':twin/original' rows pool the isomorphic-twin controls of a tier\n\n| tier/family | pairs (floor, crater) | n floor | n crater | ratio | 95% bootstrap | paired geo-mean | paired 95% | excludes 1? |\n|---|---:|---:|---:|---:|---|---:|---|---|")
    for tier, d in sorted(pooled.items()):
        if not d["pairs"]:
            continue
        xs, ys, lr = [], [], []
        for base, other in d["pairs"]:
            xs += list(base.values()); ys += list(other.values())
            lr += [math.log(other[t] / base[t]) for t in set(base) & set(other)]
        r = st.mean(ys) / st.mean(xs); lo, hi = boot_ratio(xs, ys, rng)
        m = st.mean(lr); se = st.stdev(lr) / math.sqrt(len(lr))
        row = {"pairs": len(d["pairs"]), "n_floor": len(ys), "n_crater": len(xs), "ratio": r, "ci95": [lo, hi],
               "paired_n": len(lr), "paired_geomean_ratio": math.exp(m), "paired_ci95": [math.exp(m - 1.96 * se), math.exp(m + 1.96 * se)]}
        summary["pooled"][tier] = row
        md.append(f"| {tier} | {row['pairs']} | {row['n_floor']} | {row['n_crater']} | {r:.4f} | [{lo:.4f}, {hi:.4f}] | {row['paired_geomean_ratio']:.4f} | [{row['paired_ci95'][0]:.4f}, {row['paired_ci95'][1]:.4f}] | {'yes' if not (lo <= 1 <= hi) else 'no'} |")
    os.makedirs(os.path.join(HERE, "results"), exist_ok=True)
    json.dump(summary, open(os.path.join(HERE, "results", "summary.json"), "w"), indent=1)
    open(os.path.join(HERE, "results", "summary.md"), "w").write("\n".join(md) + "\n")
    print("\n".join(md))

if __name__ == "__main__":
    main()
